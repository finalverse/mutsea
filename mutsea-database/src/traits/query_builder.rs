// mutsea-database/src/traits/query_builder.rs

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use uuid::Uuid;

use crate::error::DatabaseResult;
use crate::models::*;

/// Core trait for building and executing database queries
#[async_trait]
pub trait QueryBuilder: Send + Sync + Debug {
    /// Execute a raw SQL query with parameters
    async fn execute_raw(&self, sql: &str, params: &[QueryParam]) -> DatabaseResult<QueryResult>;
    
    /// Execute a query and return a single result
    async fn fetch_one<T>(&self, sql: &str, params: &[QueryParam]) -> DatabaseResult<T>
    where
        T: for<'de> Deserialize<'de> + Send;
    
    /// Execute a query and return multiple results
    async fn fetch_all<T>(&self, sql: &str, params: &[QueryParam]) -> DatabaseResult<Vec<T>>
    where
        T: for<'de> Deserialize<'de> + Send;
    
    /// Execute a query and return an optional result
    async fn fetch_optional<T>(&self, sql: &str, params: &[QueryParam]) -> DatabaseResult<Option<T>>
    where
        T: for<'de> Deserialize<'de> + Send;
    
    /// Begin a database transaction
    async fn begin_transaction(&self) -> DatabaseResult<Box<dyn Transaction>>;
    
    /// Get the database dialect for SQL generation
    fn dialect(&self) -> DatabaseDialect;
}

/// Transaction interface for atomic operations
#[async_trait]
pub trait Transaction: Send + Sync {
    /// Execute a query within the transaction
    async fn execute(&mut self, sql: &str, params: &[QueryParam]) -> DatabaseResult<QueryResult>;
    
    /// Fetch one result within the transaction
    async fn fetch_one<T>(&mut self, sql: &str, params: &[QueryParam]) -> DatabaseResult<T>
    where
        T: for<'de> Deserialize<'de> + Send;
    
    /// Fetch multiple results within the transaction
    async fn fetch_all<T>(&mut self, sql: &str, params: &[QueryParam]) -> DatabaseResult<Vec<T>>
    where
        T: for<'de> Deserialize<'de> + Send;
    
    /// Commit the transaction
    async fn commit(self: Box<Self>) -> DatabaseResult<()>;
    
    /// Rollback the transaction
    async fn rollback(self: Box<Self>) -> DatabaseResult<()>;
}

/// Trait for building AI-specific queries
#[async_trait]
pub trait AIQueryBuilder: QueryBuilder {
    /// Query AI decision patterns
    async fn ai_decision_patterns(
        &self,
        criteria: &AIDecisionCriteria,
    ) -> DatabaseResult<Vec<AIDecisionPattern>>;
    
    /// Query learning data for ML pipeline
    async fn learning_data_batch(
        &self,
        batch_size: usize,
        offset: usize,
    ) -> DatabaseResult<Vec<LearningData>>;
    
    /// Store AI decision feedback
    async fn store_ai_feedback(
        &self,
        feedback: &[AIFeedback],
    ) -> DatabaseResult<()>;
    
    /// Query emergent behavior patterns
    async fn emergent_behavior_analysis(
        &self,
        time_window: TimeWindow,
    ) -> DatabaseResult<Vec<EmergentBehaviorAnalysis>>;
    
    /// Performance optimization queries
    async fn performance_bottlenecks(
        &self,
        threshold: f64,
    ) -> DatabaseResult<Vec<PerformanceBottleneck>>;
}

/// Trait for building world state queries
#[async_trait]
pub trait WorldQueryBuilder: QueryBuilder {
    /// Query world states by region
    async fn world_states_by_region(
        &self,
        region_id: Uuid,
        time_range: TimeRange,
    ) -> DatabaseResult<Vec<WorldState>>;
    
    /// Query biome distributions
    async fn biome_distribution(
        &self,
        world_id: Uuid,
    ) -> DatabaseResult<Vec<BiomeDistribution>>;
    
    /// Store batch world state updates
    async fn batch_update_world_states(
        &self,
        updates: &[WorldStateUpdate],
    ) -> DatabaseResult<()>;
    
    /// Query ecosystem health metrics
    async fn ecosystem_health(
        &self,
        ecosystem_id: Uuid,
    ) -> DatabaseResult<EcosystemHealth>;
    
    /// Query resource distribution patterns
    async fn resource_patterns(
        &self,
        resource_type: ResourceType,
        region: Region,
    ) -> DatabaseResult<Vec<ResourcePattern>>;
}

/// Trait for building player behavior queries
#[async_trait]
pub trait PlayerQueryBuilder: QueryBuilder {
    /// Query player behavior patterns
    async fn player_behavior_patterns(
        &self,
        player_id: Uuid,
        time_window: TimeWindow,
    ) -> DatabaseResult<Vec<PlayerBehaviorPattern>>;
    
    /// Store player action batch
    async fn batch_store_player_actions(
        &self,
        actions: &[PlayerAction],
    ) -> DatabaseResult<()>;
    
    /// Query player preferences
    async fn player_preferences(
        &self,
        player_id: Uuid,
    ) -> DatabaseResult<PlayerPreferences>;
    
    /// Query social interaction patterns
    async fn social_interaction_patterns(
        &self,
        players: &[Uuid],
        interaction_types: &[InteractionType],
    ) -> DatabaseResult<Vec<SocialPattern>>;
    
    /// Player psychology analysis
    async fn player_psychology_profile(
        &self,
        player_id: Uuid,
    ) -> DatabaseResult<PlayerPsychologyProfile>;
}

/// Trait for building NPC and agent queries
#[async_trait]
pub trait NPCQueryBuilder: QueryBuilder {
    /// Query NPC states by region
    async fn npc_states_by_region(
        &self,
        region_id: Uuid,
    ) -> DatabaseResult<Vec<NPCState>>;
    
    /// Store NPC learning updates
    async fn update_npc_learning(
        &self,
        npc_id: Uuid,
        learning_update: &NPCLearningUpdate,
    ) -> DatabaseResult<()>;
    
    /// Query NPC relationship networks
    async fn npc_relationship_network(
        &self,
        npc_id: Uuid,
        depth: u32,
    ) -> DatabaseResult<RelationshipNetwork>;
    
    /// Batch update NPC personalities
    async fn batch_update_npc_personalities(
        &self,
        updates: &[NPCPersonalityUpdate],
    ) -> DatabaseResult<()>;
    
    /// Query NPC behavioral evolution
    async fn npc_behavioral_evolution(
        &self,
        npc_id: Uuid,
        time_range: TimeRange,
    ) -> DatabaseResult<Vec<BehavioralEvolution>>;
}

/// Trait for analytics and insights queries
#[async_trait]
pub trait AnalyticsQueryBuilder: QueryBuilder {
    /// Generate performance insights
    async fn performance_insights(
        &self,
        criteria: &PerformanceAnalysisCriteria,
    ) -> DatabaseResult<PerformanceInsights>;
    
    /// Player engagement analytics
    async fn player_engagement_metrics(
        &self,
        time_window: TimeWindow,
    ) -> DatabaseResult<Vec<EngagementMetric>>;
    
    /// AI effectiveness analysis
    async fn ai_effectiveness_analysis(
        &self,
        ai_system: AISystemType,
        time_range: TimeRange,
    ) -> DatabaseResult<AIEffectivenessReport>;
    
    /// World generation quality metrics
    async fn world_generation_metrics(
        &self,
        generation_batch_id: Uuid,
    ) -> DatabaseResult<WorldGenerationMetrics>;
    
    /// Emergent behavior detection
    async fn detect_emergent_behaviors(
        &self,
        detection_params: &EmergentDetectionParams,
    ) -> DatabaseResult<Vec<EmergentBehavior>>;
}

/// Query parameter types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryParam {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Uuid(Uuid),
    Json(serde_json::Value),
    Binary(Vec<u8>),
    Null,
}

/// Generic query result
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub rows_affected: u64,
    pub last_insert_id: Option<i64>,
    pub metadata: HashMap<String, String>,
}

/// Database dialect enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseDialect {
    PostgreSQL,
    SQLite,
    MySQL,
}

/// Time-based query criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
    pub granularity: TimeGranularity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeGranularity {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
}

/// AI-specific query structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIDecisionCriteria {
    pub decision_types: Vec<AIDecisionType>,
    pub confidence_threshold: f64,
    pub time_window: TimeWindow,
    pub context_filters: HashMap<String, QueryParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIDecisionPattern {
    pub pattern_id: Uuid,
    pub decision_type: AIDecisionType,
    pub frequency: f64,
    pub success_rate: f64,
    pub context_similarity: f64,
    pub confidence_distribution: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFeedback {
    pub decision_id: Uuid,
    pub outcome_rating: f64,
    pub player_satisfaction: Option<f64>,
    pub performance_impact: Option<f64>,
    pub context_relevance: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentBehaviorAnalysis {
    pub behavior_id: Uuid,
    pub emergence_strength: f64,
    pub participants: Vec<Uuid>,
    pub behavior_type: EmergentBehaviorType,
    pub duration: chrono::Duration,
    pub impact_radius: f64,
    pub novelty_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBottleneck {
    pub component: String,
    pub bottleneck_type: BottleneckType,
    pub severity: f64,
    pub frequency: f64,
    pub suggested_optimization: String,
    pub estimated_improvement: f64,
}

/// World state query structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeDistribution {
    pub biome_type: BiomeType,
    pub coverage_percentage: f64,
    pub health_index: f64,
    pub species_diversity: u32,
    pub resource_density: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldStateUpdate {
    pub region_id: Uuid,
    pub state_data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub update_source: UpdateSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemHealth {
    pub ecosystem_id: Uuid,
    pub overall_health: f64,
    pub biodiversity_index: f64,
    pub stability_score: f64,
    pub resilience_rating: f64,
    pub threat_level: ThreatLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePattern {
    pub resource_type: ResourceType,
    pub abundance_level: f64,
    pub distribution_type: DistributionType,
    pub regeneration_rate: f64,
    pub quality_rating: f64,
    pub accessibility_score: f64,
}

/// Player behavior query structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBehaviorPattern {
    pub pattern_type: BehaviorPatternType,
    pub frequency: f64,
    pub intensity: f64,
    pub duration_average: chrono::Duration,
    pub success_rate: f64,
    pub preference_strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPreferences {
    pub player_id: Uuid,
    pub gameplay_style: GameplayStyle,
    pub difficulty_preference: DifficultyLevel,
    pub content_preferences: Vec<ContentType>,
    pub social_preferences: SocialPreferences,
    pub ai_assistance_level: AIAssistanceLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialPattern {
    pub interaction_type: InteractionType,
    pub frequency: f64,
    pub participants: Vec<Uuid>,
    pub success_rate: f64,
    pub satisfaction_rating: f64,
    pub network_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPsychologyProfile {
    pub player_id: Uuid,
    pub personality_traits: HashMap<PersonalityTrait, f64>,
    pub motivation_factors: HashMap<MotivationFactor, f64>,
    pub stress_indicators: Vec<StressIndicator>,
    pub engagement_patterns: Vec<EngagementPattern>,
    pub learning_style: LearningStyle,
}

/// NPC and agent query structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCLearningUpdate {
    pub learning_type: LearningType,
    pub experience_data: serde_json::Value,
    pub skill_improvements: HashMap<String, f64>,
    pub behavior_adaptations: Vec<BehaviorAdaptation>,
    pub memory_updates: Vec<MemoryUpdate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipNetwork {
    pub central_npc: Uuid,
    pub relationships: Vec<NPCRelationship>,
    pub network_density: f64,
    pub influence_score: f64,
    pub social_clusters: Vec<SocialCluster>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCPersonalityUpdate {
    pub npc_id: Uuid,
    pub personality_changes: HashMap<PersonalityTrait, f64>,
    pub behavioral_shifts: Vec<BehaviorShift>,
    pub adaptation_trigger: AdaptationTrigger,
    pub confidence_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralEvolution {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub behavior_type: BehaviorType,
    pub evolution_strength: f64,
    pub adaptation_reason: String,
    pub success_metrics: HashMap<String, f64>,
}

/// Analytics query structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysisCriteria {
    pub components: Vec<String>,
    pub metrics: Vec<PerformanceMetricType>,
    pub time_window: TimeWindow,
    pub threshold_values: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceInsights {
    pub overall_score: f64,
    pub component_scores: HashMap<String, f64>,
    pub bottlenecks: Vec<PerformanceBottleneck>,
    pub optimization_recommendations: Vec<OptimizationRecommendation>,
    pub trend_analysis: TrendAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementMetric {
    pub metric_type: EngagementMetricType,
    pub value: f64,
    pub trend: TrendDirection,
    pub confidence_interval: (f64, f64),
    pub contributing_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIEffectivenessReport {
    pub system_type: AISystemType,
    pub overall_effectiveness: f64,
    pub decision_accuracy: f64,
    pub player_satisfaction_impact: f64,
    pub performance_impact: f64,
    pub improvement_suggestions: Vec<ImprovementSuggestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldGenerationMetrics {
    pub generation_time: chrono::Duration,
    pub quality_score: f64,
    pub diversity_index: f64,
    pub player_satisfaction: Option<f64>,
    pub performance_impact: f64,
    pub resource_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentDetectionParams {
    pub detection_algorithms: Vec<DetectionAlgorithm>,
    pub sensitivity_level: f64,
    pub time_window: TimeWindow,
    pub context_filters: HashMap<String, QueryParam>,
    pub minimum_participants: u32,
}

/// Supporting enums and types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIDecisionType {
    WorldGeneration,
    NPCBehavior,
    PlayerAssistance,
    ResourceAllocation,
    QuestGeneration,
    DifficultyAdjustment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergentBehaviorType {
    SocialFormation,
    EconomicPattern,
    EcosystemShift,
    PlayerCollaboration,
    NPCEvolution,
    WorldAdaptation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckType {
    CPU,
    Memory,
    Database,
    Network,
    AI_Processing,
    Rendering,
}

// Additional enums and supporting types would continue here...
// This provides the core structure for the query builder traits