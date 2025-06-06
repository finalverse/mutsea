// mutsea-database/src/models/player_behavior.rs
//! Player behavior models for AI analysis and learning
//! 
//! These models track player actions, preferences, and behavioral patterns
//! to enable AI systems to adapt and personalize the gaming experience.

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Player behavior analysis data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerBehavior {
    pub id: EntityId,
    pub player_id: PlayerId,
    pub session_id: SessionId,
    pub behavior_timestamp: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    
    // Action data
    pub action_type: PlayerActionType,
    pub action_context: ActionContext,
    pub action_outcome: ActionOutcome,
    pub action_duration_ms: u64,
    
    // Behavioral metrics
    pub decision_time_ms: u64,
    pub hesitation_count: u32,
    pub confidence_level: f32, // 0.0 to 1.0
    pub emotional_state: EmotionalState,
    
    // AI analysis
    pub ai_behavior_classification: BehaviorClassification,
    pub ai_predicted_next_action: Option<PredictedAction>,
    pub ai_confidence_score: f32,
    
    // Context information
    pub world_state_at_action: WorldStateContext,
    pub social_context: SocialContext,
    pub environmental_factors: EnvironmentalFactors,
    
    // Learning data
    pub learning_value: f32, // How much this behavior contributes to learning
    pub pattern_deviation: f32, // How much this deviates from player's normal pattern
    
    // Metadata
    pub metadata: EntityMetadata,
}

/// Types of player actions tracked
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerActionType {
    // Movement actions
    Move { from: WorldPosition, to: WorldPosition, speed: f32 },
    Teleport { from: WorldPosition, to: WorldPosition },
    Jump { position: WorldPosition, height: f32 },
    
    // Interaction actions
    Interact { target_id: EntityId, interaction_type: String },
    Communicate { target_id: Option<EntityId>, message_type: CommunicationType },
    Trade { target_id: EntityId, items: Vec<TradeItem> },
    
    // Building actions
    Build { object_type: String, position: WorldPosition, materials: Vec<MaterialUsed> },
    Destroy { target_id: EntityId, method: String },
    Modify { target_id: EntityId, changes: HashMap<String, String> },
    
    // Combat actions
    Attack { target_id: EntityId, weapon_type: String, damage_dealt: f32 },
    Defend { damage_blocked: f32, defense_type: String },
    Flee { from_position: WorldPosition, to_position: WorldPosition },
    
    // Resource actions
    Gather { resource_type: String, quantity: f32, location: WorldPosition },
    Craft { recipe: String, materials: Vec<MaterialUsed>, success: bool },
    Consume { item_type: String, quantity: f32, effects: Vec<String> },
    
    // Exploration actions
    Discover { location: WorldPosition, discovery_type: String },
    Investigate { target_id: EntityId, investigation_depth: f32 },
    Map { area_mapped: f32, accuracy: f32 },
    
    // Social actions
    GroupUp { group_id: EntityId, role: String },
    LeaveGroup { group_id: EntityId, reason: String },
    Help { target_player: PlayerId, help_type: String },
    
    // Meta actions
    OpenMenu { menu_type: String, duration_ms: u64 },
    ChangeSettings { setting: String, old_value: String, new_value: String },
    SaveGame { save_type: String },
    QuitGame { session_duration_ms: u64, reason: Option<String> },
    
    // AI-detected custom actions
    AIDetectedPattern { pattern_name: String, confidence: f32, data: HashMap<String, f32> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeItem {
    pub item_type: String,
    pub quantity: f32,
    pub quality: f32,
    pub value: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialUsed {
    pub material_type: String,
    pub quantity: f32,
    pub quality: f32,
    pub source_location: Option<WorldPosition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommunicationType {
    Text(String),
    Voice { duration_ms: u64, sentiment: f32 },
    Gesture { gesture_type: String, intensity: f32 },
    EmoteAction(String),
}

/// Context in which the action was performed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionContext {
    pub location: WorldPosition,
    pub time_of_day: TimeOfDay,
    pub weather_conditions: WeatherState,
    pub nearby_players: Vec<PlayerId>,
    pub nearby_npcs: Vec<NPCId>,
    pub available_resources: Vec<String>,
    pub active_quests: Vec<QuestId>,
    pub inventory_state: InventorySnapshot,
    pub health_status: HealthStatus,
    pub energy_level: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventorySnapshot {
    pub total_items: u32,
    pub total_weight: f32,
    pub capacity_used: f32, // 0.0 to 1.0
    pub valuable_items_count: u32,
    pub consumables_count: u32,
    pub tools_count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthStatus {
    pub current_health: f32,
    pub max_health: f32,
    pub health_percentage: f32,
    pub status_effects: Vec<String>,
    pub recent_damage: f32,
    pub healing_rate: f32,
}

/// Outcome of the player action
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionOutcome {
    pub success: bool,
    pub outcome_type: OutcomeType,
    pub rewards_gained: Vec<Reward>,
    pub penalties_incurred: Vec<Penalty>,
    pub experience_gained: f32,
    pub skill_improvements: HashMap<String, f32>,
    pub world_state_changes: Vec<WorldStateChange>,
    pub npc_reactions: Vec<NPCReaction>,
    pub player_satisfaction: f32, // AI-estimated 0.0 to 1.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OutcomeType {
    Success,
    PartialSuccess,
    Failure,
    CriticalSuccess,
    CriticalFailure,
    Unexpected,
    AIInterrupted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reward {
    pub reward_type: String,
    pub value: f32,
    pub rarity: f32,
    pub ai_generated: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Penalty {
    pub penalty_type: String,
    pub severity: f32,
    pub duration_hours: Option<f32>,
    pub reversible: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldStateChange {
    pub change_type: String,
    pub affected_area: Option<WorldPosition>,
    pub magnitude: f32,
    pub permanence: f32, // 0.0 temporary to 1.0 permanent
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NPCReaction {
    pub npc_id: NPCId,
    pub reaction_type: String,
    pub intensity: f32,
    pub reputation_change: f32,
}

/// Player's emotional state during action
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmotionalState {
    pub primary_emotion: Emotion,
    pub secondary_emotions: Vec<(Emotion, f32)>, // emotion and intensity
    pub stress_level: f32, // 0.0 to 1.0
    pub excitement_level: f32, // 0.0 to 1.0
    pub frustration_level: f32, // 0.0 to 1.0
    pub satisfaction_level: f32, // 0.0 to 1.0
    pub engagement_level: f32, // 0.0 to 1.0
    pub ai_emotion_confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Emotion {
    Joy,
    Sadness,
    Anger,
    Fear,
    Surprise,
    Disgust,
    Anticipation,
    Trust,
    Curiosity,
    Boredom,
    Excitement,
    Frustration,
    Contentment,
    Anxiety,
    Pride,
    Shame,
    AIDetected(String), // AI-detected custom emotions
}

/// AI classification of player behavior
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorClassification {
    pub primary_type: BehaviorType,
    pub secondary_types: Vec<(BehaviorType, f32)>, // type and confidence
    pub play_style: PlayStyle,
    pub risk_preference: RiskPreference,
    pub social_preference: SocialPreference,
    pub exploration_tendency: f32, // 0.0 to 1.0
    pub competitiveness: f32, // 0.0 to 1.0
    pub creativity_score: f32, // 0.0 to 1.0
    pub ai_classification_confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BehaviorType {
    Explorer,
    Builder,
    Fighter,
    Socializer,
    Achiever,
    Collector,
    Experimenter,
    Helper,
    Leader,
    Follower,
    Strategist,
    Improviser,
    AIIdentified(String), // AI-discovered behavior types
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayStyle {
    Casual,
    Hardcore,
    Competitive,
    Cooperative,
    Creative,
    Speedrunner,
    Completionist,
    Roleplay,
    AIAdaptive, // AI-adapted play style
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RiskPreference {
    VeryConservative,
    Conservative,
    Moderate,
    Aggressive,
    VeryAggressive,
    AIAdaptive, // Changes based on AI analysis
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SocialPreference {
    SoloPlayer,
    SmallGroup,
    LargeGroup,
    CommunityOriented,
    AntiSocial,
    AIFlexible, // AI determines best social context
}

/// AI prediction of next player action
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictedAction {
    pub predicted_action: PlayerActionType,
    pub confidence: f32, // 0.0 to 1.0
    pub time_to_action_ms: u64,
    pub contributing_factors: Vec<PredictionFactor>,
    pub alternative_predictions: Vec<(PlayerActionType, f32)>, // action and probability
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionFactor {
    pub factor_name: String,
    pub influence_weight: f32,
    pub description: String,
}

/// World state context at time of action
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldStateContext {
    pub world_time: TimeOfDay,
    pub weather: WeatherType,
    pub local_events: Vec<String>,
    pub resource_availability: HashMap<String, f32>,
    pub npc_activity_level: f32,
    pub player_density: f32,
    pub threat_level: f32,
    pub opportunity_score: f32, // AI-assessed opportunities available
}

/// Social context during action
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialContext {
    pub group_membership: Vec<EntityId>,
    pub leadership_role: Option<String>,
    pub reputation_scores: HashMap<String, f32>, // faction/group -> reputation
    pub recent_social_interactions: u32,
    pub social_pressure_level: f32,
    pub cooperation_opportunities: u32,
    pub conflict_situations: u32,
}

/// Environmental factors affecting behavior
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalFactors {
    pub biome_type: BiomeType,
    pub danger_level: f32,
    pub resource_richness: f32,
    pub accessibility: f32,
    pub aesthetic_appeal: f32, // AI-assessed beauty/appeal
    pub novelty_factor: f32, // How new/unexplored the area is
    pub comfort_level: f32, // Player's familiarity with environment
}

/// Player behavior pattern analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerBehaviorPattern {
    pub id: EntityId,
    pub player_id: PlayerId,
    pub pattern_name: String,
    pub pattern_type: PatternType,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    
    // Pattern characteristics
    pub frequency: f32, // How often this pattern occurs
    pub consistency: f32, // How consistent the pattern is
    pub predictability: f32, // How predictable the pattern is
    pub complexity: f32, // How complex the pattern is
    
    // Temporal aspects
    pub time_of_day_correlation: Vec<(u8, f32)>, // hour -> correlation strength
    pub day_of_week_correlation: Vec<(u8, f32)>, // day -> correlation strength
    pub seasonal_correlation: HashMap<Season, f32>,
    
    // Contextual triggers
    pub environmental_triggers: Vec<EnvironmentalTrigger>,
    pub social_triggers: Vec<SocialTrigger>,
    pub emotional_triggers: Vec<EmotionalTrigger>,
    
    // AI analysis
    pub ai_confidence: f32,
    pub ai_learning_priority: f32,
    pub related_patterns: Vec<EntityId>,
    
    // Metadata
    pub metadata: EntityMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PatternType {
    Routine, // Regular, predictable behaviors
    Reactive, // Responses to specific stimuli
    Exploratory, // Discovery and experimentation patterns
    Social, // Interaction patterns with others
    Adaptive, // Learning and adaptation patterns
    Emergent, // Unexpected patterns discovered by AI
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalTrigger {
    pub trigger_type: String,
    pub threshold_value: f32,
    pub correlation_strength: f32,
    pub confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialTrigger {
    pub trigger_type: String,
    pub required_participants: u32,
    pub social_context_requirements: Vec<String>,
    pub correlation_strength: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmotionalTrigger {
    pub emotion: Emotion,
    pub intensity_threshold: f32,
    pub duration_requirement_ms: u64,
    pub correlation_strength: f32,
}

/// Player session summary
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerSession {
    pub id: EntityId,
    pub session_id: SessionId,
    pub player_id: PlayerId,
    pub start_time: Timestamp,
    pub end_time: Option<Timestamp>,
    pub duration_ms: u64,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    
    // Session metrics
    pub total_actions: u32,
    pub unique_action_types: u32,
    pub average_action_interval_ms: u64,
    pub peak_activity_period: Option<(Timestamp, Timestamp)>,
    
    // Behavioral summary
    pub dominant_behavior_types: Vec<(BehaviorType, f32)>,
    pub emotional_journey: Vec<(Timestamp, EmotionalState)>,
    pub satisfaction_trend: Vec<(Timestamp, f32)>,
    pub engagement_score: f32,
    
    // Achievements and progress
    pub goals_achieved: Vec<String>,
    pub new_discoveries: u32,
    pub skill_improvements: HashMap<String, f32>,
    pub social_connections_made: u32,
    
    // AI insights
    pub ai_session_summary: String,
    pub ai_recommendations: Vec<AIRecommendation>,
    pub learning_opportunities_identified: Vec<String>,
    pub ai_confidence_in_analysis: f32,
    
    // Metadata
    pub metadata: EntityMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIRecommendation {
    pub recommendation_type: String,
    pub description: String,
    pub confidence: f32,
    pub expected_impact: f32,
    pub implementation_difficulty: f32,
    pub personalization_level: f32,
}

// Implement the DatabaseModel trait for PlayerBehavior
impl_database_model!(PlayerBehavior, id, created_at, updated_at);
impl_database_model!(PlayerBehaviorPattern, id, created_at, updated_at);
impl_database_model!(PlayerSession, id, created_at, updated_at);

// Implement AIModel trait for behavior types
impl AIModel for PlayerBehavior {
    fn confidence_score(&self) -> f32 {
        self.ai_confidence_score
    }
    
    fn ai_context(&self) -> &AIContext {
        // This would need to be added to the struct or derived from existing data
        // For now, we'll create a default implementation
        &AIContext {
            model_version: "behavior_analyzer_v1.0".to_string(),
            decision_algorithm: "behavioral_classification".to_string(),
            training_data_hash: None,
            confidence_threshold: 0.7,
            processing_time_ms: self.action_duration_ms,
            resource_usage: ResourceUsage {
                cpu_usage_percent: 0.0,
                memory_usage_mb: 0.0,
                gpu_usage_percent: None,
                network_io_bytes: 0,
                disk_io_bytes: 0,
            },
        }
    }
    
    fn has_learning_data(&self) -> bool {
        self.learning_value > 0.0
    }
}