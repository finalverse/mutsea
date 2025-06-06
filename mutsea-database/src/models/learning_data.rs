// mutsea-database/src/models/learning_data.rs
//! Learning data models for AI systems
//! 
//! These models capture learning experiences, skill development,
//! and knowledge acquisition for AI-driven entities.

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Learning data for AI systems
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningData {
    pub id: EntityId,
    pub learner_id: EntityId,
    pub learning_session_id: EntityId,
    pub learning_timestamp: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    
    // Learning context
    pub learning_type: LearningType,
    pub learning_domain: String,
    pub learning_objective: String,
    pub learning_environment: LearningEnvironment,
    
    // Learning process
    pub input_data: LearningInput,
    pub learning_algorithm: String,
    pub learning_parameters: HashMap<String, f32>,
    pub training_iterations: u64,
    pub convergence_criteria: ConvergenceCriteria,
    
    // Learning outcomes
    pub knowledge_acquired: Vec<KnowledgeItem>,
    pub skills_developed: Vec<SkillDevelopment>,
    pub performance_improvement: PerformanceImprovement,
    pub learning_efficiency: f32,
    
    // Validation and testing
    pub validation_results: Vec<ValidationResult>,
    pub test_performance: Vec<TestResult>,
    pub generalization_ability: f32,
    pub overfitting_detection: OverfittingAnalysis,
    
    // Transfer learning
    pub transfer_learning_data: Option<TransferLearningData>,
    pub knowledge_transfer_success: f32,
    pub adaptation_requirements: Vec<String>,
    
    // Metadata
    pub metadata: EntityMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LearningType {
    Supervised,
    Unsupervised,
    Reinforcement,
    SemiSupervised,
    TransferLearning,
    MetaLearning,
    OnlineLearning,
    ContinualLearning,
    FewShot,
    ZeroShot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningEnvironment {
    pub environment_type: EnvironmentType,
    pub complexity_level: f32,
    pub noise_level: f32,
    pub dynamic_nature: f32,
    pub available_feedback: FeedbackType,
    pub resource_constraints: ResourceConstraints,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnvironmentType {
    Simulation,
    RealWorld,
    Hybrid,
    Laboratory,
    Production,
    Sandbox,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FeedbackType {
    Immediate,
    Delayed,
    Sparse,
    Dense,
    Noisy,
    Clean,
    Human,
    Automated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceConstraints {
    pub computational_limits: ComputationalLimits,
    pub time_constraints: TimeConstraints,
    pub data_availability: DataAvailability,
    pub energy_budget: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputationalLimits {
    pub max_cpu_usage: f32,
    pub max_memory_mb: f32,
    pub max_gpu_usage: Option<f32>,
    pub parallel_processing_limit: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeConstraints {
    pub max_learning_duration: u64, // seconds
    pub real_time_requirements: bool,
    pub deadline_pressure: f32,
    pub learning_schedule: LearningSchedule,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LearningSchedule {
    Continuous,
    Episodic,
    Scheduled,
    OnDemand,
    Adaptive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataAvailability {
    pub training_data_size: u64,
    pub data_quality: DataQuality,
    pub data_diversity: f32,
    pub labeled_data_percentage: f32,
    pub data_refresh_rate: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningInput {
    pub input_type: InputType,
    pub data_format: String,
    pub input_dimensions: Vec<u32>,
    pub preprocessing_steps: Vec<String>,
    pub feature_engineering: Vec<FeatureEngineering>,
    pub data_augmentation: Vec<DataAugmentation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputType {
    Numerical,
    Categorical,
    Text,
    Image,
    Audio,
    Video,
    Sensor,
    Behavioral,
    Mixed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureEngineering {
    pub technique_name: String,
    pub input_features: Vec<String>,
    pub output_features: Vec<String>,
    pub transformation_function: String,
    pub effectiveness_score: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataAugmentation {
    pub augmentation_type: String,
    pub parameters: HashMap<String, f32>,
    pub augmentation_factor: f32,
    pub quality_preservation: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConvergenceCriteria {
    pub loss_threshold: f32,
    pub improvement_threshold: f32,
    pub patience_epochs: u32,
    pub max_iterations: u64,
    pub early_stopping: bool,
    pub convergence_achieved: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeItem {
    pub knowledge_id: EntityId,
    pub knowledge_type: KnowledgeType,
    pub knowledge_content: String,
    pub confidence_level: f32,
    pub evidence_strength: f32,
    pub applicability_scope: Vec<String>,
    pub knowledge_source: KnowledgeSource,
    pub validation_status: ValidationStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KnowledgeType {
    Factual,
    Procedural,
    Conceptual,
    Metacognitive,
    Experiential,
    Intuitive,
    Declarative,
    Contextual,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillDevelopment {
    pub skill_id: EntityId,
    pub skill_name: String,
    pub skill_category: SkillCategory,
    pub initial_proficiency: f32,
    pub final_proficiency: f32,
    pub improvement_rate: f32,
    pub learning_curve_type: LearningCurveType,
    pub mastery_indicators: Vec<MasteryIndicator>,
    pub skill_dependencies: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SkillCategory {
    Cognitive,
    Motor,
    Social,
    Creative,
    Technical,
    Communication,
    ProblemSolving,
    Leadership,
    Analytical,
    Artistic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LearningCurveType {
    Linear,
    Exponential,
    Logarithmic,
    Sigmoid,
    Plateau,
    Stepwise,
    Irregular,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MasteryIndicator {
    pub indicator_name: String,
    pub measurement_method: String,
    pub threshold_value: f32,
    pub current_value: f32,
    pub achieved: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceImprovement {
    pub baseline_performance: f32,
    pub current_performance: f32,
    pub improvement_percentage: f32,
    pub performance_metrics: Vec<PerformanceMetric>,
    pub improvement_trajectory: ImprovementTrajectory,
    pub performance_stability: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub metric_name: String,
    pub metric_value: f32,
    pub metric_weight: f32,
    pub improvement_direction: ImprovementDirection,
    pub benchmark_comparison: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImprovementDirection {
    Higher_Is_Better,
    Lower_Is_Better,
    Target_Value(f32),
    Range(f32, f32),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImprovementTrajectory {
    pub trajectory_type: TrajectoryType,
    pub learning_rate: f32,
    pub acceleration: f32,
    pub plateau_detection: bool,
    pub breakthrough_points: Vec<BreakthroughPoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrajectoryType {
    Steady_Improvement,
    Rapid_Initial_Gains,
    Slow_Start_Fast_Finish,
    Plateau_Then_Improvement,
    Cyclic_Improvement,
    Declining_Returns,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakthroughPoint {
    pub timestamp: Timestamp,
    pub performance_jump: f32,
    pub contributing_factors: Vec<String>,
    pub insight_gained: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationResult {
    pub validation_id: EntityId,
    pub validation_type: ValidationType,
    pub validation_method: String,
    pub validation_data_size: u64,
    pub accuracy_score: f32,
    pub precision_score: f32,
    pub recall_score: f32,
    pub f1_score: f32,
    pub confusion_matrix: Option<ConfusionMatrix>,
    pub cross_validation_scores: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationType {
    HoldOut,
    CrossValidation,
    BootstrapValidation,
    TimeSeriesValidation,
    LeaveOneOut,
    StratifiedValidation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfusionMatrix {
    pub true_positives: u64,
    pub false_positives: u64,
    pub true_negatives: u64,
    pub false_negatives: u64,
    pub class_labels: Vec<String>,
    pub matrix_values: Vec<Vec<u64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestResult {
    pub test_id: EntityId,
    pub test_name: String,
    pub test_type: TestType,
    pub test_data_size: u64,
    pub test_score: f32,
    pub error_analysis: ErrorAnalysis,
    pub performance_distribution: PerformanceDistribution,
    pub edge_case_performance: Vec<EdgeCaseResult>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestType {
    Unit_Test,
    Integration_Test,
    Performance_Test,
    Stress_Test,
    Adversarial_Test,
    Robustness_Test,
    Fairness_Test,
    Security_Test,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorAnalysis {
    pub total_errors: u64,
    pub error_types: HashMap<String, u64>,
    pub error_severity_distribution: HashMap<ErrorSeverity, u64>,
    pub common_failure_patterns: Vec<FailurePattern>,
    pub error_correction_suggestions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FailurePattern {
    pub pattern_name: String,
    pub frequency: u64,
    pub failure_conditions: Vec<String>,
    pub potential_causes: Vec<String>,
    pub mitigation_strategies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceDistribution {
    pub mean_performance: f32,
    pub median_performance: f32,
    pub std_deviation: f32,
    pub percentiles: HashMap<u8, f32>, // e.g., 25th, 50th, 75th, 95th percentiles
    pub outliers: Vec<OutlierResult>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutlierResult {
    pub outlier_type: OutlierType,
    pub value: f32,
    pub deviation_from_mean: f32,
    pub potential_explanation: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OutlierType {
    Exceptionally_Good,
    Exceptionally_Poor,
    Anomalous,
    Edge_Case,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeCaseResult {
    pub edge_case_name: String,
    pub case_description: String,
    pub performance_score: f32,
    pub failure_mode: Option<String>,
    pub robustness_assessment: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverfittingAnalysis {
    pub overfitting_detected: bool,
    pub overfitting_severity: f32,
    pub training_validation_gap: f32,
    pub complexity_penalty: f32,
    pub regularization_effectiveness: f32,
    pub recommended_actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferLearningData {
    pub source_domain: String,
    pub target_domain: String,
    pub domain_similarity: f32,
    pub knowledge_transferability: f32,
    pub adaptation_requirements: Vec<AdaptationRequirement>,
    pub transfer_success_metrics: TransferSuccessMetrics,
    pub fine_tuning_data: Option<FineTuningData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdaptationRequirement {
    pub requirement_type: String,
    pub adaptation_effort: f32,
    pub success_probability: f32,
    pub required_resources: HashMap<String, f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferSuccessMetrics {
    pub knowledge_retention: f32,
    pub adaptation_speed: f32,
    pub performance_on_target: f32,
    pub negative_transfer_detected: bool,
    pub positive_transfer_amount: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FineTuningData {
    pub fine_tuning_epochs: u32,
    pub learning_rate_schedule: Vec<f32>,
    pub layer_freezing_strategy: String,
    pub fine_tuning_data_size: u64,
    pub convergence_metrics: ConvergenceCriteria,
}

// Implement DatabaseModel trait
impl_database_model!(LearningData, id, created_at, updated_at);

impl AIModel for LearningData {
    fn confidence_score(&self) -> f32 {
        // Calculate confidence based on validation results and learning efficiency
        let validation_confidence = if self.validation_results.is_empty() {
            0.5
        } else {
            self.validation_results.iter()
                .map(|v| (v.accuracy_score + v.f1_score) / 2.0)
                .sum::<f32>() / self.validation_results.len() as f32
        };
        
        (validation_confidence + self.learning_efficiency + self.generalization_ability) / 3.0
    }
    
    fn ai_context(&self) -> &AIContext {
        static DEFAULT_CONTEXT: std::sync::OnceLock<AIContext> = std::sync::OnceLock::new();
        DEFAULT_CONTEXT.get_or_init(|| AIContext {
            model_version: "learning-system-v1.0".to_string(),
            decision_algorithm: self.learning_algorithm.clone(),
            training_data_hash: None,
            confidence_threshold: 0.7,
            processing_time_ms: 150,
            resource_usage: ResourceUsage {
                cpu_usage_percent: 40.0,
                memory_usage_mb: 1024.0,
                gpu_usage_percent: Some(60.0),
                network_io_bytes: 4096,
                disk_io_bytes: 8192,
            },
        })
    }
    
    fn has_learning_data(&self) -> bool {
        !self.knowledge_acquired.is_empty() || !self.skills_developed.is_empty()
    }
}

// Default implementations
impl Default for LearningEnvironment {
    fn default() -> Self {
        Self {
            environment_type: EnvironmentType::Simulation,
            complexity_level: 0.5,
            noise_level: 0.1,
            dynamic_nature: 0.3,
            available_feedback: FeedbackType::Immediate,
            resource_constraints: ResourceConstraints::default(),
        }
    }
}

impl Default for ResourceConstraints {
    fn default() -> Self {
        Self {
            computational_limits: ComputationalLimits::default(),
            time_constraints: TimeConstraints::default(),
            data_availability: DataAvailability::default(),
            energy_budget: None,
        }
    }
}

impl Default for ComputationalLimits {
    fn default() -> Self {
        Self {
            max_cpu_usage: 80.0,
            max_memory_mb: 4096.0,
            max_gpu_usage: Some(90.0),
            parallel_processing_limit: 8,
        }
    }
}

impl Default for TimeConstraints {
    fn default() -> Self {
        Self {
            max_learning_duration: 3600, // 1 hour
            real_time_requirements: false,
            deadline_pressure: 0.3,
            learning_schedule: LearningSchedule::Adaptive,
        }
    }
}

impl Default for DataAvailability {
    fn default() -> Self {
        Self {
            training_data_size: 10000,
            data_quality: DataQuality::default(),
            data_diversity: 0.7,
            labeled_data_percentage: 0.8,
            data_refresh_rate: 0.1,
        }
    }
}

impl Default for ConvergenceCriteria {
    fn default() -> Self {
        Self {
            loss_threshold: 0.01,
            improvement_threshold: 0.001,
            patience_epochs: 10,
            max_iterations: 1000,
            early_stopping: true,
            convergence_achieved: false,
        }
    }
}

impl Default for PerformanceImprovement {
    fn default() -> Self {
        Self {
            baseline_performance: 0.5,
            current_performance: 0.7,
            improvement_percentage: 40.0,
            performance_metrics: Vec::new(),
            improvement_trajectory: ImprovementTrajectory::default(),
            performance_stability: 0.8,
        }
    }
}

impl Default for ImprovementTrajectory {
    fn default() -> Self {
        Self {
            trajectory_type: TrajectoryType::Steady_Improvement,
            learning_rate: 0.1,
            acceleration: 0.0,
            plateau_detection: false,
            breakthrough_points: Vec::new(),
        }
    }
}

// Utility implementations
impl LearningData {
    /// Create new learning data
    pub fn new(
        learner_id: EntityId,
        learning_type: LearningType,
        learning_domain: String,
        learning_objective: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            learner_id,
            learning_session_id: Uuid::new_v4(),
            learning_timestamp: now,
            created_at: now,
            updated_at: now,
            learning_type,
            learning_domain,
            learning_objective,
            learning_environment: LearningEnvironment::default(),
            input_data: LearningInput::default(),
            learning_algorithm: "default_algorithm".to_string(),
            learning_parameters: HashMap::new(),
            training_iterations: 0,
            convergence_criteria: ConvergenceCriteria::default(),
            knowledge_acquired: Vec::new(),
            skills_developed: Vec::new(),
            performance_improvement: PerformanceImprovement::default(),
            learning_efficiency: 0.6,
            validation_results: Vec::new(),
            test_performance: Vec::new(),
            generalization_ability: 0.5,
            overfitting_detection: OverfittingAnalysis::default(),
            transfer_learning_data: None,
            knowledge_transfer_success: 0.0,
            adaptation_requirements: Vec::new(),
            metadata: EntityMetadata::default(),
        }
    }
    
    /// Add knowledge item
    pub fn add_knowledge(&mut self, knowledge: KnowledgeItem) {
        self.knowledge_acquired.push(knowledge);
        self.updated_at = Utc::now();
    }
    
    /// Add skill development
    pub fn add_skill_development(&mut self, skill: SkillDevelopment) {
        self.skills_developed.push(skill);
        self.updated_at = Utc::now();
    }
    
    /// Calculate overall learning success
    pub fn calculate_learning_success(&self) -> f32 {
        let knowledge_score = if self.knowledge_acquired.is_empty() {
            0.0
        } else {
            self.knowledge_acquired.iter()
                .map(|k| k.confidence_level)
                .sum::<f32>() / self.knowledge_acquired.len() as f32
        };
        
        let skill_score = if self.skills_developed.is_empty() {
            0.0
        } else {
            self.skills_developed.iter()
                .map(|s| s.final_proficiency)
                .sum::<f32>() / self.skills_developed.len() as f32
        };
        
        let performance_score = self.performance_improvement.improvement_percentage / 100.0;
        let validation_score = if self.validation_results.is_empty() {
            0.5
        } else {
            self.validation_results.iter()
                .map(|v| v.f1_score)
                .sum::<f32>() / self.validation_results.len() as f32
        };
        
        (knowledge_score + skill_score + performance_score + validation_score) / 4.0
    }
    
    /// Check if learning has converged
    pub fn has_converged(&self) -> bool {
        self.convergence_criteria.convergence_achieved
    }
    
    /// Generate learning report
    pub fn generate_learning_report(&self) -> String {
        format!(
            "Learning Report - Session {}\n\
            ================================\n\
            Learner: {}\n\
            Type: {:?}\n\
            Domain: {}\n\
            Objective: {}\n\
            \n\
            Progress:\n\
            - Knowledge Items: {}\n\
            - Skills Developed: {}\n\
            - Learning Efficiency: {:.1}%\n\
            - Overall Success: {:.1}%\n\
            - Converged: {}\n\
            \n\
            Performance:\n\
            - Baseline: {:.2}\n\
            - Current: {:.2}\n\
            - Improvement: {:.1}%\n\
            - Generalization: {:.1}%\n\
            \n\
            Validation:\n\
            - Validation Tests: {}\n\
            - Test Performance: {}\n\
            - Overfitting Detected: {}\n\
            \n\
            Transfer Learning: {}\n\
            Adaptation Requirements: {}\n",
            self.learning_session_id,
            self.learner_id,
            self.learning_type,
            self.learning_domain,
            self.learning_objective,
            self.knowledge_acquired.len(),
            self.skills_developed.len(),
            self.learning_efficiency * 100.0,
            self.calculate_learning_success() * 100.0,
            self.has_converged(),
            self.performance_improvement.baseline_performance,
            self.performance_improvement.current_performance,
            self.performance_improvement.improvement_percentage,
            self.generalization_ability * 100.0,
            self.validation_results.len(),
            self.test_performance.len(),
            self.overfitting_detection.overfitting_detected,
            if self.transfer_learning_data.is_some() { "Yes" } else { "No" },
            self.adaptation_requirements.len(),
        )
    }
}

impl Default for LearningInput {
    fn default() -> Self {
        Self {
            input_type: InputType::Mixed,
            data_format: "json".to_string(),
            input_dimensions: vec![100],
            preprocessing_steps: Vec::new(),
            feature_engineering: Vec::new(),
            data_augmentation: Vec::new(),
        }
    }
}

impl Default for OverfittingAnalysis {
    fn default() -> Self {
        Self {
            overfitting_detected: false,
            overfitting_severity: 0.0,
            training_validation_gap: 0.05,
            complexity_penalty: 0.1,
            regularization_effectiveness: 0.7,
            recommended_actions: Vec::new(),
        }
    }
}

impl SkillDevelopment {
    /// Calculate improvement rate
    pub fn calculate_improvement_rate(&self) -> f32 {
        if self.initial_proficiency > 0.0 {
            (self.final_proficiency - self.initial_proficiency) / self.initial_proficiency
        } else {
            self.final_proficiency
        }
    }
    
    /// Check if skill has been mastered
    pub fn is_mastered(&self) -> bool {
        self.mastery_indicators.iter().all(|indicator| indicator.achieved)
    }
}

impl KnowledgeItem {
    /// Check if knowledge is reliable
    pub fn is_reliable(&self) -> bool {
        self.confidence_level > 0.7 && 
        self.evidence_strength > 0.6 && 
        matches!(self.validation_status, ValidationStatus::Validated)
    }
}

impl ValidationResult {
    /// Calculate weighted score
    pub fn calculate_weighted_score(&self) -> f32 {
        (self.accuracy_score + self.precision_score + self.recall_score + self.f1_score) / 4.0
    }
}