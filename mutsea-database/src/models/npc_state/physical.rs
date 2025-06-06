// mutsea-database/src/models/npc_state/physical.rs
//! Physical state and appearance models for NPCs

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Movement and physical state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovementState {
    pub current_speed: f32,
    pub max_speed: f32,
    pub movement_type: MovementType,
    pub destination: Option<WorldPosition>,
    pub path: Vec<WorldPosition>,
    pub movement_efficiency: f32,
    pub stamina: f32, // 0.0 to 1.0
    pub mobility_constraints: Vec<MobilityConstraint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MovementType {
    Walking,
    Running,
    Flying,
    Swimming,
    Climbing,
    Teleporting,
    Phasing,
    Stationary,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MobilityConstraint {
    Injured,
    Encumbered,
    Terrain,
    Social,
    Magical,
    Technological,
}

/// Physical condition and capabilities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalCondition {
    pub health_percentage: f32, // 0.0 to 1.0
    pub strength: f32,
    pub agility: f32,
    pub endurance: f32,
    pub constitution: f32,
    pub size_category: SizeCategory,
    pub weight: f32, // kg
    pub height: f32, // meters
    pub physical_anomalies: Vec<PhysicalAnomaly>,
    pub disabilities: Vec<Disability>,
    pub enhancements: Vec<Enhancement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SizeCategory {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalAnomaly {
    pub anomaly_type: String,
    pub severity: f32,
    pub effects: Vec<String>,
    pub is_beneficial: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disability {
    pub disability_type: String,
    pub severity: f32,
    pub compensation_methods: Vec<String>,
    pub adaptive_technology: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enhancement {
    pub enhancement_type: String,
    pub enhancement_level: f32,
    pub source: EnhancementSource,
    pub side_effects: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnhancementSource {
    Biological,
    Technological,
    Magical,
    Psionic,
    Chemical,
    Training,
}

/// NPC appearance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NPCAppearance {
    pub general_description: String,
    pub distinctive_features: Vec<String>,
    pub clothing: Vec<ClothingItem>,
    pub accessories: Vec<Accessory>,
    pub scars_and_marks: Vec<BodyMark>,
    pub coloration: ColorationDetails,
    pub grooming_state: GroomingState,
    pub attractiveness_score: f32, // Subjective, context-dependent
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClothingItem {
    pub item_name: String,
    pub item_type: ClothingType,
    pub condition: ItemCondition,
    pub style: String,
    pub cultural_significance: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClothingType {
    Headwear,
    Outerwear,
    Underwear,
    Footwear,
    Gloves,
    Jewelry,
    Armor,
    Ceremonial,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemCondition {
    New,
    Good,
    Worn,
    Damaged,
    Tattered,
    Broken,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Accessory {
    pub accessory_name: String,
    pub accessory_type: String,
    pub functionality: Vec<String>,
    pub sentimental_value: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BodyMark {
    pub mark_type: BodyMarkType,
    pub location: String,
    pub description: String,
    pub origin_story: Option<String>,
    pub age_of_mark: Option<f32>, // Years
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BodyMarkType {
    Scar,
    Birthmark,
    Tattoo,
    Piercing,
    Brand,
    Ritual,
    Disease,
    Mutation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorationDetails {
    pub primary_color: String,
    pub secondary_colors: Vec<String>,
    pub pattern_type: Option<String>,
    pub seasonal_changes: bool,
    pub emotional_color_changes: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GroomingState {
    Immaculate,
    WellGroomed,
    Average,
    Unkempt,
    Disheveled,
    Feral,
}

/// Skills and abilities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillSet {
    pub combat_skills: Vec<CombatSkill>,
    pub social_skills: Vec<SocialSkill>,
    pub intellectual_skills: Vec<IntellectualSkill>,
    pub practical_skills: Vec<PracticalSkill>,
    pub creative_skills: Vec<CreativeSkill>,
    pub magical_skills: Vec<MagicalSkill>,
    pub technological_skills: Vec<TechnologicalSkill>,
    pub survival_skills: Vec<SurvivalSkill>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CombatSkill {
    pub skill_name: String,
    pub proficiency: f32, // 0.0 to 1.0
    pub experience_points: f32,
    pub specializations: Vec<String>,
    pub combat_style: String,
    pub weapon_preferences: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialSkill {
    pub skill_name: String,
    pub proficiency: f32,
    pub cultural_contexts: Vec<String>,
    pub effectiveness_modifiers: HashMap<String, f32>,
    pub preferred_situations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntellectualSkill {
    pub skill_name: String,
    pub proficiency: f32,
    pub knowledge_domains: Vec<String>,
    pub learning_methods: Vec<String>,
    pub application_areas: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PracticalSkill {
    pub skill_name: String,
    pub proficiency: f32,
    pub tool_requirements: Vec<String>,
    pub resource_efficiency: f32,
    pub quality_consistency: f32,
    pub innovation_capability: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreativeSkill {
    pub skill_name: String,
    pub proficiency: f32,
    pub artistic_style: String,
    pub inspiration_sources: Vec<String>,
    pub medium_preferences: Vec<String>,
    pub creativity_rating: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MagicalSkill {
    pub skill_name: String,
    pub proficiency: f32,
    pub magic_school: String,
    pub mana_efficiency: f32,
    pub spell_repertoire: Vec<String>,
    pub magical_focus_items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TechnologicalSkill {
    pub skill_name: String,
    pub proficiency: f32,
    pub technology_level: String,
    pub innovation_ability: f32,
    pub maintenance_capability: f32,
    pub adaptation_speed: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurvivalSkill {
    pub skill_name: String,
    pub proficiency: f32,
    pub environmental_specialization: Vec<String>,
    pub resource_identification: f32,
    pub threat_assessment: f32,
    pub adaptation_ability: f32,
}

/// NPC abilities (special powers or traits)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NPCAbility {
    pub ability_id: EntityId,
    pub ability_name: String,
    pub ability_type: AbilityType,
    pub power_level: f32,
    pub activation_cost: ActivationCost,
    pub cooldown_time: Option<u64>, // seconds
    pub range: AbilityRange,
    pub duration: Option<u64>, // seconds
    pub prerequisites: Vec<String>,
    pub side_effects: Vec<String>,
    pub mastery_level: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AbilityType {
    Physical,
    Mental,
    Magical,
    Technological,
    Spiritual,
    Social,
    Environmental,
    Temporal,
    Dimensional,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivationCost {
    pub energy_cost: f32,
    pub resource_costs: HashMap<String, f32>,
    pub health_cost: Option<f32>,
    pub time_cost: u64, // milliseconds
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AbilityRange {
    Self_,
    Touch,
    Short(f32),    // meters
    Medium(f32),   // meters
    Long(f32),     // meters
    Unlimited,
    AreaOfEffect { center: WorldPosition, radius: f32 },
    LineOfSight,
}

/// Learning progress tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningProgress {
    pub skills_being_learned: Vec<SkillLearning>,
    pub learning_efficiency: f32,
    pub learning_motivation: f32,
    pub learning_style_preferences: Vec<LearningStyle>,
    pub knowledge_retention_rate: f32,
    pub skill_transfer_ability: f32,
    pub learning_obstacles: Vec<LearningObstacle>,
    pub mentors_and_teachers: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillLearning {
    pub skill_name: String,
    pub current_level: f32,
    pub target_level: f32,
    pub learning_rate: f32,
    pub practice_time_invested: f32, // hours
    pub learning_method: String,
    pub difficulty_rating: f32,
    pub motivation_level: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LearningStyle {
    Visual,
    Auditory,
    Kinesthetic,
    Reading_Writing,
    Experiential,
    Social,
    Solitary,
    Structured,
    Exploratory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningObstacle {
    pub obstacle_type: String,
    pub severity: f32,
    pub impact_on_learning: f32,
    pub potential_solutions: Vec<String>,
    pub workaround_strategies: Vec<String>,
}

/// Environmental interactions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentInteraction {
    pub interaction_id: EntityId,
    pub interaction_type: EnvironmentInteractionType,
    pub target_object: Option<EntityId>,
    pub location: WorldPosition,
    pub duration: u64, // seconds
    pub intensity: f32,
    pub outcome: InteractionOutcome,
    pub environmental_impact: f32,
    pub learned_information: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnvironmentInteractionType {
    Observation,
    Manipulation,
    Consumption,
    Creation,
    Destruction,
    Exploration,
    Testing,
    Maintenance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InteractionOutcome {
    Success,
    Failure,
    PartialSuccess,
    Unexpected,
    Dangerous,
    Beneficial,
}

/// Territory and spatial preferences
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Territory {
    pub territory_id: EntityId,
    pub center_point: WorldPosition,
    pub boundaries: Vec<WorldPosition>,
    pub territory_type: TerritoryType,
    pub ownership_strength: f32,
    pub defense_capability: f32,
    pub resource_value: f32,
    pub strategic_importance: f32,
    pub sharing_rules: SharingRules,
    pub territorial_markers: Vec<TerritorialMarker>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TerritoryType {
    Home,
    Work,
    Hunting,
    Foraging,
    Social,
    Sacred,
    Strategic,
    Temporary,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SharingRules {
    Exclusive,
    FamilyOnly,
    GroupOnly,
    Conditional,
    Open,
    TimeShared,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerritorialMarker {
    pub marker_type: String,
    pub location: WorldPosition,
    pub visibility: f32,
    pub meaning: String,
    pub durability: f32,
}

/// Environmental preferences
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalPreferences {
    pub preferred_biomes: Vec<BiomeType>,
    pub preferred_weather: Vec<WeatherType>,
    pub preferred_time_of_day: Vec<u8>, // Hours 0-23
    pub temperature_preferences: TemperatureRange,
    pub humidity_preferences: HumidityRange,
    pub altitude_preferences: AltitudeRange,
    pub noise_tolerance: f32,
    pub light_sensitivity: f32,
    pub social_density_preference: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemperatureRange {
    pub min_comfortable: f32, // Celsius
    pub max_comfortable: f32,
    pub min_tolerable: f32,
    pub max_tolerable: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HumidityRange {
    pub min_comfortable: f32, // 0.0 to 1.0
    pub max_comfortable: f32,
    pub min_tolerable: f32,
    pub max_tolerable: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AltitudeRange {
    pub min_comfortable: f32, // meters above sea level
    pub max_comfortable: f32,
    pub min_tolerable: f32,
    pub max_tolerable: f32,
}

// Default implementations
impl Default for MovementState {
    fn default() -> Self {
        Self {
            current_speed: 0.0,
            max_speed: 5.0, // m/s
            movement_type: MovementType::Walking,
            destination: None,
            path: Vec::new(),
            movement_efficiency: 0.8,
            stamina: 1.0,
            mobility_constraints: Vec::new(),
        }
    }
}

impl Default for PhysicalCondition {
    fn default() -> Self {
        Self {
            health_percentage: 1.0,
            strength: 0.7,
            agility: 0.7,
            endurance: 0.7,
            constitution: 0.7,
            size_category: SizeCategory::Medium,
            weight: 70.0, // kg
            height: 1.75, // meters
            physical_anomalies: Vec::new(),
            disabilities: Vec::new(),
            enhancements: Vec::new(),
        }
    }
}

impl Default for NPCAppearance {
    fn default() -> Self {
        Self {
            general_description: "Average appearance".to_string(),
            distinctive_features: Vec::new(),
            clothing: Vec::new(),
            accessories: Vec::new(),
            scars_and_marks: Vec::new(),
            coloration: ColorationDetails::default(),
            grooming_state: GroomingState::Average,
            attractiveness_score: 0.5,
        }
    }
}

impl Default for ColorationDetails {
    fn default() -> Self {
        Self {
            primary_color: "Brown".to_string(),
            secondary_colors: Vec::new(),
            pattern_type: None,
            seasonal_changes: false,
            emotional_color_changes: false,
        }
    }
}

impl Default for SkillSet {
    fn default() -> Self {
        Self {
            combat_skills: Vec::new(),
            social_skills: Vec::new(),
            intellectual_skills: Vec::new(),
            practical_skills: Vec::new(),
            creative_skills: Vec::new(),
            magical_skills: Vec::new(),
            technological_skills: Vec::new(),
            survival_skills: Vec::new(),
        }
    }
}

impl Default for LearningProgress {
    fn default() -> Self {
        Self {
            skills_being_learned: Vec::new(),
            learning_efficiency: 0.6,
            learning_motivation: 0.7,
            learning_style_preferences: vec![LearningStyle::Experiential],
            knowledge_retention_rate: 0.7,
            skill_transfer_ability: 0.5,
            learning_obstacles: Vec::new(),
            mentors_and_teachers: Vec::new(),
        }
    }
}

impl Default for EnvironmentalPreferences {
    fn default() -> Self {
        Self {
            preferred_biomes: vec![BiomeType::Forest, BiomeType::Plains],
            preferred_weather: vec![WeatherType::Clear, WeatherType::PartlyCloudy],
            preferred_time_of_day: vec![8, 9, 10, 14, 15, 16], // Morning and afternoon
            temperature_preferences: TemperatureRange {
                min_comfortable: 18.0,
                max_comfortable: 25.0,
                min_tolerable: 5.0,
                max_tolerable: 35.0,
            },
            humidity_preferences: HumidityRange {
                min_comfortable: 0.4,
                max_comfortable: 0.7,
                min_tolerable: 0.2,
                max_tolerable: 0.9,
            },
            altitude_preferences: AltitudeRange {
                min_comfortable: 0.0,
                max_comfortable: 1000.0,
                min_tolerable: -100.0,
                max_tolerable: 3000.0,
            },
            noise_tolerance: 0.6,
            light_sensitivity: 0.4,
            social_density_preference: 0.5,
        }
    }
}