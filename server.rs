use crate::alerts::AlertSink;
use crate::anomaly::AnomalyDetector;
use crate::metrics::{gather_metrics, TopN};
use anyhow::Result;
use std::{pin::Pin, sync::Arc, time::Duration};
use tokio::{sync::broadcast, time::interval};
use tokio_stream::{wrappers::BroadcastStream, Stream};
use tonic::{transport::Server, Request, Response, Status};

pub mod proto {
    tonic::include_proto!("lsrm");
}
use proto::{monitor_server::{Monitor, MonitorServer}, Empty, SystemMetrics};

struct MonitorSvc {
    tx: broadcast::Sender<SystemMetrics>,
    latest: Arc<parking_lot::RwLock<SystemMetrics>>,
}

#[tonic::async_trait]
impl Monitor for MonitorSvc {
    async fn get_once(&self, _req: Request<Empty>) -> Result<Response<SystemMetrics>, Status> {
        let m = self.latest.read().clone();
        Ok(Response::new(m))
    }

    type StreamStream = Pin<Box<dyn Stream<Item = Result<SystemMetrics, Status>> + Send + 'static>>;

    async fn stream(&self, _req: Request<Empty>) -> Result<Response<Self::StreamStream>, Status> {
        let rx = self.tx.subscribe();
        let out = BroadcastStream::new(rx).filter_map(|i| async move {
            match i {
                Ok(val) => Some(Ok(val)),
                Err(_) => None,
            }
        });
        Ok(Response::new(Box::pin(out)))
    }
}

pub async fn run(addr: String, interval_ms: u64, top_n: usize) -> Result<()> {
    let (tx, _rx) = broadcast::channel::<SystemMetrics>(64);
    let latest = Arc::new(parking_lot::RwLock::new(SystemMetrics::default()));
    let svc = MonitorSvc { tx: tx.clone(), latest: latest.clone() };

    // Config from env (thresholds & webhook)
    let webhook = std::env::var("ALERT_WEBHOOK_URL").ok();
    let cpu_thresh: f64 = std::env::var("CPU_THRESHOLD_PCT").ok().and_then(|v| v.parse().ok()).unwrap_or(90.0);
    let mem_thresh: f64 = std::env::var("MEM_THRESHOLD_PCT").ok().and_then(|v| v.parse().ok()).unwrap_or(90.0);
    let disk_thresh: f64 = std::env::var("DISK_THRESHOLD_PCT").ok().and_then(|v| v.parse().ok()).unwrap_or(90.0);

    let alert_sink = AlertSink::new(webhook);
    let mut detector = AnomalyDetector::new(60); // sliding window of last 60 samples

    // Sampler task
    let tx_clone = tx.clone();
    let latest_clone = latest.clone();
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_millis(interval_ms));
        loop {
            ticker.tick().await;
            match gather_metrics(TopN(top_n)).await {
                Ok(mut m) => {
                    // Anomaly detection + threshold alerts
                    let mut alerts = vec![];
                    alerts.extend(detector.check(&m));
                    alerts.extend(alert_sink.threshold_alerts(&m, cpu_thresh, mem_thresh, disk_thresh));
                    m.alerts = alerts;

                    // publish
                    *latest_clone.write() = m.clone();
                    let _ = tx_clone.send(m.clone());
                    let _ = alert_sink.maybe_send(&m).await;
                }
                Err(e) => {
                    eprintln!("sampling error: {e:?}");
                }
            }
        }
    });

    let addr = addr.parse().unwrap();
    println!("➡️  gRPC server listening on {addr}");
    Server::builder()
        .add_service(MonitorServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
