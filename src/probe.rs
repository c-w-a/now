// async probing logic (NTP + HTTP HEAD + DNS Query)

use crate::targets::{ProbeMethod, Target};
use rsntp::AsyncSntpClient;
use std::time::Instant;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::time::{Duration, timeout};

#[derive(Debug, Clone)]
pub struct ProbeResult {
    pub target_index: usize,
    pub latency_ms: f64,
    pub success: bool,
    pub error: Option<String>,
}

// ". IN NS" query: header(12) + root label(1) + qtype NS(2) + qclass IN(2) = 17 bytes
const DNS_QUERY: [u8; 17] = [
    0xAB, 0xCD, // ID
    0x01, 0x00, // flags: RD=1
    0x00, 0x01, // QDCOUNT=1
    0x00, 0x00, // ANCOUNT=0
    0x00, 0x00, // NSCOUNT=0
    0x00, 0x00, // ARCOUNT=0
    0x00,       // root label (length 0)
    0x00, 0x02, // QTYPE=NS
    0x00, 0x01, // QCLASS=IN
];

pub async fn probe_all(targets: &[Target], timeout_ms: u64) -> Vec<ProbeResult> {
    let mut handles = Vec::new();

    for (i, target) in targets.iter().enumerate() {
        let host = target.host;
        let method = target.probe_method;
        let t_ms = timeout_ms;
        handles.push(tokio::spawn(async move {
            let start = Instant::now();
            let duration = Duration::from_millis(t_ms);
            match method {
                ProbeMethod::Ntp => {
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
                }
                ProbeMethod::HttpHead => {
                    let url = if host.starts_with("http") {
                        host.to_string()
                    } else {
                        format!("https://{host}")
                    };
                    let client = reqwest::Client::builder()
                        .timeout(duration)
                        .build()
                        .unwrap();
                    match timeout(duration, client.head(&url).send()).await {
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
                }
                ProbeMethod::DnsQuery => {
                    match timeout(duration, dns_probe(host)).await {
                        Ok(Ok(elapsed)) => ProbeResult {
                            target_index: i,
                            latency_ms: elapsed,
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
                }
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

async fn dns_probe(host: &str) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    let addr = tokio::net::lookup_host(host)
        .await?
        .next()
        .ok_or("DNS resolution failed")?;

    let bind_addr = if addr.is_ipv6() { "[::]:0" } else { "0.0.0.0:0" };
    let socket = UdpSocket::bind(bind_addr).await?;
    socket.connect(addr).await?;

    let start = Instant::now();
    socket.send(&DNS_QUERY).await?;
    let mut buf = [0u8; 512];
    socket.recv(&mut buf).await?;
    Ok(start.elapsed().as_secs_f64() * 1000.0)
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
