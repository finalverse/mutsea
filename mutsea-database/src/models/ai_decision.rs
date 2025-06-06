// mutsea-database/src/models/ai_decision.rs
//! AI decision models for tracking and analyzing AI system decisions
//! 
//! These models capture the decision-making process of AI systems,
//! their reasoning, outcomes, and learning feedback for continuous improvement.

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI decision record with full context and reasoning
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIDecision {
    pub id: EntityId,
    pub decision_id: Uuid,
    pub ai_system_id: EntityId,
    pub decision_timestamp: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    
    // Decision context
    pub decision_type: AIDecisionType,
    pub decision_domain: DecisionDomain,
    pub input_context: DecisionInputContext,
    pub decision_urgency: DecisionUrgency,
    
    // AI reasoning process
    pub reasoning_chain: Vec<ReasoningStep>,
    pub considered_alternatives: Vec<DecisionAlternative>,
    pub selected_decision: SelectedDecision,
    pub confidence_score: f32, // 0.0 to 1.0
    
    // Decision execution
    pub execution_plan: ExecutionPlan,
    pub execution_status: ExecutionStatus,
    pub execution_duration_ms: Option<u64>,
    
    // Outcome tracking
    pub predicted_outcome: PredictedOutcome,
    pub actual_outcome: Option<ActualOutcome>,
    pub outcome_variance: Option<f32>, // Difference between predicted and actual
    
    // Learning and feedback
    pub feedback_score: Option<f32>, // -1.0 to 1.0
    pub learning_value: f32, // How much this decision contributes to learning
    pub error_analysis: Option<ErrorAnalysis>,
    
    // Performance metrics
    pub decision_time_ms: u64,
    pub computational_cost: ComputationalCost,
    pub resource_usage: ResourceUsage,
    
    // Related decisions
    pub parent_decision_id: Option<EntityId>,
    pub child_decisions: Vec<EntityId>,
    pub influenced_by: Vec<EntityId>,
    pub influences: Vec<EntityId>,
    
    // Metadata
    pub metadata: EntityMetadata,
}

/// Types of AI decisions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AIDecisionType {
    // World generation decisions
    WorldGeneration {
        generation_type: String,
        scope: GenerationScope,
        parameters: HashMap<String, f32>,
    },
    
    // NPC behavior decisions
    NPCBehavior {
        npc_id: NPCId,
        behavior_type: String,
        social_context: bool,
    },
    
    // Player assistance decisions
    PlayerAssistance {
        player_id: PlayerId,
        assistance_type: String,
        intervention_level: f32,
    },
    
    // Resource management decisions
    ResourceManagement {
        resource_type: String,
        allocation_strategy: String,
        efficiency_target: f32,
    },
    
    // Story generation decisions
    StoryGeneration {
        narrative_type: String,
        target_audience: Vec<PlayerId>,
        complexity_level: f32,
    },
    
    // Ecosystem management decisions
    EcosystemManagement {
        biome_id: BiomeId,
        intervention_type: String,
        impact_scope: f32,
    },
    
    // Performance optimization decisions
    PerformanceOptimization {
        optimization_target: String,
        trade_offs: Vec<String>,
        expected_improvement: f32,
    },
    
    // Emergency response decisions
    EmergencyResponse {
        emergency_type: String,
        severity: f32,
        response_speed: ResponseSpeed,
    },
    
    // Learning and adaptation decisions
    LearningAdaptation {
        learning_type: String,
        adaptation_scope: String,
        confidence_threshold: f32,
    },
    
    // Custom AI-generated decision types
    AIGenerated {
        decision_name: String,
        decision_category: String,
        parameters: HashMap<String, f32>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GenerationScope {
    Local,
    Regional,
    Global,
    Contextual,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseSpeed {
    Immediate,   // < 100ms
    Fast,        // < 1s
    Moderate,    // < 10s
    Deliberate,  // > 10s
}

/// Domain classification for AI decisions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecisionDomain {
    WorldManagement,
    PlayerExperience,
    NPCIntelligence,
    ResourceOptimization,
    StoryNarrative,
    EcosystemBalance,
    PerformanceManagement,
    SocialDynamics,
    EmergencyHandling,
    LearningSystem,
    CreativeGeneration,
    SystemMaintenance,
}

/// Urgency level of the decision
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecisionUrgency {
    Critical,    // Immediate action required
    High,        // Quick response needed
    Medium,      // Standard processing time
    Low,         // Can be deferred
    Background,  // Process when resources available
}

/// Input context for AI decision making
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionInputContext {
    pub world_state_snapshot: WorldStateSnapshot,
    pub player_states: Vec<PlayerStateSnapshot>,
    pub npc_states: Vec<NPCStateSnapshot>,
    pub system_metrics: SystemMetricsSnapshot,
    pub environmental_factors: EnvironmentalFactors,
    pub temporal_context: TemporalContext,
    pub historical_context: Vec<EntityId>, // Reference to related past decisions
    pub constraint_set: ConstraintSet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldStateSnapshot {
    pub timestamp: Timestamp,
    pub key_metrics: HashMap<String, f32>,
    pub active_events: Vec<String>,
    pub resource_levels: HashMap<String, f32>,
    pub stability_indicators: HashMap<String, f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerStateSnapshot {
    pub player_id: PlayerId,
    pub behavior_pattern: String,
    pub current_activity: String,
    pub satisfaction_level: f32,
    pub engagement_level: f32,
    pub needs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NPCStateSnapshot {
    pub npc_id: NPCId,
    pub current_goal: String,
    pub emotional_state: String,
    pub social_connections: u32,
    pub activity_level: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemMetricsSnapshot {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub network_latency: f32,
    pub active_connections: u32,
    pub performance_score: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemporalContext {
    pub time_of_day: TimeOfDay,
    pub season: Season,
    pub recent_events: Vec<String>,
    pub upcoming_events: Vec<String>,
    pub temporal_patterns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintSet {
    pub resource_constraints: HashMap<String, f32>,
    pub performance_constraints: HashMap<String, f32>,
    pub ethical_constraints: Vec<String>,
    pub player_preference_constraints: HashMap<PlayerId, Vec<String>>,
    pub system_limitations: Vec<String>,
}

/// Step in the AI reasoning process
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReasoningStep {
    pub step_number: u32,
    pub reasoning_type: ReasoningType,
    pub description: String,
    pub input_data: HashMap<String, f32>,
    pub processing_algorithm: String,
    pub output_data: HashMap<String, f32>,
    pub confidence: f32,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReasoningType {
    DataAnalysis,
    PatternRecognition,
    Prediction,
    Optimization,
    ConstraintSatisfaction,
    RiskAssessment,
    BenefitAnalysis,
    Simulation,
    HeuristicEvaluation,
    MachineLearning,
    NeuralNetworkInference,
    LogicalDeduction,
    CreativeGeneration,
}

/// Alternative decisions considered by AI
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionAlternative {
    pub alternative_id: u32,
    pub description: String,
    pub predicted_outcome: PredictedOutcome,
    pub confidence: f32,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
    pub risk_level: f32,
    pub resource_requirements: ResourceRequirements,
    pub rejection_reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub cpu_requirements: f32,
    pub memory_requirements: f32,
    pub network_bandwidth: f32,
    pub storage_space: f32,
    pub time_requirements_ms: u64,
    pub external_dependencies: Vec<String>,
}

/// The selected decision with full details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectedDecision {
    pub decision_description: String,
    pub selection_reasoning: String,
    pub expected_benefits: Vec<ExpectedBenefit>,
    pub acknowledged_risks: Vec<AcknowledgedRisk>,
    pub mitigation_strategies: Vec<MitigationStrategy>,
    pub success_criteria: Vec<SuccessCriterion>,
    pub fallback_plans: Vec<FallbackPlan>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpectedBenefit {
    pub benefit_type: String,
    pub description: String,
    pub quantified_value: Option<f32>,
    pub probability: f32,
    pub timeframe_hours: f32,
    pub stakeholders: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcknowledgedRisk {
    pub risk_type: String,
    pub description: String,
    pub probability: f32,
    pub impact_severity: f32,
    pub affected_systems: Vec<String>,
    pub detection_method: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MitigationStrategy {
    pub strategy_name: String,
    pub description: String,
    pub trigger_conditions: Vec<String>,
    pub implementation_steps: Vec<String>,
    pub effectiveness_rating: f32,
    pub resource_cost: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuccessCriterion {
    pub criterion_name: String,
    pub description: String,
    pub measurement_method: String,
    pub target_value: f32,
    pub evaluation_timeframe_hours: f32,
    pub importance_weight: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FallbackPlan {
    pub plan_name: String,
    pub trigger_conditions: Vec<String>,
    pub fallback_actions: Vec<String>,
    pub expected_outcome: String,
    pub activation_threshold: f32,
}

/// Execution plan for the AI decision
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub plan_id: EntityId,
    pub execution_steps: Vec<ExecutionStep>,
    pub estimated_duration_ms: u64,
    pub resource_allocation: ResourceAllocation,
    pub dependencies: Vec<ExecutionDependency>,
    pub monitoring_checkpoints: Vec<MonitoringCheckpoint>,
    pub rollback_strategy: Option<RollbackStrategy>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionStep {
    pub step_id: u32,
    pub step_name: String,
    pub description: String,
    pub estimated_duration_ms: u64,
    pub required_resources: ResourceRequirements,
    pub preconditions: Vec<String>,
    pub success_conditions: Vec<String>,
    pub failure_conditions: Vec<String>,
    pub parallel_execution: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub cpu_allocation_percent: f32,
    pub memory_allocation_mb: f32,
    pub network_bandwidth_mbps: f32,
    pub storage_allocation_mb: f32,
    pub priority_level: ExecutionPriority,
    pub shared_resources: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionPriority {
    Critical,
    High,
    Medium,
    Low,
    Background,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionDependency {
    pub dependency_type: String,
    pub dependency_id: EntityId,
    pub relationship: DependencyRelationship,
    pub blocking: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DependencyRelationship {
    RequiresBefore,
    RequiresAfter,
    RequiresConcurrent,
    RequiresCompletion,
    RequiresResource,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitoringCheckpoint {
    pub checkpoint_id: u32,
    pub checkpoint_name: String,
    pub trigger_time_ms: u64,
    pub metrics_to_check: Vec<String>,
    pub success_thresholds: HashMap<String, f32>,
    pub failure_thresholds: HashMap<String, f32>,
    pub actions_on_success: Vec<String>,
    pub actions_on_failure: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RollbackStrategy {
    pub rollback_triggers: Vec<String>,
    pub rollback_steps: Vec<String>,
    pub state_restoration_method: String,
    pub data_backup_required: bool,
    pub estimated_rollback_time_ms: u64,
}

/// Current execution status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Pending,
    InProgress {
        current_step: u32,
        progress_percent: f32,
        estimated_completion_ms: u64,
    },
    Completed {
        completion_time: Timestamp,
        success_rate: f32,
    },
    Failed {
        failure_time: Timestamp,
        failure_reason: String,
        steps_completed: u32,
    },
    Cancelled {
        cancellation_time: Timestamp,
        cancellation_reason: String,
    },
    RolledBack {
        rollback_time: Timestamp,
        rollback_reason: String,
    },
}

/// Predicted outcome of the decision
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictedOutcome {
    pub outcome_description: String,
    pub confidence_level: f32,
    pub predicted_metrics: HashMap<String, f32>,
    pub success_probability: f32,
    pub risk_factors: Vec<RiskFactor>,
    pub side_effects: Vec<SideEffect>,
    pub long_term_implications: Vec<LongTermImplication>,
    pub prediction_model: String,
    pub prediction_timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor_name: String,
    pub description: String,
    pub probability: f32,
    pub impact_severity: f32,
    pub mitigation_available: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SideEffect {
    pub effect_name: String,
    pub description: String,
    pub probability: f32,
    pub affected_systems: Vec<String>,
    pub duration_hours: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LongTermImplication {
    pub implication_name: String,
    pub description: String,
    pub timeframe_hours: f32,
    pub probability: f32,
    pub impact_areas: Vec<String>,
}

/// Actual outcome after execution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActualOutcome {
    pub outcome_description: String,
    pub actual_metrics: HashMap<String, f32>,
    pub success_achieved: bool,
    pub completion_time: Timestamp,
    pub deviations_from_prediction: Vec<OutcomeDeviation>,
    pub unexpected_benefits: Vec<UnexpectedBenefit>,
    pub unexpected_problems: Vec<UnexpectedProblem>,
    pub lessons_learned: Vec<LessonLearned>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutcomeDeviation {
    pub metric_name: String,
    pub predicted_value: f32,
    pub actual_value: f32,
    pub deviation_percentage: f32,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnexpectedBenefit {
    pub benefit_name: String,
    pub description: String,
    pub value_assessment: f32,
    pub discovery_method: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnexpectedProblem {
    pub problem_name: String,
    pub description: String,
    pub severity: f32,
    pub resolution_status: ProblemResolutionStatus,
    pub resolution_actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProblemResolutionStatus {
    Unresolved,
    InProgress,
    Resolved,
    Mitigated,
    Accepted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LessonLearned {
    pub lesson_name: String,
    pub description: String,
    pub applicability: Vec<String>,
    pub confidence_in_lesson: f32,
    pub integration_status: LessonIntegrationStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LessonIntegrationStatus {
    Identified,
    UnderReview,
    Integrated,
    Rejected,
}

/// Error analysis for failed or suboptimal decisions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorAnalysis {
    pub error_type: ErrorType,
    pub error_source: ErrorSource,
    pub contributing_factors: Vec<ContributingFactor>,
    pub severity_assessment: f32,
    pub impact_assessment: ImpactAssessment,
    pub corrective_actions: Vec<CorrectiveAction>,
    pub prevention_strategies: Vec<PreventionStrategy>,
    pub analysis_timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorType {
    LogicalError,
    DataError,
    ModelError,
    SystemError,
    HumanError,
    EnvironmentalError,
    UnknownError,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorSource {
    InputData,
    ProcessingAlgorithm,
    ModelParameters,
    SystemConstraints,
    ExternalFactors,
    HumanIntervention,
    HardwareFailure,
    SoftwareBug,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContributingFactor {
    pub factor_name: String,
    pub description: String,
    pub contribution_weight: f32,
    pub factor_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub primary_impact: String,
    pub secondary_impacts: Vec<String>,
    pub affected_stakeholders: Vec<String>,
    pub financial_impact: Option<f32>,
    pub performance_impact: Option<f32>,
    pub user_experience_impact: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorrectiveAction {
    pub action_name: String,
    pub description: String,
    pub implementation_timeline: String,
    pub resource_requirements: ResourceRequirements,
    pub expected_effectiveness: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreventionStrategy {
    pub strategy_name: String,
    pub description: String,
    pub implementation_scope: String,
    pub monitoring_requirements: Vec<String>,
    pub effectiveness_rating: f32,
}

/// Computational cost tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputationalCost {
    pub cpu_cycles: u64,
    pub memory_peak_mb: f32,
    pub gpu_utilization_percent: Option<f32>,
    pub network_io_bytes: u64,
    pub disk_io_bytes: u64,
    pub energy_consumption_joules: Option<f32>,
    pub cost_optimization_opportunities: Vec<String>,
}

/// AI decision effectiveness metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionEffectiveness {
    pub decision_id: EntityId,
    pub effectiveness_score: f32, // 0.0 to 1.0
    pub prediction_accuracy: f32,
    pub execution_efficiency: f32,
    pub outcome_satisfaction: f32,
    pub learning_value: f32,
    pub cost_benefit_ratio: f32,
    pub stakeholder_satisfaction: HashMap<String, f32>,
    pub comparative_analysis: Option<ComparativeAnalysis>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComparativeAnalysis {
    pub alternative_decisions: Vec<EntityId>,
    pub performance_comparison: HashMap<String, f32>,
    pub counterfactual_analysis: Option<String>,
    pub improvement_suggestions: Vec<String>,
}

/// AI decision pattern analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionPattern {
    pub pattern_id: EntityId,
    pub pattern_name: String,
    pub pattern_type: DecisionPatternType,
    pub decisions_in_pattern: Vec<EntityId>,
    pub pattern_frequency: f32,
    pub success_rate: f32,
    pub contexts: Vec<String>,
    pub triggers: Vec<String>,
    pub outcomes: Vec<String>,
    pub confidence_in_pattern: f32,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecisionPatternType {
    Sequential,
    Conditional,
    Cyclic,
    Hierarchical,
    Emergent,
    Reactive,
    Proactive,
}

// Implement DatabaseModel trait for AI decision types
impl_database_model!(AIDecision, id, created_at, updated_at);
impl_database_model!(DecisionPattern, pattern_id, created_at, updated_at);

impl AIModel for AIDecision {
    fn confidence_score(&self) -> f32 {
        self.confidence_score
    }
    
    fn ai_context(&self) -> &AIContext {
        // In a real implementation, this would be stored as a field
        static DEFAULT_CONTEXT: std::sync::OnceLock<AIContext> = std::sync::OnceLock::new();
        DEFAULT_CONTEXT.get_or_init(|| AIContext {
            model_version: "ai-decision-v1.0".to_string(),
            decision_algorithm: "multi-factor-analysis".to_string(),
            training_data_hash: None,
            confidence_threshold: 0.7,
            processing_time_ms: 100,
            resource_usage: ResourceUsage {
                cpu_usage_percent: 25.0,
                memory_usage_mb: 256.0,
                gpu_usage_percent: Some(15.0),
                network_io_bytes: 512,
                disk_io_bytes: 1024,
            },
        })
    }
    
    fn has_learning_data(&self) -> bool {
        self.learning_value > 0.0
    }
}

impl AIDecision {
    /// Create a new AI decision
    pub fn new(
        ai_system_id: EntityId,
        decision_type: AIDecisionType,
        decision_domain: DecisionDomain,
        input_context: DecisionInputContext,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            decision_id: Uuid::new_v4(),
            ai_system_id,
            decision_timestamp: now,
            created_at: now,
            updated_at: now,
            decision_type,
            decision_domain,
            input_context,
            decision_urgency: DecisionUrgency::Medium,
            reasoning_chain: Vec::new(),
            considered_alternatives: Vec::new(),
            selected_decision: SelectedDecision::default(),
            confidence_score: 0.0,
            execution_plan: ExecutionPlan::default(),
            execution_status: ExecutionStatus::Pending,
            execution_duration_ms: None,
            predicted_outcome: PredictedOutcome::default(),
            actual_outcome: None,
            outcome_variance: None,
            feedback_score: None,
            learning_value: 0.0,
            error_analysis: None,
            decision_time_ms: 0,
            computational_cost: ComputationalCost::default(),
            resource_usage: ResourceUsage::default(),
            parent_decision_id: None,
            child_decisions: Vec::new(),
            influenced_by: Vec::new(),
            influences: Vec::new(),
            metadata: EntityMetadata::default(),
        }
    }
    
    /// Add a reasoning step to the decision chain
    pub fn add_reasoning_step(&mut self, step: ReasoningStep) {
        self.reasoning_chain.push(step);
        self.updated_at = Utc::now();
    }
    
    /// Add an alternative that was considered
    pub fn add_alternative(&mut self, alternative: DecisionAlternative) {
        self.considered_alternatives.push(alternative);
        self.updated_at = Utc::now();
    }
    
    /// Update execution status
    pub fn update_execution_status(&mut self, status: ExecutionStatus) {
        self.execution_status = status;
        self.updated_at = Utc::now();
    }
    
    /// Set actual outcome and calculate variance
    pub fn set_actual_outcome(&mut self, outcome: ActualOutcome) {
        // Calculate outcome variance if possible
        if let Some(predicted_value) = self.predicted_outcome.predicted_metrics.get("primary") {
            if let Some(actual_value) = outcome.actual_metrics.get("primary") {
                self.outcome_variance = Some((actual_value - predicted_value).abs() / predicted_value);
            }
        }
        
        self.actual_outcome = Some(outcome);
        self.updated_at = Utc::now();
    }
    
    /// Calculate decision effectiveness
    pub fn calculate_effectiveness(&self) -> Option<DecisionEffectiveness> {
        let actual = self.actual_outcome.as_ref()?;
        
        let prediction_accuracy = if let Some(variance) = self.outcome_variance {
            1.0 - variance.min(1.0)
        } else {
            0.5
        };
        
        let execution_efficiency = match &self.execution_status {
            ExecutionStatus::Completed { success_rate, .. } => *success_rate,
            _ => 0.0,
        };
        
        let outcome_satisfaction = if actual.success_achieved { 1.0 } else { 0.0 };
        
        let effectiveness_score = (prediction_accuracy + execution_efficiency + outcome_satisfaction) / 3.0;
        
        Some(DecisionEffectiveness {
            decision_id: self.id,
            effectiveness_score,
            prediction_accuracy,
            execution_efficiency,
            outcome_satisfaction,
            learning_value: self.learning_value,
            cost_benefit_ratio: 0.5, // Would need more complex calculation
            stakeholder_satisfaction: HashMap::new(),
            comparative_analysis: None,
        })
    }
}

// Default implementations for complex types
impl Default for SelectedDecision {
    fn default() -> Self {
        Self {
            decision_description: String::new(),
            selection_reasoning: String::new(),
            expected_benefits: Vec::new(),
            acknowledged_risks: Vec::new(),
            mitigation_strategies: Vec::new(),
            success_criteria: Vec::new(),
            fallback_plans: Vec::new(),
        }
    }
}

impl Default for ExecutionPlan {
    fn default() -> Self {
        Self {
            plan_id: Uuid::new_v4(),
            execution_steps: Vec::new(),
            estimated_duration_ms: 0,
            resource_allocation: ResourceAllocation::default(),
            dependencies: Vec::new(),
            monitoring_checkpoints: Vec::new(),
            rollback_strategy: None,
        }
    }
}

impl Default for ResourceAllocation {
    fn default() -> Self {
        Self {
            cpu_allocation_percent: 10.0,
            memory_allocation_mb: 100.0,
            network_bandwidth_mbps: 1.0,
            storage_allocation_mb: 50.0,
            priority_level: ExecutionPriority::Medium,
            shared_resources: Vec::new(),
        }
    }
}

impl Default for PredictedOutcome {
    fn default() -> Self {
        Self {
            outcome_description: String::new(),
            confidence_level: 0.5,
            predicted_metrics: HashMap::new(),
            success_probability: 0.5,
            risk_factors: Vec::new(),
            side_effects: Vec::new(),
            long_term_implications: Vec::new(),
            prediction_model: "default_predictor".to_string(),
            prediction_timestamp: Utc::now(),
        }
    }
}

impl Default for ComputationalCost {
    fn default() -> Self {
        Self {
            cpu_cycles: 0,
            memory_peak_mb: 0.0,
            gpu_utilization_percent: None,
            network_io_bytes: 0,
            disk_io_bytes: 0,
            energy_consumption_joules: None,
            cost_optimization_opportunities: Vec::new(),
        }
    }
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            memory_usage_mb: 0.0,
            gpu_usage_percent: None,
            network_io_bytes: 0,
            disk_io_bytes: 0,
        }
    }
}

impl Default for DecisionInputContext {
    fn default() -> Self {
        Self {
            world_state_snapshot: WorldStateSnapshot::default(),
            player_states: Vec::new(),
            npc_states: Vec::new(),
            system_metrics: SystemMetricsSnapshot::default(),
            environmental_factors: EnvironmentalFactors::default(),
            temporal_context: TemporalContext::default(),
            historical_context: Vec::new(),
            constraint_set: ConstraintSet::default(),
        }
    }
}

impl Default for WorldStateSnapshot {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            key_metrics: HashMap::new(),
            active_events: Vec::new(),
            resource_levels: HashMap::new(),
            stability_indicators: HashMap::new(),
        }
    }
}

impl Default for SystemMetricsSnapshot {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            network_latency: 0.0,
            active_connections: 0,
            performance_score: 1.0,
        }
    }
}

impl Default for EnvironmentalFactors {
    fn default() -> Self {
        Self {
            biome_type: BiomeType::Forest,
            danger_level: 0.0,
            resource_richness: 0.5,
            accessibility: 1.0,
            aesthetic_appeal: 0.5,
            novelty_factor: 0.0,
            comfort_level: 1.0,
        }
    }
}

impl Default for TemporalContext {
    fn default() -> Self {
        Self {
            time_of_day: TimeOfDay::default(),
            season: Season::Spring,
            recent_events: Vec::new(),
            upcoming_events: Vec::new(),
            temporal_patterns: Vec::new(),
        }
    }
}

impl Default for ConstraintSet {
    fn default() -> Self {
        Self {
            resource_constraints: HashMap::new(),
            performance_constraints: HashMap::new(),
            ethical_constraints: Vec::new(),
            player_preference_constraints: HashMap::new(),
            system_limitations: Vec::new(),
        }
    }
}

// Utility functions for AI decision management
impl AIDecision {
    /// Check if the decision is still valid based on current context
    pub fn is_valid(&self, current_context: &DecisionInputContext) -> bool {
        // Simple validation - in practice this would be more sophisticated
        let time_diff = (Utc::now() - self.decision_timestamp).num_seconds();
        
        // Decisions older than 1 hour might be stale
        if time_diff > 3600 {
            return false;
        }
        
        // Check if key metrics have changed significantly
        for (key, current_value) in &current_context.world_state_snapshot.key_metrics {
            if let Some(original_value) = self.input_context.world_state_snapshot.key_metrics.get(key) {
                let change_percent = ((current_value - original_value) / original_value).abs();
                if change_percent > 0.3 { // 30% change threshold
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Get decision age in seconds
    pub fn age_seconds(&self) -> i64 {
        (Utc::now() - self.decision_timestamp).num_seconds()
    }
    
    /// Check if decision requires urgent attention
    pub fn is_urgent(&self) -> bool {
        matches!(self.decision_urgency, DecisionUrgency::Critical | DecisionUrgency::High)
    }
    
    /// Get execution progress percentage
    pub fn execution_progress(&self) -> f32 {
        match &self.execution_status {
            ExecutionStatus::Pending => 0.0,
            ExecutionStatus::InProgress { progress_percent, .. } => *progress_percent,
            ExecutionStatus::Completed { .. } => 100.0,
            ExecutionStatus::Failed { steps_completed, .. } => {
                if self.execution_plan.execution_steps.is_empty() {
                    0.0
                } else {
                    (*steps_completed as f32 / self.execution_plan.execution_steps.len() as f32) * 100.0
                }
            },
            ExecutionStatus::Cancelled { .. } => 0.0,
            ExecutionStatus::RolledBack { .. } => 0.0,
        }
    }
    
    /// Calculate estimated completion time
    pub fn estimated_completion_time(&self) -> Option<Timestamp> {
        match &self.execution_status {
            ExecutionStatus::InProgress { estimated_completion_ms, .. } => {
                Some(self.decision_timestamp + chrono::Duration::milliseconds(*estimated_completion_ms as i64))
            },
            ExecutionStatus::Pending => {
                Some(self.decision_timestamp + chrono::Duration::milliseconds(self.execution_plan.estimated_duration_ms as i64))
            },
            _ => None,
        }
    }
}

// Pattern analysis utilities
impl DecisionPattern {
    /// Create a new decision pattern
    pub fn new(pattern_name: String, pattern_type: DecisionPatternType) -> Self {
        let now = Utc::now();
        Self {
            pattern_id: Uuid::new_v4(),
            pattern_name,
            pattern_type,
            decisions_in_pattern: Vec::new(),
            pattern_frequency: 0.0,
            success_rate: 0.0,
            contexts: Vec::new(),
            triggers: Vec::new(),
            outcomes: Vec::new(),
            confidence_in_pattern: 0.0,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Add a decision to this pattern
    pub fn add_decision(&mut self, decision_id: EntityId) {
        if !self.decisions_in_pattern.contains(&decision_id) {
            self.decisions_in_pattern.push(decision_id);
            self.updated_at = Utc::now();
        }
    }
    
    /// Calculate pattern strength based on frequency and success rate
    pub fn pattern_strength(&self) -> f32 {
        (self.pattern_frequency * self.success_rate * self.confidence_in_pattern).powf(1.0/3.0)
    }
    
    /// Check if pattern is considered stable
    pub fn is_stable(&self) -> bool {
        self.decisions_in_pattern.len() >= 5 && 
        self.confidence_in_pattern > 0.7 && 
        self.success_rate > 0.6
    }
}

// Error analysis utilities
impl ErrorAnalysis {
    /// Create a new error analysis
    pub fn new(error_type: ErrorType, error_source: ErrorSource) -> Self {
        Self {
            error_type,
            error_source,
            contributing_factors: Vec::new(),
            severity_assessment: 0.0,
            impact_assessment: ImpactAssessment::default(),
            corrective_actions: Vec::new(),
            prevention_strategies: Vec::new(),
            analysis_timestamp: Utc::now(),
        }
    }
    
    /// Add a contributing factor to the error
    pub fn add_contributing_factor(&mut self, factor: ContributingFactor) {
        self.contributing_factors.push(factor);
    }
    
    /// Calculate overall error severity
    pub fn overall_severity(&self) -> f32 {
        let base_severity = self.severity_assessment;
        let factor_weight: f32 = self.contributing_factors.iter()
            .map(|f| f.contribution_weight)
            .sum();
        
        (base_severity + factor_weight * 0.1).min(1.0)
    }
}

impl Default for ImpactAssessment {
    fn default() -> Self {
        Self {
            primary_impact: String::new(),
            secondary_impacts: Vec::new(),
            affected_stakeholders: Vec::new(),
            financial_impact: None,
            performance_impact: None,
            user_experience_impact: None,
        }
    }
}

// Decision effectiveness utilities
impl DecisionEffectiveness {
    /// Create comprehensive effectiveness report
    pub fn generate_report(&self) -> String {
        format!(
            "Decision Effectiveness Report\n\
            ============================\n\
            Overall Score: {:.2}\n\
            Prediction Accuracy: {:.2}\n\
            Execution Efficiency: {:.2}\n\
            Outcome Satisfaction: {:.2}\n\
            Learning Value: {:.2}\n\
            Cost-Benefit Ratio: {:.2}\n\
            \n\
            Analysis: {}\n",
            self.effectiveness_score,
            self.prediction_accuracy,
            self.execution_efficiency,
            self.outcome_satisfaction,
            self.learning_value,
            self.cost_benefit_ratio,
            if self.effectiveness_score > 0.8 {
                "Excellent decision with high effectiveness across all metrics."
            } else if self.effectiveness_score > 0.6 {
                "Good decision with room for improvement in some areas."
            } else if self.effectiveness_score > 0.4 {
                "Moderate decision effectiveness. Consider reviewing decision process."
            } else {
                "Poor decision effectiveness. Requires thorough analysis and process improvement."
            }
        )
    }
    
    /// Get improvement recommendations
    pub fn get_improvement_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if self.prediction_accuracy < 0.7 {
            recommendations.push("Improve prediction models and data quality".to_string());
        }
        
        if self.execution_efficiency < 0.7 {
            recommendations.push("Optimize execution planning and resource allocation".to_string());
        }
        
        if self.outcome_satisfaction < 0.7 {
            recommendations.push("Better align decisions with stakeholder expectations".to_string());
        }
        
        if self.learning_value < 0.5 {
            recommendations.push("Enhance learning data collection and analysis".to_string());
        }
        
        if self.cost_benefit_ratio < 0.6 {
            recommendations.push("Improve cost-benefit analysis and resource optimization".to_string());
        }
        
        recommendations
    }
}