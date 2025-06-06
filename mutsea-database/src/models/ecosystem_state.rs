// mutsea-database/src/models/ecosystem_state.rs
//! Ecosystem state models for AI-driven ecosystem simulation
//! 
//! These models represent the state of virtual ecosystems including
//! species populations, resource flows, and environmental conditions.

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete ecosystem state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcosystemState {
    pub id: EntityId,
    pub ecosystem_id: EntityId,
    pub state_timestamp: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    
    // Basic ecosystem properties
    pub ecosystem_name: String,
    pub biome_type: BiomeType,
    pub geographic_extent: GeographicExtent,
    pub climate_conditions: ClimateConditions,
    
    // Ecosystem health and stability
    pub overall_health: f32, // 0.0 to 1.0
    pub biodiversity_index: f32,
    pub stability_score: f32,
    pub resilience_score: f32,
    pub sustainability_index: f32,
    
    // Species and populations
    pub species_populations: Vec<SpeciesPopulation>,
    pub keystone_species: Vec<EntityId>,
    pub endangered_species: Vec<EntityId>,
    pub invasive_species: Vec<EntityId>,
    pub species_interactions: Vec<SpeciesInteraction>,
    
    // Resource management
    pub resource_pools: Vec<ResourcePool>,
    pub resource_flows: Vec<ResourceFlow>,
    pub nutrient_cycles: Vec<NutrientCycle>,
    pub energy_flows: Vec<EnergyFlow>,
    
    // Environmental factors
    pub environmental_stressors: Vec<EnvironmentalStressor>,
    pub disturbance_events: Vec<DisturbanceEvent>,
    pub seasonal_patterns: Vec<SeasonalPattern>,
    pub microhabitats: Vec<Microhabitat>,
    
    // AI management
    pub ai_interventions: Vec<EcosystemIntervention>,
    pub management_goals: Vec<ManagementGoal>,
    pub monitoring_metrics: Vec<EcosystemMetric>,
    pub predictive_models: Vec<EcosystemPrediction>,
    
    // Metadata
    pub metadata: EntityMetadata,
}

/// Geographic extent of the ecosystem
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeographicExtent {
    pub center_coordinates: WorldPosition,
    pub boundary_points: Vec<WorldPosition>,
    pub total_area: f32, // square meters
    pub elevation_range: (f32, f32), // min, max meters
    pub water_coverage_percentage: f32,
    pub terrain_complexity: f32,
}

/// Climate conditions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClimateConditions {
    pub temperature_range: TemperatureRange,
    pub precipitation_patterns: PrecipitationPattern,
    pub humidity_levels: HumidityRange,
    pub wind_patterns: Vec<WindPattern>,
    pub seasonal_variation: f32,
    pub climate_stability: f32,
    pub microclimates: Vec<Microclimate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrecipitationPattern {
    pub annual_rainfall: f32, // mm
    pub seasonal_distribution: HashMap<Season, f32>,
    pub drought_frequency: f32,
    pub flood_risk: f32,
    pub precipitation_variability: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindPattern {
    pub direction: Vector3,
    pub average_speed: f32,
    pub seasonal_variation: f32,
    pub impact_on_ecosystem: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Microclimate {
    pub location: WorldPosition,
    pub temperature_offset: f32,
    pub humidity_offset: f32,
    pub unique_conditions: Vec<String>,
    pub species_adaptations: Vec<EntityId>,
}

/// Species population data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeciesPopulation {
    pub species_id: EntityId,
    pub species_name: String,
    pub trophic_level: TrophicLevel,
    pub current_population: u64,
    pub carrying_capacity: u64,
    pub population_trend: PopulationTrend,
    pub age_structure: AgeStructure,
    pub genetic_diversity: f32,
    pub habitat_requirements: HabitatRequirements,
    pub reproductive_data: ReproductiveData,
    pub migration_patterns: Vec<MigrationPattern>,
    pub threats: Vec<Threat>,
    pub conservation_status: ConservationStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrophicLevel {
    PrimaryProducer,
    PrimaryConsumer,
    SecondaryConsumer,
    TertiaryConsumer,
    Apex_Predator,
    Decomposer,
    Omnivore,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PopulationTrend {
    Increasing,
    Stable,
    Decreasing,
    Fluctuating,
    Cyclical,
    Extinct,
    Recovering,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgeStructure {
    pub juveniles: u64,
    pub adults: u64,
    pub elderly: u64,
    pub reproductive_age_percentage: f32,
    pub population_pyramid_type: PyramidType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PyramidType {
    Expanding,   // High birth rate
    Stable,      // Balanced
    Contracting, // Aging population
    Irregular,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HabitatRequirements {
    pub required_biomes: Vec<BiomeType>,
    pub territory_size: f32, // square meters per individual
    pub food_requirements: Vec<FoodRequirement>,
    pub shelter_requirements: Vec<String>,
    pub environmental_tolerances: EnvironmentalTolerances,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FoodRequirement {
    pub food_type: String,
    pub daily_amount: f32,
    pub seasonal_variation: f32,
    pub alternative_foods: Vec<String>,
    pub foraging_behavior: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalTolerances {
    pub temperature_range: TemperatureRange,
    pub humidity_range: HumidityRange,
    pub ph_tolerance: (f32, f32),
    pub pollution_tolerance: f32,
    pub noise_tolerance: f32,
    pub human_disturbance_tolerance: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReproductiveData {
    pub breeding_season: Vec<Season>,
    pub gestation_period: u32, // days
    pub offspring_per_cycle: f32,
    pub reproductive_age: (f32, f32), // min, max years
    pub parental_care_duration: u32, // days
    pub reproductive_success_rate: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MigrationPattern {
    pub migration_type: MigrationType,
    pub start_location: WorldPosition,
    pub end_location: WorldPosition,
    pub migration_trigger: String,
    pub duration: u32, // days
    pub survival_rate: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MigrationType {
    Seasonal,
    Breeding,
    Feeding,
    Climate,
    Resource,
    Nomadic,
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Threat {
    pub threat_type: ThreatType,
    pub severity: f32, // 0.0 to 1.0
    pub probability: f32,
    pub impact_on_population: f32,
    pub mitigation_strategies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThreatType {
    Habitat_Loss,
    Climate_Change,
    Pollution,
    Disease,
    Predation,
    Competition,
    Human_Activity,
    Natural_Disaster,
    Invasive_Species,
    Resource_Depletion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConservationStatus {
    Least_Concern,
    Near_Threatened,
    Vulnerable,
    Endangered,
    Critically_Endangered,
    Extinct_In_Wild,
    Extinct,
    Data_Deficient,
}

/// Resource pools in the ecosystem
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcePool {
    pub resource_id: EntityId,
    pub resource_type: EcosystemResourceType,
    pub current_amount: f32,
    pub maximum_capacity: f32,
    pub regeneration_rate: f32,
    pub depletion_rate: f32,
    pub quality_index: f32,
    pub accessibility: f32,
    pub seasonal_variation: HashMap<Season, f32>,
    pub dependent_species: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EcosystemResourceType {
    Water,
    Soil_Nutrients,
    Sunlight,
    Organic_Matter,
    Minerals,
    Food_Sources,
    Nesting_Sites,
    Shelter_Areas,
    Breeding_Grounds,
    Migration_Corridors,
}

/// Resource flows between ecosystem components
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceFlow {
    pub flow_id: EntityId,
    pub source_id: EntityId,
    pub destination_id: EntityId,
    pub resource_type: EcosystemResourceType,
    pub flow_rate: f32,
    pub efficiency: f32,
    pub seasonality: HashMap<Season, f32>,
    pub limiting_factors: Vec<String>,
    pub flow_direction: FlowDirection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlowDirection {
    Unidirectional,
    Bidirectional,
    Cyclical,
    Variable,
}

/// Nutrient cycling processes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NutrientCycle {
    pub cycle_id: EntityId,
    pub nutrient_type: NutrientType,
    pub cycle_stages: Vec<CycleStage>,
    pub cycle_efficiency: f32,
    pub residence_time: f32, // days
    pub bottlenecks: Vec<String>,
    pub human_impacts: Vec<HumanImpact>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NutrientType {
    Nitrogen,
    Phosphorus,
    Carbon,
    Sulfur,
    Potassium,
    Calcium,
    Magnesium,
    Iron,
    Organic_Compounds,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CycleStage {
    pub stage_name: String,
    pub duration: f32, // days
    pub key_organisms: Vec<EntityId>,
    pub environmental_requirements: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HumanImpact {
    pub impact_type: String,
    pub magnitude: f32,
    pub affected_stages: Vec<String>,
    pub mitigation_options: Vec<String>,
}

/// Energy flow through the ecosystem
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyFlow {
    pub flow_id: EntityId,
    pub source_trophic_level: TrophicLevel,
    pub destination_trophic_level: TrophicLevel,
    pub energy_transfer_efficiency: f32,
    pub energy_amount: f32, // joules
    pub seasonal_patterns: HashMap<Season, f32>,
    pub limiting_factors: Vec<String>,
}

/// Environmental stressors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalStressor {
    pub stressor_id: EntityId,
    pub stressor_type: StressorType,
    pub intensity: f32,
    pub duration: Option<u64>, // seconds
    pub affected_area: GeographicExtent,
    pub affected_species: Vec<EntityId>,
    pub ecosystem_impact: f32,
    pub recovery_time: Option<u64>, // seconds
    pub mitigation_measures: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StressorType {
    Temperature_Extreme,
    Drought,
    Flooding,
    Pollution,
    Noise,
    Light,
    Habitat_Fragmentation,
    Overexploitation,
    Disease_Outbreak,
    Invasive_Species,
    Human_Development,
}

/// Disturbance events
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisturbanceEvent {
    pub event_id: EntityId,
    pub event_type: DisturbanceType,
    pub magnitude: f32,
    pub frequency: f32, // events per year
    pub predictability: f32,
    pub spatial_extent: GeographicExtent,
    pub ecological_effects: Vec<EcologicalEffect>,
    pub recovery_potential: f32,
    pub succession_triggers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisturbanceType {
    Fire,
    Storm,
    Volcanic_Activity,
    Earthquake,
    Landslide,
    Pest_Outbreak,
    Disease_Epidemic,
    Human_Intervention,
    Climate_Shift,
    Asteroid_Impact,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcologicalEffect {
    pub effect_type: String,
    pub magnitude: f32,
    pub duration: u64, // seconds
    pub affected_components: Vec<String>,
    pub cascade_effects: Vec<String>,
}

/// Seasonal patterns in the ecosystem
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeasonalPattern {
    pub pattern_id: EntityId,
    pub pattern_name: String,
    pub season: Season,
    pub affected_processes: Vec<String>,
    pub pattern_strength: f32,
    pub predictability: f32,
    pub climate_dependency: f32,
    pub species_responses: Vec<SpeciesResponse>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeciesResponse {
    pub species_id: EntityId,
    pub response_type: ResponseType,
    pub response_magnitude: f32,
    pub adaptation_level: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseType {
    Behavioral_Change,
    Physiological_Change,
    Migration,
    Dormancy,
    Reproduction_Timing,
    Diet_Shift,
    Territory_Change,
    Social_Structure_Change,
}

/// Microhabitats within the ecosystem
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Microhabitat {
    pub habitat_id: EntityId,
    pub habitat_type: String,
    pub location: WorldPosition,
    pub area: f32, // square meters
    pub unique_conditions: EnvironmentalConditions,
    pub resident_species: Vec<EntityId>,
    pub resource_availability: HashMap<EcosystemResourceType, f32>,
    pub connectivity: Vec<HabitatConnection>,
    pub conservation_value: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalConditions {
    pub temperature: f32,
    pub humidity: f32,
    pub light_levels: f32,
    pub soil_ph: Option<f32>,
    pub water_availability: f32,
    pub shelter_quality: f32,
    pub disturbance_level: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HabitatConnection {
    pub connected_habitat_id: EntityId,
    pub connection_type: ConnectionType,
    pub permeability: f32, // How easily species can move between habitats
    pub distance: f32, // meters
    pub barriers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectionType {
    Direct,
    Corridor,
    Stepping_Stone,
    Partial,
    Seasonal,
    None,
}

/// AI ecosystem interventions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcosystemIntervention {
    pub intervention_id: EntityId,
    pub intervention_type: InterventionType,
    pub target_component: String,
    pub intervention_goals: Vec<String>,
    pub implementation_method: String,
    pub resource_requirements: HashMap<String, f32>,
    pub expected_outcomes: Vec<ExpectedOutcome>,
    pub risk_assessment: RiskAssessment,
    pub monitoring_plan: MonitoringPlan,
    pub timeline: InterventionTimeline,
    pub success_criteria: Vec<SuccessCriterion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterventionType {
    Species_Reintroduction,
    Habitat_Restoration,
    Invasive_Species_Control,
    Resource_Supplementation,
    Population_Management,
    Pollution_Remediation,
    Climate_Adaptation,
    Connectivity_Enhancement,
    Monitoring_Enhancement,
    Research_Initiative,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpectedOutcome {
    pub outcome_description: String,
    pub probability: f32,
    pub timeframe: u64, // seconds
    pub measurable_indicators: Vec<String>,
    pub confidence_level: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterventionTimeline {
    pub start_date: Timestamp,
    pub phases: Vec<InterventionPhase>,
    pub total_duration: u64, // seconds
    pub milestones: Vec<Milestone>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterventionPhase {
    pub phase_name: String,
    pub duration: u64, // seconds
    pub activities: Vec<String>,
    pub resource_allocation: HashMap<String, f32>,
    pub success_indicators: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Milestone {
    pub milestone_name: String,
    pub target_date: Timestamp,
    pub success_criteria: Vec<String>,
    pub dependencies: Vec<String>,
}

/// Management goals for the ecosystem
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManagementGoal {
    pub goal_id: EntityId,
    pub goal_name: String,
    pub goal_category: GoalCategory,
    pub priority: f32,
    pub target_metrics: Vec<TargetMetric>,
    pub current_progress: f32,
    pub deadline: Option<Timestamp>,
    pub associated_interventions: Vec<EntityId>,
    pub stakeholders: Vec<String>,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GoalCategory {
    Conservation,
    Restoration,
    Sustainability,
    Biodiversity,
    Climate_Resilience,
    Human_Wellbeing,
    Economic_Value,
    Research,
    Education,
    Recreation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetMetric {
    pub metric_name: String,
    pub current_value: f32,
    pub target_value: f32,
    pub measurement_unit: String,
    pub importance_weight: f32,
}

/// Ecosystem monitoring metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcosystemMetric {
    pub metric_id: EntityId,
    pub metric_name: String,
    pub metric_category: MetricCategory,
    pub current_value: f32,
    pub historical_values: Vec<HistoricalValue>,
    pub measurement_frequency: MeasurementFrequency,
    pub data_quality: DataQuality,
    pub trend_analysis: TrendAnalysis,
    pub threshold_values: ThresholdValues,
    pub alert_conditions: Vec<AlertCondition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetricCategory {
    Biodiversity,
    Population,
    Habitat_Quality,
    Resource_Availability,
    Environmental_Conditions,
    Ecosystem_Services,
    Human_Impact,
    Climate_Indicators,
    Pollution_Levels,
    Connectivity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricalValue {
    pub timestamp: Timestamp,
    pub value: f32,
    pub data_source: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MeasurementFrequency {
    Continuous,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Seasonal,
    Annual,
    Event_Triggered,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataQuality {
    pub accuracy: f32,
    pub precision: f32,
    pub completeness: f32,
    pub timeliness: f32,
    pub reliability: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub trend_direction: TrendDirection,
    pub trend_strength: f32,
    pub trend_confidence: f32,
    pub seasonal_component: Option<f32>,
    pub anomalies_detected: Vec<Anomaly>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Cyclical,
    Irregular,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Anomaly {
    pub anomaly_type: String,
    pub timestamp: Timestamp,
    pub deviation_magnitude: f32,
    pub potential_causes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertCondition {
    pub condition_name: String,
    pub trigger_threshold: f32,
    pub severity_level: AlertSeverity,
    pub response_actions: Vec<String>,
    pub notification_targets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// Ecosystem predictions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcosystemPrediction {
    pub prediction_id: EntityId,
    pub prediction_type: PredictionType,
    pub target_component: String,
    pub prediction_horizon: u64, // seconds
    pub predicted_values: Vec<PredictedValue>,
    pub confidence_intervals: Vec<ConfidenceInterval>,
    pub model_used: String,
    pub input_parameters: HashMap<String, f32>,
    pub uncertainty_factors: Vec<UncertaintyFactor>,
    pub validation_data: Vec<ValidationPoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PredictionType {
    Population_Dynamics,
    Habitat_Change,
    Climate_Response,
    Species_Distribution,
    Resource_Availability,
    Ecosystem_Services,
    Disturbance_Impact,
    Recovery_Trajectory,
    Tipping_Points,
    Adaptation_Success,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictedValue {
    pub timestamp: Timestamp,
    pub value: f32,
    pub confidence: f32,
    pub scenario: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    pub timestamp: Timestamp,
    pub lower_bound: f32,
    pub upper_bound: f32,
    pub confidence_level: f32, // e.g., 0.95 for 95%
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UncertaintyFactor {
    pub factor_name: String,
    pub uncertainty_level: f32,
    pub impact_on_prediction: f32,
    pub mitigation_strategies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationPoint {
    pub timestamp: Timestamp,
    pub predicted_value: f32,
    pub actual_value: f32,
    pub error: f32,
    pub validation_method: String,
}

// Implement DatabaseModel and AIModel traits
impl_database_model!(EcosystemState, id, created_at, updated_at);

impl AIModel for EcosystemState {
    fn confidence_score(&self) -> f32 {
        // Calculate overall confidence based on health metrics and prediction accuracy
        let health_confidence = (self.overall_health + self.stability_score + self.resilience_score) / 3.0;
        let prediction_confidence = if self.predictive_models.is_empty() {
            0.5
        } else {
            self.predictive_models.iter()
                .flat_map(|p| &p.predicted_values)
                .map(|pv| pv.confidence)
                .sum::<f32>() / (self.predictive_models.len() as f32)
        };
        
        (health_confidence + prediction_confidence) / 2.0
    }
    
    fn ai_context(&self) -> &AIContext {
        static DEFAULT_CONTEXT: std::sync::OnceLock<AIContext> = std::sync::OnceLock::new();
        DEFAULT_CONTEXT.get_or_init(|| AIContext {
            model_version: "ecosystem-ai-v1.0".to_string(),
            decision_algorithm: "ecosystem-management-ai".to_string(),
            training_data_hash: None,
            confidence_threshold: 0.7,
            processing_time_ms: 200,
            resource_usage: ResourceUsage {
                cpu_usage_percent: 35.0,
                memory_usage_mb: 512.0,
                gpu_usage_percent: Some(20.0),
                network_io_bytes: 2048,
                disk_io_bytes: 4096,
            },
        })
    }
    
    fn has_learning_data(&self) -> bool {
        !self.ai_interventions.is_empty() || 
        !self.monitoring_metrics.is_empty() ||
        !self.predictive_models.is_empty()
    }
}

// Default implementations
impl Default for GeographicExtent {
    fn default() -> Self {
        Self {
            center_coordinates: WorldPosition::new(0.0, 0.0, 0.0),
            boundary_points: Vec::new(),
            total_area: 10000.0, // 1 hectare
            elevation_range: (0.0, 100.0),
            water_coverage_percentage: 0.1,
            terrain_complexity: 0.5,
        }
    }
}

impl Default for ClimateConditions {
    fn default() -> Self {
        Self {
            temperature_range: TemperatureRange {
                min_comfortable: 15.0,
                max_comfortable: 25.0,
                min_tolerable: 5.0,
                max_tolerable: 35.0,
            },
            precipitation_patterns: PrecipitationPattern::default(),
            humidity_levels: HumidityRange {
                min_comfortable: 0.4,
                max_comfortable: 0.7,
                min_tolerable: 0.2,
                max_tolerable: 0.9,
            },
            wind_patterns: Vec::new(),
            seasonal_variation: 0.3,
            climate_stability: 0.7,
            microclimates: Vec::new(),
        }
    }
}

impl Default for PrecipitationPattern {
    fn default() -> Self {
        Self {
            annual_rainfall: 1000.0, // mm
            seasonal_distribution: HashMap::new(),
            drought_frequency: 0.1,
            flood_risk: 0.1,
            precipitation_variability: 0.2,
        }
    }
}

impl Default for DataQuality {
    fn default() -> Self {
        Self {
            accuracy: 0.8,
            precision: 0.7,
            completeness: 0.9,
            timeliness: 0.8,
            reliability: 0.8,
        }
    }
}

// Utility implementations for EcosystemState
impl EcosystemState {
    /// Create a new ecosystem state
    pub fn new(
        ecosystem_name: String,
        biome_type: BiomeType,
        center_coordinates: WorldPosition,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            ecosystem_id: Uuid::new_v4(),
            state_timestamp: now,
            created_at: now,
            updated_at: now,
            ecosystem_name,
            biome_type,
            geographic_extent: GeographicExtent {
                center_coordinates,
                ..Default::default()
            },
            climate_conditions: ClimateConditions::default(),
            overall_health: 0.7,
            biodiversity_index: 0.6,
            stability_score: 0.8,
            resilience_score: 0.7,
            sustainability_index: 0.6,
            species_populations: Vec::new(),
            keystone_species: Vec::new(),
            endangered_species: Vec::new(),
            invasive_species: Vec::new(),
            species_interactions: Vec::new(),
            resource_pools: Vec::new(),
            resource_flows: Vec::new(),
            nutrient_cycles: Vec::new(),
            energy_flows: Vec::new(),
            environmental_stressors: Vec::new(),
            disturbance_events: Vec::new(),
            seasonal_patterns: Vec::new(),
            microhabitats: Vec::new(),
            ai_interventions: Vec::new(),
            management_goals: Vec::new(),
            monitoring_metrics: Vec::new(),
            predictive_models: Vec::new(),
            metadata: EntityMetadata::default(),
        }
    }
    
    /// Update the ecosystem timestamp
    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now();
        self.state_timestamp = self.updated_at;
    }
    
    /// Calculate ecosystem complexity
    pub fn calculate_complexity(&self) -> f32 {
        let species_complexity = self.species_populations.len() as f32 * 0.1;
        let interaction_complexity = self.species_interactions.len() as f32 * 0.05;
        let resource_complexity = self.resource_pools.len() as f32 * 0.02;
        let habitat_complexity = self.microhabitats.len() as f32 * 0.03;
        
        (species_complexity + interaction_complexity + resource_complexity + habitat_complexity).min(1.0)
    }
    
    /// Add a species population
    pub fn add_species_population(&mut self, population: SpeciesPopulation) {
        // Check if population is endangered
        if matches!(population.conservation_status, ConservationStatus::Endangered | ConservationStatus::Critically_Endangered) {
            if !self.endangered_species.contains(&population.species_id) {
                self.endangered_species.push(population.species_id);
            }
        }
        
        self.species_populations.push(population);
        self.update_timestamp();
    }
    
    /// Get species population by ID
    pub fn get_species_population(&self, species_id: EntityId) -> Option<&SpeciesPopulation> {
        self.species_populations.iter().find(|pop| pop.species_id == species_id)
    }
    
    /// Calculate total population across all species
    pub fn calculate_total_population(&self) -> u64 {
        self.species_populations.iter().map(|pop| pop.current_population).sum()
    }
    
    /// Calculate carrying capacity utilization
    pub fn calculate_carrying_capacity_utilization(&self) -> f32 {
        let total_population = self.calculate_total_population();
        let total_capacity: u64 = self.species_populations.iter().map(|pop| pop.carrying_capacity).sum();
        
        if total_capacity > 0 {
            total_population as f32 / total_capacity as f32
        } else {
            0.0
        }
    }
    
    /// Check if ecosystem is in critical state
    pub fn is_in_critical_state(&self) -> bool {
        self.overall_health < 0.3 ||
        self.stability_score < 0.3 ||
        self.endangered_species.len() > self.species_populations.len() / 3 ||
        self.environmental_stressors.iter().any(|s| s.intensity > 0.8)
    }
    
    /// Get active management goals
    pub fn get_active_management_goals(&self) -> Vec<&ManagementGoal> {
        self.management_goals.iter()
            .filter(|goal| goal.current_progress < 1.0)
            .collect()
    }
    
    /// Calculate biodiversity index
    pub fn calculate_biodiversity_index(&self) -> f32 {
        if self.species_populations.is_empty() {
            return 0.0;
        }
        
        // Shannon diversity index calculation
        let total_population = self.calculate_total_population() as f32;
        if total_population == 0.0 {
            return 0.0;
        }
        
        let shannon_index: f32 = self.species_populations.iter()
            .map(|pop| {
                let proportion = pop.current_population as f32 / total_population;
                if proportion > 0.0 {
                    proportion * proportion.ln()
                } else {
                    0.0
                }
            })
            .sum::<f32>() * -1.0;
        
        // Normalize to 0-1 scale (assuming max diversity around 3.0)
        (shannon_index / 3.0).min(1.0)
    }
    
    /// Generate ecosystem health report
    pub fn generate_health_report(&self) -> String {
        format!(
            "Ecosystem Health Report: {}\n\
            ================================\n\
            Overall Health: {:.1}%\n\
            Biodiversity Index: {:.1}%\n\
            Stability Score: {:.1}%\n\
            Resilience Score: {:.1}%\n\
            Sustainability Index: {:.1}%\n\
            \n\
            Species Information:\n\
            - Total Species: {}\n\
            - Total Population: {}\n\
            - Endangered Species: {}\n\
            - Invasive Species: {}\n\
            - Keystone Species: {}\n\
            \n\
            Environmental Status:\n\
            - Active Stressors: {}\n\
            - Recent Disturbances: {}\n\
            - Microhabitats: {}\n\
            \n\
            Management:\n\
            - Active Goals: {}\n\
            - AI Interventions: {}\n\
            - Monitoring Metrics: {}\n\
            \n\
            Critical Status: {}\n\
            Carrying Capacity Utilization: {:.1}%\n",
            self.ecosystem_name,
            self.overall_health * 100.0,
            self.biodiversity_index * 100.0,
            self.stability_score * 100.0,
            self.resilience_score * 100.0,
            self.sustainability_index * 100.0,
            self.species_populations.len(),
            self.calculate_total_population(),
            self.endangered_species.len(),
            self.invasive_species.len(),
            self.keystone_species.len(),
            self.environmental_stressors.len(),
            self.disturbance_events.len(),
            self.microhabitats.len(),
            self.get_active_management_goals().len(),
            self.ai_interventions.len(),
            self.monitoring_metrics.len(),
            if self.is_in_critical_state() { "CRITICAL" } else { "Stable" },
            self.calculate_carrying_capacity_utilization() * 100.0,
        )
    }
}