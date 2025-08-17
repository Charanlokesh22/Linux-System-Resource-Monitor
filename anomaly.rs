use crate::server::proto::SystemMetrics;
use std::collections::VecDeque;

pub struct AnomalyDetector {
    window: usize,
    cpu_hist: VecDeque<f64>,
    mem_hist: VecDeque<f64>, // percent used
}

impl AnomalyDetector {
    pub fn new(window: usize) -> Self {
        Self { window, cpu_hist: VecDeque::new(), mem_hist: VecDeque::new() }
    }

    pub fn check(&mut self, m: &SystemMetrics) -> Vec<String> {
        let mem_pct = if m.mem_total_bytes == 0 { 0.0 } else { m.mem_used_bytes as f64 * 100.0 / m.mem_total_bytes as f64 };

        self.push(&mut self.cpu_hist, m.cpu_percent);
        self.push(&mut self.mem_hist, mem_pct);

        let mut alerts = vec![];
        if let Some((mean, std)) = stats(&self.cpu_hist) {
            if std > 0.0 && m.cpu_percent > mean + 3.0 * std {
                alerts.push(format!("CPU anomaly: {:.2}% > mean+3σ ({:.2}+{:.2})", m.cpu_percent, mean, 3.0*std));
            }
        }
        if let Some((mean, std)) = stats(&self.mem_hist) {
            if std > 0.0 && mem_pct > mean + 3.0 * std {
                alerts.push(format!("Memory anomaly: {:.2}% > mean+3σ ({:.2}+{:.2})", mem_pct, mean, 3.0*std));
            }
        }
        alerts
    }

    fn push(&self, q: &mut VecDeque<f64>, v: f64) {
        q.push_back(v);
        if q.len() > self.window { q.pop_front(); }
    }
}

fn stats(q: &VecDeque<f64>) -> Option<(f64, f64)> {
    if q.len() < 5 { return None; }
    let n = q.len() as f64;
    let mean = q.iter().sum::<f64>() / n;
    let var = q.iter().map(|x| (x-mean)*(x-mean)).sum::<f64>() / n.max(1.0);
    Some((mean, var.sqrt()))
}
