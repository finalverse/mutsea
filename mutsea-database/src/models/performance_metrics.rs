// mutsea-database/src/models/performance_metrics.rs
//! Performance metrics models for AI systems and game engine components
//! 
//! These models track performance, efficiency, and system health metrics
//! for various components of the Mutsea AI engine.

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export submodules
pub mod core_metrics;
pub mod system_metrics;
pub mod ai_metrics;
pub mod health_metrics;
pub mod analysis_metrics;

pub use core_metrics::*;
pub use system_metrics::*;
pub use ai_metrics::*;
pub use health_metrics::*;
pub use analysis_metrics::*;

/// Main performance metrics container
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
    
    // Core system metrics
    pub system_metrics: SystemMetrics,
    
    // AI-specific metrics
    pub ai_metrics: Option<AIMetrics>,
    
    // Health and diagnostics
    pub health_metrics: HealthMetrics,
    
    // Analysis and optimization
    pub analysis_metrics: AnalysisMetrics,
    
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrendDirection {
    Rising,
    Falling,
    Stable,
    Volatile,
    Cyclical,
    Unknown,
}

// Implement DatabaseModel and AIModel traits
impl_database_model!(PerformanceMetrics, id, created_at, updated_at);

impl AIModel for PerformanceMetrics {
    fn confidence_score(&self) -> f32 {
        // Calculate confidence based on health indicators and data quality
        let health_confidence = self.health_metrics.health_indicators.overall_health_score;
        let reliability_confidence = self.system_metrics.reliability_metrics.success_rate;
        let accuracy_confidence = self.system_metrics.accuracy_metrics.overall_accuracy;
        
        (health_confidence + reliability_confidence + accuracy_confidence) / 3.0
    }
    
    fn ai_context(&self) -> &AIContext {
        static DEFAULT_CONTEXT: std::sync::OnceLock<AIContext> = std::sync::OnceLock::new();
        DEFAULT_CONTEXT.get_or_init(|| AIContext {
            model_version: "performance-monitor-v1.0".to_string(),
            decision_algorithm: "performance-analysis-ai".to_string(),
            training_data_hash: None,
            confidence_threshold: 0.8,
            processing_time_ms: 50,
            resource_usage: ResourceUsage {
                cpu_usage_percent: 5.0,
                memory_usage_mb: 128.0,
                gpu_usage_percent: None,
                network_io_bytes: 1024,
                disk_io_bytes: 2048,
            },
        })
    }
    
    fn has_learning_data(&self) -> bool {
        self.ai_metrics.as_ref().map_or(false, |ai| ai.learning_metrics.is_some()) ||
        !self.analysis_metrics.bottleneck_analysis.optimization_recommendations.is_empty()
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            component_id: Uuid::new_v4(),
            metric_timestamp: now,
            created_at: now,
            updated_at: now,
            component_name: "DefaultComponent".to_string(),
            component_type: ComponentType::Custom("Default".to_string()),
            component_version: "1.0.0".to_string(),
            measurement_session_id: Uuid::new_v4(),
            system_metrics: SystemMetrics::default(),
            ai_metrics: None,
            health_metrics: HealthMetrics::default(),
            analysis_metrics: AnalysisMetrics::default(),
            metadata: EntityMetadata::default(),
        }
    }
}

// Utility implementations
impl PerformanceMetrics {
    /// Create new performance metrics
    pub fn new(component_id: EntityId, component_name: String, component_type: ComponentType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            component_id,
            metric_timestamp: now,
            created_at: now,
            updated_at: now,
            component_name,
            component_type,
            component_version: "1.0.0".to_string(),
            measurement_session_id: Uuid::new_v4(),
            system_metrics: SystemMetrics::default(),
            ai_metrics: None,
            health_metrics: HealthMetrics::default(),
            analysis_metrics: AnalysisMetrics::default(),
            metadata: EntityMetadata::default(),
        }
    }
    
    /// Calculate overall performance score
    pub fn calculate_overall_performance_score(&self) -> f32 {
        let cpu_score = 1.0 - (self.system_metrics.cpu_metrics.usage_percentage / 100.0).min(1.0);
        let memory_score = 1.0 - (self.system_metrics.memory_metrics.memory_usage_percentage / 100.0).min(1.0);
        let response_time_score = (200.0 / self.system_metrics.response_time_metrics.average_response_time_ms).min(1.0);
        let reliability_score = self.system_metrics.reliability_metrics.success_rate;
        let health_score = self.health_metrics.health_indicators.overall_health_score;
        
        (cpu_score + memory_score + response_time_score + reliability_score + health_score) / 5.0
    }
    
    /// Check if system is under stress
    pub fn is_under_stress(&self) -> bool {
        self.system_metrics.cpu_metrics.usage_percentage > 80.0 ||
        self.system_metrics.memory_metrics.memory_usage_percentage > 85.0 ||
        self.system_metrics.response_time_metrics.average_response_time_ms > 500.0 ||
        self.health_metrics.error_metrics.error_rate > 0.05 ||
        self.health_metrics.health_indicators.critical_alerts > 0
    }
    
    /// Get performance recommendations
    pub fn get_performance_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if self.system_metrics.cpu_metrics.usage_percentage > 80.0 {
            recommendations.push("Consider CPU optimization or scaling".to_string());
        }
        
        if self.system_metrics.memory_metrics.memory_usage_percentage > 85.0 {
            recommendations.push("Investigate memory usage and potential leaks".to_string());
        }
        
        if self.system_metrics.response_time_metrics.average_response_time_ms > 300.0 {
            recommendations.push("Optimize response time performance".to_string());
        }
        
        if self.health_metrics.error_metrics.error_rate > 0.02 {
            recommendations.push("Address error rate issues".to_string());
        }
        
        if self.system_metrics.resource_efficiency.overall_efficiency_score < 0.7 {
            recommendations.push("Improve resource efficiency".to_string());
        }
        
        // Add specific optimization recommendations
        recommendations.extend(
            self.analysis_metrics.bottleneck_analysis.optimization_recommendations.iter()
                .map(|rec| rec.recommendation_type.clone())
        );
        
        recommendations
    }
    
    /// Generate performance report
    pub fn generate_performance_report(&self) -> String {
        format!(
            "Performance Report: {}\n\
            =============================\n\
            Component: {} ({:?})\n\
            Overall Score: {:.1}%\n\
            Status: {}\n\
            \n\
            Resource Usage:\n\
            - CPU: {:.1}%\n\
            - Memory: {:.1}%\n\
            - Storage: {:.1}%\n\
            - Network: {:.1} Mbps\n\
            \n\
            Performance:\n\
            - Avg Response Time: {:.1}ms\n\
            - Throughput: {:.1} ops/sec\n\
            - Error Rate: {:.3}%\n\
            - Uptime: {:.2}%\n\
            \n\
            Health:\n\
            - Health Score: {:.1}%\n\
            - Critical Alerts: {}\n\
            - Warning Alerts: {}\n\
            \n\
            Efficiency:\n\
            - Resource Efficiency: {:.1}%\n\
            - Energy Efficiency: {:.1} W\n\
            \n\
            Recommendations: {}\n",
            self.component_name,
            self.component_name,
            self.component_type,
            self.calculate_overall_performance_score() * 100.0,
            if self.is_under_stress() { "UNDER STRESS" } else { "Healthy" },
            self.system_metrics.cpu_metrics.usage_percentage,
            self.system_metrics.memory_metrics.memory_usage_percentage,
            self.system_metrics.storage_metrics.storage_usage_percentage,
            self.system_metrics.network_metrics.bandwidth_usage_mbps,
            self.system_metrics.response_time_metrics.average_response_time_ms,
            self.system_metrics.throughput_metrics.operations_per_second,
            self.health_metrics.error_metrics.error_rate * 100.0,
            self.system_metrics.availability_metrics.availability_percentage,
            self.health_metrics.health_indicators.overall_health_score * 100.0,
            self.health_metrics.health_indicators.critical_alerts,
            self.health_metrics.health_indicators.warning_alerts,
            self.system_metrics.resource_efficiency.overall_efficiency_score * 100.0,
            self.system_metrics.energy_efficiency.power_consumption_watts,
            self.get_performance_recommendations().len(),
        )
    }
    
    /// Update metrics timestamp
    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now();
        self.metric_timestamp = self.updated_at;
    }
    
    /// Add AI metrics
    pub fn set_ai_metrics(&mut self, ai_metrics: AIMetrics) {
        self.ai_metrics = Some(ai_metrics);
        self.update_timestamp();
    }
    
    /// Check if component is AI-enabled
    pub fn is_ai_enabled(&self) -> bool {
        self.ai_metrics.is_some()
    }
    
    /// Get component utilization summary
    pub fn get_utilization_summary(&self) -> ComponentUtilizationSummary {
        ComponentUtilizationSummary {
            cpu_utilization: self.system_metrics.cpu_metrics.usage_percentage,
            memory_utilization: self.system_metrics.memory_metrics.memory_usage_percentage,
            gpu_utilization: self.system_metrics.gpu_metrics.as_ref().map(|gpu| gpu.gpu_usage_percentage),
            network_utilization: (self.system_metrics.network_metrics.bandwidth_usage_mbps / 
                                 self.system_metrics.network_metrics.bandwidth_available_mbps) * 100.0,
            storage_utilization: self.system_metrics.storage_metrics.storage_usage_percentage,
            overall_efficiency: self.system_metrics.resource_efficiency.overall_efficiency_score,
        }
    }
    
    /// Check if performance is degrading
    pub fn is_performance_degrading(&self) -> bool {
        self.health_metrics.health_indicators.performance_degradation_detected ||
        self.health_metrics.health_indicators.health_trends.degrading ||
        matches!(self.system_metrics.response_time_metrics.response_time_distribution.over_1000ms, x if x > 100)
    }
    
    /// Get critical issues
    pub fn get_critical_issues(&self) -> Vec<String> {
        let mut issues = Vec::new();
        
        if self.health_metrics.health_indicators.critical_alerts > 0 {
            issues.push(format!("{} critical alerts active", self.health_metrics.health_indicators.critical_alerts));
        }
        
        if self.system_metrics.cpu_metrics.usage_percentage > 95.0 {
            issues.push("CPU usage critically high".to_string());
        }
        
        if self.system_metrics.memory_metrics.memory_usage_percentage > 95.0 {
            issues.push("Memory usage critically high".to_string());
        }
        
        if self.health_metrics.error_metrics.error_rate > 0.1 {
            issues.push("Error rate critically high".to_string());
        }
        
        if self.system_metrics.availability_metrics.availability_percentage < 95.0 {
            issues.push("Availability below SLA".to_string());
        }
        
        // Add AI-specific critical issues
        if let Some(ai_metrics) = &self.ai_metrics {
            if ai_metrics.ai_specific_metrics.model_accuracy < 0.7 {
                issues.push("AI model accuracy critically low".to_string());
            }
        }
        
        issues
    }
}

/// Component utilization summary
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentUtilizationSummary {
    pub cpu_utilization: f32,
    pub memory_utilization: f32,
    pub gpu_utilization: Option<f32>,
    pub network_utilization: f32,
    pub storage_utilization: f32,
    pub overall_efficiency: f32,
}

impl ComponentUtilizationSummary {
    /// Calculate average utilization across all resources
    pub fn calculate_average_utilization(&self) -> f32 {
        let mut total = self.cpu_utilization + self.memory_utilization + 
                       self.network_utilization + self.storage_utilization;
        let mut count = 4.0;
        
        if let Some(gpu_util) = self.gpu_utilization {
            total += gpu_util;
            count += 1.0;
        }
        
        total / count
    }
    
    /// Check if any resource is over-utilized
    pub fn is_over_utilized(&self, threshold: f32) -> bool {
        self.cpu_utilization > threshold ||
        self.memory_utilization > threshold ||
        self.network_utilization > threshold ||
        self.storage_utilization > threshold ||
        self.gpu_utilization.map_or(false, |gpu| gpu > threshold)
    }
    
    /// Get the most utilized resource
    pub fn get_highest_utilized_resource(&self) -> (String, f32) {
        let mut max_resource = ("CPU".to_string(), self.cpu_utilization);
        
        if self.memory_utilization > max_resource.1 {
            max_resource = ("Memory".to_string(), self.memory_utilization);
        }
        
        if self.network_utilization > max_resource.1 {
            max_resource = ("Network".to_string(), self.network_utilization);
        }
        
        if self.storage_utilization > max_resource.1 {
            max_resource = ("Storage".to_string(), self.storage_utilization);
        }
        
        if let Some(gpu_util) = self.gpu_utilization {
            if gpu_util > max_resource.1 {
                max_resource = ("GPU".to_string(), gpu_util);
            }
        }
        
        max_resource
    }
}