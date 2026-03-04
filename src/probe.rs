// async NTP probing logic

use crate::targets::Target;
use rsntp::AsyncSntpClient;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::time::{Duration, timeout};

#[derive(Debug, Clone)]
pub struct ProbeResult {
    pub target_index: usize,
    pub latency_ms: f64,
    pub success: bool,
    pub error: Option<String>,
}

pub async fn probe_all(targets: &[Target], timeout_ms: u64) -> Vec<ProbeResult> {
    let mut handles = Vec::new();

    for (i, target) in targets.iter().enumerate() {
        let host = target.host;
        let t_ms = timeout_ms;
        handles.push(tokio::spawn(async move {
            let start = Instant::now();
            let duration = Duration::from_millis(t_ms);
            match timeout(duration, AsyncSntpClient::new().synchronize(host)).await {
                Ok(Ok(_)) => ProbeResult {
                    target_index: i,
                    latency_ms: start.elapsed().as_secs_f64() * 1000.0,
                    success: true,
                    error: None,
                },
                Ok(Err(e)) => ProbeResult {
                    target_index: i,
                    latency_ms: start.elapsed().as_secs_f64() * 1000.0,
                    success: false,
                    error: Some(e.to_string()),
                },
                Err(_) => ProbeResult {
                    target_index: i,
                    latency_ms: t_ms as f64,
                    success: false,
                    error: Some("TIMEOUT".into()),
                },
            }
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(r) = handle.await {
            results.push(r);
        }
    }
    results
}

pub fn spawn_probe_loop(
    targets: &'static [Target],
    tx: mpsc::UnboundedSender<Vec<ProbeResult>>,
    timeout_ms: u64,
    interval_secs: u64,
) {
    tokio::spawn(async move {
        loop {
            let results = probe_all(targets, timeout_ms).await;
            if tx.send(results).is_err() {
                break;
            }
            tokio::time::sleep(Duration::from_secs(interval_secs)).await;
        }
    });
}
