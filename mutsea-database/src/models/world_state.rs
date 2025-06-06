// mutsea-database/src/models/world_state.rs
//! World state models for the AI-driven Mutsea engine
//! 
//! These models represent the current state of the virtual world,
//! including terrain, weather, resources, and dynamic elements
//! that are influenced by AI systems.

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete world state snapshot
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldState {
    pub id: EntityId,
    pub world_id: WorldId,
    pub snapshot_timestamp: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    
    // World properties
    pub world_name: String,
    pub world_seed: u64,
    pub world_version: String,
    pub world_size: WorldSize,
    
    // Environmental state
    pub weather_state: WeatherState,
    pub time_of_day: TimeOfDay,
    pub season: Season,
    pub global_temperature: f32,
    pub global_humidity: f32,
    
    // AI-generated content metrics
    pub ai_generated_regions: u32,
    pub ai_active_processes: u32,
    pub world_complexity_score: f32,
    pub emergent_behaviors_count: u32,
    
    // Resource distribution
    pub resource_nodes: Vec<ResourceNode>,
    pub biome_distribution: HashMap<BiomeId, BiomeState>,
    
    // Dynamic elements
    pub active_events: Vec<WorldEvent>,
    pub npc_population: NPCPopulationStats,
    pub player_activity_zones: Vec<ActivityZone>,
    
    // Performance metrics
    pub performance_metrics: WorldPerformanceMetrics,
    pub ai_processing_load: f32,
    
    // Metadata
    pub metadata: EntityMetadata,
}

/// World size configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldSize {
    pub width_chunks: u32,
    pub height_chunks: u32,
    pub chunk_size: u32,
    pub max_altitude: f32,
    pub min_altitude: f32,
}

/// Weather state controlled by AI
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherState {
    pub current_weather: WeatherType,
    pub weather_intensity: f32, // 0.0 to 1.0
    pub wind_direction: Vector3,
    pub wind_speed: f32,
    pub precipitation_amount: f32,
    pub cloud_coverage: f32, // 0.0 to 1.0
    pub visibility: f32, // meters
    pub pressure: f32, // hPa
    pub ai_weather_pattern: Option<AIWeatherPattern>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WeatherType {
    Clear,
    PartlyCloudy,
    Cloudy,
    LightRain,
    HeavyRain,
    Thunderstorm,
    Snow,
    Blizzard,
    Fog,
    Sandstorm,
    AuroraBorealis, // AI-generated special weather
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIWeatherPattern {
    pub pattern_name: String,
    pub ai_confidence: f32,
    pub predicted_duration_hours: f32,
    pub influences: Vec<WeatherInfluence>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherInfluence {
    pub factor: String, // "player_activity", "ecosystem_health", etc.
    pub influence_strength: f32, // -1.0 to 1.0
}

/// Time of day with AI-driven dynamic progression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeOfDay {
    pub hour: u8, // 0-23
    pub minute: u8, // 0-59
    pub time_scale: f32, // Time multiplier controlled by AI
    pub is_ai_accelerated: bool,
    pub lighting_conditions: LightingConditions,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LightingConditions {
    pub ambient_light_level: f32, // 0.0 to 1.0
    pub sun_angle: f32, // degrees
    pub moon_phase: f32, // 0.0 to 1.0
    pub artificial_light_sources: u32,
}

/// Seasons with AI-driven transitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
    AIGenerated(String), // AI can create custom seasons
}

/// Resource nodes managed by AI
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceNode {
    pub node_id: EntityId,
    pub position: WorldPosition,
    pub resource_type: ResourceType,
    pub quantity_available: f32,
    pub quality_rating: f32, // 0.0 to 1.0
    pub regeneration_rate: f32,
    pub ai_managed: bool,
    pub access_difficulty: f32,
    pub discovery_timestamp: Option<Timestamp>,
    pub depletion_risk: f32, // AI prediction
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    // Basic resources
    Wood,
    Stone,
    Metal,
    Water,
    Food,
    
    // Energy resources
    Coal,
    Oil,
    Solar,
    Wind,
    Geothermal,
    
    // Rare resources
    Crystal,
    Gems,
    RareEarth,
    
    // AI-generated resources
    AIGenerated {
        name: String,
        properties: HashMap<String, f32>,
        rarity_score: f32,
    },
}

/// Biome state within the world
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BiomeState {
    pub biome_id: BiomeId,
    pub biome_type: BiomeType,
    pub health_score: f32, // 0.0 to 1.0
    pub biodiversity_index: f32,
    pub ai_stability_score: f32,
    pub area_coverage: f32, // percentage of world
    pub dominant_species: Vec<String>,
    pub climate_conditions: ClimateConditions,
    pub ai_interventions_count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BiomeType {
    Forest,
    Desert,
    Plains,
    Mountains,
    Ocean,
    River,
    Swamp,
    Tundra,
    Jungle,
    Volcanic,
    AIGenerated {
        name: String,
        characteristics: HashMap<String, f32>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClimateConditions {
    pub average_temperature: f32,
    pub temperature_variance: f32,
    pub humidity: f32,
    pub precipitation: f32,
    pub wind_patterns: Vec<Vector3>,
}

/// World events generated by AI
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldEvent {
    pub event_id: EntityId,
    pub event_type: WorldEventType,
    pub position: Option<WorldPosition>,
    pub affected_area_radius: f32,
    pub severity: f32, // 0.0 to 1.0
    pub start_time: Timestamp,
    pub estimated_duration: Option<u64>, // seconds
    pub ai_generated: bool,
    pub ai_confidence: f32,
    pub effects: Vec<EventEffect>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorldEventType {
    // Natural events
    Earthquake,
    Flood,
    Drought,
    Wildfire,
    Storm,
    
    // Ecological events
    Migration,
    Bloom,
    Disease,
    Extinction,
    Evolution,
    
    // AI-generated events
    AIAnomaly,
    EmergentBehavior,
    SystemGlitch,
    AICreatedEvent(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventEffect {
    pub effect_type: String,
    pub intensity: f32,
    pub duration_hours: f32,
    pub affected_entities: Vec<EntityId>,
}

/// NPC population statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NPCPopulationStats {
    pub total_npcs: u32,
    pub active_npcs: u32,
    pub ai_controlled_npcs: u32,
    pub average_intelligence_level: f32,
    pub population_density_per_chunk: f32,
    pub social_network_connections: u32,
    pub collective_mood: f32, // -1.0 to 1.0
}

/// Player activity zones
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivityZone {
    pub zone_id: EntityId,
    pub center_position: WorldPosition,
    pub radius: f32,
    pub activity_type: ActivityType,
    pub intensity: f32, // 0.0 to 1.0
    pub player_count: u32,
    pub duration_hours: f32,
    pub ai_recommendations: Vec<AIRecommendation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActivityType {
    Building,
    Combat,
    Exploration,
    ResourceGathering,
    Trading,
    Social,
    Crafting,
    Research,
    AIExperiment,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIRecommendation {
    pub recommendation_type: RecommendationType,
    pub confidence: f32,
    pub priority: u8, // 1-10
    pub description: String,
    pub expected_benefit: f32,
    pub resource_cost: Option<ResourceCost>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecommendationType {
    OptimizeResources,
    ExpandTerritory,
    ImproveDefenses,
    EnhanceProduction,
    SocialInteraction,
    ExploreArea,
    CompleteQuest,
    AIGenerated(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceCost {
    pub resources: HashMap<ResourceType, f32>,
    pub time_hours: f32,
    pub energy_cost: f32,
}

/// World performance metrics tracked by AI
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldPerformanceMetrics {
    pub fps_average: f32,
    pub frame_time_ms: f32,
    pub memory_usage_mb: f32,
    pub cpu_usage_percent: f32,
    pub gpu_usage_percent: Option<f32>,
    pub network_latency_ms: f32,
    pub active_entities_count: u32,
    pub render_distance: f32,
    pub chunk_loading_time_ms: f32,
    pub ai_processing_time_ms: f32,
    pub physics_simulation_time_ms: f32,
    pub audio_processing_time_ms: f32,
}

/// Terrain chunk information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerrainChunk {
    pub chunk_id: EntityId,
    pub position: ChunkPosition,
    pub terrain_type: TerrainType,
    pub elevation_map: Vec<f32>,
    pub vegetation_density: f32,
    pub water_level: f32,
    pub mineral_deposits: Vec<ResourceNode>,
    pub ai_generated_features: Vec<AIGeneratedFeature>,
    pub stability_score: f32,
    pub last_modified: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32,
    pub region_x: i32,
    pub region_z: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TerrainType {
    Flat,
    Hills,
    Mountains,
    Valley,
    Plateau,
    Canyon,
    Crater,
    Island,
    AIGenerated {
        name: String,
        characteristics: HashMap<String, f32>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIGeneratedFeature {
    pub feature_id: EntityId,
    pub feature_type: String,
    pub position: WorldPosition,
    pub size: Vector3,
    pub ai_confidence: f32,
    pub generation_algorithm: String,
    pub parameters: HashMap<String, f32>,
    pub creation_timestamp: Timestamp,
}

/// World generation parameters for AI
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldGenerationParams {
    pub seed: u64,
    pub world_size: WorldSize,
    pub biome_preferences: HashMap<BiomeType, f32>,
    pub resource_abundance: f32,
    pub terrain_complexity: f32,
    pub weather_variability: f32,
    pub ai_creativity_level: f32, // 0.0 to 1.0
    pub player_count_target: u32,
    pub performance_target: PerformanceTarget,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceTarget {
    pub target_fps: u32,
    pub max_memory_mb: u32,
    pub max_cpu_usage: f32,
    pub max_network_bandwidth_mbps: f32,
}

/// Ecosystem health metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcosystemHealth {
    pub overall_health: f32, // 0.0 to 1.0
    pub biodiversity_score: f32,
    pub sustainability_index: f32,
    pub pollution_level: f32,
    pub carrying_capacity_usage: f32,
    pub species_balance: SpeciesBalance,
    pub resource_regeneration_rate: f32,
    pub ai_intervention_frequency: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeciesBalance {
    pub predators: u32,
    pub prey: u32,
    pub producers: u32,
    pub decomposers: u32,
    pub balance_ratio: f32, // Ideal is close to 1.0
}

/// World state delta for efficient updates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldStateDelta {
    pub delta_id: EntityId,
    pub world_id: WorldId,
    pub base_state_id: EntityId,
    pub timestamp: Timestamp,
    pub changes: Vec<StateChange>,
    pub ai_generated_changes: u32,
    pub player_triggered_changes: u32,
    pub system_triggered_changes: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateChange {
    pub change_id: EntityId,
    pub entity_id: EntityId,
    pub change_type: ChangeType,
    pub field_path: String,
    pub old_value: Option<serde_json::Value>,
    pub new_value: serde_json::Value,
    pub confidence: f32,
    pub source: ChangeSource,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeType {
    Create,
    Update,
    Delete,
    Move,
    Transform,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeSource {
    PlayerAction(PlayerId),
    AISystem(String),
    GameLogic,
    ExternalEvent,
    SystemMaintenance,
}

// Implement DatabaseModel for WorldState
impl_database_model!(WorldState, id, created_at, updated_at);

impl DatabaseModel for WorldState {
    fn id(&self) -> EntityId {
        self.id
    }
    
    fn created_at(&self) -> Timestamp {
        self.created_at
    }
    
    fn updated_at(&self) -> Timestamp {
        self.updated_at
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        if self.world_name.is_empty() {
            return Err(ValidationError {
                field: "world_name".to_string(),
                message: "World name cannot be empty".to_string(),
                code: ValidationErrorCode::Required,
            });
        }
        
        if self.world_complexity_score < 0.0 || self.world_complexity_score > 1.0 {
            return Err(ValidationError {
                field: "world_complexity_score".to_string(),
                message: "Complexity score must be between 0.0 and 1.0".to_string(),
                code: ValidationErrorCode::OutOfRange,
            });
        }
        
        if self.ai_processing_load < 0.0 || self.ai_processing_load > 1.0 {
            return Err(ValidationError {
                field: "ai_processing_load".to_string(),
                message: "AI processing load must be between 0.0 and 1.0".to_string(),
                code: ValidationErrorCode::OutOfRange,
            });
        }
        
        Ok(())
    }
}

impl AIModel for WorldState {
    fn confidence_score(&self) -> f32 {
        // Average confidence across AI-generated content
        let mut total_confidence = 0.0;
        let mut count = 0;
        
        if let Some(weather_pattern) = &self.weather_state.ai_weather_pattern {
            total_confidence += weather_pattern.ai_confidence;
            count += 1;
        }
        
        for event in &self.active_events {
            if event.ai_generated {
                total_confidence += event.ai_confidence;
                count += 1;
            }
        }
        
        if count > 0 {
            total_confidence / count as f32
        } else {
            0.0
        }
    }
    
    fn ai_context(&self) -> &AIContext {
        // This would typically be stored as a field, but for now we'll create a static reference
        // In a real implementation, this should be a field in the struct
        static DEFAULT_CONTEXT: std::sync::OnceLock<AIContext> = std::sync::OnceLock::new();
        DEFAULT_CONTEXT.get_or_init(|| AIContext {
            model_version: "world-ai-v1.0".to_string(),
            decision_algorithm: "world-state-manager".to_string(),
            training_data_hash: None,
            confidence_threshold: 0.7,
            processing_time_ms: 100,
            resource_usage: ResourceUsage {
                cpu_usage_percent: 15.0,
                memory_usage_mb: 512.0,
                gpu_usage_percent: Some(8.0),
                network_io_bytes: 1024,
                disk_io_bytes: 2048,
            },
        })
    }
    
    fn has_learning_data(&self) -> bool {
        self.ai_generated_regions > 0 || self.emergent_behaviors_count > 0
    }
}

// Utility implementations
impl WorldState {
    pub fn new(world_id: WorldId, world_name: String, world_seed: u64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            world_id,
            snapshot_timestamp: now,
            created_at: now,
            updated_at: now,
            world_name,
            world_seed,
            world_version: "1.0.0".to_string(),
            world_size: WorldSize {
                width_chunks: 1000,
                height_chunks: 1000,
                chunk_size: 16,
                max_altitude: 256.0,
                min_altitude: -64.0,
            },
            weather_state: WeatherState::default(),
            time_of_day: TimeOfDay::default(),
            season: Season::Spring,
            global_temperature: 20.0,
            global_humidity: 0.5,
            ai_generated_regions: 0,
            ai_active_processes: 0,
            world_complexity_score: 0.5,
            emergent_behaviors_count: 0,
            resource_nodes: Vec::new(),
            biome_distribution: HashMap::new(),
            active_events: Vec::new(),
            npc_population: NPCPopulationStats::default(),
            player_activity_zones: Vec::new(),
            performance_metrics: WorldPerformanceMetrics::default(),
            ai_processing_load: 0.0,
            metadata: EntityMetadata::default(),
        }
    }
    
    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now();
        self.snapshot_timestamp = self.updated_at;
    }
    
    pub fn add_resource_node(&mut self, node: ResourceNode) {
        self.resource_nodes.push(node);
        self.update_timestamp();
    }
    
    pub fn get_biome_health(&self, biome_id: &BiomeId) -> Option<f32> {
        self.biome_distribution.get(biome_id).map(|b| b.health_score)
    }
    
    pub fn calculate_world_stability(&self) -> f32 {
        let biome_stability: f32 = self.biome_distribution.values()
            .map(|b| b.ai_stability_score)
            .sum::<f32>() / self.biome_distribution.len() as f32;
        
        let event_stability = 1.0 - (self.active_events.len() as f32 * 0.1).min(1.0);
        let performance_stability = (self.performance_metrics.fps_average / 60.0).min(1.0);
        
        (biome_stability + event_stability + performance_stability) / 3.0
    }
}

// Default implementations for complex types
impl Default for WeatherState {
    fn default() -> Self {
        Self {
            current_weather: WeatherType::Clear,
            weather_intensity: 0.5,
            wind_direction: Vector3::new(1.0, 0.0, 0.0),
            wind_speed: 5.0,
            precipitation_amount: 0.0,
            cloud_coverage: 0.3,
            visibility: 10000.0,
            pressure: 1013.25,
            ai_weather_pattern: None,
        }
    }
}

impl Default for TimeOfDay {
    fn default() -> Self {
        Self {
            hour: 12,
            minute: 0,
            time_scale: 1.0,
            is_ai_accelerated: false,
            lighting_conditions: LightingConditions {
                ambient_light_level: 0.8,
                sun_angle: 45.0,
                moon_phase: 0.5,
                artificial_light_sources: 0,
            },
        }
    }
}

impl Default for NPCPopulationStats {
    fn default() -> Self {
        Self {
            total_npcs: 0,
            active_npcs: 0,
            ai_controlled_npcs: 0,
            average_intelligence_level: 0.5,
            population_density_per_chunk: 0.0,
            social_network_connections: 0,
            collective_mood: 0.0,
        }
    }
}

impl Default for WorldPerformanceMetrics {
    fn default() -> Self {
        Self {
            fps_average: 60.0,
            frame_time_ms: 16.67,
            memory_usage_mb: 1024.0,
            cpu_usage_percent: 25.0,
            gpu_usage_percent: Some(30.0),
            network_latency_ms: 50.0,
            active_entities_count: 1000,
            render_distance: 128.0,
            chunk_loading_time_ms: 10.0,
            ai_processing_time_ms: 5.0,
            physics_simulation_time_ms: 2.0,
            audio_processing_time_ms: 1.0,
        }
    }
}