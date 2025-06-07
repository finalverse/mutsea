//! Environmental conditions and climate models
//!
//! This module provides comprehensive environmental modeling capabilities including:
//! - Temperature, precipitation, and atmospheric conditions
//! - Seasonal cycles and climate patterns
//! - Environmental stress factors and disturbances
//! - Habitat quality assessments
//! - Climate change projections and scenarios

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

/// Core environmental state containing all physical conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalState {
    /// Current timestamp
    pub timestamp: DateTime<Utc>,
    /// Temperature conditions
    pub temperature: TemperatureState,
    /// Precipitation and humidity conditions
    pub precipitation: PrecipitationState,
    /// Atmospheric conditions
    pub atmosphere: AtmosphericState,
    /// Soil and substrate conditions
    pub soil: SoilState,
    /// Water body conditions
    pub water: WaterState,
    /// Light and radiation conditions
    pub light: LightState,
    /// Environmental disturbances
    pub disturbances: Vec<EnvironmentalDisturbance>,
    /// Habitat quality metrics
    pub habitat_quality: HabitatQuality,
}

/// Temperature-related environmental conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureState {
    /// Current air temperature in Celsius
    pub air_temperature: f64,
    /// Current water temperature in Celsius
    pub water_temperature: f64,
    /// Current soil temperature in Celsius
    pub soil_temperature: f64,
    /// Daily temperature range
    pub daily_range: f64,
    /// Seasonal temperature trend
    pub seasonal_trend: f64,
    /// Temperature gradient by depth/altitude
    pub gradient: TemperatureGradient,
    /// Heat index considering humidity
    pub heat_index: f64,
    /// Wind chill factor
    pub wind_chill: f64,
}

/// Temperature gradients across different zones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureGradient {
    /// Surface temperature
    pub surface: f64,
    /// Temperature at 1m depth/height
    pub depth_1m: f64,
    /// Temperature at 5m depth/height
    pub depth_5m: f64,
    /// Temperature at 10m depth/height
    pub depth_10m: f64,
}

/// Precipitation and moisture conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecipitationState {
    /// Current precipitation rate (mm/hour)
    pub current_rate: f64,
    /// Daily precipitation total (mm)
    pub daily_total: f64,
    /// Monthly precipitation total (mm)
    pub monthly_total: f64,
    /// Relative humidity (0-100%)
    pub humidity: f64,
    /// Dew point temperature
    pub dew_point: f64,
    /// Precipitation type
    pub precipitation_type: PrecipitationType,
    /// Drought index
    pub drought_index: f64,
    /// Soil moisture content
    pub soil_moisture: f64,
}

/// Types of precipitation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrecipitationType {
    None,
    Rain,
    Snow,
    Sleet,
    Hail,
    Fog,
    Mist,
}

/// Atmospheric conditions and air quality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphericState {
    /// Atmospheric pressure (hPa)
    pub pressure: f64,
    /// Wind speed (m/s)
    pub wind_speed: f64,
    /// Wind direction (degrees from north)
    pub wind_direction: f64,
    /// Air quality index
    pub air_quality_index: f64,
    /// Atmospheric pollutants
    pub pollutants: HashMap<String, f64>,
    /// UV index
    pub uv_index: f64,
    /// Cloud cover percentage
    pub cloud_cover: f64,
    /// Visibility (km)
    pub visibility: f64,
}

/// Soil and substrate physical/chemical properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilState {
    /// Soil pH level
    pub ph: f64,
    /// Organic matter content (%)
    pub organic_matter: f64,
    /// Nutrient concentrations
    pub nutrients: NutrientLevels,
    /// Soil texture classification
    pub texture: SoilTexture,
    /// Soil compaction level
    pub compaction: f64,
    /// Erosion rate
    pub erosion_rate: f64,
    /// Contamination levels
    pub contamination: HashMap<String, f64>,
}

/// Soil texture classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SoilTexture {
    Sand,
    LoamySand,
    SandyLoam,
    Loam,
    SiltLoam,
    Silt,
    SandyClayLoam,
    ClayLoam,
    SiltyClayLoam,
    SandyClay,
    SiltyClay,
    Clay,
}

/// Water body conditions for aquatic environments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterState {
    /// Water pH level
    pub ph: f64,
    /// Dissolved oxygen (mg/L)
    pub dissolved_oxygen: f64,
    /// Salinity (ppt)
    pub salinity: f64,
    /// Turbidity (NTU)
    pub turbidity: f64,
    /// Water flow rate (m/s)
    pub flow_rate: f64,
    /// Water depth (m)
    pub depth: f64,
    /// Chemical pollutants
    pub pollutants: HashMap<String, f64>,
    /// Algae bloom intensity
    pub algae_bloom: f64,
}

/// Light and radiation conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightState {
    /// Photosynthetically active radiation (μmol/m²/s)
    pub par: f64,
    /// Total solar radiation (W/m²)
    pub solar_radiation: f64,
    /// Photoperiod (hours of daylight)
    pub photoperiod: f64,
    /// Light penetration depth in water (m)
    pub penetration_depth: f64,
    /// Spectral composition
    pub spectral_bands: HashMap<String, f64>,
    /// Shade percentage
    pub shade_cover: f64,
}

/// Environmental disturbance events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalDisturbance {
    /// Type of disturbance
    pub disturbance_type: DisturbanceType,
    /// Intensity/magnitude (0-1 scale)
    pub intensity: f64,
    /// Duration of the disturbance
    pub duration: Duration,
    /// Start time of disturbance
    pub start_time: DateTime<Utc>,
    /// Spatial extent affected
    pub spatial_extent: f64,
    /// Recovery time estimate
    pub recovery_time: Duration,
}

/// Types of environmental disturbances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisturbanceType {
    Storm,
    Drought,
    Flood,
    Fire,
    Earthquake,
    VolcanicActivity,
    HumanDisturbance,
    PollutionEvent,
    TemperatureExtreme,
    DiseaseOutbreak,
}

/// Overall habitat quality assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitatQuality {
    /// Overall quality score (0-1)
    pub overall_score: f64,
    /// Temperature suitability
    pub temperature_suitability: f64,
    /// Water availability
    pub water_availability: f64,
    /// Resource abundance
    pub resource_abundance: f64,
    /// Shelter availability
    pub shelter_availability: f64,
    /// Disturbance level
    pub disturbance_level: f64,
    /// Connectivity to other habitats
    pub connectivity: f64,
}

/// Nutrient levels in soil/water
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientLevels {
    /// Nitrogen (mg/kg or mg/L)
    pub nitrogen: f64,
    /// Phosphorus (mg/kg or mg/L)
    pub phosphorus: f64,
    /// Potassium (mg/kg or mg/L)
    pub potassium: f64,
    /// Carbon (mg/kg or mg/L)
    pub carbon: f64,
    /// Trace elements
    pub trace_elements: HashMap<String, f64>,
}

/// Climate scenario for long-term projections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateScenario {
    /// Scenario name/identifier
    pub name: String,
    /// Temperature change projection (°C)
    pub temperature_change: f64,
    /// Precipitation change projection (%)
    pub precipitation_change: f64,
    /// CO2 concentration (ppm)
    pub co2_concentration: f64,
    /// Sea level change (m)
    pub sea_level_change: f64,
    /// Extreme event frequency multiplier
    pub extreme_event_multiplier: f64,
}

/// Seasonal cycle parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalCycle {
    /// Base temperature amplitude
    pub temperature_amplitude: f64,
    /// Base precipitation amplitude
    pub precipitation_amplitude: f64,
    /// Phase offset for temperature cycle
    pub temperature_phase: f64,
    /// Phase offset for precipitation cycle
    pub precipitation_phase: f64,
    /// Daylight variation amplitude
    pub daylight_amplitude: f64,
}

impl EnvironmentalState {
    /// Create a new environmental state with default values
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            temperature: TemperatureState::default(),
            precipitation: PrecipitationState::default(),
            atmosphere: AtmosphericState::default(),
            soil: SoilState::default(),
            water: WaterState::default(),
            light: LightState::default(),
            disturbances: Vec::new(),
            habitat_quality: HabitatQuality::default(),
        }
    }

    /// Update environmental state based on time progression
    pub fn update(&mut self, time_step: Duration, seasonal_cycle: &SeasonalCycle) {
        self.timestamp += time_step;
        self.update_temperature_cycle(seasonal_cycle);
        self.update_precipitation_cycle(seasonal_cycle);
        self.update_light_cycle(seasonal_cycle);
        self.process_disturbances();
        self.calculate_habitat_quality();
    }

    /// Apply seasonal temperature variations
    fn update_temperature_cycle(&mut self, cycle: &SeasonalCycle) {
        let day_of_year = self.timestamp.ordinal() as f64;
        let seasonal_factor = (2.0 * std::f64::consts::PI * 
            (day_of_year + cycle.temperature_phase) / 365.25).sin();
        
        self.temperature.seasonal_trend = cycle.temperature_amplitude * seasonal_factor;
        self.temperature.air_temperature += self.temperature.seasonal_trend * 0.1;
    }

    /// Apply seasonal precipitation variations
    fn update_precipitation_cycle(&mut self, cycle: &SeasonalCycle) {
        let day_of_year = self.timestamp.ordinal() as f64;
        let seasonal_factor = (2.0 * std::f64::consts::PI * 
            (day_of_year + cycle.precipitation_phase) / 365.25).sin();
        
        let seasonal_modifier = 1.0 + (cycle.precipitation_amplitude * seasonal_factor);
        self.precipitation.current_rate *= seasonal_modifier.max(0.1);
    }

    /// Update daylight and solar radiation cycles
    fn update_light_cycle(&mut self, cycle: &SeasonalCycle) {
        let day_of_year = self.timestamp.ordinal() as f64;
        let seasonal_factor = (2.0 * std::f64::consts::PI * day_of_year / 365.25).sin();
        
        self.light.photoperiod = 12.0 + cycle.daylight_amplitude * seasonal_factor;
        self.light.solar_radiation = 800.0 + 400.0 * seasonal_factor.max(0.0);
    }

    /// Process active environmental disturbances
    fn process_disturbances(&mut self) {
        self.disturbances.retain(|disturbance| {
            let elapsed = self.timestamp - disturbance.start_time;
            elapsed < disturbance.duration
        });

        // Apply disturbance effects
        for disturbance in &self.disturbances {
            self.apply_disturbance_effects(disturbance);
        }
    }

    /// Apply effects of a specific disturbance
    fn apply_disturbance_effects(&mut self, disturbance: &EnvironmentalDisturbance) {
        match disturbance.disturbance_type {
            DisturbanceType::Storm => {
                self.atmosphere.wind_speed += disturbance.intensity * 20.0;
                self.precipitation.current_rate += disturbance.intensity * 10.0;
            },
            DisturbanceType::Drought => {
                self.precipitation.current_rate *= 1.0 - disturbance.intensity;
                self.soil.erosion_rate += disturbance.intensity * 0.5;
            },
            DisturbanceType::Fire => {
                self.temperature.air_temperature += disturbance.intensity * 10.0;
                self.atmosphere.air_quality_index -= disturbance.intensity * 50.0;
            },
            DisturbanceType::PollutionEvent => {
                self.atmosphere.air_quality_index -= disturbance.intensity * 30.0;
                self.water.pollutants.entry("general".to_string())
                    .and_modify(|e| *e += disturbance.intensity * 10.0)
                    .or_insert(disturbance.intensity * 10.0);
            },
            _ => {}, // Handle other disturbance types as needed
        }
    }

    /// Calculate overall habitat quality based on current conditions
    fn calculate_habitat_quality(&mut self) {
        // Temperature suitability (assuming optimal range 15-25°C)
        let temp_diff = (self.temperature.air_temperature - 20.0).abs();
        self.habitat_quality.temperature_suitability = (1.0 - temp_diff / 20.0).max(0.0);

        // Water availability based on precipitation and soil moisture
        self.habitat_quality.water_availability = 
            (self.precipitation.soil_moisture / 100.0).min(1.0);

        // Resource abundance based on soil nutrients and water quality
        let nutrient_score = (self.soil.nutrients.nitrogen + 
                            self.soil.nutrients.phosphorus + 
                            self.soil.nutrients.potassium) / 300.0;
        self.habitat_quality.resource_abundance = nutrient_score.min(1.0);

        // Disturbance level (inverse of quality)
        let active_disturbance_intensity: f64 = self.disturbances
            .iter()
            .map(|d| d.intensity)
            .sum();
        self.habitat_quality.disturbance_level = 1.0 - active_disturbance_intensity.min(1.0);

        // Overall quality score
        self.habitat_quality.overall_score = (
            self.habitat_quality.temperature_suitability +
            self.habitat_quality.water_availability +
            self.habitat_quality.resource_abundance +
            self.habitat_quality.disturbance_level
        ) / 4.0;
    }

    /// Add a new environmental disturbance
    pub fn add_disturbance(&mut self, disturbance: EnvironmentalDisturbance) {
        self.disturbances.push(disturbance);
    }

    /// Get current stress factors for organisms
    pub fn get_stress_factors(&self) -> HashMap<String, f64> {
        let mut stress_factors = HashMap::new();
        
        // Temperature stress
        let temp_stress = ((self.temperature.air_temperature - 20.0).abs() / 30.0).min(1.0);
        stress_factors.insert("temperature".to_string(), temp_stress);
        
        // Drought stress
        stress_factors.insert("drought".to_string(), self.precipitation.drought_index);
        
        // Pollution stress
        let pollution_stress = (100.0 - self.atmosphere.air_quality_index) / 100.0;
        stress_factors.insert("pollution".to_string(), pollution_stress.max(0.0));
        
        // Disturbance stress
        let disturbance_stress: f64 = self.disturbances
            .iter()
            .map(|d| d.intensity)
            .sum::<f64>()
            .min(1.0);
        stress_factors.insert("disturbance".to_string(), disturbance_stress);
        
        stress_factors
    }
}

// Default implementations
impl Default for TemperatureState {
    fn default() -> Self {
        Self {
            air_temperature: 20.0,
            water_temperature: 18.0,
            soil_temperature: 19.0,
            daily_range: 8.0,
            seasonal_trend: 0.0,
            gradient: TemperatureGradient {
                surface: 20.0,
                depth_1m: 19.5,
                depth_5m: 18.0,
                depth_10m: 16.0,
            },
            heat_index: 20.0,
            wind_chill: 20.0,
        }
    }
}

impl Default for PrecipitationState {
    fn default() -> Self {
        Self {
            current_rate: 0.0,
            daily_total: 2.0,
            monthly_total: 60.0,
            humidity: 65.0,
            dew_point: 15.0,
            precipitation_type: PrecipitationType::None,
            drought_index: 0.0,
            soil_moisture: 50.0,
        }
    }
}

impl Default for AtmosphericState {
    fn default() -> Self {
        Self {
            pressure: 1013.25,
            wind_speed: 3.0,
            wind_direction: 180.0,
            air_quality_index: 80.0,
            pollutants: HashMap::new(),
            uv_index: 5.0,
            cloud_cover: 30.0,
            visibility: 10.0,
        }
    }
}

impl Default for SoilState {
    fn default() -> Self {
        Self {
            ph: 6.8,
            organic_matter: 3.5,
            nutrients: NutrientLevels::default(),
            texture: SoilTexture::Loam,
            compaction: 0.3,
            erosion_rate: 0.1,
            contamination: HashMap::new(),
        }
    }
}

impl Default for WaterState {
    fn default() -> Self {
        Self {
            ph: 7.2,
            dissolved_oxygen: 8.5,
            salinity: 0.0,
            turbidity: 2.0,
            flow_rate: 0.5,
            depth: 1.0,
            pollutants: HashMap::new(),
            algae_bloom: 0.0,
        }
    }
}

impl Default for LightState {
    fn default() -> Self {
        Self {
            par: 800.0,
            solar_radiation: 600.0,
            photoperiod: 12.0,
            penetration_depth: 2.0,
            spectral_bands: HashMap::new(),
            shade_cover: 20.0,
        }
    }
}

impl Default for HabitatQuality {
    fn default() -> Self {
        Self {
            overall_score: 0.8,
            temperature_suitability: 0.8,
            water_availability: 0.8,
            resource_abundance: 0.7,
            shelter_availability: 0.8,
            disturbance_level: 0.9,
            connectivity: 0.7,
        }
    }
}

impl Default for NutrientLevels {
    fn default() -> Self {
        Self {
            nitrogen: 50.0,
            phosphorus: 25.0,
            potassium: 80.0,
            carbon: 150.0,
            trace_elements: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environmental_state_creation() {
        let env_state = EnvironmentalState::new();
        assert!(env_state.habitat_quality.overall_score > 0.0);
        assert_eq!(env_state.disturbances.len(), 0);
    }

    #[test]
    fn test_disturbance_effects() {
        let mut env_state = EnvironmentalState::new();
        let initial_wind = env_state.atmosphere.wind_speed;
        
        let storm = EnvironmentalDisturbance {
            disturbance_type: DisturbanceType::Storm,
            intensity: 0.5,
            duration: Duration::hours(6),
            start_time: Utc::now(),
            spatial_extent: 100.0,
            recovery_time: Duration::days(1),
        };
        
        env_state.add_disturbance(storm);
        env_state.process_disturbances();
        
        assert!(env_state.atmosphere.wind_speed > initial_wind);
    }

    #[test]
    fn test_habitat_quality_calculation() {
        let mut env_state = EnvironmentalState::new();
        env_state.calculate_habitat_quality();
        
        assert!(env_state.habitat_quality.overall_score >= 0.0);
        assert!(env_state.habitat_quality.overall_score <= 1.0);
    }

    #[test]
    fn test_seasonal_cycle_update() {
        let mut env_state = EnvironmentalState::new();
        let cycle = SeasonalCycle {
            temperature_amplitude: 10.0,
            precipitation_amplitude: 0.5,
            temperature_phase: 90.0,
            precipitation_phase: 180.0,
            daylight_amplitude: 6.0,
        };
        
        let initial_temp = env_state.temperature.air_temperature;
        env_state.update(Duration::days(30), &cycle);
        
        // Temperature should change due to seasonal cycle
        assert_ne!(env_state.temperature.air_temperature, initial_temp);
    }
}
