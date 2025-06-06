// mutsea-database/src/models/npc_state/mod.rs
//! NPC state models for AI-driven non-player characters
//! 
//! These models represent the state, behavior, intelligence, and social
//! dynamics of NPCs in the Mutsea AI engine, enabling complex AI-driven
//! character interactions and evolution.

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export submodules
pub mod physical;
pub mod cognitive;
pub mod social;
pub mod behavioral;
pub mod economic;
pub mod ai_controller;
pub mod health;

pub use physical::*;
pub use cognitive::*;
pub use social::*;
pub use behavioral::*;
pub use economic::*;
pub use ai_controller::*;
pub use health::*;

/// Complete NPC state representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NPCState {
    pub id: EntityId,
    pub npc_id: NPCId,
    pub state_timestamp: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    
    // Basic identity
    pub name: String,
    pub npc_type: NPCType,
    pub species: String,
    pub gender: Option<String>,
    pub age: NPCAge,
    
    // Physical state
    pub position: WorldPosition,
    pub orientation: Vector3,
    pub movement_state: MovementState,
    pub physical_condition: PhysicalCondition,
    pub appearance: NPCAppearance,
    
    // Cognitive state
    pub intelligence: IntelligenceProfile,
    pub consciousness_level: f32, // 0.0 to 1.0
    pub awareness_radius: f32,
    pub attention_focus: AttentionFocus,
    pub memory_state: MemoryState,
    
    // Emotional and psychological state
    pub emotional_state: EmotionalState,
    pub personality: PersonalityProfile,
    pub psychological_traits: PsychologicalTraits,
    pub stress_level: f32,
    pub mood_stability: f32,
    
    // Social state
    pub social_status: SocialStatus,
    pub relationships: Vec<NPCRelationship>,
    pub group_memberships: Vec<GroupMembership>,
    pub reputation: ReputationProfile,
    pub communication_state: CommunicationState,
    
    // Behavioral state
    pub current_behavior: CurrentBehavior,
    pub behavior_patterns: Vec<BehaviorPattern>,
    pub goals: Vec<NPCGoal>,
    pub motivations: Vec<Motivation>,
    pub decision_making_state: DecisionMakingState,
    
    // Skills and abilities
    pub skills: SkillSet,
    pub abilities: Vec<NPCAbility>,
    pub learning_progress: LearningProgress,
    pub adaptability_score: f32,
    
    // Economic state
    pub inventory: NPCInventory,
    pub economic_status: EconomicStatus,
    pub trade_preferences: TradePreferences,
    pub resource_needs: Vec<ResourceNeed>,
    
    // AI-specific state
    pub ai_controller: Option<AIController>,
    pub ai_learning_data: AILearningData,
    pub ai_performance_metrics: AIPerformanceMetrics,
    pub ai_decision_history: Vec<EntityId>, // References to AI decisions
    
    // Environmental interactions
    pub environment_interactions: Vec<EnvironmentInteraction>,
    pub territory: Option<Territory>,
    pub environmental_preferences: EnvironmentalPreferences,
    
    // Health and survival
    pub health_state: HealthState,
    pub survival_needs: SurvivalNeeds,
    pub lifecycle_stage: LifecycleStage,
    pub reproduction_state: Option<ReproductionState>,
    
    // Metadata
    pub metadata: EntityMetadata,
}

/// Types of NPCs in the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NPCType {
    // Intelligent humanoids
    Humanoid {
        culture: String,
        profession: String,
        social_rank: SocialRank,
    },
    
    // Animals and creatures
    Creature {
        species_type: CreatureType,
        domestication_level: DomesticationLevel,
        intelligence_level: CreatureIntelligence,
    },
    
    // AI entities
    ArtificialIntelligence {
        ai_type: String,
        processing_power: f32,
        autonomy_level: AutonomyLevel,
    },
    
    // Mechanical entities
    Robot {
        robot_type: String,
        functionality: Vec<String>,
        maintenance_state: MaintenanceState,
    },
    
    // Supernatural or fantasy entities
    Supernatural {
        entity_type: String,
        power_level: f32,
        origin_realm: String,
    },
    
    // Dynamic AI-generated types
    AIGenerated {
        type_name: String,
        characteristics: HashMap<String, f32>,
        generation_algorithm: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SocialRank {
    Peasant,
    Commoner,
    Merchant,
    Artisan,
    Noble,
    Royalty,
    Religious,
    Military,
    Scholar,
    Outcast,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreatureType {
    Mammal,
    Bird,
    Reptile,
    Amphibian,
    Fish,
    Insect,
    Mythical,
    Hybrid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomesticationLevel {
    Wild,
    SemiDomesticated,
    Domesticated,
    Companion,
    WorkingAnimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreatureIntelligence {
    Instinctual,
    Basic,
    Moderate,
    High,
    SapientLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AutonomyLevel {
    Scripted,
    SemiAutonomous,
    Autonomous,
    FullyAutonomous,
    Emergent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaintenanceState {
    Optimal,
    Good,
    Fair,
    Poor,
    Critical,
    Inoperative,
}

/// NPC age representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NPCAge {
    pub chronological_age: f32, // Years
    pub developmental_stage: DevelopmentalStage,
    pub maturity_level: f32, // 0.0 to 1.0
    pub life_expectancy: Option<f32>,
    pub aging_rate: f32, // How fast they age relative to normal
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DevelopmentalStage {
    Infant,
    Child,
    Adolescent,
    YoungAdult,
    Adult,
    MiddleAged,
    Elder,
    Ancient,
    Timeless, // For immortal entities
}

/// Lifecycle stage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LifecycleStage {
    Birth,
    Infancy,
    Childhood,
    Adolescence,
    YoungAdulthood,
    Adulthood,
    MiddleAge,
    OldAge,
    Death,
    Undead,
    Immortal,
    Artificial,
}

/// Reproduction state (if applicable)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReproductionState {
    pub reproductive_capacity: f32,
    pub mating_season: Option<Season>,
    pub mate_selection_criteria: Vec<MateSelectionCriterion>,
    pub offspring_count: u32,
    pub parental_investment: f32,
    pub reproductive_strategy: ReproductiveStrategy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MateSelectionCriterion {
    pub criterion_name: String,
    pub importance: f32,
    pub preferred_value: f32,
    pub tolerance_range: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReproductiveStrategy {
    Monogamous,
    Polygamous,
    Promiscuous,
    Seasonal,
    Opportunistic,
    Asexual,
    None,
}

// Implement DatabaseModel and AIModel traits
impl_database_model!(NPCState, id, created_at, updated_at);

impl AIModel for NPCState {
    fn confidence_score(&self) -> f32 {
        // Calculate overall AI confidence based on various factors
        let mut confidence_sum = 0.0;
        let mut count = 0;
        
        if let Some(ai_controller) = &self.ai_controller {
            confidence_sum += ai_controller.decision_confidence;
            count += 1;
        }
        
        confidence_sum += self.consciousness_level;
        count += 1;
        
        confidence_sum += self.intelligence.overall_intelligence / 10.0; // Normalize to 0-1
        count += 1;
        
        if count > 0 {
            confidence_sum / count as f32
        } else {
            0.5
        }
    }
    
    fn ai_context(&self) -> &AIContext {
        static DEFAULT_CONTEXT: std::sync::OnceLock<AIContext> = std::sync::OnceLock::new();
        DEFAULT_CONTEXT.get_or_init(|| AIContext {
            model_version: "npc-ai-v1.0".to_string(),
            decision_algorithm: "multi-agent-npc-intelligence".to_string(),
            training_data_hash: None,
            confidence_threshold: 0.6,
            processing_time_ms: 50,
            resource_usage: ResourceUsage {
                cpu_usage_percent: 20.0,
                memory_usage_mb: 256.0,
                gpu_usage_percent: Some(10.0),
                network_io_bytes: 512,
                disk_io_bytes: 1024,
            },
        })
    }
    
    fn has_learning_data(&self) -> bool {
        !self.ai_learning_data.learning_experiences.is_empty() || 
        !self.learning_progress.skills_being_learned.is_empty()
    }
}

// Implementation methods for NPCState
impl NPCState {
    /// Create a new NPC with basic default values
    pub fn new(name: String, npc_type: NPCType, position: WorldPosition) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            npc_id: Uuid::new_v4(),
            state_timestamp: now,
            created_at: now,
            updated_at: now,
            name,
            npc_type,
            species: "Human".to_string(),
            gender: Some("Neutral".to_string()),
            age: NPCAge::default(),
            position,
            orientation: Vector3::new(0.0, 0.0, 1.0),
            movement_state: MovementState::default(),
            physical_condition: PhysicalCondition::default(),
            appearance: NPCAppearance::default(),
            intelligence: IntelligenceProfile::default(),
            consciousness_level: 0.7,
            awareness_radius: 10.0,
            attention_focus: AttentionFocus::default(),
            memory_state: MemoryState::default(),
            emotional_state: EmotionalState::default(),
            personality: PersonalityProfile::default(),
            psychological_traits: PsychologicalTraits::default(),
            stress_level: 0.2,
            mood_stability: 0.8,
            social_status: SocialStatus::default(),
            relationships: Vec::new(),
            group_memberships: Vec::new(),
            reputation: ReputationProfile::default(),
            communication_state: CommunicationState::default(),
            current_behavior: CurrentBehavior::default(),
            behavior_patterns: Vec::new(),
            goals: Vec::new(),
            motivations: Vec::new(),
            decision_making_state: DecisionMakingState::default(),
            skills: SkillSet::default(),
            abilities: Vec::new(),
            learning_progress: LearningProgress::default(),
            adaptability_score: 0.6,
            inventory: NPCInventory::default(),
            economic_status: EconomicStatus::default(),
            trade_preferences: TradePreferences::default(),
            resource_needs: Vec::new(),
            ai_controller: Some(AIController::default()),
            ai_learning_data: AILearningData::default(),
            ai_performance_metrics: AIPerformanceMetrics::default(),
            ai_decision_history: Vec::new(),
            environment_interactions: Vec::new(),
            territory: None,
            environmental_preferences: EnvironmentalPreferences::default(),
            health_state: HealthState::default(),
            survival_needs: SurvivalNeeds::default(),
            lifecycle_stage: LifecycleStage::Adulthood,
            reproduction_state: None,
            metadata: EntityMetadata::default(),
        }
    }
    
    /// Update the NPC's timestamp
    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now();
        self.state_timestamp = self.updated_at;
    }
    
    /// Calculate overall NPC wellness score
    pub fn calculate_wellness_score(&self) -> f32 {
        let health_score = self.health_state.overall_health;
        let stress_factor = 1.0 - self.stress_level;
        let needs_satisfaction = self.calculate_needs_satisfaction();
        let social_factor = self.calculate_social_satisfaction();
        
        (health_score + stress_factor + needs_satisfaction + social_factor) / 4.0
    }
    
    /// Calculate how well survival needs are met
    pub fn calculate_needs_satisfaction(&self) -> f32 {
        let needs = &self.survival_needs;
        let total_dissatisfaction = needs.hunger_level + 
                                   needs.thirst_level + 
                                   needs.fatigue_level + 
                                   needs.warmth_need + 
                                   needs.shelter_need + 
                                   needs.social_need + 
                                   needs.safety_need + 
                                   needs.comfort_need;
        
        1.0 - (total_dissatisfaction / 8.0).min(1.0)
    }
    
    /// Calculate social satisfaction based on relationships and status
    pub fn calculate_social_satisfaction(&self) -> f32 {
        let relationship_quality: f32 = if self.relationships.is_empty() {
            0.5 // Neutral if no relationships
        } else {
            self.relationships.iter()
                .map(|rel| (rel.emotional_bond + rel.trust_level) / 2.0)
                .sum::<f32>() / self.relationships.len() as f32
        };
        
        let social_status_factor = self.social_status.hierarchy_level;
        let reputation_factor = (self.reputation.overall_reputation + 1.0) / 2.0; // Normalize from -1,1 to 0,1
        
        (relationship_quality + social_status_factor + reputation_factor) / 3.0
    }
    
    /// Get the NPC's current primary goal
    pub fn get_primary_goal(&self) -> Option<&NPCGoal> {
        self.goals.iter()
            .filter(|goal| matches!(goal.goal_status, GoalStatus::Active))
            .max_by(|a, b| a.goal_priority.partial_cmp(&b.goal_priority).unwrap_or(std::cmp::Ordering::Equal))
    }
    
    /// Check if NPC is in critical condition
    pub fn is_in_critical_condition(&self) -> bool {
        self.health_state.overall_health < 0.2 ||
        self.survival_needs.hunger_level > 0.9 ||
        self.survival_needs.thirst_level > 0.9 ||
        self.survival_needs.safety_need > 0.9 ||
        self.stress_level > 0.9
    }
    
    /// Generate status report
    pub fn generate_status_report(&self) -> String {
        format!(
            "NPC Status Report: {}\n\
            ==================\n\
            Type: {:?}\n\
            Health: {:.1}%\n\
            Wellness: {:.1}%\n\
            Primary Goal: {}\n\
            Current Behavior: {}\n\
            Stress Level: {:.1}%\n\
            Consciousness: {:.1}%\n\
            Relationships: {}\n\
            Active Goals: {}\n\
            Critical Condition: {}\n\
            \n\
            Survival Needs:\n\
            - Hunger: {:.1}%\n\
            - Thirst: {:.1}%\n\
            - Fatigue: {:.1}%\n\
            - Safety: {:.1}%\n\
            - Social: {:.1}%\n",
            self.name,
            self.npc_type,
            self.health_state.overall_health * 100.0,
            self.calculate_wellness_score() * 100.0,
            self.get_primary_goal().map(|g| &g.goal_name).unwrap_or(&"None".to_string()),
            self.current_behavior.behavior_name,
            self.stress_level * 100.0,
            self.consciousness_level * 100.0,
            self.relationships.len(),
            self.goals.iter().filter(|g| matches!(g.goal_status, GoalStatus::Active)).count(),
            self.is_in_critical_condition(),
            self.survival_needs.hunger_level * 100.0,
            self.survival_needs.thirst_level * 100.0,
            self.survival_needs.fatigue_level * 100.0,
            self.survival_needs.safety_need * 100.0,
            self.survival_needs.social_need * 100.0,
        )
    }
}

// Default implementation for NPCAge
impl Default for NPCAge {
    fn default() -> Self {
        Self {
            chronological_age: 25.0,
            developmental_stage: DevelopmentalStage::Adult,
            maturity_level: 0.8,
            life_expectancy: Some(75.0),
            aging_rate: 1.0,
        }
    }
}