// mutsea-database/src/models/npc_state/health.rs
//! Health state models for NPCs including physical and mental health

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete health state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthState {
    pub overall_health: f32, // 0.0 to 1.0
    pub physical_health: PhysicalHealth,
    pub mental_health: MentalHealth,
    pub energy_level: f32,
    pub pain_level: f32,
    pub recovery_rate: f32,
    pub immune_system_strength: f32,
    pub health_conditions: Vec<HealthCondition>,
    pub medications: Vec<Medication>,
    pub health_history: Vec<HealthEvent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalHealth {
    pub vitality: f32,
    pub coordination: f32,
    pub reflexes: f32,
    pub disease_resistance: f32,
    pub toxin_tolerance: f32,
    pub healing_factor: f32,
    pub physical_injuries: Vec<Injury>,
    pub chronic_conditions: Vec<ChronicCondition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MentalHealth {
    pub cognitive_clarity: f32,
    pub emotional_stability: f32,
    pub stress_resistance: f32,
    pub mental_fatigue: f32,
    pub focus_ability: f32,
    pub memory_function: f32,
    pub mental_disorders: Vec<MentalDisorder>,
    pub therapy_progress: Vec<TherapyProgress>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthCondition {
    pub condition_name: String,
    pub severity: f32,
    pub progression_rate: f32,
    pub symptoms: Vec<String>,
    pub treatments: Vec<String>,
    pub prognosis: Prognosis,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Prognosis {
    Excellent,
    Good,
    Fair,
    Poor,
    Terminal,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medication {
    pub medication_name: String,
    pub dosage: f32,
    pub frequency: String,
    pub side_effects: Vec<String>,
    pub effectiveness: f32,
    pub dependency_risk: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthEvent {
    pub event_type: String,
    pub timestamp: Timestamp,
    pub severity: f32,
    pub treatment_received: Vec<String>,
    pub outcome: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Injury {
    pub injury_type: String,
    pub location: String,
    pub severity: f32,
    pub healing_progress: f32,
    pub complications: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChronicCondition {
    pub condition_name: String,
    pub onset_date: Timestamp,
    pub management_plan: Vec<String>,
    pub flare_up_triggers: Vec<String>,
    pub quality_of_life_impact: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MentalDisorder {
    pub disorder_name: String,
    pub diagnosis_date: Timestamp,
    pub severity: f32,
    pub treatment_plan: Vec<String>,
    pub support_system: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TherapyProgress {
    pub therapy_type: String,
    pub sessions_completed: u32,
    pub progress_rating: f32,
    pub goals_achieved: Vec<String>,
    pub remaining_goals: Vec<String>,
}

/// Survival needs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurvivalNeeds {
    pub hunger_level: f32,     // 0.0 satisfied to 1.0 starving
    pub thirst_level: f32,     // 0.0 satisfied to 1.0 dehydrated
    pub fatigue_level: f32,    // 0.0 rested to 1.0 exhausted
    pub warmth_need: f32,      // 0.0 comfortable to 1.0 freezing/overheating
    pub shelter_need: f32,     // 0.0 secure to 1.0 exposed
    pub social_need: f32,      // 0.0 satisfied to 1.0 lonely
    pub safety_need: f32,      // 0.0 safe to 1.0 threatened
    pub comfort_need: f32,     // 0.0 comfortable to 1.0 uncomfortable
    pub hygiene_need: f32,     // 0.0 clean to 1.0 dirty
    pub purpose_need: f32,     // 0.0 fulfilled to 1.0 meaningless
}

// Default implementations
impl Default for HealthState {
    fn default() -> Self {
        Self {
            overall_health: 0.8,
            physical_health: PhysicalHealth::default(),
            mental_health: MentalHealth::default(),
            energy_level: 0.8,
            pain_level: 0.1,
            recovery_rate: 0.7,
            immune_system_strength: 0.7,
            health_conditions: Vec::new(),
            medications: Vec::new(),
            health_history: Vec::new(),
        }
    }
}

impl Default for PhysicalHealth {
    fn default() -> Self {
        Self {
            vitality: 0.8,
            coordination: 0.7,
            reflexes: 0.7,
            disease_resistance: 0.7,
            toxin_tolerance: 0.6,
            healing_factor: 0.7,
            physical_injuries: Vec::new(),
            chronic_conditions: Vec::new(),
        }
    }
}

impl Default for MentalHealth {
    fn default() -> Self {
        Self {
            cognitive_clarity: 0.8,
            emotional_stability: 0.7,
            stress_resistance: 0.6,
            mental_fatigue: 0.2,
            focus_ability: 0.7,
            memory_function: 0.8,
            mental_disorders: Vec::new(),
            therapy_progress: Vec::new(),
        }
    }
}

impl Default for SurvivalNeeds {
    fn default() -> Self {
        Self {
            hunger_level: 0.3,
            thirst_level: 0.2,
            fatigue_level: 0.3,
            warmth_need: 0.1,
            shelter_need: 0.1,
            social_need: 0.4,
            safety_need: 0.2,
            comfort_need: 0.3,
            hygiene_need: 0.2,
            purpose_need: 0.4,
        }
    }
}

// mutsea-database/src/models/npc_state/ai_controller.rs
//! AI controller models for NPC behavior and decision making

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI controller for NPC behavior
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIController {
    pub controller_id: EntityId,
    pub ai_type: AIType,
    pub processing_power: f32,
    pub decision_frequency: f32, // Decisions per second
    pub learning_enabled: bool,
    pub adaptation_enabled: bool,
    pub creativity_level: f32,
    pub autonomy_level: f32,
    pub goal_management: GoalManagement,
    pub behavior_planner: BehaviorPlanner,
    pub decision_confidence: f32,
    pub error_rate: f32,
    pub optimization_targets: Vec<OptimizationTarget>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AIType {
    RuleBased,
    NeuralNetwork,
    ReinforcementLearning,
    HybridAI,
    EmergentAI,
    CustomAI(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoalManagement {
    pub goal_prioritization_method: String,
    pub goal_updating_frequency: f32,
    pub goal_conflict_resolution: String,
    pub dynamic_goal_creation: bool,
    pub goal_abandonment_threshold: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorPlanner {
    pub planning_horizon: u64, // seconds
    pub replanning_frequency: f32,
    pub contingency_planning: bool,
    pub behavior_prediction_accuracy: f32,
    pub adaptation_speed: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptimizationTarget {
    pub target_name: String,
    pub current_value: f32,
    pub target_value: f32,
    pub importance_weight: f32,
    pub optimization_method: String,
}

/// AI learning data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AILearningData {
    pub learning_experiences: Vec<LearningExperience>,
    pub skill_improvements: HashMap<String, f32>,
    pub behavior_adaptations: Vec<BehaviorAdaptation>,
    pub knowledge_updates: Vec<KnowledgeUpdate>,
    pub pattern_recognitions: Vec<PatternRecognition>,
    pub prediction_accuracy_history: Vec<PredictionAccuracy>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningExperience {
    pub experience_id: EntityId,
    pub experience_type: String,
    pub outcome: f32, // -1.0 negative to 1.0 positive
    pub lesson_learned: String,
    pub applicability_contexts: Vec<String>,
    pub confidence_in_lesson: f32,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorAdaptation {
    pub behavior_name: String,
    pub original_parameters: HashMap<String, f32>,
    pub adapted_parameters: HashMap<String, f32>,
    pub adaptation_reason: String,
    pub effectiveness_improvement: f32,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeUpdate {
    pub knowledge_domain: String,
    pub update_type: UpdateType,
    pub new_information: String,
    pub confidence_level: f32,
    pub source: String,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UpdateType {
    NewKnowledge,
    CorrectedKnowledge,
    ExpandedKnowledge,
    DeprecatedKnowledge,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatternRecognition {
    pub pattern_name: String,
    pub pattern_description: String,
    pub confidence_score: f32,
    pub occurrences_observed: u32,
    pub prediction_accuracy: f32,
    pub applicable_contexts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionAccuracy {
    pub prediction_type: String,
    pub predicted_value: f32,
    pub actual_value: f32,
    pub accuracy_score: f32,
    pub timestamp: Timestamp,
}

/// AI performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIPerformanceMetrics {
    pub decision_quality_score: f32,
    pub response_time_ms: f32,
    pub goal_achievement_rate: f32,
    pub adaptation_effectiveness: f32,
    pub learning_rate: f32,
    pub error_recovery_time: f32,
    pub resource_efficiency: f32,
    pub player_satisfaction_impact: f32,
    pub emergent_behavior_generation: f32,
    pub system_stability_contribution: f32,
}

// Default implementations
impl Default for AIController {
    fn default() -> Self {
        Self {
            controller_id: Uuid::new_v4(),
            ai_type: AIType::HybridAI,
            processing_power: 0.7,
            decision_frequency: 1.0,
            learning_enabled: true,
            adaptation_enabled: true,
            creativity_level: 0.5,
            autonomy_level: 0.7,
            goal_management: GoalManagement::default(),
            behavior_planner: BehaviorPlanner::default(),
            decision_confidence: 0.6,
            error_rate: 0.1,
            optimization_targets: Vec::new(),
        }
    }
}

impl Default for GoalManagement {
    fn default() -> Self {
        Self {
            goal_prioritization_method: "utility_based".to_string(),
            goal_updating_frequency: 0.1,
            goal_conflict_resolution: "priority_based".to_string(),
            dynamic_goal_creation: true,
            goal_abandonment_threshold: 0.1,
        }
    }
}

impl Default for BehaviorPlanner {
    fn default() -> Self {
        Self {
            planning_horizon: 3600, // 1 hour
            replanning_frequency: 0.05,
            contingency_planning: true,
            behavior_prediction_accuracy: 0.6,
            adaptation_speed: 0.7,
        }
    }
}

impl Default for AILearningData {
    fn default() -> Self {
        Self {
            learning_experiences: Vec::new(),
            skill_improvements: HashMap::new(),
            behavior_adaptations: Vec::new(),
            knowledge_updates: Vec::new(),
            pattern_recognitions: Vec::new(),
            prediction_accuracy_history: Vec::new(),
        }
    }
}

impl Default for AIPerformanceMetrics {
    fn default() -> Self {
        Self {
            decision_quality_score: 0.6,
            response_time_ms: 100.0,
            goal_achievement_rate: 0.5,
            adaptation_effectiveness: 0.6,
            learning_rate: 0.4,
            error_recovery_time: 500.0,
            resource_efficiency: 0.7,
            player_satisfaction_impact: 0.5,
            emergent_behavior_generation: 0.3,
            system_stability_contribution: 0.7,
        }
    }
}