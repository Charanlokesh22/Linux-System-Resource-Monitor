use crate::server::proto::SystemMetrics;

pub struct AlertSink {
    webhook: Option<String>,
    client: reqwest::Client,
}

impl AlertSink {
    pub fn new(webhook: Option<String>) -> Self {
        Self { webhook, client: reqwest::Client::new() }
    }

    pub fn threshold_alerts(&self, m: &SystemMetrics, cpu_thresh: f64, mem_thresh: f64, disk_thresh: f64) -> Vec<String> {
        let mut v = vec![];
        if m.cpu_percent > cpu_thresh {
            v.push(format!("CPU threshold exceeded: {:.2}% > {:.2}%", m.cpu_percent, cpu_thresh));
        }
        let mem_pct = if m.mem_total_bytes == 0 { 0.0 } else { m.mem_used_bytes as f64 * 100.0 / m.mem_total_bytes as f64 };
        if mem_pct > mem_thresh {
            v.push(format!("Memory threshold exceeded: {:.2}% > {:.2}%", mem_pct, mem_thresh));
        }
        let disk_pct = if m.disk_total_bytes == 0 { 0.0 } else { m.disk_used_bytes as f64 * 100.0 / m.disk_total_bytes as f64 };
        if disk_pct > disk_thresh {
            v.push(format!("Disk threshold exceeded: {:.2}% > {:.2}%", disk_pct, disk_thresh));
        }
        v
    }

    pub async fn maybe_send(&self, m: &SystemMetrics) -> Result<(), reqwest::Error> {
        if self.webhook.is_none() || m.alerts.is_empty() { return Ok(()); }
        let url = self.webhook.as_ref().unwrap();
        let body = serde_json::json!({
            "timestamp_ms": m.timestamp_unix_ms,
            "cpu_percent": m.cpu_percent,
            "mem_used_bytes": m.mem_used_bytes,
            "mem_total_bytes": m.mem_total_bytes,
            "disk_used_bytes": m.disk_used_bytes,
            "disk_total_bytes": m.disk_total_bytes,
            "alerts": m.alerts,
        });
        let _ = self.client.post(url).json(&body).send().await?;
        Ok(())
    }
}
