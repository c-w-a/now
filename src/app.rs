// app state and update logic

use crate::probe::ProbeResult;
use crate::targets::TargetGroup;
use std::collections::VecDeque;

pub struct App {
    pub group: &'static TargetGroup,
    pub latest: Vec<Option<ProbeResult>>,
    pub history: Vec<VecDeque<f64>>,
    pub log: VecDeque<String>,
    pub running: bool,
}

const MAX_HISTORY: usize = 50;
const MAX_LOG: usize = 100;

impl App {
    pub fn new(group: &'static TargetGroup) -> Self {
        let n = group.targets.len();
        Self {
            group,
            latest: vec![None; n],
            history: vec![VecDeque::new(); n],
            log: VecDeque::new(),
            running: true,
        }
    }

    pub fn update(&mut self, results: Vec<ProbeResult>) {
        for r in results {
            let i = r.target_index;
            if i >= self.group.targets.len() {
                continue;
            }
            let name = self.group.targets[i].name;
            if r.success {
                self.push_log(format!("{name}: {:.1}ms", r.latency_ms));
                let h = &mut self.history[i];
                h.push_back(r.latency_ms);
                if h.len() > MAX_HISTORY {
                    h.pop_front();
                }
            } else {
                let err = r.error.as_deref().unwrap_or("unknown");
                self.push_log(format!("{name}: FAIL ({err})"));
            }
            self.latest[i] = Some(r);
        }
    }

    fn push_log(&mut self, msg: String) {
        self.log.push_back(msg);
        if self.log.len() > MAX_LOG {
            self.log.pop_front();
        }
    }

    pub fn median_latency(&self) -> Option<f64> {
        let mut vals: Vec<f64> = self.latest.iter()
            .filter_map(|r| r.as_ref())
            .filter(|r| r.success)
            .map(|r| r.latency_ms)
            .collect();
        if vals.is_empty() { return None; }
        vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Some(vals[vals.len() / 2])
    }

    pub fn loss_count(&self) -> usize {
        self.latest.iter()
            .filter_map(|r| r.as_ref())
            .filter(|r| !r.success)
            .count()
    }

    pub fn fastest_index(&self) -> Option<usize> {
        self.latest.iter()
            .enumerate()
            .filter_map(|(i, r)| r.as_ref().filter(|r| r.success).map(|r| (i, r.latency_ms)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(i, _)| i)
    }

    pub fn jitter(&self) -> Option<f64> {
        let vals: Vec<f64> = self.latest.iter()
            .filter_map(|r| r.as_ref())
            .filter(|r| r.success)
            .map(|r| r.latency_ms)
            .collect();
        if vals.len() < 2 { return None; }
        let mean = vals.iter().sum::<f64>() / vals.len() as f64;
        let variance = vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / vals.len() as f64;
        Some(variance.sqrt())
    }
}
