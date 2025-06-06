//! mutsea-network/src/lludp_server/stats.rs
//! Server statistics tracking

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// Enhanced server statistics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ServerStats {
    pub packets_received: u64,
    pub packets_sent: u64,
    pub bytes_received: u64,
    pub bytes_sent: u64,
    pub connections: u64,
    pub active_sessions: u64,
    pub errors: u64,
    pub login_attempts: u64,
    pub successful_logins: u64,
    pub heartbeats_sent: u64,
    pub reliable_resends: u64,
    pub start_time: Option<Instant>,
}

impl ServerStats {
    pub fn new() -> Self {
        Self {
            start_time: Some(Instant::now()),
            ..Default::default()
        }
    }

    /// Calculate packets per second
    pub fn packets_per_second(&self) -> f64 {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                (self.packets_received + self.packets_sent) as f64 / elapsed
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Calculate error rate percentage
    pub fn error_rate(&self) -> f64 {
        if self.packets_received > 0 {
            (self.errors as f64 / self.packets_received as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Calculate login success rate
    pub fn login_success_rate(&self) -> f64 {
        if self.login_attempts > 0 {
            (self.successful_logins as f64 / self.login_attempts as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Get uptime duration
    pub fn uptime(&self) -> std::time::Duration {
        if let Some(start_time) = self.start_time {
            start_time.elapsed()
        } else {
            std::time::Duration::ZERO
        }
    }
}