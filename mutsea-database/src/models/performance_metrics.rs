// mutsea-database/src/models/performance_metrics.rs
//! Performance metrics models for AI systems and game engine components
//! 
//! These models track performance, efficiency, and system health metrics
//! for various components of the Mutsea AI engine.

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Performance metrics for system components
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub id: EntityId,
    pub component_id: EntityId,
    pub metric_timestamp: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    
    // Component identification
    pub component_name: String,
    pub component_type: ComponentType,
    pub component_version: String,
    pub measurement_session_id: EntityId,
    
    // Performance metrics
    pub cpu_metrics: CPUMetrics,
    pub memory_metrics: MemoryMetrics,
    pub gpu_metrics: Option<GPUMetrics>,
    pub network_metrics: NetworkMetrics,
    pub storage_metrics: StorageMetrics,
    
    // Timing metrics
    pub response_time_metrics: ResponseTimeMetrics,
    pub throughput_metrics: ThroughputMetrics,
    pub latency_metrics: LatencyMetrics,
    
    // Quality metrics
    pub accuracy_metrics: AccuracyMetrics,
    pub reliability_metrics: ReliabilityMetrics,
    pub availability_metrics: AvailabilityMetrics,
    
    // Efficiency metrics
    pub resource_efficiency: ResourceEfficiency,
    pub energy_efficiency: EnergyEfficiency,
    pub scalability_metrics: ScalabilityMetrics,
    
    // AI-specific metrics
    pub ai_performance_metrics: Option<AISpecificMetrics>,
    pub learning_metrics: Option<LearningMetrics>,
    pub prediction_metrics: Option<PredictionMetrics>,
    
    // System health
    pub health_indicators: HealthIndicators,
    pub error_metrics: ErrorMetrics,
    pub bottleneck_analysis: BottleneckAnalysis,
    
    // Metadata
    pub metadata: EntityMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentType {
    AICore,
    NeuralNetwork,
    RenderingEngine,
    PhysicsEngine,
    AudioEngine,
    NetworkingLayer,
    DatabaseLayer,
    WorldGenerator,
    NPCController,
    PlayerSystem,
    EcosystemSimulator,
    LearningSystem,
    DecisionEngine,
    MemoryManager,
    ResourceManager,
    SecurityModule,
    UIFramework,
    EventSystem,
    AnalyticsEngine,
    Custom(String),
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