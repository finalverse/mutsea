// mutsea-database/src/models/performance_metrics/health_metrics.rs
//! Health and diagnostic metrics for system monitoring and alerting

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Container for all health-related metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecurringError {
    pub error_signature: String,
    pub frequency: u32,
    pub last_occurrence: super::Timestamp,
    pub root_cause_identified: bool,
    pub fix_attempts: u32,
    pub impact_level: ErrorImpactLevel,
    pub pattern_analysis: ErrorPattern,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorImpactLevel {
    Negligible,
    Minor,
    Moderate,
    Major,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorPattern {
    pub temporal_pattern: TemporalPattern,
    pub correlation_factors: Vec<CorrelationFactor>,
    pub prediction_confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TemporalPattern {
    Random,
    Periodic,
    Increasing,
    Decreasing,
    Burst,
    Seasonal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorrelationFactor {
    pub factor_name: String,
    pub correlation_strength: f32,
    pub causality_likelihood: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorTrends {
    pub error_rate_increasing: bool,
    pub new_error_types_emerging: bool,
    pub error_severity_increasing: bool,
    pub resolution_time_improving: bool,
    pub trend_analysis_confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorImpactAnalysis {
    pub user_impact_score: f32,
    pub business_impact_score: f32,
    pub system_impact_score: f32,
    pub financial_impact_estimate: Option<f32>,
    pub reputation_impact_score: f32,
    pub affected_user_count: u64,
    pub service_disruption_duration: u64, // seconds
}

/// Alert metrics and management
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertMetrics {
    pub active_alerts: Vec<Alert>,
    pub alert_history: AlertHistory,
    pub alert_effectiveness: AlertEffectiveness,
    pub escalation_metrics: EscalationMetrics,
    pub notification_metrics: NotificationMetrics,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    pub alert_id: String,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub triggered_timestamp: super::Timestamp,
    pub acknowledged: bool,
    pub acknowledged_by: Option<String>,
    pub resolved: bool,
    pub resolution_timestamp: Option<super::Timestamp>,
    pub affected_components: Vec<String>,
    pub correlation_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlertType {
    Performance,
    Error,
    Security,
    Capacity,
    Availability,
    Compliance,
    Anomaly,
    Threshold,
    Predictive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
    Emergency,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertHistory {
    pub total_alerts_24h: u32,
    pub total_alerts_7d: u32,
    pub total_alerts_30d: u32,
    pub alert_frequency_trend: super::TrendDirection,
    pub most_common_alert_types: Vec<(AlertType, u32)>,
    pub alert_resolution_stats: AlertResolutionStats,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertResolutionStats {
    pub average_resolution_time_minutes: f32,
    pub median_resolution_time_minutes: f32,
    pub resolution_rate_percentage: f32,
    pub false_positive_rate: f32,
    pub escalation_rate: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertEffectiveness {
    pub precision: f32, // True positive / (True positive + False positive)
    pub recall: f32,    // True positive / (True positive + False negative)
    pub f1_score: f32,
    pub lead_time_accuracy: f32, // How early alerts are triggered before issues
    pub noise_level: f32, // Amount of alert noise/spam
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EscalationMetrics {
    pub escalation_frequency: f32,
    pub escalation_success_rate: f32,
    pub average_escalation_time_minutes: f32,
    pub escalation_paths: Vec<EscalationPath>,
    pub escalation_effectiveness: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EscalationPath {
    pub path_name: String,
    pub trigger_conditions: Vec<String>,
    pub escalation_steps: Vec<EscalationStep>,
    pub success_rate: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EscalationStep {
    pub step_order: u32,
    pub responsible_party: String,
    pub response_time_sla_minutes: u32,
    pub action_required: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationMetrics {
    pub notification_delivery_rate: f32,
    pub notification_response_rate: f32,
    pub average_response_time_minutes: f32,
    pub notification_channels: Vec<NotificationChannel>,
    pub delivery_failures: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationChannel {
    pub channel_name: String,
    pub channel_type: ChannelType,
    pub delivery_success_rate: f32,
    pub average_delivery_time_seconds: f32,
    pub preferred_severity_levels: Vec<AlertSeverity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChannelType {
    Email,
    SMS,
    Push,
    Slack,
    PagerDuty,
    Webhook,
    Phone,
}

/// Diagnostic metrics for troubleshooting
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticMetrics {
    pub diagnostic_tests: Vec<DiagnosticTest>,
    pub health_checks: Vec<HealthCheck>,
    pub performance_profiling: PerformanceProfiling,
    pub resource_analysis: ResourceAnalysis,
    pub dependency_analysis: DependencyAnalysis,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticTest {
    pub test_name: String,
    pub test_type: DiagnosticTestType,
    pub last_run_timestamp: super::Timestamp,
    pub test_result: TestResult,
    pub execution_time_ms: u64,
    pub test_data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiagnosticTestType {
    Connectivity,
    Performance,
    Functional,
    Security,
    Compliance,
    Integration,
    LoadTest,
    StressTest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestResult {
    Pass,
    Fail,
    Warning,
    Inconclusive,
    NotRun,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthCheck {
    pub check_name: String,
    pub check_frequency_seconds: u32,
    pub last_check_timestamp: super::Timestamp,
    pub check_status: HealthCheckStatus,
    pub response_time_ms: f32,
    pub check_details: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealthCheckStatus {
    Healthy,
    Unhealthy,
    Degraded,
    Timeout,
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceProfiling {
    pub cpu_profiling: CPUProfiling,
    pub memory_profiling: MemoryProfiling,
    pub io_profiling: IOProfiling,
    pub network_profiling: NetworkProfiling,
    pub application_profiling: ApplicationProfiling,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CPUProfiling {
    pub hotspots: Vec<CPUHotspot>,
    pub function_call_frequency: HashMap<String, u64>,
    pub cpu_usage_by_thread: HashMap<String, f32>,
    pub context_switch_overhead: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CPUHotspot {
    pub function_name: String,
    pub cpu_percentage: f32,
    pub call_count: u64,
    pub average_execution_time_us: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryProfiling {
    pub memory_allocations: Vec<MemoryAllocation>,
    pub garbage_collection_impact: GCImpact,
    pub memory_leaks: Vec<MemoryLeak>,
    pub heap_fragmentation: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryAllocation {
    pub allocation_site: String,
    pub allocated_bytes: u64,
    pub allocation_frequency: u64,
    pub deallocation_frequency: u64,
    pub leak_probability: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GCImpact {
    pub gc_pause_time_ms: f32,
    pub gc_frequency_per_minute: f32,
    pub gc_cpu_overhead_percentage: f32,
    pub gc_memory_overhead_percentage: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryLeak {
    pub leak_source: String,
    pub estimated_leak_rate_mb_per_hour: f32,
    pub confidence: f32,
    pub impact_severity: MemoryLeakSeverity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MemoryLeakSeverity {
    Minor,
    Moderate,
    Severe,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IOProfiling {
    pub disk_io_patterns: Vec<IOPattern>,
    pub network_io_patterns: Vec<IOPattern>,
    pub io_wait_times: HashMap<String, f32>,
    pub io_bottlenecks: Vec<IOBottleneck>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IOPattern {
    pub operation_type: IOOperationType,
    pub frequency: u64,
    pub average_size_bytes: u64,
    pub average_latency_ms: f32,
    pub sequential_vs_random_ratio: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IOOperationType {
    DiskRead,
    DiskWrite,
    NetworkRead,
    NetworkWrite,
    DatabaseQuery,
    CacheAccess,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IOBottleneck {
    pub bottleneck_location: String,
    pub bottleneck_type: IOBottleneckType,
    pub impact_percentage: f32,
    pub suggested_optimizations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IOBottleneckType {
    DiskIOPS,
    DiskThroughput,
    NetworkBandwidth,
    NetworkLatency,
    DatabaseConnection,
    CacheMiss,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkProfiling {
    pub connection_patterns: Vec<ConnectionPattern>,
    pub bandwidth_utilization: BandwidthUtilization,
    pub protocol_analysis: ProtocolAnalysis,
    pub network_errors: NetworkErrorAnalysis,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionPattern {
    pub connection_type: String,
    pub connection_frequency: u64,
    pub average_duration_seconds: f32,
    pub data_transfer_patterns: DataTransferPattern,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataTransferPattern {
    pub upload_pattern: TransferPattern,
    pub download_pattern: TransferPattern,
    pub bidirectional_efficiency: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferPattern {
    pub average_size_mb: f32,
    pub peak_rate_mbps: f32,
    pub sustained_rate_mbps: f32,
    pub burst_characteristics: BurstCharacteristics,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BurstCharacteristics {
    pub burst_frequency: f32,
    pub burst_duration_seconds: f32,
    pub burst_intensity_multiplier: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BandwidthUtilization {
    pub peak_utilization_percentage: f32,
    pub average_utilization_percentage: f32,
    pub utilization_efficiency: f32,
    pub congestion_events: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtocolAnalysis {
    pub protocol_distribution: HashMap<String, f32>,
    pub protocol_efficiency: HashMap<String, f32>,
    pub overhead_analysis: ProtocolOverhead,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtocolOverhead {
    pub header_overhead_percentage: f32,
    pub encryption_overhead_percentage: f32,
    pub compression_efficiency: f32,
    pub retransmission_overhead: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkErrorAnalysis {
    pub error_types: HashMap<String, u64>,
    pub error_patterns: Vec<NetworkErrorPattern>,
    pub recovery_effectiveness: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkErrorPattern {
    pub error_type: String,
    pub frequency_pattern: TemporalPattern,
    pub correlation_factors: Vec<CorrelationFactor>,
    pub impact_assessment: NetworkErrorImpact,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkErrorImpact {
    pub user_experience_impact: f32,
    pub throughput_impact: f32,
    pub reliability_impact: f32,
    pub cost_impact: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProfiling {
    pub request_patterns: Vec<RequestPattern>,
    pub response_patterns: Vec<ResponsePattern>,
    pub business_logic_performance: BusinessLogicMetrics,
    pub user_interaction_patterns: UserInteractionMetrics,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestPattern {
    pub endpoint: String,
    pub request_frequency: u64,
    pub average_processing_time_ms: f32,
    pub resource_consumption: RequestResourceConsumption,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestResourceConsumption {
    pub cpu_time_ms: f32,
    pub memory_allocation_mb: f32,
    pub io_operations: u32,
    pub database_queries: u32,
    pub external_service_calls: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponsePattern {
    pub response_size_distribution: SizeDistribution,
    pub response_time_distribution: TimeDistribution,
    pub error_rate_by_endpoint: HashMap<String, f32>,
    pub caching_effectiveness: CachingMetrics,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SizeDistribution {
    pub min_size_bytes: u64,
    pub max_size_bytes: u64,
    pub average_size_bytes: u64,
    pub percentile_95_bytes: u64,
    pub size_variance: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeDistribution {
    pub min_time_ms: f32,
    pub max_time_ms: f32,
    pub average_time_ms: f32,
    pub percentile_95_ms: f32,
    pub time_variance: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CachingMetrics {
    pub cache_hit_rate: f32,
    pub cache_miss_rate: f32,
    pub cache_eviction_rate: f32,
    pub cache_efficiency_score: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BusinessLogicMetrics {
    pub transaction_success_rate: f32,
    pub business_rule_execution_time: HashMap<String, f32>,
    pub workflow_completion_rate: f32,
    pub data_consistency_score: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserInteractionMetrics {
    pub session_duration_minutes: f32,
    pub actions_per_session: f32,
    pub user_satisfaction_indicators: UserSatisfactionMetrics,
    pub interaction_efficiency: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSatisfactionMetrics {
    pub bounce_rate: f32,
    pub task_completion_rate: f32,
    pub error_encounter_rate: f32,
    pub feature_adoption_rate: f32,
}

/// Resource analysis for capacity planning
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceAnalysis {
    pub current_utilization: ResourceUtilizationAnalysis,
    pub capacity_planning: CapacityPlanningAnalysis,
    pub resource_optimization: ResourceOptimizationAnalysis,
    pub cost_analysis: CostAnalysis,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceUtilizationAnalysis {
    pub cpu_utilization_patterns: UtilizationPattern,
    pub memory_utilization_patterns: UtilizationPattern,
    pub storage_utilization_patterns: UtilizationPattern,
    pub network_utilization_patterns: UtilizationPattern,
    pub resource_correlation_analysis: Vec<ResourceCorrelation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilizationPattern {
    pub peak_hours: Vec<u8>, // Hours of day (0-23)
    pub seasonal_patterns: Vec<SeasonalPattern>,
    pub trend_analysis: TrendAnalysis,
    pub anomaly_detection: UtilizationAnomalies,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeasonalPattern {
    pub pattern_type: SeasonalPatternType,
    pub amplitude: f32,
    pub confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SeasonalPatternType {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
    Holiday,
    Event,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub trend_direction: super::TrendDirection,
    pub trend_strength: f32,
    pub trend_duration_days: u32,
    pub projection_confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilizationAnomalies {
    pub anomaly_count: u32,
    pub anomaly_severity_distribution: HashMap<AnomalySeverity, u32>,
    pub most_recent_anomaly: Option<super::Timestamp>,
    pub anomaly_prediction_accuracy: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceCorrelation {
    pub resource1: String,
    pub resource2: String,
    pub correlation_coefficient: f32,
    pub causality_direction: CausalityDirection,
    pub lag_time_minutes: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CausalityDirection {
    Resource1CausesResource2,
    Resource2CausesResource1,
    Bidirectional,
    NoDirectCausality,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapacityPlanningAnalysis {
    pub current_capacity_headroom: f32, // Percentage
    pub projected_capacity_needs: CapacityProjection,
    pub scaling_recommendations: Vec<ScalingRecommendation>,
    pub bottleneck_predictions: Vec<BottleneckPrediction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapacityProjection {
    pub projection_horizon_months: u32,
    pub projected_growth_rate: f32,
    pub confidence_interval: (f32, f32), // Lower and upper bounds
    pub resource_projections: HashMap<String, ResourceProjection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceProjection {
    pub current_usage: f32,
    pub projected_usage: f32,
    pub time_to_capacity_limit: Option<u32>, // Days
    pub recommended_action_timeline: Option<u32>, // Days
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScalingRecommendation {
    pub resource_type: String,
    pub scaling_type: ScalingType,
    pub recommended_increase_percentage: f32,
    pub cost_impact: f32,
    pub implementation_timeline_days: u32,
    pub risk_assessment: ScalingRisk,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScalingType {
    Vertical,
    Horizontal,
    Storage,
    Network,
    Hybrid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScalingRisk {
    pub technical_risk: f32,
    pub operational_risk: f32,
    pub financial_risk: f32,
    pub mitigation_strategies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BottleneckPrediction {
    pub resource_type: String,
    pub predicted_bottleneck_time: super::Timestamp,
    pub confidence: f32,
    pub impact_severity: f32,
    pub prevention_actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceOptimizationAnalysis {
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
    pub waste_reduction_potential: WasteReductionAnalysis,
    pub efficiency_improvements: Vec<EfficiencyImprovement>,
    pub cost_optimization_potential: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptimizationOpportunity {
    pub opportunity_type: OptimizationType,
    pub description: String,
    pub potential_savings_percentage: f32,
    pub implementation_effort: ImplementationEffort,
    pub payback_period_months: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OptimizationType {
    ResourceRightSizing,
    LoadBalancing,
    Caching,
    Compression,
    Deduplication,
    SchedulingOptimization,
    AlgorithmOptimization,
    ArchitectureOptimization,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImplementationEffort {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WasteReductionAnalysis {
    pub underutilized_resources: Vec<UnderutilizedResource>,
    pub overprovisioned_resources: Vec<OverprovisionedResource>,
    pub total_waste_percentage: f32,
    pub waste_cost_estimate: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnderutilizedResource {
    pub resource_name: String,
    pub current_utilization: f32,
    pub optimal_utilization: f32,
    pub waste_amount: f32,
    pub reallocation_suggestions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverprovisionedResource {
    pub resource_name: String,
    pub current_capacity: f32,
    pub actual_need: f32,
    pub overprovisioning_percentage: f32,
    pub downsizing_recommendation: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EfficiencyImprovement {
    pub improvement_area: String,
    pub current_efficiency: f32,
    pub target_efficiency: f32,
    pub improvement_actions: Vec<String>,
    pub expected_timeline_weeks: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostAnalysis {
    pub current_cost_breakdown: CostBreakdown,
    pub cost_trends: CostTrends,
    pub cost_optimization_opportunities: Vec<CostOptimizationOpportunity>,
    pub budget_variance_analysis: BudgetVarianceAnalysis,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostBreakdown {
    pub compute_costs: f32,
    pub storage_costs: f32,
    pub network_costs: f32,
    pub licensing_costs: f32,
    pub operational_costs: f32,
    pub total_costs: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostTrends {
    pub monthly_cost_change_percentage: f32,
    pub cost_per_unit_trend: super::TrendDirection,
    pub cost_efficiency_trend: super::TrendDirection,
    pub projected_annual_cost: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostOptimizationOpportunity {
    pub opportunity_name: String,
    pub potential_savings_monthly: f32,
    pub implementation_cost: f32,
    pub roi_months: f32,
    pub risk_level: ImplementationRisk,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImplementationRisk {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BudgetVarianceAnalysis {
    pub budgeted_amount: f32,
    pub actual_amount: f32,
    pub variance_percentage: f32,
    pub variance_explanation: String,
    pub forecast_accuracy: f32,
}

/// Dependency analysis for service relationships
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DependencyAnalysis {
    pub service_dependencies: Vec<ServiceDependency>,
    pub dependency_health_scores: HashMap<String, f32>,
    pub critical_path_analysis: CriticalPathAnalysis,
    pub failure_impact_analysis: FailureImpactAnalysis,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceDependency {
    pub service_name: String,
    pub dependency_type: DependencyType,
    pub criticality: DependencyCriticality,
    pub failure_tolerance: f32,
    pub circuit_breaker_status: CircuitBreakerStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DependencyType {
    Synchronous,
    Asynchronous,
    Database,
    Cache,
    ExternalAPI,
    MessageQueue,
    FileSystem,
    Network,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DependencyCriticality {
    Critical,
    Important,
    Moderate,
    Low,
    Optional,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CircuitBreakerStatus {
    Closed,
    Open,
    HalfOpen,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CriticalPathAnalysis {
    pub critical_dependencies: Vec<String>,
    pub path_reliability: f32,
    pub bottleneck_services: Vec<String>,Eq, Serialize, Deserialize)]
pub struct HealthMetrics {
    pub health_indicators: HealthIndicators,
    pub error_metrics: ErrorMetrics,
    pub alert_metrics: AlertMetrics,
    pub diagnostic_metrics: DiagnosticMetrics,
    pub monitoring_metrics: MonitoringMetrics,
}

/// System health indicators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthIndicators {
    pub overall_health_score: f32, // 0.0 to 1.0
    pub component_health_scores: HashMap<String, f32>,
    pub critical_alerts: u32,
    pub warning_alerts: u32,
    pub health_trends: HealthTrends,
    pub system_stability_score: f32,
    pub performance_degradation_detected: bool,
    pub anomaly_detection_results: Vec<AnomalyDetectionResult>,
    pub service_dependencies_health: Vec<DependencyHealth>,
    pub compliance_status: ComplianceStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthTrends {
    pub improving: bool,
    pub stable: bool,
    pub degrading: bool,
    pub volatile: bool,
    pub trend_confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnomalyDetectionResult {
    pub anomaly_type: String,
    pub severity: AnomalySeverity,
    pub confidence: f32,
    pub affected_metrics: Vec<String>,
    pub potential_causes: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub detection_timestamp: super::Timestamp,
    pub anomaly_duration: Option<u64>, // seconds
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DependencyHealth {
    pub service_name: String,
    pub health_status: ServiceHealthStatus,
    pub response_time_ms: f32,
    pub availability_percentage: f32,
    pub last_check_timestamp: super::Timestamp,
    pub failure_count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServiceHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
    Unreachable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplianceStatus {
    pub sla_compliance: f32, // 0.0 to 1.0
    pub security_compliance: f32,
    pub regulatory_compliance: f32,
    pub data_protection_compliance: f32,
    pub audit_findings: Vec<AuditFinding>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditFinding {
    pub finding_id: String,
    pub severity: AuditSeverity,
    pub description: String,
    pub remediation_required: bool,
    pub deadline: Option<super::Timestamp>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuditSeverity {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

/// Error performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorMetrics {
    pub total_errors: u64,
    pub error_rate: f32, // errors per second
    pub error_types: HashMap<String, u64>,
    pub error_severity_distribution: HashMap<String, u64>,
    pub mean_time_to_detection: f32, // seconds
    pub mean_time_to_resolution: f32, // seconds
    pub recurring_errors: Vec<RecurringError>,
    pub error_trends: ErrorTrends,
    pub error_impact_analysis: ErrorImpactAnalysis,
}

#[derive(Debug, Clone, Partial