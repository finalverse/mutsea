// mutsea-database/src/models/performance_metrics/core_metrics.rs
//! Core performance metrics - timing, throughput, accuracy, and reliability

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response time performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseTimeMetrics {
    pub average_response_time_ms: f32,
    pub median_response_time_ms: f32,
    pub p95_response_time_ms: f32,
    pub p99_response_time_ms: f32,
    pub min_response_time_ms: f32,
    pub max_response_time_ms: f32,
    pub response_time_distribution: ResponseTimeDistribution,
    pub slow_requests_count: u64,
    pub timeout_count: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseTimeDistribution {
    pub under_10ms: u64,
    pub under_50ms: u64,
    pub under_100ms: u64,
    pub under_500ms: u64,
    pub under_1000ms: u64,
    pub over_1000ms: u64,
}

/// Throughput performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThroughputMetrics {
    pub requests_per_second: f32,
    pub transactions_per_second: f32,
    pub operations_per_second: f32,
    pub data_processed_mb_per_second: f32,
    pub peak_throughput: f32,
    pub sustained_throughput: f32,
    pub throughput_efficiency: f32,
    pub bottleneck_identified: Option<String>,
}

/// Latency performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatencyMetrics {
    pub processing_latency_ms: f32,
    pub network_latency_ms: f32,
    pub storage_latency_ms: f32,
    pub queue_latency_ms: f32,
    pub total_end_to_end_latency_ms: f32,
    pub latency_variance: f32,
    pub latency_trends: LatencyTrends,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatencyTrends {
    pub trend_direction: super::TrendDirection,
    pub trend_strength: f32,
    pub seasonal_patterns: bool,
    pub anomalies_detected: u32,
}

/// Accuracy performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccuracyMetrics {
    pub overall_accuracy: f32,
    pub precision: f32,
    pub recall: f32,
    pub f1_score: f32,
    pub false_positive_rate: f32,
    pub false_negative_rate: f32,
    pub accuracy_over_time: Vec<TimestampedValue>,
    pub accuracy_by_category: HashMap<String, f32>,
    pub confidence_intervals: ConfidenceIntervals,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimestampedValue {
    pub timestamp: super::Timestamp,
    pub value: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfidenceIntervals {
    pub lower_bound: f32,
    pub upper_bound: f32,
    pub confidence_level: f32, // e.g., 0.95 for 95%
}

/// Reliability performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReliabilityMetrics {
    pub uptime_percentage: f32,
    pub mtbf_hours: f32, // Mean Time Between Failures
    pub mttr_hours: f32, // Mean Time To Recovery
    pub failure_rate: f32,
    pub error_rate: f32,
    pub success_rate: f32,
    pub fault_tolerance_score: f32,
    pub recovery_time_distribution: Vec<f32>,
    pub reliability_trends: ReliabilityTrends,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReliabilityTrends {
    pub improving: bool,
    pub degrading: bool,
    pub stable: bool,
    pub trend_confidence: f32,
}

/// Availability performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AvailabilityMetrics {
    pub availability_percentage: f32,
    pub planned_downtime_hours: f32,
    pub unplanned_downtime_hours: f32,
    pub service_level_agreement_compliance: f32,
    pub availability_zones_status: HashMap<String, bool>,
    pub redundancy_level: f32,
    pub failover_success_rate: f32,
    pub backup_system_status: BackupSystemStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackupSystemStatus {
    pub backup_available: bool,
    pub backup_sync_status: f32,
    pub last_backup_timestamp: super::Timestamp,
    pub backup_integrity_verified: bool,
}

// Default implementations
impl Default for ResponseTimeMetrics {
    fn default() -> Self {
        Self {
            average_response_time_ms: 100.0,
            median_response_time_ms: 80.0,
            p95_response_time_ms: 200.0,
            p99_response_time_ms: 500.0,
            min_response_time_ms: 10.0,
            max_response_time_ms: 1000.0,
            response_time_distribution: ResponseTimeDistribution {
                under_10ms: 100,
                under_50ms: 500,
                under_100ms: 800,
                under_500ms: 950,
                under_1000ms: 990,
                over_1000ms: 10,
            },
            slow_requests_count: 50,
            timeout_count: 5,
        }
    }
}

impl Default for ThroughputMetrics {
    fn default() -> Self {
        Self {
            requests_per_second: 100.0,
            transactions_per_second: 80.0,
            operations_per_second: 150.0,
            data_processed_mb_per_second: 10.0,
            peak_throughput: 200.0,
            sustained_throughput: 120.0,
            throughput_efficiency: 0.8,
            bottleneck_identified: None,
        }
    }
}

impl Default for LatencyMetrics {
    fn default() -> Self {
        Self {
            processing_latency_ms: 50.0,
            network_latency_ms: 20.0,
            storage_latency_ms: 10.0,
            queue_latency_ms: 5.0,
            total_end_to_end_latency_ms: 85.0,
            latency_variance: 15.0,
            latency_trends: LatencyTrends {
                trend_direction: super::TrendDirection::Stable,
                trend_strength: 0.1,
                seasonal_patterns: false,
                anomalies_detected: 0,
            },
        }
    }
}

impl Default for AccuracyMetrics {
    fn default() -> Self {
        Self {
            overall_accuracy: 0.85,
            precision: 0.82,
            recall: 0.78,
            f1_score: 0.80,
            false_positive_rate: 0.05,
            false_negative_rate: 0.08,
            accuracy_over_time: Vec::new(),
            accuracy_by_category: HashMap::new(),
            confidence_intervals: ConfidenceIntervals {
                lower_bound: 0.80,
                upper_bound: 0.90,
                confidence_level: 0.95,
            },
        }
    }
}

impl Default for ReliabilityMetrics {
    fn default() -> Self {
        Self {
            uptime_percentage: 99.5,
            mtbf_hours: 720.0, // 30 days
            mttr_hours: 2.0,
            failure_rate: 0.001,
            error_rate: 0.005,
            success_rate: 0.995,
            fault_tolerance_score: 0.8,
            recovery_time_distribution: vec![1.0, 2.0, 3.0, 5.0, 10.0],
            reliability_trends: ReliabilityTrends {
                improving: false,
                degrading: false,
                stable: true,
                trend_confidence: 0.8,
            },
        }
    }
}

impl Default for AvailabilityMetrics {
    fn default() -> Self {
        use chrono::Utc;
        Self {
            availability_percentage: 99.9,
            planned_downtime_hours: 2.0,
            unplanned_downtime_hours: 0.5,
            service_level_agreement_compliance: 0.99,
            availability_zones_status: HashMap::new(),
            redundancy_level: 0.8,
            failover_success_rate: 0.95,
            backup_system_status: BackupSystemStatus {
                backup_available: true,
                backup_sync_status: 1.0,
                last_backup_timestamp: Utc::now(),
                backup_integrity_verified: true,
            },
        }
    }
}

// Utility implementations
impl ResponseTimeMetrics {
    /// Check if response times are within acceptable range
    pub fn is_response_time_acceptable(&self, threshold_ms: f32) -> bool {
        self.average_response_time_ms <= threshold_ms && 
        self.p95_response_time_ms <= threshold_ms * 2.0
    }
    
    /// Calculate response time score (0.0 to 1.0, higher is better)
    pub fn calculate_response_time_score(&self) -> f32 {
        let target_time = 100.0; // Target 100ms
        (target_time / self.average_response_time_ms).min(1.0)
    }
    
    /// Get response time performance level
    pub fn get_performance_level(&self) -> ResponseTimePerformanceLevel {
        match self.average_response_time_ms {
            x if x <= 50.0 => ResponseTimePerformanceLevel::Excellent,
            x if x <= 100.0 => ResponseTimePerformanceLevel::Good,
            x if x <= 200.0 => ResponseTimePerformanceLevel::Fair,
            x if x <= 500.0 => ResponseTimePerformanceLevel::Poor,
            _ => ResponseTimePerformanceLevel::Critical,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseTimePerformanceLevel {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

impl ThroughputMetrics {
    /// Calculate throughput efficiency score
    pub fn calculate_efficiency_score(&self) -> f32 {
        if self.peak_throughput > 0.0 {
            self.sustained_throughput / self.peak_throughput
        } else {
            0.0
        }
    }
    
    /// Check if throughput meets target
    pub fn meets_target(&self, target_ops_per_second: f32) -> bool {
        self.operations_per_second >= target_ops_per_second
    }
    
    /// Get throughput utilization percentage
    pub fn get_utilization_percentage(&self) -> f32 {
        if self.peak_throughput > 0.0 {
            (self.operations_per_second / self.peak_throughput * 100.0).min(100.0)
        } else {
            0.0
        }
    }
}

impl AccuracyMetrics {
    /// Calculate composite accuracy score
    pub fn calculate_composite_score(&self) -> f32 {
        (self.overall_accuracy + self.precision + self.recall + self.f1_score) / 4.0
    }
    
    /// Check if accuracy meets minimum threshold
    pub fn meets_accuracy_threshold(&self, threshold: f32) -> bool {
        self.overall_accuracy >= threshold && 
        self.f1_score >= threshold * 0.9 // Slightly lower threshold for F1
    }
    
    /// Get accuracy trend from time series data
    pub fn get_accuracy_trend(&self) -> super::TrendDirection {
        if self.accuracy_over_time.len() < 2 {
            return super::TrendDirection::Unknown;
        }
        
        let recent_values: Vec<f32> = self.accuracy_over_time
            .iter()
            .rev()
            .take(10)
            .map(|tv| tv.value)
            .collect();
        
        if recent_values.len() < 2 {
            return super::TrendDirection::Unknown;
        }
        
        let first_half_avg = recent_values[recent_values.len()/2..].iter().sum::<f32>() 
            / (recent_values.len() - recent_values.len()/2) as f32;
        let second_half_avg = recent_values[..recent_values.len()/2].iter().sum::<f32>() 
            / (recent_values.len()/2) as f32;
        
        let diff = second_half_avg - first_half_avg;
        
        if diff.abs() < 0.01 {
            super::TrendDirection::Stable
        } else if diff > 0.0 {
            super::TrendDirection::Rising
        } else {
            super::TrendDirection::Falling
        }
    }
}

impl ReliabilityMetrics {
    /// Calculate overall reliability score
    pub fn calculate_reliability_score(&self) -> f32 {
        let uptime_factor = self.uptime_percentage / 100.0;
        let success_factor = self.success_rate;
        let fault_tolerance_factor = self.fault_tolerance_score;
        let mtbf_factor = (self.mtbf_hours / 720.0).min(1.0); // Normalize against 30 days
        
        (uptime_factor + success_factor + fault_tolerance_factor + mtbf_factor) / 4.0
    }
    
    /// Check if reliability meets SLA requirements
    pub fn meets_sla(&self, required_uptime: f32, max_error_rate: f32) -> bool {
        self.uptime_percentage >= required_uptime && self.error_rate <= max_error_rate
    }
    
    /// Get reliability status
    pub fn get_reliability_status(&self) -> ReliabilityStatus {
        match self.uptime_percentage {
            x if x >= 99.9 => ReliabilityStatus::Excellent,
            x if x >= 99.5 => ReliabilityStatus::Good,
            x if x >= 99.0 => ReliabilityStatus::Fair,
            x if x >= 95.0 => ReliabilityStatus::Poor,
            _ => ReliabilityStatus::Critical,
        }
    }
    
    /// Calculate expected time to next failure
    pub fn expected_time_to_failure(&self) -> f32 {
        if self.failure_rate > 0.0 {
            1.0 / self.failure_rate
        } else {
            f32::INFINITY
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReliabilityStatus {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

impl AvailabilityMetrics {
    /// Calculate total downtime in hours
    pub fn total_downtime_hours(&self) -> f32 {
        self.planned_downtime_hours + self.unplanned_downtime_hours
    }
    
    /// Check if availability meets target
    pub fn meets_availability_target(&self, target_percentage: f32) -> bool {
        self.availability_percentage >= target_percentage
    }
    
    /// Calculate availability score
    pub fn calculate_availability_score(&self) -> f32 {
        let base_score = self.availability_percentage / 100.0;
        let sla_compliance_bonus = self.service_level_agreement_compliance * 0.1;
        let redundancy_bonus = self.redundancy_level * 0.05;
        let failover_bonus = self.failover_success_rate * 0.05;
        
        (base_score + sla_compliance_bonus + redundancy_bonus + failover_bonus).min(1.0)
    }
    
    /// Get downtime impact assessment
    pub fn get_downtime_impact(&self) -> DowntimeImpact {
        let unplanned_ratio = if self.total_downtime_hours() > 0.0 {
            self.unplanned_downtime_hours / self.total_downtime_hours()
        } else {
            0.0
        };
        
        match (self.total_downtime_hours(), unplanned_ratio) {
            (total, _) if total > 24.0 => DowntimeImpact::Severe,
            (total, unplanned) if total > 8.0 && unplanned > 0.5 => DowntimeImpact::High,
            (total, unplanned) if total > 2.0 && unplanned > 0.3 => DowntimeImpact::Medium,
            (total, _) if total > 0.5 => DowntimeImpact::Low,
            _ => DowntimeImpact::Minimal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DowntimeImpact {
    Minimal,
    Low,
    Medium,
    High,
    Severe,
}