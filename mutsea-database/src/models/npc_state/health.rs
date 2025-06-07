
// mutsea-database/src/models/npc_state/health.rs
//! Health and survival state models for NPCs

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Comprehensive health state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthState {
    pub overall_health: f32, // 0.0 to 1.0
    pub physical_health: PhysicalHealth,
    pub mental_health: MentalHealth,
    pub vital_signs: VitalSigns,
    pub injuries: Vec<Injury>,
    pub diseases: Vec<Disease>,
    pub medications: Vec<Medication>,
    pub health_trends: HealthTrends,
    pub regeneration_abilities: RegenerationAbilities,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalHealth {
    pub constitution: f32,
    pub immune_system_strength: f32,
    pub fitness_level: f32,
    pub muscle_mass: f32,
    pub bone_density: f32,
    pub cardiovascular_health: f32,
    pub respiratory_health: f32,
    pub neurological_health: f32,
    pub sensory_acuity: SensoryAcuity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensoryAcuity {
    pub vision: f32,
    pub hearing: f32,
    pub smell: f32,
    pub taste: f32,
    pub touch: f32,
    pub proprioception: f32,
    pub balance: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MentalHealth {
    pub psychological_stability: f32,
    pub cognitive_function: f32,
    pub emotional_regulation: f32,
    pub stress_resistance: f32,
    pub trauma_recovery: f32,
    pub social_well_being: f32,
    pub purpose_fulfillment: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VitalSigns {
    pub heart_rate: f32,
    pub blood_pressure: BloodPressure,
    pub body_temperature: f32,
    pub respiratory_rate: f32,
    pub oxygen_saturation: f32,
    pub blood_sugar: f32,
    pub hydration_level: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BloodPressure {
    pub systolic: f32,
    pub diastolic: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Injury {
    pub injury_id: EntityId,
    pub injury_type: InjuryType,
    pub severity: f32,
    pub location: BodyLocation,
    pub healing_progress: f32,
    pub complications: Vec<String>,
    pub treatment_applied: Vec<Treatment>,
    pub estimated_recovery_time: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InjuryType {
    Cut,
    Bruise,
    Burn,
    Fracture,
    Sprain,
    Concussion,
    Infection,
    Poisoning,
    Magical,
    Technological,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BodyLocation {
    Head,
    Torso,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
    Internal,
    Systemic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Treatment {
    pub treatment_type: String,
    pub effectiveness: f32,
    pub side_effects: Vec<String>,
    pub cost: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disease {
    pub disease_id: EntityId,
    pub disease_name: String,
    pub disease_type: DiseaseType,
    pub progression_stage: f32,
    pub symptoms: Vec<Symptom>,
    pub treatments: Vec<Treatment>,
    pub prognosis: Prognosis,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiseaseType {
    Infectious,
    Genetic,
    Autoimmune,
    Degenerative,
    Metabolic,
    Mental,
    Magical,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Symptom {
    pub symptom_name: String,
    pub severity: f32,
    pub frequency: f32,
    pub impact_on_function: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prognosis {
    pub expected_outcome: String,
    pub recovery_probability: f32,
    pub time_to_recovery: Option<u64>,
    pub complications_risk: f32,
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
pub struct HealthTrends {
    pub improving: bool,
    pub stable: bool,
    pub declining: bool,
    pub rate_of_change: f32,
    pub trend_confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegenerationAbilities {
    pub natural_healing_rate: f32,
    pub magical_healing_affinity: f32,
    pub technological_enhancement: f32,
    pub regeneration_limits: Vec<String>,
}

/// Survival needs tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurvivalNeeds {
    pub hunger_level: f32,      // 0.0 satisfied to 1.0 starving
    pub thirst_level: f32,      // 0.0 satisfied to 1.0 dehydrated
    pub fatigue_level: f32,     // 0.0 rested to 1.0 exhausted
    pub warmth_need: f32,       // 0.0 comfortable to 1.0 freezing/overheating
    pub shelter_need: f32,      // 0.0 secure to 1.0 exposed
    pub social_need: f32,       // 0.0 fulfilled to 1.0 isolated
    pub safety_need: f32,       // 0.0 safe to 1.0 in danger
    pub comfort_need: f32,      // 0.0 comfortable to 1.0 uncomfortable
    pub purpose_need: f32,      // 0.0 fulfilled to 1.0 meaningless
    pub autonomy_need: f32,     // 0.0 free to 1.0 controlled
    
    // Advanced survival factors
    pub oxygen_level: f32,
    pub toxin_exposure: f32,
    pub radiation_exposure: f32,
    pub magical_energy_balance: f32,
    
    // Survival skills and knowledge
    pub survival_knowledge: f32,
    pub resource_gathering_skill: f32,
    pub threat_assessment_ability: f32,
    pub adaptation_capability: f32,
}

impl Default for HealthState {
    fn default() -> Self {
        Self {
            overall_health: 0.85,
            physical_health: PhysicalHealth::default(),
            mental_health: MentalHealth::default(),
            vital_signs: VitalSigns::default(),
            injuries: Vec::new(),
            diseases: Vec::new(),
            medications: Vec::new(),
            health_trends: HealthTrends {
                improving: false,
                stable: true,
                declining: false,
                rate_of_change: 0.0,
                trend_confidence: 0.8,
            },
            regeneration_abilities: RegenerationAbilities::default(),
        }
    }
}

impl Default for PhysicalHealth {
    fn default() -> Self {
        Self {
            constitution: 0.8,
            immune_system_strength: 0.7,
            fitness_level: 0.6,
            muscle_mass: 0.7,
            bone_density: 0.8,
            cardiovascular_health: 0.8,
            respiratory_health: 0.8,
            neurological_health: 0.9,
            sensory_acuity: SensoryAcuity::default(),
        }
    }
}

impl Default for SensoryAcuity {
    fn default() -> Self {
        Self {
            vision: 0.9,
            hearing: 0.9,
            smell: 0.8,
            taste: 0.8,
            touch: 0.9,
            proprioception: 0.8,
            balance: 0.8,
        }
    }
}

impl Default for MentalHealth {
    fn default() -> Self {
        Self {
            psychological_stability: 0.8,
            cognitive_function: 0.9,
            emotional_regulation: 0.7,
            stress_resistance: 0.6,
            trauma_recovery: 0.8,
            social_well_being: 0.7,
            purpose_fulfillment: 0.6,
        }
    }
}

impl Default for VitalSigns {
    fn default() -> Self {
        Self {
            heart_rate: 72.0,
            blood_pressure: BloodPressure { systolic: 120.0, diastolic: 80.0 },
            body_temperature: 37.0,
            respiratory_rate: 16.0,
            oxygen_saturation: 98.0,
            blood_sugar: 90.0,
            hydration_level: 0.8,
        }
    }
}

impl Default for RegenerationAbilities {
    fn default() -> Self {
        Self {
            natural_healing_rate: 0.7,
            magical_healing_affinity: 0.3,
            technological_enhancement: 0.1,
            regeneration_limits: vec!["Cannot regrow limbs".to_string()],
        }
    }
}

impl Default for SurvivalNeeds {
    fn default() -> Self {
        Self {
            hunger_level: 0.2,
            thirst_level: 0.1,
            fatigue_level: 0.3,
            warmth_need: 0.1,
            shelter_need: 0.0,
            social_need: 0.4,
            safety_need: 0.2,
            comfort_need: 0.3,
            purpose_need: 0.5,
            autonomy_need: 0.2,
            oxygen_level: 0.0,
            toxin_exposure: 0.0,
            radiation_exposure: 0.0,
            magical_energy_balance: 0.0,
            survival_knowledge: 0.6,
            resource_gathering_skill: 0.5,
            threat_assessment_ability: 0.7,
            adaptation_capability: 0.6,
        }
    }
}