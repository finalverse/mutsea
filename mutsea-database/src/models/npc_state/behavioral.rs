// mutsea-database/src/models/npc_state/behavioral.rs
//! Behavioral state models for NPCs including goals, motivations, and decision making

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Current behavior state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentBehavior {
    pub behavior_name: String,
    pub behavior_type: BehaviorType,
    pub behavior_intensity: f32,
    pub behavior_start_time: Timestamp,
    pub estimated_duration: Option<u64>, // seconds
    pub behavior_context: BehaviorContext,
    pub behavior_triggers: Vec<BehaviorTrigger>,
    pub behavior_modifiers: Vec<BehaviorModifier>,
    pub interruption_resistance: f32,
    pub social_visibility: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BehaviorType {
    Idle,
    Working,
    Socializing,
    Exploring,
    Learning,
    Resting,
    Eating,
    Traveling,
    Combat,
    CreativeActivity,
    Maintenance,
    Ritual,
    Emergency,
    AIGenerated(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorContext {
    pub location_type: String,
    pub social_context: String,
    pub time_context: String,
    pub environmental_factors: Vec<String>,
    pub available_resources: Vec<String>,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorTrigger {
    pub trigger_type: TriggerType,
    pub trigger_source: TriggerSource,
    pub trigger_strength: f32,
    pub trigger_timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriggerType {
    Internal,    // From within the NPC
    External,    // From environment
    Social,      // From other entities
    Temporal,    // Time-based
    Conditional, // Based on conditions
    Random,      // Stochastic
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriggerSource {
    BasicNeed,
    Emotion,
    Goal,
    Stimulus,
    Memory,
    Social_Pressure,
    Authority,
    Habit,
    AI_Decision,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorModifier {
    pub modifier_type: String,
    pub effect: f32, // Positive or negative multiplier
    pub duration: Option<u64>, // seconds
    pub source: String,
}

/// Behavior patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorPattern {
    pub pattern_id: EntityId,
    pub pattern_name: String,
    pub pattern_type: PatternType,
    pub behaviors_in_pattern: Vec<String>,
    pub pattern_frequency: PatternFrequency,
    pub pattern_reliability: f32, // How consistently this pattern occurs
    pub pattern_conditions: Vec<PatternCondition>,
    pub pattern_variations: Vec<PatternVariation>,
    pub learned_pattern: bool,
    pub pattern_efficiency: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PatternType {
    Daily_Routine,
    Weekly_Cycle,
    Seasonal,
    Situational,
    Social,
    Emergency,
    Learning,
    Adaptive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PatternFrequency {
    Once,
    Daily,
    Weekly,
    Monthly,
    Seasonal,
    Yearly,
    Situational,
    Random,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatternCondition {
    pub condition_type: String,
    pub condition_value: f32,
    pub condition_operator: ConditionOperator,
    pub condition_importance: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equal,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    NotEqual,
    Contains,
    InRange,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatternVariation {
    pub variation_name: String,
    pub probability: f32,
    pub conditions: Vec<String>,
    pub behavioral_changes: Vec<String>,
}

/// NPC goals and motivations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NPCGoal {
    pub goal_id: EntityId,
    pub goal_name: String,
    pub goal_type: GoalType,
    pub goal_priority: f32, // 0.0 to 1.0
    pub goal_urgency: f32,  // 0.0 to 1.0
    pub goal_progress: f32, // 0.0 to 1.0
    pub goal_status: GoalStatus,
    pub subgoals: Vec<EntityId>,
    pub required_resources: Vec<RequiredResource>,
    pub obstacles: Vec<Obstacle>,
    pub deadline: Option<Timestamp>,
    pub motivation_sources: Vec<MotivationSource>,
    pub success_criteria: Vec<SuccessCriterion>,
    pub abandonment_conditions: Vec<AbandonmentCondition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GoalType {
    Survival,
    Comfort,
    Achievement,
    Social,
    Creative,
    Learning,
    Spiritual,
    Economic,
    Political,
    Personal_Growth,
    Relationship,
    Legacy,
    AIAssigned,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GoalStatus {
    Active,
    Paused,
    Completed,
    Failed,
    Abandoned,
    Modified,
    Pending,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequiredResource {
    pub resource_type: String,
    pub quantity_needed: f32,
    pub quantity_available: f32,
    pub acquisition_difficulty: f32,
    pub alternative_resources: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Obstacle {
    pub obstacle_type: String,
    pub difficulty_level: f32,
    pub probability_of_occurrence: f32,
    pub potential_solutions: Vec<String>,
    pub impact_on_goal: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MotivationSource {
    Intrinsic,
    Extrinsic,
    Social_Pressure,
    Fear,
    Desire,
    Duty,
    Curiosity,
    Necessity,
    Habit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuccessCriterion {
    pub criterion_name: String,
    pub measurement_method: String,
    pub target_value: f32,
    pub current_value: f32,
    pub importance_weight: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbandonmentCondition {
    pub condition_description: String,
    pub threshold_value: f32,
    pub probability_of_triggering: f32,
    pub consequences: Vec<String>,
}

/// Motivations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Motivation {
    pub motivation_id: EntityId,
    pub motivation_name: String,
    pub motivation_category: MotivationCategory,
    pub strength: f32, // 0.0 to 1.0
    pub persistence: f32, // How long this motivation lasts
    pub satisfaction_level: f32, // How satisfied this motivation currently is
    pub satisfaction_decay_rate: f32,
    pub associated_needs: Vec<String>,
    pub conflicting_motivations: Vec<EntityId>,
    pub supporting_motivations: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MotivationCategory {
    Physiological,
    Safety,
    Social,
    Esteem,
    SelfActualization,
    Autonomy,
    Competence,
    Relatedness,
    Power,
    Achievement,
    Affiliation,
}

/// Decision making state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionMakingState {
    pub current_decision: Option<CurrentDecision>,
    pub decision_making_style: DecisionMakingStyle,
    pub decision_speed: f32,
    pub decision_quality: f32,
    pub risk_assessment_ability: f32,
    pub information_processing_capacity: f32,
    pub decision_confidence: f32,
    pub regret_tendency: f32,
    pub second_guessing_frequency: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentDecision {
    pub decision_context: String,
    pub options_considered: Vec<DecisionOption>,
    pub evaluation_criteria: Vec<EvaluationCriterion>,
    pub time_pressure: f32,
    pub information_availability: f32,
    pub emotional_influence: f32,
    pub social_influence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionOption {
    pub option_name: String,
    pub option_description: String,
    pub estimated_outcomes: Vec<EstimatedOutcome>,
    pub resource_requirements: Vec<RequiredResource>,
    pub risks: Vec<DecisionRisk>,
    pub benefits: Vec<DecisionBenefit>,
    pub feasibility: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EstimatedOutcome {
    pub outcome_description: String,
    pub probability: f32,
    pub desirability: f32,
    pub confidence_in_estimate: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionRisk {
    pub risk_description: String,
    pub probability: f32,
    pub severity: f32,
    pub mitigation_options: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionBenefit {
    pub benefit_description: String,
    pub probability: f32,
    pub value: f32,
    pub timeframe: Option<u64>, // seconds
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluationCriterion {
    pub criterion_name: String,
    pub importance_weight: f32,
    pub measurement_method: String,
    pub preference_direction: PreferenceDirection, // Higher or lower values preferred
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PreferenceDirection {
    Higher,
    Lower,
    Target(f32), // Specific target value
    Range(f32, f32), // Acceptable range
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecisionMakingStyle {
    Rational,      // Logical, systematic
    Intuitive,     // Gut feeling, fast
    Dependent,     // Relies on others
    Avoidant,      // Postpones decisions
    Spontaneous,   // Quick, impulsive
    Maximizing,    // Seeks best option
    Satisficing,   // Seeks good enough option
}

// Default implementations
impl Default for CurrentBehavior {
    fn default() -> Self {
        Self {
            behavior_name: "Idle".to_string(),
            behavior_type: BehaviorType::Idle,
            behavior_intensity: 0.3,
            behavior_start_time: Utc::now(),
            estimated_duration: None,
            behavior_context: BehaviorContext::default(),
            behavior_triggers: Vec::new(),
            behavior_modifiers: Vec::new(),
            interruption_resistance: 0.3,
            social_visibility: 0.5,
        }
    }
}

impl Default for BehaviorContext {
    fn default() -> Self {
        Self {
            location_type: "Unknown".to_string(),
            social_context: "Alone".to_string(),
            time_context: "Day".to_string(),
            environmental_factors: Vec::new(),
            available_resources: Vec::new(),
            constraints: Vec::new(),
        }
    }
}

impl Default for DecisionMakingState {
    fn default() -> Self {
        Self {
            current_decision: None,
            decision_making_style: DecisionMakingStyle::Rational,
            decision_speed: 0.6,
            decision_quality: 0.7,
            risk_assessment_ability: 0.6,
            information_processing_capacity: 0.7,
            decision_confidence: 0.6,
            regret_tendency: 0.3,
            second_guessing_frequency: 0.3,
        }
    }
}

// Utility implementations for behavioral functions
impl NPCGoal {
    /// Create a new goal
    pub fn new(goal_name: String, goal_type: GoalType, priority: f32) -> Self {
        Self {
            goal_id: Uuid::new_v4(),
            goal_name,
            goal_type,
            goal_priority: priority.clamp(0.0, 1.0),
            goal_urgency: 0.5,
            goal_progress: 0.0,
            goal_status: GoalStatus::Active,
            subgoals: Vec::new(),
            required_resources: Vec::new(),
            obstacles: Vec::new(),
            deadline: None,
            motivation_sources: Vec::new(),
            success_criteria: Vec::new(),
            abandonment_conditions: Vec::new(),
        }
    }
    
    /// Update goal progress
    pub fn update_progress(&mut self, progress: f32) {
        self.goal_progress = progress.clamp(0.0, 1.0);
        if self.goal_progress >= 1.0 {
            self.goal_status = GoalStatus::Completed;
        }
    }
    
    /// Check if goal should be abandoned
    pub fn should_abandon(&self) -> bool {
        for condition in &self.abandonment_conditions {
            if condition.probability_of_triggering > 0.8 {
                return true;
            }
        }
        false
    }
    
    /// Calculate goal completion likelihood
    pub fn calculate_completion_likelihood(&self) -> f32 {
        let progress_factor = self.goal_progress;
        let resource_factor = if self.required_resources.is_empty() {
            1.0
        } else {
            self.required_resources.iter()
                .map(|res| res.quantity_available / res.quantity_needed.max(0.001))
                .sum::<f32>() / self.required_resources.len() as f32
        };
        
        let obstacle_factor = if self.obstacles.is_empty() {
            1.0
        } else {
            1.0 - (self.obstacles.iter()
                .map(|obs| obs.difficulty_level * obs.probability_of_occurrence)
                .sum::<f32>() / self.obstacles.len() as f32)
        };
        
        let urgency_factor = self.goal_urgency;
        
        (progress_factor + resource_factor + obstacle_factor + urgency_factor) / 4.0
    }
    
    /// Add a subgoal
    pub fn add_subgoal(&mut self, subgoal_id: EntityId) {
        if !self.subgoals.contains(&subgoal_id) {
            self.subgoals.push(subgoal_id);
        }
    }
    
    /// Remove a subgoal
    pub fn remove_subgoal(&mut self, subgoal_id: EntityId) {
        self.subgoals.retain(|&id| id != subgoal_id);
    }
}

impl BehaviorPattern {
    /// Create a new behavior pattern
    pub fn new(pattern_name: String, pattern_type: PatternType) -> Self {
        Self {
            pattern_id: Uuid::new_v4(),
            pattern_name,
            pattern_type,
            behaviors_in_pattern: Vec::new(),
            pattern_frequency: PatternFrequency::Daily,
            pattern_reliability: 0.5,
            pattern_conditions: Vec::new(),
            pattern_variations: Vec::new(),
            learned_pattern: false,
            pattern_efficiency: 0.7,
        }
    }
    
    /// Add behavior to pattern
    pub fn add_behavior(&mut self, behavior: String) {
        if !self.behaviors_in_pattern.contains(&behavior) {
            self.behaviors_in_pattern.push(behavior);
        }
    }
    
    /// Check if pattern conditions are met
    pub fn conditions_met(&self, context: &HashMap<String, f32>) -> bool {
        for condition in &self.pattern_conditions {
            if let Some(&value) = context.get(&condition.condition_type) {
                let meets_condition = match condition.condition_operator {
                    ConditionOperator::Equal => (value - condition.condition_value).abs() < 0.1,
                    ConditionOperator::GreaterThan => value > condition.condition_value,
                    ConditionOperator::LessThan => value < condition.condition_value,
                    ConditionOperator::GreaterEqual => value >= condition.condition_value,
                    ConditionOperator::LessEqual => value <= condition.condition_value,
                    ConditionOperator::NotEqual => (value - condition.condition_value).abs() >= 0.1,
                    _ => true, // Other operators not implemented for f32
                };
                
                if !meets_condition && condition.condition_importance > 0.7 {
                    return false;
                }
            }
        }
        true
    }
    
    /// Calculate pattern strength
    pub fn calculate_strength(&self) -> f32 {
        (self.pattern_reliability + self.pattern_efficiency) / 2.0
    }
}

impl DecisionMakingState {
    /// Start a new decision process
    pub fn start_decision(&mut self, context: String) {
        self.current_decision = Some(CurrentDecision {
            decision_context: context,
            options_considered: Vec::new(),
            evaluation_criteria: Vec::new(),
            time_pressure: 0.5,
            information_availability: 0.7,
            emotional_influence: 0.3,
            social_influence: 0.2,
        });
    }
    
    /// Add option to current decision
    pub fn add_option(&mut self, option: DecisionOption) {
        if let Some(ref mut decision) = self.current_decision {
            decision.options_considered.push(option);
        }
    }
    
    /// Complete current decision
    pub fn complete_decision(&mut self) -> Option<String> {
        if let Some(decision) = self.current_decision.take() {
            // Simple decision logic - choose option with highest expected value
            let best_option = decision.options_considered.iter()
                .max_by(|a, b| {
                    let a_value = a.estimated_outcomes.iter()
                        .map(|o| o.probability * o.desirability)
                        .sum::<f32>();
                    let b_value = b.estimated_outcomes.iter()
                        .map(|o| o.probability * o.desirability)
                        .sum::<f32>();
                    a_value.partial_cmp(&b_value).unwrap_or(std::cmp::Ordering::Equal)
                });
            
            best_option.map(|option| option.option_name.clone())
        } else {
            None
        }
    }
    
    /// Calculate decision effectiveness
    pub fn calculate_decision_effectiveness(&self) -> f32 {
        let quality_factor = self.decision_quality;
        let speed_factor = self.decision_speed;
        let confidence_factor = self.decision_confidence;
        let regret_factor = 1.0 - self.regret_tendency;
        
        (quality_factor + speed_factor + confidence_factor + regret_factor) / 4.0
    }
}

impl Motivation {
    /// Create a new motivation
    pub fn new(name: String, category: MotivationCategory, strength: f32) -> Self {
        Self {
            motivation_id: Uuid::new_v4(),
            motivation_name: name,
            motivation_category: category,
            strength: strength.clamp(0.0, 1.0),
            persistence: 0.7,
            satisfaction_level: 0.5,
            satisfaction_decay_rate: 0.01,
            associated_needs: Vec::new(),
            conflicting_motivations: Vec::new(),
            supporting_motivations: Vec::new(),
        }
    }
    
    /// Update satisfaction level
    pub fn update_satisfaction(&mut self, change: f32) {
        self.satisfaction_level = (self.satisfaction_level + change).clamp(0.0, 1.0);
    }
    
    /// Apply natural satisfaction decay
    pub fn apply_decay(&mut self, time_elapsed_hours: f32) {
        let decay = self.satisfaction_decay_rate * time_elapsed_hours;
        self.satisfaction_level = (self.satisfaction_level - decay).max(0.0);
    }
    
    /// Check if motivation is urgent
    pub fn is_urgent(&self) -> bool {
        self.satisfaction_level < 0.3 && self.strength > 0.7
    }
    
    /// Calculate motivation pressure
    pub fn calculate_pressure(&self) -> f32 {
        let unsatisfied_amount = 1.0 - self.satisfaction_level;
        let strength_factor = self.strength;
        let persistence_factor = self.persistence;
        
        unsatisfied_amount * strength_factor * persistence_factor
    }
}