// mutsea-database/src/models/ecosystem_state.rs
//! Ecosystem state models for AI-driven environmental simulation

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export submodules
pub mod populations;
pub mod dynamics;
pub mod health;
pub mod management;
pub mod disturbances;
pub mod resources;
pub mod ai_predictions;

pub use populations::*;
pub use dynamics::*;
pub use health::*;
pub use management::*;
pub use disturbances::*;
pub use resources::*;
pub use ai_predictions::*;

/// Complete ecosystem state representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcosystemState {
    pub id: EntityId,
    pub ecosystem_id: EntityId,
    pub state_timestamp: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    
    // Basic ecosystem properties
    pub ecosystem_name: String,
    pub ecosystem_type: EcosystemType,
    pub biome: BiomeType,
    pub location: WorldPosition,
    pub area_coverage: f32, // square kilometers
    
    // Environmental conditions
    pub climate_conditions: ClimateConditions,
    pub seasonal_state: SeasonalState,
    pub weather_influence: WeatherInfluence,
    pub geological_features: GeologicalFeatures,
    
    // Biological components
    pub flora_populations: Vec<FloraPopulation>,
    pub fauna_populations: Vec<FaunaPopulation>,
    pub microorganism_populations: Vec<MicroorganismPopulation>,
    pub species_diversity: SpeciesDiversity,
    
    // Ecosystem dynamics
    pub food_webs: Vec<FoodWeb>,
    pub nutrient_cycles: Vec<NutrientCycle>,
    pub energy_flows: Vec<EnergyFlow>,
    pub population_dynamics: PopulationDynamics,
    
    // Health and stability
    pub ecosystem_health: EcosystemHealthMetrics,
    pub stability_indicators: StabilityIndicators,
    pub resilience_metrics: ResilienceMetrics,
    pub stress_factors: Vec<StressFactor>,
    
    // Human and AI impact
    pub human_impact: HumanImpact,
    pub ai_management: AIEcosystemManagement,
    pub conservation_status: ConservationStatus,
    
    // Resource availability
    pub natural_resources: Vec<NaturalResource>,
    pub renewable_resources: Vec<RenewableResource>,
    pub carrying_capacity: CarryingCapacity,
    
    // Disturbances and events
    pub natural_disturbances: Vec<NaturalDisturbance>,
    pub anthropogenic_disturbances: Vec<AnthropogenicDisturbance>,
    pub succession_stage: SuccessionStage,
    
    // AI predictions and modeling
    pub ecosystem_predictions: EcosystemPredictions,
    pub ai_interventions: Vec<AIIntervention>,
    
    // Metadata
    pub metadata: EntityMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EcosystemType {
    Terrestrial(TerrestrialEcosystem),
    Aquatic(AquaticEcosystem),
    Marine(MarineEcosystem),
    Freshwater(FreshwaterEcosystem),
    Wetland(WetlandEcosystem),
    Urban(UrbanEcosystem),
    Agricultural(AgriculturalEcosystem),
    Artificial(ArtificialEcosystem),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TerrestrialEcosystem {
    Forest,
    Grassland,
    Desert,
    Tundra,
    Savanna,
    Shrubland,
    Mountain,
    Cave,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AquaticEcosystem {
    River,
    Lake,
    Pond,
    Stream,
    Spring,
    Estuary,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarineEcosystem {
    Ocean,
    CoralReef,
    DeepSea,
    Continental_Shelf,
    Kelp_Forest,
    Mangrove,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FreshwaterEcosystem {
    Lake,
    River,
    Stream,
    Wetland,
    Bog,
    Marsh,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WetlandEcosystem {
    Marsh,
    Swamp,
    Bog,
    Fen,
    Estuary,
    Floodplain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UrbanEcosystem {
    City,
    Suburb,
    Park,
    Garden,
    Industrial,
    Residential,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgriculturalEcosystem {
    Cropland,
    Pasture,
    Orchard,
    Vineyard,
    Aquaculture,
    Agroforestry,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArtificialEcosystem {
    Greenhouse,
    Laboratory,
    Terrarium,
    Aquarium,
    Biodome,
    SpaceHabitat,
}

/// AI intervention in ecosystem management
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIIntervention {
    pub intervention_id: EntityId,
    pub intervention_name: String,
    pub intervention_type: AIInterventionType,
    pub target_components: Vec<String>,
    pub implementation_date: Timestamp,
    pub duration: Option<u64>, // days
    pub success_metrics: Vec<SuccessMetric>,
    pub resource_requirements: Vec<String>,
    pub risk_assessment: InterventionRisk,
    pub monitoring_plan: InterventionMonitoring,
    pub effectiveness: Option<f32>,
    pub side_effects: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AIInterventionType {
    SpeciesReintroduction,
    HabitatRestoration,
    InvasiveSpeciesControl,
    PollutionRemediation,
    WaterManagement,
    FireManagement,
    CorridorCreation,
    PopulationControl,
    DiseaseManagement,
    ClimateAdaptation,
    GeneticRescue,
    EcosystemEngineering,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuccessMetric {
    pub metric_name: String,
    pub target_value: f32,
    pub current_value: Option<f32>,
    pub measurement_method: String,
    pub evaluation_timeline: u64, // days
    pub weight: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterventionRisk {
    pub risk_level: RiskLevel,
    pub potential_risks: Vec<PotentialRisk>,
    pub mitigation_strategies: Vec<String>,
    pub contingency_plans: Vec<String>,
    pub monitoring_indicators: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PotentialRisk {
    pub risk_name: String,
    pub probability: f32,
    pub impact_severity: f32,
    pub affected_components: Vec<String>,
    pub risk_category: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterventionMonitoring {
    pub monitoring_frequency: f32, // measurements per month
    pub key_indicators: Vec<String>,
    pub data_collection_methods: Vec<String>,
    pub quality_assurance: Vec<String>,
    pub reporting_schedule: String,
    pub adaptive_triggers: Vec<String>,
}

// Implement DatabaseModel and AIModel traits
impl_database_model!(EcosystemState, id, created_at, updated_at);

impl AIModel for EcosystemState {
    fn confidence_score(&self) -> f32 {
        // Calculate confidence based on various factors
        let health_confidence = self.ecosystem_health.overall_health_score;
        let prediction_confidence = if !self.ecosystem_predictions.short_term_predictions.is_empty() {
            self.ecosystem_predictions.short_term_predictions.iter()
                .map(|p| p.confidence_level)
                .sum::<f32>() / self.ecosystem_predictions.short_term_predictions.len() as f32
        } else {
            0.5
        };
        let management_confidence = if !self.ai_management.predictive_models.is_empty() {
            self.ai_management.predictive_models.iter()
                .map(|m| m.accuracy_metrics.overall_accuracy)
                .sum::<f32>() / self.ai_management.predictive_models.len() as f32
        } else {
            0.5
        };
        
        (health_confidence + prediction_confidence + management_confidence) / 3.0
    }
    
    fn ai_context(&self) -> &AIContext {
        static DEFAULT_CONTEXT: std::sync::OnceLock<AIContext> = std::sync::OnceLock::new();
        DEFAULT_CONTEXT.get_or_init(|| AIContext {
            model_version: "ecosystem-ai-v1.0".to_string(),
            decision_algorithm: "ecosystem-management-ai".to_string(),
            training_data_hash: None,
            confidence_threshold: 0.7,
            processing_time_ms: 500,
            resource_usage: ResourceUsage {
                cpu_usage_percent: 40.0,
                memory_usage_mb: 1024.0,
                gpu_usage_percent: Some(30.0),
                network_io_bytes: 2048,
                disk_io_bytes: 4096,
            },
        })
    }
    
    fn has_learning_data(&self) -> bool {
        !self.ai_interventions.is_empty() ||
        !self.ai_management.adaptive_management.learning_mechanisms.is_empty()
    }
}

impl Default for EcosystemState {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            ecosystem_id: Uuid::new_v4(),
            state_timestamp: now,
            created_at: now,
            updated_at: now,
            ecosystem_name: "Default Ecosystem".to_string(),
            ecosystem_type: EcosystemType::Terrestrial(TerrestrialEcosystem::Forest),
            biome: BiomeType::Forest,
            location: WorldPosition::new(0.0, 0.0, 0.0),
            area_coverage: 100.0,
            climate_conditions: ClimateConditions::default(),
            seasonal_state: SeasonalState::default(),
            weather_influence: WeatherInfluence::default(),
            geological_features: GeologicalFeatures::default(),
            flora_populations: Vec::new(),
            fauna_populations: Vec::new(),
            microorganism_populations: Vec::new(),
            species_diversity: SpeciesDiversity::default(),
            food_webs: Vec::new(),
            nutrient_cycles: Vec::new(),
            energy_flows: Vec::new(),
            population_dynamics: PopulationDynamics::default(),
            ecosystem_health: EcosystemHealthMetrics::default(),
            stability_indicators: StabilityIndicators::default(),
            resilience_metrics: ResilienceMetrics::default(),
            stress_factors: Vec::new(),
            human_impact: HumanImpact::default(),
            ai_management: AIEcosystemManagement::default(),
            conservation_status: ConservationStatus::default(),
            natural_resources: Vec::new(),
            renewable_resources: Vec::new(),
            carrying_capacity: CarryingCapacity::default(),
            natural_disturbances: Vec::new(),
            anthropogenic_disturbances: Vec::new(),
            succession_stage: SuccessionStage::default(),
            ecosystem_predictions: EcosystemPredictions::default(),
            ai_interventions: Vec::new(),
            metadata: EntityMetadata::default(),
        }
    }
}

// Utility implementations
impl EcosystemState {
    /// Create a new ecosystem state
    pub fn new(name: String, ecosystem_type: EcosystemType, location: WorldPosition) -> Self {
        let mut ecosystem = Self::default();
        ecosystem.ecosystem_name = name;
        ecosystem.ecosystem_type = ecosystem_type;
        ecosystem.location = location;
        ecosystem
    }
    
    /// Calculate overall ecosystem vitality
    pub fn calculate_vitality(&self) -> f32 {
        let health_factor = self.ecosystem_health.overall_health_score;
        let diversity_factor = self.species_diversity.shannon_diversity_index / 5.0; // Normalize
        let stability_factor = self.stability_indicators.ecosystem_resilience;
        let management_factor = self.calculate_management_effectiveness();
        
        (health_factor + diversity_factor + stability_factor + management_factor) / 4.0
    }
    
    /// Calculate management effectiveness
    pub fn calculate_management_effectiveness(&self) -> f32 {
        let conservation_effectiveness = self.conservation_status.conservation_success_metrics.cost_effectiveness;
        let ai_effectiveness = if !self.ai_management.predictive_models.is_empty() {
            self.ai_management.predictive_models.iter()
                .map(|m| m.accuracy_metrics.overall_accuracy)
                .sum::<f32>() / self.ai_management.predictive_models.len() as f32
        } else {
            0.5
        };
        
        (conservation_effectiveness + ai_effectiveness) / 2.0
    }
    
    /// Check if ecosystem requires immediate attention
    pub fn requires_immediate_attention(&self) -> bool {
        self.ecosystem_health.overall_health_score < 0.3 ||
        self.stability_indicators.regime_shift_risk > 0.8 ||
        !self.stress_factors.iter().any(|s| s.intensity > 0.9) ||
        self.species_diversity.threatened_species_count > self.species_diversity.total_species_count / 4
    }
    
    /// Get critical issues
    pub fn get_critical_issues(&self) -> Vec<String> {
        let mut issues = Vec::new();
        
        if self.ecosystem_health.overall_health_score < 0.3 {
            issues.push("Ecosystem health critically low".to_string());
        }
        
        if self.stability_indicators.regime_shift_risk > 0.8 {
            issues.push("High risk of regime shift".to_string());
        }
        
        if self.species_diversity.invasive_species_count > 10 {
            issues.push("High invasive species pressure".to_string());
        }
        
        if self.carrying_capacity.capacity_utilization > 0.95 {
            issues.push("Carrying capacity exceeded".to_string());
        }
        
        for stress in &self.stress_factors {
            if stress.intensity > 0.9 {
                issues.push(format!("Severe stress from {}", stress.factor_name));
            }
        }
        
        issues
    }
    
    /// Update ecosystem timestamp
    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now();
        self.state_timestamp = self.updated_at;
    }
    
    /// Add AI intervention
    pub fn add_ai_intervention(&mut self, intervention: AIIntervention) {
        self.ai_interventions.push(intervention);
        self.update_timestamp();
    }
    
    /// Generate ecosystem report
    pub fn generate_ecosystem_report(&self) -> String {
        format!(
            "Ecosystem Report: {}\n\
            ==================\n\
            Type: {:?}\n\
            Location: ({:.2}, {:.2}, {:.2})\n\
            Area: {:.1} km²\n\
            Vitality Score: {:.1}%\n\
            Health Score: {:.1}%\n\
            Species Count: {}\n\
            Diversity Index: {:.2}\n\
            Stability: {:.1}%\n\
            Conservation Status: {:?}\n\
            Critical Issues: {}\n\
            AI Interventions: {}\n\
            \n\
            Key Metrics:\n\
            - Resilience: {:.1}%\n\
            - Stress Level: {:.1}%\n\
            - Management Effectiveness: {:.1}%\n\
            - Carrying Capacity Usage: {:.1}%\n",
            self.ecosystem_name,
            self.ecosystem_type,
            self.location.x, self.location.y, self.location.z,
            self.area_coverage,
            self.calculate_vitality() * 100.0,
            self.ecosystem_health.overall_health_score * 100.0,
            self.species_diversity.total_species_count,
            self.species_diversity.shannon_diversity_index,
            self.stability_indicators.ecosystem_resilience * 100.0,
            self.conservation_status.protection_level,
            self.get_critical_issues().len(),
            self.ai_interventions.len(),
            self.resilience_metrics.ecological_resilience * 100.0,
            self.stress_factors.iter().map(|s| s.intensity).sum::<f32>() / self.stress_factors.len().max(1) as f32 * 100.0,
            self.calculate_management_effectiveness() * 100.0,
            self.carrying_capacity.capacity_utilization * 100.0,
        )
    }
}

// Now let's create the module structure:

// mutsea-database/src/models/ecosystem_state/mod.rs
//! Ecosystem state models split into logical modules

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export all submodules
pub mod populations;
pub mod dynamics;
pub mod health;
pub mod management;
pub mod disturbances;
pub mod resources;
pub mod ai_predictions;
pub mod environmental;

pub use populations::*;
pub use dynamics::*;
pub use health::*;
pub use management::*;
pub use disturbances::*;
pub use resources::*;
pub use ai_predictions::*;
pub use environmental::*;

// Core ecosystem types remain in main module
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EcosystemState {
    pub id: EntityId,
    pub ecosystem_id: EntityId,
    pub state_timestamp: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    
    // Basic ecosystem properties
    pub ecosystem_name: String,
    pub ecosystem_type: EcosystemType,
    pub biome: BiomeType,
    pub location: WorldPosition,
    pub area_coverage: f32,
    
    // Environmental conditions (from environmental module)
    pub climate_conditions: ClimateConditions,
    pub seasonal_state: SeasonalState,
    pub weather_influence: WeatherInfluence,
    pub geological_features: GeologicalFeatures,
    
    // Biological components (from populations module)
    pub flora_populations: Vec<FloraPopulation>,
    pub fauna_populations: Vec<FaunaPopulation>,
    pub microorganism_populations: Vec<MicroorganismPopulation>,
    pub species_diversity: SpeciesDiversity,
    
    // Ecosystem dynamics (from dynamics module)
    pub food_webs: Vec<FoodWeb>,
    pub nutrient_cycles: Vec<NutrientCycle>,
    pub energy_flows: Vec<EnergyFlow>,
    pub population_dynamics: PopulationDynamics,
    
    // Health and stability (from health module)
    pub ecosystem_health: EcosystemHealthMetrics,
    pub stability_indicators: StabilityIndicators,
    pub resilience_metrics: ResilienceMetrics,
    pub stress_factors: Vec<StressFactor>,
    
    // Management (from management module)
    pub human_impact: HumanImpact,
    pub ai_management: AIEcosystemManagement,
    pub conservation_status: ConservationStatus,
    
    // Resources (from resources module)
    pub natural_resources: Vec<NaturalResource>,
    pub renewable_resources: Vec<RenewableResource>,
    pub carrying_capacity: CarryingCapacity,
    
    // Disturbances (from disturbances module)
    pub natural_disturbances: Vec<NaturalDisturbance>,
    pub anthropogenic_disturbances: Vec<AnthropogenicDisturbance>,
    pub succession_stage: SuccessionStage,
    
    // AI predictions (from ai_predictions module)
    pub ecosystem_predictions: EcosystemPredictions,
    pub ai_interventions: Vec<AIIntervention>,
    
    // Metadata
    pub metadata: EntityMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EcosystemType {
    Terrestrial(TerrestrialEcosystem),
    Aquatic(AquaticEcosystem),
    Marine(MarineEcosystem),
    Freshwater(FreshwaterEcosystem),
    Wetland(WetlandEcosystem),
    Urban(UrbanEcosystem),
    Agricultural(AgriculturalEcosystem),
    Artificial(ArtificialEcosystem),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TerrestrialEcosystem {
    Forest,
    Grassland,
    Desert,
    Tundra,
    Savanna,
    Shrubland,
    Mountain,
    Cave,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AquaticEcosystem {
    River,
    Lake,
    Pond,
    Stream,
    Spring,
    Estuary,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarineEcosystem {
    Ocean,
    CoralReef,
    DeepSea,
    Continental_Shelf,
    Kelp_Forest,
    Mangrove,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FreshwaterEcosystem {
    Lake,
    River,
    Stream,
    Wetland,
    Bog,
    Marsh,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WetlandEcosystem {
    Marsh,
    Swamp,
    Bog,
    Fen,
    Estuary,
    Floodplain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UrbanEcosystem {
    City,
    Suburb,
    Park,
    Garden,
    Industrial,
    Residential,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgriculturalEcosystem {
    Cropland,
    Pasture,
    Orchard,
    Vineyard,
    Aquaculture,
    Agroforestry,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArtificialEcosystem {
    Greenhouse,
    Laboratory,
    Terrarium,
    Aquarium,
    Biodome,
    SpaceHabitat,
}

// Implement traits
impl_database_model!(EcosystemState, id, created_at, updated_at);

impl AIModel for EcosystemState {
    fn confidence_score(&self) -> f32 {
        let health_confidence = self.ecosystem_health.overall_health_score;
        let prediction_confidence = if !self.ecosystem_predictions.short_term_predictions.is_empty() {
            self.ecosystem_predictions.short_term_predictions.iter()
                .map(|p| p.confidence_level)
                .sum::<f32>() / self.ecosystem_predictions.short_term_predictions.len() as f32
        } else {
            0.5
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
            processing_time_ms: 500,
            resource_usage: ResourceUsage {
                cpu_usage_percent: 40.0,
                memory_usage_mb: 1024.0,
                gpu_usage_percent: Some(30.0),
                network_io_bytes: 2048,
                disk_io_bytes: 4096,
            },
        })
    }
    
    fn has_learning_data(&self) -> bool {
        !self.ai_interventions.is_empty() ||
        !self.ai_management.adaptive_management.learning_mechanisms.is_empty()
    }
}

impl Default for EcosystemState {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            ecosystem_id: Uuid::new_v4(),
            state_timestamp: now,
            created_at: now,
            updated_at: now,
            ecosystem_name: "Default Ecosystem".to_string(),
            ecosystem_type: EcosystemType::Terrestrial(TerrestrialEcosystem::Forest),
            biome: BiomeType::Forest,
            location: WorldPosition::new(0.0, 0.0, 0.0),
            area_coverage: 100.0,
            climate_conditions: ClimateConditions::default(),
            seasonal_state: SeasonalState::default(),
            weather_influence: WeatherInfluence::default(),
            geological_features: GeologicalFeatures::default(),
            flora_populations: Vec::new(),
            fauna_populations: Vec::new(),
            microorganism_populations: Vec::new(),
            species_diversity: SpeciesDiversity::default(),
            food_webs: Vec::new(),
            nutrient_cycles: Vec::new(),
            energy_flows: Vec::new(),
            population_dynamics: PopulationDynamics::default(),
            ecosystem_health: EcosystemHealthMetrics::default(),
            stability_indicators: StabilityIndicators::default(),
            resilience_metrics: ResilienceMetrics::default(),
            stress_factors: Vec::new(),
            human_impact: HumanImpact::default(),
            ai_management: AIEcosystemManagement::default(),
            conservation_status: ConservationStatus::default(),
            natural_resources: Vec::new(),
            renewable_resources: Vec::new(),
            carrying_capacity: CarryingCapacity::default(),
            natural_disturbances: Vec::new(),
            anthropogenic_disturbances: Vec::new(),
            succession_stage: SuccessionStage::default(),
            ecosystem_predictions: EcosystemPredictions::default(),
            ai_interventions: Vec::new(),
            metadata: EntityMetadata::default(),
        }
    }
}

// Utility implementations (same as before but moved to impl block)
impl EcosystemState {
    /// Create a new ecosystem state
    pub fn new(name: String, ecosystem_type: EcosystemType, location: WorldPosition) -> Self {
        let mut ecosystem = Self::default();
        ecosystem.ecosystem_name = name;
        ecosystem.ecosystem_type = ecosystem_type;
        ecosystem.location = location;
        ecosystem
    }
    
    /// Calculate overall ecosystem vitality
    pub fn calculate_vitality(&self) -> f32 {
        let health_factor = self.ecosystem_health.overall_health_score;
        let diversity_factor = self.species_diversity.shannon_diversity_index / 5.0;
        let stability_factor = self.stability_indicators.ecosystem_resilience;
        let management_factor = self.calculate_management_effectiveness();
        
        (health_factor + diversity_factor + stability_factor + management_factor) / 4.0
    }
    
    /// Calculate management effectiveness
    pub fn calculate_management_effectiveness(&self) -> f32 {
        let conservation_effectiveness = self.conservation_status.conservation_success_metrics.cost_effectiveness;
        let ai_effectiveness = if !self.ai_management.predictive_models.is_empty() {
            self.ai_management.predictive_models.iter()
                .map(|m| m.accuracy_metrics.overall_accuracy)
                .sum::<f32>() / self.ai_management.predictive_models.len() as f32
        } else {
            0.5
        };
        
        (conservation_effectiveness + ai_effectiveness) / 2.0
    }
    
    /// Check if ecosystem requires immediate attention
    pub fn requires_immediate_attention(&self) -> bool {
        self.ecosystem_health.overall_health_score < 0.3 ||
        self.stability_indicators.regime_shift_risk > 0.8 ||
        self.species_diversity.threatened_species_count > self.species_diversity.total_species_count / 4
    }
    
    /// Update ecosystem timestamp
    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now();
        self.state_timestamp = self.updated_at;
    }
}