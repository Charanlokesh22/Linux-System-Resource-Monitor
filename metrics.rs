use crate::server::proto::{ProcessInfo, SystemMetrics};
use anyhow::Result;
use sysinfo::{System, SystemExt, CpuExt, ProcessExt, Disks, DisksExt};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy)]
pub struct TopN(pub usize);

pub async fn gather_metrics(topn: TopN) -> Result<SystemMetrics> {
    tokio::task::spawn_blocking(move || collect(topn)).await?
}

fn collect(topn: TopN) -> Result<SystemMetrics> {
    let mut sys = System::new_all();
    // Refresh CPU twice to get non-zero usage (sysinfo behavior)
    sys.refresh_all();
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu();

    let cpu_percent = sys.global_cpu_info().cpu_usage() as f64;

    sys.refresh_memory();
    let mem_total_bytes = sys.total_memory() * 1024; // KiB -> bytes
    let mem_used_bytes = sys.used_memory() * 1024;

    let disks = Disks::new_with_refreshed_list();
    let mut disk_total = 0u64;
    let mut disk_used = 0u64;
    for d in disks.iter() {
        let total = d.total_space();
        let avail = d.available_space();
        disk_total += total;
        disk_used += total.saturating_sub(avail);
    }

    sys.refresh_processes();
    let mut procs: Vec<_> = sys.processes().iter().map(|(pid, p)| {
        ProcessInfo {
            pid: pid.as_u32(),
            name: p.name().to_string(),
            cpu_percent: p.cpu_usage() as f64,
            mem_bytes: p.memory() * 1024, // KiB -> bytes
        }
    }).collect();

    procs.sort_by(|a, b| b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap());
    procs.truncate(topn.0);

    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;

    Ok(SystemMetrics {
        cpu_percent,
        mem_total_bytes,
        mem_used_bytes,
        disk_total_bytes: disk_total,
        disk_used_bytes: disk_used,
        timestamp_unix_ms: ts,
        top_processes: procs,
        alerts: vec![],
    })
}
