// mutsea-database/src/models/npc_state/ai_controller.rs
//! AI controller state for NPCs - the brain of intelligent NPCs

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI controller for NPC behavior
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIController {
    pub controller_id: EntityId,
    pub ai_version: String,
    pub controller_type: AIControllerType,
    pub intelligence_level: f32,
    pub decision_making_speed: f32,
    pub decision_confidence: f32,
    pub learning_enabled: bool,
    pub adaptation_rate: f32,
    
    // Core AI components
    pub neural_networks: Vec<NeuralNetworkComponent>,
    pub decision_trees: Vec<DecisionTree>,
    pub behavior_models: Vec<BehaviorModel>,
    pub learning_algorithms: Vec<LearningAlgorithm>,
    
    // AI state
    pub current_objective: Option<AIObjective>,
    pub active_strategies: Vec<AIStrategy>,
    pub knowledge_base: AIKnowledgeBase,
    pub memory_system: AIMemorySystem,
    
    // Performance metrics
    pub decision_accuracy: f32,
    pub goal_achievement_rate: f32,
    pub adaptation_success_rate: f32,
    pub computational_efficiency: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AIControllerType {
    ReactiveBehavior,
    DeliberativePlanning,
    HybridArchitecture,
    NeuralNetwork,
    ReinforcementLearning,
    EvolutionaryAlgorithm,
    SwarmIntelligence,
    ExpertSystem,
    FuzzyLogic,
    CaseBasedReasoning,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeuralNetworkComponent {
    pub network_id: EntityId,
    pub network_type: NetworkType,
    pub layer_count: u32,
    pub neuron_count: u32,
    pub activation_functions: Vec<String>,
    pub training_status: TrainingStatus,
    pub accuracy: f32,
    pub specialized_function: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NetworkType {
    Feedforward,
    Recurrent,
    LSTM,
    GRU,
    Transformer,
    CNN,
    GAN,
    VAE,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrainingStatus {
    Untrained,
    Training,
    Trained,
    Retraining,
    Optimizing,
    Deployed,
    Deprecated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionTree {
    pub tree_id: EntityId,
    pub tree_name: String,
    pub decision_domain: String,
    pub node_count: u32,
    pub depth: u32,
    pub accuracy: f32,
    pub confidence_threshold: f32,
    pub pruning_level: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorModel {
    pub model_id: EntityId,
    pub model_name: String,
    pub behavior_type: String,
    pub complexity_level: f32,
    pub success_rate: f32,
    pub adaptability: f32,
    pub resource_requirements: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningAlgorithm {
    pub algorithm_id: EntityId,
    pub algorithm_type: LearningType,
    pub learning_rate: f32,
    pub convergence_threshold: f32,
    pub exploration_rate: f32,
    pub memory_capacity: u32,
    pub performance_metrics: LearningPerformanceMetrics,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LearningType {
    SupervisedLearning,
    UnsupervisedLearning,
    ReinforcementLearning,
    SemiSupervisedLearning,
    TransferLearning,
    MetaLearning,
    ContinualLearning,
    FederatedLearning,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningPerformanceMetrics {
    pub accuracy: f32,
    pub precision: f32,
    pub recall: f32,
    pub f1_score: f32,
    pub learning_speed: f32,
    pub knowledge_retention: f32,
    pub transfer_ability: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIObjective {
    pub objective_id: EntityId,
    pub objective_type: ObjectiveType,
    pub priority: f32,
    pub deadline: Option<Timestamp>,
    pub success_criteria: Vec<SuccessCriterion>,
    pub resource_allocation: f32,
    pub progress: f32,
    pub sub_objectives: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ObjectiveType {
    Survival,
    Optimization,
    Learning,
    Exploration,
    SocialInteraction,
    ResourceGathering,
    ProblemSolving,
    CreativeTask,
    Maintenance,
    Emergency,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIStrategy {
    pub strategy_id: EntityId,
    pub strategy_name: String,
    pub strategy_type: StrategyType,
    pub effectiveness: f32,
    pub resource_cost: f32,
    pub risk_level: f32,
    pub adaptation_capability: f32,
    pub execution_steps: Vec<StrategyStep>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StrategyType {
    Greedy,
    OptimalPlanning,
    Heuristic,
    RandomizedSearch,
    GeneticAlgorithm,
    SimulatedAnnealing,
    ParticleSwarm,
    AntColony,
    DeepSearch,
    BreadthFirst,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrategyStep {
    pub step_id: u32,
    pub action: String,
    pub expected_outcome: String,
    pub resource_requirement: f32,
    pub execution_time: u64,
    pub success_probability: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIKnowledgeBase {
    pub facts: Vec<KnowledgeFact>,
    pub rules: Vec<KnowledgeRule>,
    pub patterns: Vec<KnowledgePattern>,
    pub concepts: Vec<KnowledgeConcept>,
    pub relationships: Vec<KnowledgeRelationship>,
    pub confidence_levels: HashMap<String, f32>,
    pub update_frequency: f32,
    pub knowledge_quality: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeFact {
    pub fact_id: EntityId,
    pub statement: String,
    pub confidence: f32,
    pub source: String,
    pub timestamp: Timestamp,
    pub verification_status: VerificationStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VerificationStatus {
    Unverified,
    Verified,
    Disputed,
    Obsolete,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeRule {
    pub rule_id: EntityId,
    pub condition: String,
    pub action: String,
    pub certainty: f32,
    pub usage_frequency: u32,
    pub success_rate: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgePattern {
    pub pattern_id: EntityId,
    pub pattern_description: String,
    pub occurrence_frequency: f32,
    pub predictive_power: f32,
    pub confidence: f32,
    pub applications: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeConcept {
    pub concept_id: EntityId,
    pub concept_name: String,
    pub definition: String,
    pub abstraction_level: f32,
    pub related_concepts: Vec<EntityId>,
    pub understanding_level: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeRelationship {
    pub relationship_id: EntityId,
    pub subject: EntityId,
    pub predicate: String,
    pub object: EntityId,
    pub strength: f32,
    pub confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIMemorySystem {
    pub working_memory: WorkingMemoryAI,
    pub long_term_memory: LongTermMemoryAI,
    pub episodic_memory: EpisodicMemoryAI,
    pub semantic_memory: SemanticMemoryAI,
    pub procedural_memory: ProceduralMemoryAI,
    pub meta_memory: MetaMemory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkingMemoryAI {
    pub capacity: u32,
    pub current_load: u32,
    pub active_items: Vec<MemoryItemAI>,
    pub attention_focus: Vec<EntityId>,
    pub processing_speed: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LongTermMemoryAI {
    pub storage_capacity: u64,
    pub current_usage: u64,
    pub retrieval_speed: f32,
    pub consolidation_rate: f32,
    pub memory_networks: Vec<MemoryNetwork>,
    pub indexing_efficiency: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpisodicMemoryAI {
    pub autobiographical_events: Vec<AutobiographicalEvent>,
    pub temporal_organization: bool,
    pub contextual_binding: f32,
    pub emotional_enhancement: f32,
    pub reconstruction_accuracy: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutobiographicalEvent {
    pub event_id: EntityId,
    pub description: String,
    pub timestamp: Timestamp,
    pub location: WorldPosition,
    pub participants: Vec<EntityId>,
    pub emotional_impact: f32,
    pub significance: f32,
    pub vividness: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticMemoryAI {
    pub conceptual_knowledge: Vec<ConceptualKnowledge>,
    pub factual_information: Vec<FactualInformation>,
    pub categorical_organization: bool,
    pub abstraction_levels: Vec<AbstractionLevel>,
    pub generalization_ability: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConceptualKnowledge {
    pub concept: String,
    pub definition: String,
    pub properties: Vec<String>,
    pub relationships: Vec<String>,
    pub confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactualInformation {
    pub fact: String,
    pub domain: String,
    pub accuracy: f32,
    pub source: String,
    pub last_verified: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbstractionLevel {
    pub level: u32,
    pub description: String,
    pub concepts: Vec<String>,
    pub complexity: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProceduralMemoryAI {
    pub skills: Vec<ProcSkill>,
    pub habits: Vec<ProcHabit>,
    pub motor_programs: Vec<MotorProgram>,
    pub cognitive_procedures: Vec<CognitiveProcedure>,
    pub automaticity_level: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcSkill {
    pub skill_name: String,
    pub proficiency: f32,
    pub execution_speed: f32,
    pub error_rate: f32,
    pub practice_time: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcHabit {
    pub habit_name: String,
    pub strength: f32,
    pub trigger_conditions: Vec<String>,
    pub execution_probability: f32,
    pub resistance_to_change: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotorProgram {
    pub program_name: String,
    pub motor_sequence: Vec<String>,
    pub coordination_level: f32,
    pub timing_precision: f32,
    pub adaptability: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CognitiveProcedure {
    pub procedure_name: String,
    pub steps: Vec<String>,
    pub efficiency: f32,
    pub flexibility: f32,
    pub transfer_ability: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaMemory {
    pub memory_monitoring: f32,
    pub memory_control: f32,
    pub memory_awareness: f32,
    pub metamemory_accuracy: f32,
    pub strategy_selection: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryItemAI {
    pub item_id: EntityId,
    pub content_type: AIMemoryContentType,
    pub priority: f32,
    pub activation_level: f32,
    pub decay_rate: f32,
    pub associations: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AIMemoryContentType {
    Sensory,
    Conceptual,
    Procedural,
    Episodic,
    Semantic,
    Emotional,
    Spatial,
    Temporal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryNetwork {
    pub network_id: EntityId,
    pub nodes: Vec<MemoryNode>,
    pub connections: Vec<MemoryConnection>,
    pub activation_pattern: Vec<f32>,
    pub network_strength: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryNode {
    pub node_id: EntityId,
    pub content: String,
    pub activation: f32,
    pub threshold: f32,
    pub decay_rate: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryConnection {
    pub from_node: EntityId,
    pub to_node: EntityId,
    pub weight: f32,
    pub connection_type: ConnectionType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectionType {
    Excitatory,
    Inhibitory,
    Associative,
    Causal,
    Temporal,
    Spatial,
}

/// AI learning data for continuous improvement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AILearningData {
    pub learning_experiences: Vec<LearningExperience>,
    pub performance_history: Vec<PerformanceRecord>,
    pub adaptation_events: Vec<AdaptationEvent>,
    pub knowledge_acquisition: Vec<KnowledgeAcquisition>,
    pub skill_development: Vec<SkillDevelopment>,
    pub error_corrections: Vec<ErrorCorrection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningExperience {
    pub experience_id: EntityId,
    pub experience_type: ExperienceType,
    pub context: String,
    pub outcome: String,
    pub lessons_learned: Vec<String>,
    pub confidence: f32,
    pub transferability: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExperienceType {
    Success,
    Failure,
    Observation,
    Experimentation,
    Instruction,
    Discovery,
    Collaboration,
    Reflection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceRecord {
    pub record_id: EntityId,
    pub task: String,
    pub performance_score: f32,
    pub execution_time: u64,
    pub resource_usage: f32,
    pub quality_metrics: HashMap<String, f32>,
    pub improvement_areas: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdaptationEvent {
    pub event_id: EntityId,
    pub trigger: String,
    pub adaptation_type: AdaptationType,
    pub changes_made: Vec<String>,
    pub effectiveness: f32,
    pub time_to_adapt: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdaptationType {
    ParameterAdjustment,
    StrategyChange,
    BehaviorModification,
    KnowledgeUpdate,
    SkillAcquisition,
    GoalRedefinition,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeAcquisition {
    pub acquisition_id: EntityId,
    pub knowledge_type: String,
    pub source: String,
    pub confidence: f32,
    pub verification_method: String,
    pub integration_success: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillDevelopment {
    pub skill_name: String,
    pub initial_level: f32,
    pub current_level: f32,
    pub learning_rate: f32,
    pub practice_time: u64,
    pub mastery_indicators: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorCorrection {
    pub error_id: EntityId,
    pub error_type: String,
    pub context: String,
    pub correction_applied: String,
    pub effectiveness: f32,
    pub prevention_strategy: String,
}

/// AI performance metrics for NPCs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIPerformanceMetrics {
    pub decision_accuracy: f32,
    pub response_time_ms: f32,
    pub goal_achievement_rate: f32,
    pub learning_efficiency: f32,
    pub adaptation_speed: f32,
    pub resource_utilization: f32,
    pub error_rate: f32,
    pub consistency_score: f32,
    pub creativity_index: f32,
    pub social_interaction_quality: f32,
    pub performance_trends: PerformanceTrends,
    pub benchmark_comparisons: Vec<BenchmarkComparison>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceTrends {
    pub improving_metrics: Vec<String>,
    pub declining_metrics: Vec<String>,
    pub stable_metrics: Vec<String>,
    pub trend_confidence: f32,
    pub prediction_accuracy: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchmarkComparison {
    pub benchmark_name: String,
    pub npc_score: f32,
    pub benchmark_score: f32,
    pub percentile_ranking: f32,
    pub improvement_potential: f32,
}

// Default implementations
impl Default for AIController {
    fn default() -> Self {
        Self {
            controller_id: Uuid::new_v4(),
            ai_version: "1.0.0".to_string(),
            controller_type: AIControllerType::HybridArchitecture,
            intelligence_level: 0.7,
            decision_making_speed: 0.8,
            decision_confidence: 0.7,
            learning_enabled: true,
            adaptation_rate: 0.6,
            neural_networks: Vec::new(),
            decision_trees: Vec::new(),
            behavior_models: Vec::new(),
            learning_algorithms: Vec::new(),
            current_objective: None,
            active_strategies: Vec::new(),
            knowledge_base: AIKnowledgeBase::default(),
            memory_system: AIMemorySystem::default(),
            decision_accuracy: 0.8,
            goal_achievement_rate: 0.7,
            adaptation_success_rate: 0.6,
            computational_efficiency: 0.8,
        }
    }
}

impl Default for AIKnowledgeBase {
    fn default() -> Self {
        Self {
            facts: Vec::new(),
            rules: Vec::new(),
            patterns: Vec::new(),
            concepts: Vec::new(),
            relationships: Vec::new(),
            confidence_levels: HashMap::new(),
            update_frequency: 0.1,
            knowledge_quality: 0.7,
        }
    }
}

impl Default for AIMemorySystem {
    fn default() -> Self {
        Self {
            working_memory: WorkingMemoryAI {
                capacity: 7,
                current_load: 3,
                active_items: Vec::new(),
                attention_focus: Vec::new(),
                processing_speed: 0.8,
            },
            long_term_memory: LongTermMemoryAI {
                storage_capacity: 1_000_000,
                current_usage: 100_000,
                retrieval_speed: 0.7,
                consolidation_rate: 0.8,
                memory_networks: Vec::new(),
                indexing_efficiency: 0.85,
            },
            episodic_memory: EpisodicMemoryAI {
                autobiographical_events: Vec::new(),
                temporal_organization: true,
                contextual_binding: 0.8,
                emotional_enhancement: 0.7,
                reconstruction_accuracy: 0.75,
            },
            semantic_memory: SemanticMemoryAI {
                conceptual_knowledge: Vec::new(),
                factual_information: Vec::new(),
                categorical_organization: true,
                abstraction_levels: Vec::new(),
                generalization_ability: 0.7,
            },
            procedural_memory: ProceduralMemoryAI {
                skills: Vec::new(),
                habits: Vec::new(),
                motor_programs: Vec::new(),
                cognitive_procedures: Vec::new(),
                automaticity_level: 0.6,
            },
            meta_memory: MetaMemory {
                memory_monitoring: 0.7,
                memory_control: 0.6,
                memory_awareness: 0.8,
                metamemory_accuracy: 0.7,
                strategy_selection: 0.75,
            },
        }
    }
}

impl Default for AILearningData {
    fn default() -> Self {
        Self {
            learning_experiences: Vec::new(),
            performance_history: Vec::new(),
            adaptation_events: Vec::new(),
            knowledge_acquisition: Vec::new(),
            skill_development: Vec::new(),
            error_corrections: Vec::new(),
        }
    }
}

impl Default for AIPerformanceMetrics {
    fn default() -> Self {
        Self {
            decision_accuracy: 0.8,
            response_time_ms: 100.0,
            goal_achievement_rate: 0.7,
            learning_efficiency: 0.6,
            adaptation_speed: 0.7,
            resource_utilization: 0.75,
            error_rate: 0.05,
            consistency_score: 0.8,
            creativity_index: 0.5,
            social_interaction_quality: 0.7,
            performance_trends: PerformanceTrends {
                improving_metrics: Vec::new(),
                declining_metrics: Vec::new(),
                stable_metrics: Vec::new(),
                trend_confidence: 0.7,
                prediction_accuracy: 0.6,
            },
            benchmark_comparisons: Vec::new(),
        }
    }
}
