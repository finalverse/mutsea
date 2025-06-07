// mutsea-database/src/models/performance_metrics/ai_metrics.rs
//! AI-specific performance metrics for machine learning models and intelligent systems

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Container for all AI-related metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIMetrics {
    pub ai_specific_metrics: AISpecificMetrics,
    pub learning_metrics: Option<LearningMetrics>,
    pub prediction_metrics: Option<PredictionMetrics>,
    pub model_performance_metrics: ModelPerformanceMetrics,
    pub inference_metrics: InferenceMetrics,
    pub training_metrics: Option<TrainingMetrics>,
}

/// AI-specific performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AISpecificMetrics {
    pub inference_time_ms: f32,
    pub training_time_hours: Option<f32>,
    pub model_accuracy: f32,
    pub model_size_mb: f32,
    pub inference_throughput: f32, // inferences per second
    pub gpu_utilization_ai: f32,
    pub memory_usage_ai_mb: f32,
    pub ai_pipeline_efficiency: f32,
}

/// Model performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelPerformanceMetrics {
    pub convergence_rate: f32,
    pub overfitting_score: f32,
    pub generalization_ability: f32,
    pub robustness_score: f32,
    pub interpretability_score: f32,
    pub fairness_metrics: FairnessMetrics,
    pub model_complexity: ModelComplexity,
    pub feature_importance: Vec<FeatureImportance>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FairnessMetrics {
    pub demographic_parity: f32,
    pub equalized_odds: f32,
    pub calibration_score: f32,
    pub bias_detection_results: Vec<BiasDetectionResult>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BiasDetectionResult {
    pub bias_type: String,
    pub severity: f32,
    pub affected_groups: Vec<String>,
    pub mitigation_suggestions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelComplexity {
    pub parameter_count: u64,
    pub layer_count: u32,
    pub computational_complexity: f32,
    pub memory_complexity: f32,
    pub interpretability_level: InterpretabilityLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterpretabilityLevel {
    HighlyInterpretable,
    ModeratelyInterpretable,
    LowInterpretability,
    BlackBox,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureImportance {
    pub feature_name: String,
    pub importance_score: f32,
    pub confidence: f32,
    pub correlation_with_target: f32,
}

/// Learning performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningMetrics {
    pub learning_rate: f32,
    pub knowledge_retention_rate: f32,
    pub skill_acquisition_speed: f32,
    pub adaptation_capability: f32,
    pub transfer_learning_effectiveness: f32,
    pub learning_efficiency_score: f32,
    pub forgetting_curve_analysis: ForgettingCurveAnalysis,
    pub learning_plateau_detection: PlateauDetection,
    pub catastrophic_forgetting_risk: f32,
    pub continual_learning_performance: ContinualLearningMetrics,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForgettingCurveAnalysis {
    pub initial_retention: f32,
    pub retention_after_24h: f32,
    pub retention_after_week: f32,
    pub retention_after_month: f32,
    pub forgetting_rate: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlateauDetection {
    pub plateau_detected: bool,
    pub plateau_duration: Option<u64>, // seconds
    pub plateau_level: f32,
    pub breakthrough_strategies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinualLearningMetrics {
    pub new_task_learning_speed: f32,
    pub old_task_retention: f32,
    pub forward_transfer: f32,
    pub backward_transfer: f32,
    pub learning_without_forgetting_score: f32,
}

/// Prediction performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionMetrics {
    pub prediction_accuracy: f32,
    pub prediction_precision: f32,
    pub prediction_recall: f32,
    pub mean_absolute_error: f32,
    pub root_mean_square_error: f32,
    pub prediction_latency_ms: f32,
    pub confidence_calibration: f32,
    pub prediction_stability: f32,
    pub forecast_horizon_accuracy: HashMap<String, f32>, // time_horizon -> accuracy
    pub uncertainty_quantification: UncertaintyMetrics,
    pub prediction_consistency: PredictionConsistency,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UncertaintyMetrics {
    pub epistemic_uncertainty: f32,
    pub aleatoric_uncertainty: f32,
    pub total_uncertainty: f32,
    pub uncertainty_calibration: f32,
    pub confidence_intervals_coverage: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionConsistency {
    pub temporal_consistency: f32,
    pub spatial_consistency: f32,
    pub logical_consistency: f32,
    pub consistency_violations: u32,
}

/// Inference performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InferenceMetrics {
    pub batch_inference_time_ms: f32,
    pub single_inference_time_ms: f32,
    pub inference_queue_length: u32,
    pub inference_cache_hit_rate: f32,
    pub model_loading_time_ms: f32,
    pub preprocessing_time_ms: f32,
    pub postprocessing_time_ms: f32,
    pub inference_resource_utilization: InferenceResourceUtilization,
    pub inference_scalability: InferenceScalability,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InferenceResourceUtilization {
    pub cpu_utilization_inference: f32,
    pub gpu_utilization_inference: f32,
    pub memory_utilization_inference: f32,
    pub network_io_inference: f32,
    pub storage_io_inference: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InferenceScalability {
    pub max_concurrent_inferences: u32,
    pub throughput_scaling_factor: f32,
    pub latency_scaling_factor: f32,
    pub resource_scaling_efficiency: f32,
}

/// Training performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainingMetrics {
    pub training_loss: f32,
    pub validation_loss: f32,
    pub training_accuracy: f32,
    pub validation_accuracy: f32,
    pub epochs_completed: u32,
    pub convergence_epoch: Option<u32>,
    pub training_time_per_epoch_minutes: f32,
    pub data_loading_time_percentage: f32,
    pub gradient_computation_time_percentage: f32,
    pub parameter_update_time_percentage: f32,
    pub training_stability: TrainingStability,
    pub hyperparameter_sensitivity: HyperparameterSensitivity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainingStability {
    pub loss_variance: f32,
    pub gradient_variance: f32,
    pub weight_update_variance: f32,
    pub training_instability_events: u32,
    pub nan_gradient_occurrences: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HyperparameterSensitivity {
    pub learning_rate_sensitivity: f32,
    pub batch_size_sensitivity: f32,
    pub regularization_sensitivity: f32,
    pub architecture_sensitivity: f32,
    pub optimization_algorithm_sensitivity: f32,
}

// Default implementations
impl Default for AIMetrics {
    fn default() -> Self {
        Self {
            ai_specific_metrics: AISpecificMetrics::default(),
            learning_metrics: None,
            prediction_metrics: None,
            model_performance_metrics: ModelPerformanceMetrics::default(),
            inference_metrics: InferenceMetrics::default(),
            training_metrics: None,
        }
    }
}

impl Default for AISpecificMetrics {
    fn default() -> Self {
        Self {
            inference_time_ms: 50.0,
            training_time_hours: None,
            model_accuracy: 0.85,
            model_size_mb: 100.0,
            inference_throughput: 100.0,
            gpu_utilization_ai: 70.0,
            memory_usage_ai_mb: 2048.0,
            ai_pipeline_efficiency: 0.8,
        }
    }
}

impl Default for ModelPerformanceMetrics {
    fn default() -> Self {
        Self {
            convergence_rate: 0.8,
            overfitting_score: 0.2,
            generalization_ability: 0.75,
            robustness_score: 0.7,
            interpretability_score: 0.6,
            fairness_metrics: FairnessMetrics::default(),
            model_complexity: ModelComplexity::default(),
            feature_importance: Vec::new(),
        }
    }
}

impl Default for FairnessMetrics {
    fn default() -> Self {
        Self {
            demographic_parity: 0.8,
            equalized_odds: 0.75,
            calibration_score: 0.85,
            bias_detection_results: Vec::new(),
        }
    }
}

impl Default for ModelComplexity {
    fn default() -> Self {
        Self {
            parameter_count: 1_000_000,
            layer_count: 10,
            computational_complexity: 0.6,
            memory_complexity: 0.5,
            interpretability_level: InterpretabilityLevel::ModeratelyInterpretable,
        }
    }
}

impl Default for InferenceMetrics {
    fn default() -> Self {
        Self {
            batch_inference_time_ms: 100.0,
            single_inference_time_ms: 10.0,
            inference_queue_length: 5,
            inference_cache_hit_rate: 0.8,
            model_loading_time_ms: 1000.0,
            preprocessing_time_ms: 5.0,
            postprocessing_time_ms: 2.0,
            inference_resource_utilization: InferenceResourceUtilization::default(),
            inference_scalability: InferenceScalability::default(),
        }
    }
}

impl Default for InferenceResourceUtilization {
    fn default() -> Self {
        Self {
            cpu_utilization_inference: 40.0,
            gpu_utilization_inference: 80.0,
            memory_utilization_inference: 60.0,
            network_io_inference: 10.0,
            storage_io_inference: 5.0,
        }
    }
}

impl Default for InferenceScalability {
    fn default() -> Self {
        Self {
            max_concurrent_inferences: 100,
            throughput_scaling_factor: 0.9,
            latency_scaling_factor: 1.1,
            resource_scaling_efficiency: 0.85,
        }
    }
}

// Utility implementations
impl AISpecificMetrics {
    /// Calculate overall AI performance score
    pub fn calculate_ai_performance_score(&self) -> f32 {
        let accuracy_score = self.model_accuracy;
        let efficiency_score = self.ai_pipeline_efficiency;
        let throughput_score = (self.inference_throughput / 1000.0).min(1.0);
        let latency_score = (100.0 / self.inference_time_ms).min(1.0);
        
        (accuracy_score + efficiency_score + throughput_score + latency_score) / 4.0
    }
    
    /// Check if AI performance is acceptable
    pub fn is_performance_acceptable(&self, min_accuracy: f32, max_latency_ms: f32) -> bool {
        self.model_accuracy >= min_accuracy && self.inference_time_ms <= max_latency_ms
    }
    
    /// Get AI performance level
    pub fn get_performance_level(&self) -> AIPerformanceLevel {
        let score = self.calculate_ai_performance_score();
        match score {
            x if x >= 0.9 => AIPerformanceLevel::Excellent,
            x if x >= 0.8 => AIPerformanceLevel::Good,
            x if x >= 0.7 => AIPerformanceLevel::Fair,
            x if x >= 0.6 => AIPerformanceLevel::Poor,
            _ => AIPerformanceLevel::Critical,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AIPerformanceLevel {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

impl ModelPerformanceMetrics {
    /// Check if model shows signs of overfitting
    pub fn is_overfitting(&self) -> bool {
        self.overfitting_score > 0.3 || self.generalization_ability < 0.6
    }
    
    /// Calculate model quality score
    pub fn calculate_model_quality(&self) -> f32 {
        let accuracy_factor = self.convergence_rate;
        let generalization_factor = self.generalization_ability;
        let robustness_factor = self.robustness_score;
        let fairness_factor = (self.fairness_metrics.demographic_parity + 
                              self.fairness_metrics.equalized_odds + 
                              self.fairness_metrics.calibration_score) / 3.0;
        let overfitting_penalty = 1.0 - self.overfitting_score;
        
        (accuracy_factor + generalization_factor + robustness_factor + 
         fairness_factor + overfitting_penalty) / 5.0
    }
    
    /// Get model readiness for production
    pub fn is_production_ready(&self) -> bool {
        self.convergence_rate > 0.8 &&
        self.generalization_ability > 0.7 &&
        self.robustness_score > 0.7 &&
        !self.is_overfitting() &&
        self.fairness_metrics.bias_detection_results.is_empty()
    }
}

impl LearningMetrics {
    /// Calculate learning effectiveness
    pub fn calculate_learning_effectiveness(&self) -> f32 {
        let retention_score = self.knowledge_retention_rate;
        let adaptation_score = self.adaptation_capability;
        let efficiency_score = self.learning_efficiency_score;
        let transfer_score = self.transfer_learning_effectiveness;
        let continual_score = (self.continual_learning_performance.new_task_learning_speed + 
                              self.continual_learning_performance.old_task_retention) / 2.0;
        
        (retention_score + adaptation_score + efficiency_score + transfer_score + continual_score) / 5.0
    }
    
    /// Check if learning is plateauing
    pub fn is_learning_plateauing(&self) -> bool {
        self.learning_plateau_detection.plateau_detected
    }
    
    /// Get learning status
    pub fn get_learning_status(&self) -> LearningStatus {
        if self.catastrophic_forgetting_risk > 0.7 {
            LearningStatus::CatastrophicForgetting
        } else if self.is_learning_plateauing() {
            LearningStatus::Plateaued
        } else if self.learning_efficiency_score > 0.8 {
            LearningStatus::Optimal
        } else if self.learning_efficiency_score > 0.6 {
            LearningStatus::Good
        } else {
            LearningStatus::Poor
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LearningStatus {
    Optimal,
    Good,
    Poor,
    Plateaued,
    CatastrophicForgetting,
}

impl PredictionMetrics {
    /// Calculate prediction quality score
    pub fn calculate_prediction_quality(&self) -> f32 {
        let accuracy_score = self.prediction_accuracy;
        let precision_score = self.prediction_precision;
        let recall_score = self.prediction_recall;
        let calibration_score = self.confidence_calibration;
        let stability_score = self.prediction_stability;
        let consistency_score = (self.prediction_consistency.temporal_consistency + 
                                self.prediction_consistency.spatial_consistency + 
                                self.prediction_consistency.logical_consistency) / 3.0;
        
        (accuracy_score + precision_score + recall_score + calibration_score + 
         stability_score + consistency_score) / 6.0
    }
    
    /// Check if predictions are reliable
    pub fn are_predictions_reliable(&self) -> bool {
        self.prediction_accuracy > 0.8 &&
        self.confidence_calibration > 0.7 &&
        self.prediction_stability > 0.7 &&
        self.prediction_consistency.consistency_violations < 5
    }
    
    /// Get uncertainty level
    pub fn get_uncertainty_level(&self) -> UncertaintyLevel {
        let total_uncertainty = self.uncertainty_quantification.total_uncertainty;
        match total_uncertainty {
            x if x < 0.1 => UncertaintyLevel::VeryLow,
            x if x < 0.2 => UncertaintyLevel::Low,
            x if x < 0.4 => UncertaintyLevel::Moderate,
            x if x < 0.6 => UncertaintyLevel::High,
            _ => UncertaintyLevel::VeryHigh,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UncertaintyLevel {
    VeryLow,
    Low,
    Moderate,
    High,
    VeryHigh,
}

impl InferenceMetrics {
    /// Calculate inference efficiency
    pub fn calculate_inference_efficiency(&self) -> f32 {
        let latency_score = (50.0 / self.single_inference_time_ms).min(1.0);
        let cache_score = self.inference_cache_hit_rate;
        let queue_score = 1.0 - (self.inference_queue_length as f32 / 100.0).min(1.0);
        let resource_score = (self.inference_resource_utilization.cpu_utilization_inference + 
                             self.inference_resource_utilization.gpu_utilization_inference + 
                             self.inference_resource_utilization.memory_utilization_inference) / 300.0;
        
        (latency_score + cache_score + queue_score + resource_score.min(1.0)) / 4.0
    }
    
    /// Check if inference is real-time capable
    pub fn is_real_time_capable(&self, max_latency_ms: f32) -> bool {
        self.single_inference_time_ms <= max_latency_ms && 
        self.inference_queue_length < 10
    }
    
    /// Get inference performance status
    pub fn get_performance_status(&self) -> InferencePerformanceStatus {
        let efficiency = self.calculate_inference_efficiency();
        match efficiency {
            x if x >= 0.9 => InferencePerformanceStatus::Excellent,
            x if x >= 0.8 => InferencePerformanceStatus::Good,
            x if x >= 0.6 => InferencePerformanceStatus::Acceptable,
            x if x >= 0.4 => InferencePerformanceStatus::Poor,
            _ => InferencePerformanceStatus::Critical,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InferencePerformanceStatus {
    Excellent,
    Good,
    Acceptable,
    Poor,
    Critical,
}

impl TrainingMetrics {
    /// Check if training is converging
    pub fn is_converging(&self) -> bool {
        self.training_loss > self.validation_loss * 0.8 && // Not too much overfitting
        self.training_stability.loss_variance < 0.1 &&
        self.training_stability.training_instability_events < 3
    }
    
    /// Calculate training efficiency
    pub fn calculate_training_efficiency(&self) -> f32 {
        let convergence_speed = if let Some(conv_epoch) = self.convergence_epoch {
            (50.0 / conv_epoch as f32).min(1.0) // Prefer faster convergence
        } else {
            0.5 // Default if not converged yet
        };
        
        let stability_score = 1.0 - self.training_stability.loss_variance;
        let accuracy_score = (self.training_accuracy + self.validation_accuracy) / 2.0;
        let time_efficiency = (30.0 / self.training_time_per_epoch_minutes).min(1.0);
        
        (convergence_speed + stability_score + accuracy_score + time_efficiency) / 4.0
    }
    
    /// Get training status
    pub fn get_training_status(&self) -> TrainingStatus {
        if self.training_stability.nan_gradient_occurrences > 0 {
            TrainingStatus::Failed
        } else if self.convergence_epoch.is_some() {
            TrainingStatus::Converged
        } else if self.is_converging() {
            TrainingStatus::Converging
        } else if self.training_stability.training_instability_events > 5 {
            TrainingStatus::Unstable
        } else {
            TrainingStatus::InProgress
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrainingStatus {
    InProgress,
    Converging,
    Converged,
    Unstable,
    Failed,
}

impl AIMetrics {
    /// Calculate overall AI system performance
    pub fn calculate_overall_ai_performance(&self) -> f32 {
        let mut total_score = 0.0;
        let mut count = 0.0;
        
        // Always include basic AI metrics
        total_score += self.ai_specific_metrics.calculate_ai_performance_score();
        count += 1.0;
        
        total_score += self.model_performance_metrics.calculate_model_quality();
        count += 1.0;
        
        total_score += self.inference_metrics.calculate_inference_efficiency();
        count += 1.0;
        
        // Include optional metrics if available
        if let Some(learning) = &self.learning_metrics {
            total_score += learning.calculate_learning_effectiveness();
            count += 1.0;
        }
        
        if let Some(prediction) = &self.prediction_metrics {
            total_score += prediction.calculate_prediction_quality();
            count += 1.0;
        }
        
        if let Some(training) = &self.training_metrics {
            total_score += training.calculate_training_efficiency();
            count += 1.0;
        }
        
        total_score / count
    }
    
    /// Check if AI system is production ready
    pub fn is_production_ready(&self) -> bool {
        self.model_performance_metrics.is_production_ready() &&
        self.ai_specific_metrics.is_performance_acceptable(0.8, 100.0) &&
        self.inference_metrics.is_real_time_capable(50.0) &&
        self.prediction_metrics.as_ref().map_or(true, |p| p.are_predictions_reliable())
    }
    
    /// Get AI system health status
    pub fn get_ai_health_status(&self) -> AIHealthStatus {
        let overall_performance = self.calculate_overall_ai_performance();
        
        if !self.model_performance_metrics.is_production_ready() {
            AIHealthStatus::ModelIssues
        } else if self.learning_metrics.as_ref().map_or(false, |l| 
            matches!(l.get_learning_status(), LearningStatus::CatastrophicForgetting)) {
            AIHealthStatus::LearningIssues
        } else if overall_performance >= 0.9 {
            AIHealthStatus::Excellent
        } else if overall_performance >= 0.8 {
            AIHealthStatus::Good
        } else if overall_performance >= 0.6 {
            AIHealthStatus::Fair
        } else {
            AIHealthStatus::Poor
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AIHealthStatus {
    Excellent,
    Good,
    Fair,
    Poor,
    ModelIssues,
    LearningIssues,
}