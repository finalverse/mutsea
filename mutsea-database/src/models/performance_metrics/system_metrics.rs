// mutsea-database/src/models/performance_metrics/system_metrics.rs
//! System-level performance metrics - CPU, memory, GPU, network, storage

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Container for all system metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_metrics: CPUMetrics,
    pub memory_metrics: MemoryMetrics,
    pub gpu_metrics: Option<GPUMetrics>,
    pub network_metrics: NetworkMetrics,
    pub storage_metrics: StorageMetrics,
    pub response_time_metrics: super::core_metrics::ResponseTimeMetrics,
    pub throughput_metrics: super::core_metrics::ThroughputMetrics,
    pub latency_metrics: super::core_metrics::LatencyMetrics,
    pub accuracy_metrics: super::core_metrics::AccuracyMetrics,
    pub reliability_metrics: super::core_metrics::ReliabilityMetrics,
    pub availability_metrics: super::core_metrics::AvailabilityMetrics,
    pub resource_efficiency: ResourceEfficiency,
    pub energy_efficiency: EnergyEfficiency,
    pub scalability_metrics: ScalabilityMetrics,
}

/// CPU performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CPUMetrics {
    pub usage_percentage: f32,
    pub core_usage: Vec<f32>, // Usage per core
    pub frequency_mhz: f32,
    pub temperature_celsius: Option<f32>,
    pub context_switches: u64,
    pub interrupts_per_second: u64,
    pub load_average: LoadAverage,
    pub cpu_time_breakdown: CPUTimeBreakdown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadAverage {
    pub one_minute: f32,
    pub five_minutes: f32,
    pub fifteen_minutes: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CPUTimeBreakdown {
    pub user_time_percentage: f32,
    pub system_time_percentage: f32,
    pub idle_time_percentage: f32,
    pub io_wait_percentage: f32,
    pub interrupt_time_percentage: f32,
}

/// Memory performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub total_memory_mb: f32,
    pub used_memory_mb: f32,
    pub available_memory_mb: f32,
    pub memory_usage_percentage: f32,
    pub cache_usage_mb: f32,
    pub buffer_usage_mb: f32,
    pub swap_usage_mb: f32,
    pub memory_allocation_rate: f32, // MB/s
    pub memory_deallocation_rate: f32, // MB/s
    pub garbage_collection_metrics: Option<GarbageCollectionMetrics>,
    pub memory_fragmentation: f32,
    pub memory_leaks_detected: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GarbageCollectionMetrics {
    pub gc_frequency: f32, // Collections per second
    pub average_gc_duration_ms: f32,
    pub memory_reclaimed_mb: f32,
    pub gc_overhead_percentage: f32,
    pub generational_gc_stats: Option<GenerationalGCStats>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerationalGCStats {
    pub young_generation_collections: u64,
    pub old_generation_collections: u64,
    pub young_gen_duration_ms: f32,
    pub old_gen_duration_ms: f32,
}

/// GPU performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPUMetrics {
    pub gpu_usage_percentage: f32,
    pub memory_usage_mb: f32,
    pub memory_total_mb: f32,
    pub temperature_celsius: f32,
    pub power_consumption_watts: f32,
    pub clock_speed_mhz: f32,
    pub memory_clock_mhz: f32,
    pub compute_utilization: f32,
    pub memory_bandwidth_utilization: f32,
    pub shader_utilization: f32,
    pub render_target_utilization: f32,
}

/// Network performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub bandwidth_usage_mbps: f32,
    pub bandwidth_available_mbps: f32,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub packets_lost: u64,
    pub packet_loss_rate: f32,
    pub round_trip_time_ms: f32,
    pub jitter_ms: f32,
    pub connection_count: u32,
    pub active_sessions: u32,
    pub data_transfer_rates: DataTransferRates,
    pub network_errors: NetworkErrors,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataTransferRates {
    pub upload_rate_mbps: f32,
    pub download_rate_mbps: f32,
    pub peak_upload_mbps: f32,
    pub peak_download_mbps: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkErrors {
    pub connection_timeouts: u64,
    pub dns_resolution_failures: u64,
    pub protocol_errors: u64,
    pub authentication_failures: u64,
}

/// Storage performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageMetrics {
    pub total_storage_gb: f32,
    pub used_storage_gb: f32,
    pub available_storage_gb: f32,
    pub storage_usage_percentage: f32,
    pub read_iops: f32,
    pub write_iops: f32,
    pub read_throughput_mbps: f32,
    pub write_throughput_mbps: f32,
    pub average_read_latency_ms: f32,
    pub average_write_latency_ms: f32,
    pub storage_health: StorageHealth,
    pub cache_hit_ratio: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageHealth {
    pub bad_sectors: u64,
    pub temperature_celsius: Option<f32>,
    pub power_on_hours: u64,
    pub wear_leveling_count: Option<u64>,
    pub error_rate: f32,
    pub health_percentage: f32,
    pub estimated_lifetime_remaining: Option<u64>, // hours
}

/// Resource efficiency metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceEfficiency {
    pub cpu_efficiency: f32,
    pub memory_efficiency: f32,
    pub gpu_efficiency: Option<f32>,
    pub network_efficiency: f32,
    pub storage_efficiency: f32,
    pub overall_efficiency_score: f32,
    pub resource_waste_percentage: f32,
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptimizationOpportunity {
    pub resource_type: String,
    pub potential_improvement: f32,
    pub implementation_effort: f32,
    pub recommendation: String,
}

/// Energy efficiency metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyEfficiency {
    pub power_consumption_watts: f32,
    pub energy_per_operation_joules: f32,
    pub power_usage_effectiveness: f32,
    pub idle_power_consumption: f32,
    pub peak_power_consumption: f32,
    pub energy_efficiency_rating: EnergyRating,
    pub carbon_footprint_kg_co2: Option<f32>,
    pub energy_optimization_score: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnergyRating {
    A_Plus_Plus_Plus,
    A_Plus_Plus,
    A_Plus,
    A,
    B,
    C,
    D,
    F,
}

/// Scalability metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScalabilityMetrics {
    pub horizontal_scalability: f32,
    pub vertical_scalability: f32,
    pub load_handling_capacity: f32,
    pub scaling_efficiency: f32,
    pub auto_scaling_effectiveness: f32,
    pub bottleneck_points: Vec<BottleneckPoint>,
    pub capacity_planning_metrics: CapacityPlanningMetrics,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BottleneckPoint {
    pub resource_type: String,
    pub threshold_percentage: f32,
    pub impact_severity: f32,
    pub mitigation_strategies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapacityPlanningMetrics {
    pub current_capacity_utilization: f32,
    pub projected_capacity_needs: f32,
    pub time_to_capacity_limit: Option<u64>, // seconds
    pub scaling_recommendations: Vec<String>,
}

// Default implementations
impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_metrics: CPUMetrics::default(),
            memory_metrics: MemoryMetrics::default(),
            gpu_metrics: None,
            network_metrics: NetworkMetrics::default(),
            storage_metrics: StorageMetrics::default(),
            response_time_metrics: super::core_metrics::ResponseTimeMetrics::default(),
            throughput_metrics: super::core_metrics::ThroughputMetrics::default(),
            latency_metrics: super::core_metrics::LatencyMetrics::default(),
            accuracy_metrics: super::core_metrics::AccuracyMetrics::default(),
            reliability_metrics: super::core_metrics::ReliabilityMetrics::default(),
            availability_metrics: super::core_metrics::AvailabilityMetrics::default(),
            resource_efficiency: ResourceEfficiency::default(),
            energy_efficiency: EnergyEfficiency::default(),
            scalability_metrics: ScalabilityMetrics::default(),
        }
    }
}

impl Default for CPUMetrics {
    fn default() -> Self {
        Self {
            usage_percentage: 25.0,
            core_usage: vec![20.0, 25.0, 30.0, 25.0], // 4 cores
            frequency_mhz: 2400.0,
            temperature_celsius: Some(45.0),
            context_switches: 1000,
            interrupts_per_second: 500,
            load_average: LoadAverage { 
                one_minute: 0.5, 
                five_minutes: 0.6, 
                fifteen_minutes: 0.7 
            },
            cpu_time_breakdown: CPUTimeBreakdown {
                user_time_percentage: 60.0,
                system_time_percentage: 15.0,
                idle_time_percentage: 20.0,
                io_wait_percentage: 3.0,
                interrupt_time_percentage: 2.0,
            },
        }
    }
}

impl Default for MemoryMetrics {
    fn default() -> Self {
        Self {
            total_memory_mb: 8192.0,
            used_memory_mb: 4096.0,
            available_memory_mb: 4096.0,
            memory_usage_percentage: 50.0,
            cache_usage_mb: 1024.0,
            buffer_usage_mb: 512.0,
            swap_usage_mb: 0.0,
            memory_allocation_rate: 100.0,
            memory_deallocation_rate: 95.0,
            garbage_collection_metrics: None,
            memory_fragmentation: 0.05,
            memory_leaks_detected: 0,
        }
    }
}

impl Default for NetworkMetrics {
    fn default() -> Self {
        Self {
            bandwidth_usage_mbps: 10.0,
            bandwidth_available_mbps: 100.0,
            packets_sent: 10000,
            packets_received: 9950,
            packets_lost: 50,
            packet_loss_rate: 0.005,
            round_trip_time_ms: 50.0,
            jitter_ms: 5.0,
            connection_count: 100,
            active_sessions: 50,
            data_transfer_rates: DataTransferRates {
                upload_rate_mbps: 5.0,
                download_rate_mbps: 8.0,
                peak_upload_mbps: 15.0,
                peak_download_mbps: 25.0,
            },
            network_errors: NetworkErrors {
                connection_timeouts: 5,
                dns_resolution_failures: 2,
                protocol_errors: 1,
                authentication_failures: 0,
            },
        }
    }
}

impl Default for StorageMetrics {
    fn default() -> Self {
        Self {
            total_storage_gb: 1000.0,
            used_storage_gb: 500.0,
            available_storage_gb: 500.0,
            storage_usage_percentage: 50.0,
            read_iops: 1000.0,
            write_iops: 800.0,
            read_throughput_mbps: 100.0,
            write_throughput_mbps: 80.0,
            average_read_latency_ms: 5.0,
            average_write_latency_ms: 8.0,
            storage_health: StorageHealth::default(),
            cache_hit_ratio: 0.85,
        }
    }
}

impl Default for StorageHealth {
    fn default() -> Self {
        Self {
            bad_sectors: 0,
            temperature_celsius: Some(35.0),
            power_on_hours: 8760, // 1 year
            wear_leveling_count: Some(100),
            error_rate: 0.001,
            health_percentage: 95.0,
            estimated_lifetime_remaining: Some(43800), // 5 years
        }
    }
}

impl Default for ResourceEfficiency {
    fn default() -> Self {
        Self {
            cpu_efficiency: 0.8,
            memory_efficiency: 0.75,
            gpu_efficiency: None,
            network_efficiency: 0.85,
            storage_efficiency: 0.9,
            overall_efficiency_score: 0.82,
            resource_waste_percentage: 18.0,
            optimization_opportunities: Vec::new(),
        }
    }
}

impl Default for EnergyEfficiency {
    fn default() -> Self {
        Self {
            power_consumption_watts: 250.0,
            energy_per_operation_joules: 0.001,
            power_usage_effectiveness: 1.2,
            idle_power_consumption: 100.0,
            peak_power_consumption: 400.0,
            energy_efficiency_rating: EnergyRating::B,
            carbon_footprint_kg_co2: Some(0.5),
            energy_optimization_score: 0.7,
        }
    }
}

impl Default for ScalabilityMetrics {
    fn default() -> Self {
        Self {
            horizontal_scalability: 0.8,
            vertical_scalability: 0.7,
            load_handling_capacity: 0.75,
            scaling_efficiency: 0.8,
            auto_scaling_effectiveness: 0.7,
            bottleneck_points: Vec::new(),
            capacity_planning_metrics: CapacityPlanningMetrics {
                current_capacity_utilization: 0.6,
                projected_capacity_needs: 0.8,
                time_to_capacity_limit: Some(86400 * 30), // 30 days
                scaling_recommendations: Vec::new(),
            },
        }
    }
}

// Utility implementations
impl CPUMetrics {
    /// Check if CPU is under high load
    pub fn is_under_high_load(&self) -> bool {
        self.usage_percentage > 80.0 || self.load_average.one_minute > 0.8
    }
    
    /// Get CPU efficiency score
    pub fn get_efficiency_score(&self) -> f32 {
        let usage_efficiency = if self.usage_percentage > 0.0 {
            (self.cpu_time_breakdown.user_time_percentage / self.usage_percentage).min(1.0)
        } else {
            1.0
        };
        
        let thermal_efficiency = if let Some(temp) = self.temperature_celsius {
            (1.0 - (temp - 40.0) / 40.0).max(0.0).min(1.0) // Optimal at 40°C, decreases linearly
        } else {
            0.8 // Default if no temperature data
        };
        
        (usage_efficiency + thermal_efficiency) / 2.0
    }
    
    /// Calculate CPU performance index
    pub fn calculate_performance_index(&self) -> f32 {
        let frequency_factor = self.frequency_mhz / 3000.0; // Normalize against 3GHz
        let utilization_factor = self.usage_percentage / 100.0;
        let load_factor = 1.0 - (self.load_average.one_minute - 0.5).max(0.0);
        
        (frequency_factor * utilization_factor * load_factor).min(1.0)
    }
}

impl MemoryMetrics {
    /// Check if memory usage is critical
    pub fn is_memory_critical(&self) -> bool {
        self.memory_usage_percentage > 90.0 || 
        self.swap_usage_mb > self.total_memory_mb * 0.1 ||
        self.memory_leaks_detected > 0
    }
    
    /// Calculate memory efficiency
    pub fn calculate_memory_efficiency(&self) -> f32 {
        let usage_efficiency = 1.0 - (self.memory_usage_percentage / 100.0 - 0.7).max(0.0) / 0.3;
        let fragmentation_efficiency = 1.0 - self.memory_fragmentation;
        let allocation_efficiency = if self.memory_allocation_rate > 0.0 {
            self.memory_deallocation_rate / self.memory_allocation_rate
        } else {
            1.0
        };
        
        (usage_efficiency + fragmentation_efficiency + allocation_efficiency.min(1.0)) / 3.0
    }
    
    /// Get memory pressure level
    pub fn get_memory_pressure(&self) -> MemoryPressureLevel {
        match self.memory_usage_percentage {
            x if x < 50.0 => MemoryPressureLevel::Low,
            x if x < 70.0 => MemoryPressureLevel::Moderate,
            x if x < 85.0 => MemoryPressureLevel::High,
            x if x < 95.0 => MemoryPressureLevel::Critical,
            _ => MemoryPressureLevel::Emergency,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MemoryPressureLevel {
    Low,
    Moderate,
    High,
    Critical,
    Emergency,
}

impl NetworkMetrics {
    /// Check if network performance is degraded
    pub fn is_network_degraded(&self) -> bool {
        self.packet_loss_rate > 0.01 || // 1% packet loss
        self.round_trip_time_ms > 100.0 ||
        self.jitter_ms > 10.0
    }
    
    /// Calculate network efficiency
    pub fn calculate_network_efficiency(&self) -> f32 {
        let bandwidth_efficiency = if self.bandwidth_available_mbps > 0.0 {
            (self.bandwidth_usage_mbps / self.bandwidth_available_mbps).min(1.0)
        } else {
            0.0
        };
        
        let reliability_factor = 1.0 - self.packet_loss_rate;
        let latency_factor = (100.0 / self.round_trip_time_ms).min(1.0);
        
        (bandwidth_efficiency + reliability_factor + latency_factor) / 3.0
    }
    
    /// Get network quality score
    pub fn get_quality_score(&self) -> NetworkQuality {
        let quality_score = self.calculate_network_efficiency();
        
        match quality_score {
            x if x >= 0.9 => NetworkQuality::Excellent,
            x if x >= 0.8 => NetworkQuality::Good,
            x if x >= 0.6 => NetworkQuality::Fair,
            x if x >= 0.4 => NetworkQuality::Poor,
            _ => NetworkQuality::Critical,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NetworkQuality {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

impl StorageMetrics {
    /// Check if storage needs attention
    pub fn needs_attention(&self) -> bool {
        self.storage_usage_percentage > 85.0 ||
        self.storage_health.health_percentage < 80.0 ||
        self.average_read_latency_ms > 20.0 ||
        self.average_write_latency_ms > 30.0
    }
    
    /// Calculate storage performance score
    pub fn calculate_performance_score(&self) -> f32 {
        let capacity_score = 1.0 - (self.storage_usage_percentage / 100.0 - 0.8).max(0.0) / 0.2;
        let health_score = self.storage_health.health_percentage / 100.0;
        let latency_score = (10.0 / (self.average_read_latency_ms + self.average_write_latency_ms)).min(1.0);
        let iops_score = ((self.read_iops + self.write_iops) / 2000.0).min(1.0);
        
        (capacity_score + health_score + latency_score + iops_score) / 4.0
    }
    
    /// Get storage status
    pub fn get_storage_status(&self) -> StorageStatus {
        if self.storage_health.health_percentage < 50.0 {
            StorageStatus::Critical
        } else if self.storage_usage_percentage > 95.0 {
            StorageStatus::Full
        } else if self.needs_attention() {
            StorageStatus::Warning
        } else {
            StorageStatus::Healthy
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StorageStatus {
    Healthy,
    Warning,
    Full,
    Critical,
}

impl SystemMetrics {
    /// Calculate overall system health score
    pub fn calculate_system_health(&self) -> f32 {
        let cpu_health = self.cpu_metrics.get_efficiency_score();
        let memory_health = self.memory_metrics.calculate_memory_efficiency();
        let network_health = self.network_metrics.calculate_network_efficiency();
        let storage_health = self.storage_metrics.calculate_performance_score();
        let reliability_health = self.reliability_metrics.calculate_reliability_score();
        
        let mut total_health = cpu_health + memory_health + network_health + storage_health + reliability_health;
        let mut count = 5.0;
        
        if let Some(gpu_metrics) = &self.gpu_metrics {
            total_health += gpu_metrics.calculate_efficiency_score();
            count += 1.0;
        }
        
        total_health / count
    }
    
    /// Get system bottlenecks
    pub fn get_system_bottlenecks(&self) -> Vec<String> {
        let mut bottlenecks = Vec::new();
        
        if self.cpu_metrics.is_under_high_load() {
            bottlenecks.push("CPU under high load".to_string());
        }
        
        if self.memory_metrics.is_memory_critical() {
            bottlenecks.push("Memory usage critical".to_string());
        }
        
        if self.network_metrics.is_network_degraded() {
            bottlenecks.push("Network performance degraded".to_string());
        }
        
        if self.storage_metrics.needs_attention() {
            bottlenecks.push("Storage needs attention".to_string());
        }
        
        bottlenecks
    }
}

impl GPUMetrics {
    /// Calculate GPU efficiency score
    pub fn calculate_efficiency_score(&self) -> f32 {
        let utilization_score = self.gpu_usage_percentage / 100.0;
        let memory_score = 1.0 - (self.memory_usage_mb / self.memory_total_mb - 0.8).max(0.0) / 0.2;
        let thermal_score = (1.0 - (self.temperature_celsius - 70.0) / 30.0).max(0.0).min(1.0);
        let power_score = (300.0 / self.power_consumption_watts).min(1.0);
        
        (utilization_score + memory_score + thermal_score + power_score) / 4.0
    }
    
    /// Check if GPU is throttling
    pub fn is_throttling(&self) -> bool {
        self.temperature_celsius > 85.0 || self.power_consumption_watts > 350.0
    }
}