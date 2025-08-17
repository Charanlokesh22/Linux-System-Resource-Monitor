use crate::server::proto::{monitor_client::MonitorClient, Empty, SystemMetrics};
use anyhow::Result;

pub async fn run(addr: String, stream: bool, json: bool) -> Result<()> {
    let mut client = MonitorClient::connect(addr).await?;
    if stream {
        let mut s = client.stream(Empty {}).await?.into_inner();
        while let Some(m) = s.message().await? {
            print_metrics(m, json);
        }
    } else {
        let m = client.get_once(Empty {}).await?.into_inner();
        print_metrics(m, json);
    }
    Ok(())
}

fn print_metrics(m: SystemMetrics, json: bool) {
    if json {
        println!("{}", serde_json::to_string(&m).unwrap());
    } else {
        println!("--- System Metrics @ {} ---", m.timestamp_unix_ms);
        println!("CPU: {:.2}%", m.cpu_percent);
        let mem_used_gb = m.mem_used_bytes as f64 / 1e9;
        let mem_tot_gb = m.mem_total_bytes as f64 / 1e9;
        println!("Memory: {:.2} / {:.2} GB", mem_used_gb, mem_tot_gb);
        let disk_used_gb = m.disk_used_bytes as f64 / 1e9;
        let disk_tot_gb = m.disk_total_bytes as f64 / 1e9;
        println!("Disk: {:.2} / {:.2} GB", disk_used_gb, disk_tot_gb);
        println!("Top processes:");
        for p in m.top_processes.iter() {
            println!("  {} (pid {}) CPU:{:.2}% MEM:{} MB", p.name, p.pid, p.cpu_percent, (p.mem_bytes as f64 / 1e6) as i64);
        }
        if !m.alerts.is_empty() {
            println!("ALERTS:");
            for a in m.alerts {
                println!("  - {a}");
            }
        }
    }
}
