// mutsea-database/src/analytics/mod.rs

pub mod player_analytics;
pub mod ai_analytics;
pub mod ecosystem_analytics;
pub mod performance_analytics;
pub mod cache;

use crate::error::Result;
use crate::utils::parameter_binding::ParameterBinder;
use crate::utils::sql_loader::SqlLoader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;

/// Central analytics coordinator
pub struct AnalyticsEngine {
    sql_loader: SqlLoader,
    cache: cache::AnalyticsCache,
    player_analytics: player_analytics::PlayerAnalytics,
    ai_analytics: ai_analytics::AIAnalytics,
    ecosystem_analytics: ecosystem_analytics::EcosystemAnalytics,
    performance_analytics: performance_analytics::PerformanceAnalytics,
}

impl AnalyticsEngine {
    pub fn new(sql_loader: SqlLoader) -> Self {
        let cache = cache::AnalyticsCache::new();
        Self {
            player_analytics: player_analytics::PlayerAnalytics::new(sql_loader.clone()),
            ai_analytics: ai_analytics::AIAnalytics::new(sql_loader.clone()),
            ecosystem_analytics: ecosystem_analytics::EcosystemAnalytics::new(sql_loader.clone()),
            performance_analytics: performance_analytics::PerformanceAnalytics::new(sql_loader.clone()),
            sql_loader,
            cache,
        }
    }

    /// Generate comprehensive analytics report
    pub async fn generate_comprehensive_report(
        &self,
        time_range: TimeRange,
        report_config: &AnalyticsReportConfig,
    ) -> Result<ComprehensiveAnalyticsReport> {
        let mut report = ComprehensiveAnalyticsReport::new(time_range.clone());

        // Player analytics
        if report_config.include_player_analytics {
            report.player_analytics = Some(
                self.player_analytics.generate_player_behavior_summary(time_range.clone()).await?
            );
        }

        // AI analytics
        if report_config.include_ai_analytics {
            report.ai_analytics = Some(
                self.ai_analytics.generate_ai_effectiveness_report(time_range.clone()).await?
            );
        }

        // Ecosystem analytics
        if report_config.include_ecosystem_analytics {
            report.ecosystem_analytics = Some(
                self.ecosystem_analytics.generate_ecosystem_health_report(time_range.clone()).await?
            );
        }

        // Performance analytics
        if report_config.include_performance_analytics {
            report.performance_analytics = Some(
                self.performance_analytics.generate_performance_summary(time_range.clone()).await?
            );
        }

        // Cross-system correlations
        if report_config.include_correlations {
            report.correlations = Some(self.calculate_cross_system_correlations(&report).await?);
        }

        Ok(report)
    }

    /// Real-time analytics dashboard data
    pub async fn get_realtime_dashboard_data(
        &self,
        dashboard_config: &DashboardConfig,
    ) -> Result<RealtimeDashboardData> {
        let cache_key = format!("dashboard_{}", dashboard_config.dashboard_id);
        
        // Check cache first
        if let Some(cached_data) = self.cache.get_dashboard_data(&cache_key).await? {
            if cached_data.is_fresh(dashboard_config.refresh_interval_seconds) {
                return Ok(cached_data);
            }
        }

        // Generate fresh data
        let dashboard_data = self.generate_dashboard_data(dashboard_config).await?;
        
        // Cache the result
        self.cache.store_dashboard_data(&cache_key, &dashboard_data, dashboard_config.cache_ttl_seconds).await?;

        Ok(dashboard_data)
    }

    /// Detect anomalies across all systems
    pub async fn detect_anomalies(
        &self,
        detection_config: &AnomalyDetectionConfig,
    ) -> Result<Vec<SystemAnomaly>> {
        let mut anomalies = Vec::new();

        // Player behavior anomalies
        anomalies.extend(
            self.player_analytics.detect_behavior_anomalies(detection_config.clone()).await?
        );

        // AI decision anomalies
        anomalies.extend(
            self.ai_analytics.detect_decision_anomalies(detection_config.clone()).await?
        );

        // Ecosystem anomalies
        anomalies.extend(
            self.ecosystem_analytics.detect_ecosystem_anomalies(detection_config.clone()).await?
        );

        // Performance anomalies
        anomalies.extend(
            self.performance_analytics.detect_performance_anomalies(detection_config.clone()).await?
        );

        // Sort by severity
        anomalies.sort_by(|a, b| b.severity.partial_cmp(&a.severity).unwrap());

        Ok(anomalies)
    }

    /// Generate predictive insights
    pub async fn generate_predictive_insights(
        &self,
        prediction_config: &PredictionConfig,
    ) -> Result<Vec<PredictiveInsight>> {
        let mut insights = Vec::new();

        // Player behavior predictions
        insights.extend(
            self.player_analytics.predict_player_behavior(prediction_config.clone()).await?
        );

        // AI effectiveness predictions
        insights.extend(
            self.ai_analytics.predict_ai_effectiveness(prediction_config.clone()).await?
        );

        // Ecosystem evolution predictions
        insights.extend(
            self.ecosystem_analytics.predict_ecosystem_changes(prediction_config.clone()).await?
        );

        // Performance predictions
        insights.extend(
            self.performance_analytics.predict_performance_issues(prediction_config.clone()).await?
        );

        Ok(insights)
    }

    async fn calculate_cross_system_correlations(
        &self,
        report: &ComprehensiveAnalyticsReport,
    ) -> Result<CrossSystemCorrelations> {
        // This would implement correlation analysis between different system metrics
        // For now, return empty correlations
        Ok(CrossSystemCorrelations::default())
    }

    async fn generate_dashboard_data(
        &self,
        dashboard_config: &DashboardConfig,
    ) -> Result<RealtimeDashboardData> {
        let mut data = RealtimeDashboardData::new();
        let now = Utc::now();
        let time_range = TimeRange::new(
            now - Duration::hours(dashboard_config.time_window_hours as i64),
            now,
        );

        // Current metrics
        data.current_players = self.player_analytics.get_current_player_count().await?;
        data.ai_decisions_per_minute = self.ai_analytics.get_decisions_per_minute().await?;
        data.ecosystem_health = self.ecosystem_analytics.get_overall_health_score().await?;
        data.system_performance = self.performance_analytics.get_current_performance_score().await?;

        // Trend data
        data.player_trends = self.player_analytics.get_activity_trends(time_range.clone()).await?;
        data.ai_trends = self.ai_analytics.get_effectiveness_trends(time_range.clone()).await?;
        data.ecosystem_trends = self.ecosystem_analytics.get_health_trends(time_range.clone()).await?;
        data.performance_trends = self.performance_analytics.get_performance_trends(time_range).await?;

        // Recent events
        data.recent_anomalies = self.detect_anomalies(&AnomalyDetectionConfig::default()).await?
            .into_iter()
            .take(10)
            .collect();

        data.generated_at = now;
        Ok(data)
    }
}

/// Time range for analytics queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl TimeRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { start, end }
    }

    pub fn last_hour() -> Self {
        let now = Utc::now();
        Self::new(now - Duration::hours(1), now)
    }

    pub fn last_day() -> Self {
        let now = Utc::now();
        Self::new(now - Duration::days(1), now)
    }

    pub fn last_week() -> Self {
        let now = Utc::now();
        Self::new(now - Duration::weeks(1), now)
    }

    pub fn duration(&self) -> Duration {
        self.end - self.start
    }
}

/// Analytics report configuration
#[derive(Debug, Clone)]
pub struct AnalyticsReportConfig {
    pub include_player_analytics: bool,
    pub include_ai_analytics: bool,
    pub include_ecosystem_analytics: bool,
    pub include_performance_analytics: bool,
    pub include_correlations: bool,
    pub detail_level: ReportDetailLevel,
}

impl Default for AnalyticsReportConfig {
    fn default() -> Self {
        Self {
            include_player_analytics: true,
            include_ai_analytics: true,
            include_ecosystem_analytics: true,
            include_performance_analytics: true,
            include_correlations: true,
            detail_level: ReportDetailLevel::Standard,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReportDetailLevel {
    Summary,
    Standard,
    Detailed,
    Comprehensive,
}

/// Comprehensive analytics report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveAnalyticsReport {
    pub time_range: TimeRange,
    pub generated_at: DateTime<Utc>,
    pub player_analytics: Option<player_analytics::PlayerBehaviorSummary>,
    pub ai_analytics: Option<ai_analytics::AIEffectivenessReport>,
    pub ecosystem_analytics: Option<ecosystem_analytics::EcosystemHealthReport>,
    pub performance_analytics: Option<performance_analytics::PerformanceSummary>,
    pub correlations: Option<CrossSystemCorrelations>,
}

impl ComprehensiveAnalyticsReport {
    pub fn new(time_range: TimeRange) -> Self {
        Self {
            time_range,
            generated_at: Utc::now(),
            player_analytics: None,
            ai_analytics: None,
            ecosystem_analytics: None,
            performance_analytics: None,
            correlations: None,
        }
    }
}

/// Real-time dashboard configuration
#[derive(Debug, Clone)]
pub struct DashboardConfig {
    pub dashboard_id: String,
    pub time_window_hours: u32,
    pub refresh_interval_seconds: u32,
    pub cache_ttl_seconds: u32,
}

/// Real-time dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeDashboardData {
    pub generated_at: DateTime<Utc>,
    pub current_players: u64,
    pub ai_decisions_per_minute: f64,
    pub ecosystem_health: f64,
    pub system_performance: f64,
    pub player_trends: Vec<TrendPoint>,
    pub ai_trends: Vec<TrendPoint>,
    pub ecosystem_trends: Vec<TrendPoint>,
    pub performance_trends: Vec<TrendPoint>,
    pub recent_anomalies: Vec<SystemAnomaly>,
}

impl RealtimeDashboardData {
    pub fn new() -> Self {
        Self {
            generated_at: Utc::now(),
            current_players: 0,
            ai_decisions_per_minute: 0.0,
            ecosystem_health: 0.0,
            system_performance: 0.0,
            player_trends: Vec::new(),
            ai_trends: Vec::new(),
            ecosystem_trends: Vec::new(),
            performance_trends: Vec::new(),
            recent_anomalies: Vec::new(),
        }
    }

    pub fn is_fresh(&self, max_age_seconds: u32) -> bool {
        let age = Utc::now() - self.generated_at;
        age.num_seconds() < max_age_seconds as i64
    }
}

/// Trend data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// System anomaly detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAnomaly {
    pub id: Uuid,
    pub detected_at: DateTime<Utc>,
    pub system: String,
    pub anomaly_type: String,
    pub description: String,
    pub severity: f64,
    pub affected_metrics: Vec<String>,
    pub potential_causes: Vec<String>,
    pub recommended_actions: Vec<String>,
}

/// Anomaly detection configuration
#[derive(Debug, Clone)]
pub struct AnomalyDetectionConfig {
    pub sensitivity: f64,
    pub time_window_hours: u32,
    pub min_severity_threshold: f64,
}

impl Default for AnomalyDetectionConfig {
    fn default() -> Self {
        Self {
            sensitivity: 0.8,
            time_window_hours: 24,
            min_severity_threshold: 0.5,
        }
    }
}

/// Predictive insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveInsight {
    pub id: Uuid,
    pub generated_at: DateTime<Utc>,
    pub system: String,
    pub insight_type: String,
    pub prediction: String,
    pub confidence: f64,
    pub time_horizon_hours: u32,
    pub impact_assessment: String,
    pub supporting_data: HashMap<String, serde_json::Value>,
}

/// Prediction configuration
#[derive(Debug, Clone)]
pub struct PredictionConfig {
    pub time_horizon_hours: u32,
    pub confidence_threshold: f64,
    pub include_uncertainty: bool,
    pub prediction_models: Vec<PredictionModel>,
}

impl Default for PredictionConfig {
    fn default() -> Self {
        Self {
            time_horizon_hours: 24,
            confidence_threshold: 0.7,
            include_uncertainty: true,
            prediction_models: vec![
                PredictionModel::LinearRegression,
                PredictionModel::TimeSeriesForecasting,
                PredictionModel::AnomalyProjection,
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub enum PredictionModel {
    LinearRegression,
    TimeSeriesForecasting,
    AnomalyProjection,
    NeuralNetwork,
    EnsembleMethod,
}

/// Cross-system correlations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CrossSystemCorrelations {
    pub player_ai_correlation: f64,
    pub ai_ecosystem_correlation: f64,
    pub ecosystem_performance_correlation: f64,
    pub player_performance_correlation: f64,
    pub correlation_matrix: HashMap<String, HashMap<String, f64>>,
    pub significant_correlations: Vec<CorrelationInsight>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationInsight {
    pub system_a: String,
    pub system_b: String,
    pub correlation_coefficient: f64,
    pub significance_level: f64,
    pub interpretation: String,
    pub potential_causality: Option<CausalityDirection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CausalityDirection {
    AtoB,
    BtoA,
    Bidirectional,
    Spurious,
}

/// Analytics metric aggregation
#[derive(Debug, Clone)]
pub struct MetricAggregator;

impl MetricAggregator {
    pub fn aggregate_by_time_window(
        data_points: &[TrendPoint],
        window_size_minutes: u32,
        aggregation_type: AggregationType,
    ) -> Vec<TrendPoint> {
        let mut aggregated = Vec::new();
        let window_duration = Duration::minutes(window_size_minutes as i64);
        
        if data_points.is_empty() {
            return aggregated;
        }

        let start_time = data_points[0].timestamp;
        let end_time = data_points.last().unwrap().timestamp;
        
        let mut current_window_start = start_time;
        
        while current_window_start < end_time {
            let window_end = current_window_start + window_duration;
            
            let window_points: Vec<&TrendPoint> = data_points
                .iter()
                .filter(|p| p.timestamp >= current_window_start && p.timestamp < window_end)
                .collect();
            
            if !window_points.is_empty() {
                let aggregated_value = match aggregation_type {
                    AggregationType::Average => {
                        window_points.iter().map(|p| p.value).sum::<f64>() / window_points.len() as f64
                    }
                    AggregationType::Sum => window_points.iter().map(|p| p.value).sum::<f64>(),
                    AggregationType::Min => window_points.iter().map(|p| p.value).fold(f64::INFINITY, f64::min),
                    AggregationType::Max => window_points.iter().map(|p| p.value).fold(f64::NEG_INFINITY, f64::max),
                    AggregationType::Median => {
                        let mut values: Vec<f64> = window_points.iter().map(|p| p.value).collect();
                        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        let mid = values.len() / 2;
                        if values.len() % 2 == 0 {
                            (values[mid - 1] + values[mid]) / 2.0
                        } else {
                            values[mid]
                        }
                    }
                    AggregationType::Count => window_points.len() as f64,
                };
                
                aggregated.push(TrendPoint {
                    timestamp: current_window_start + window_duration / 2,
                    value: aggregated_value,
                    metadata: Some(HashMap::from([
                        ("window_start".to_string(), serde_json::to_value(current_window_start).unwrap()),
                        ("window_end".to_string(), serde_json::to_value(window_end).unwrap()),
                        ("sample_count".to_string(), serde_json::Value::Number(window_points.len().into())),
                    ])),
                });
            }
            
            current_window_start = window_end;
        }
        
        aggregated
    }

    pub fn calculate_moving_average(
        data_points: &[TrendPoint],
        window_size: usize,
    ) -> Vec<TrendPoint> {
        if data_points.len() < window_size {
            return Vec::new();
        }

        let mut moving_averages = Vec::new();
        
        for i in window_size - 1..data_points.len() {
            let window_sum: f64 = data_points[i - window_size + 1..=i]
                .iter()
                .map(|p| p.value)
                .sum();
            let average = window_sum / window_size as f64;
            
            moving_averages.push(TrendPoint {
                timestamp: data_points[i].timestamp,
                value: average,
                metadata: Some(HashMap::from([
                    ("window_size".to_string(), serde_json::Value::Number(window_size.into())),
                    ("raw_value".to_string(), serde_json::Value::Number(
                        serde_json::Number::from_f64(data_points[i].value).unwrap()
                    )),
                ])),
            });
        }
        
        moving_averages
    }

    pub fn detect_trends(data_points: &[TrendPoint]) -> TrendAnalysis {
        if data_points.len() < 2 {
            return TrendAnalysis::default();
        }

        // Calculate linear regression
        let n = data_points.len() as f64;
        let x_values: Vec<f64> = (0..data_points.len()).map(|i| i as f64).collect();
        let y_values: Vec<f64> = data_points.iter().map(|p| p.value).collect();
        
        let x_mean = x_values.iter().sum::<f64>() / n;
        let y_mean = y_values.iter().sum::<f64>() / n;
        
        let numerator: f64 = x_values
            .iter()
            .zip(y_values.iter())
            .map(|(x, y)| (x - x_mean) * (y - y_mean))
            .sum();
        
        let denominator: f64 = x_values
            .iter()
            .map(|x| (x - x_mean).powi(2))
            .sum();
        
        let slope = if denominator != 0.0 { numerator / denominator } else { 0.0 };
        let intercept = y_mean - slope * x_mean;
        
        // Calculate R-squared
        let ss_res: f64 = x_values
            .iter()
            .zip(y_values.iter())
            .map(|(x, y)| {
                let predicted = slope * x + intercept;
                (y - predicted).powi(2)
            })
            .sum();
        
        let ss_tot: f64 = y_values
            .iter()
            .map(|y| (y - y_mean).powi(2))
            .sum();
        
        let r_squared = if ss_tot != 0.0 { 1.0 - (ss_res / ss_tot) } else { 0.0 };
        
        // Determine trend direction
        let trend_direction = if slope > 0.1 {
            TrendDirection::Increasing
        } else if slope < -0.1 {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        };
        
        // Calculate volatility
        let volatility = if y_values.len() > 1 {
            let mean = y_mean;
            let variance = y_values.iter().map(|y| (y - mean).powi(2)).sum::<f64>() / (n - 1.0);
            variance.sqrt()
        } else {
            0.0
        };
        
        TrendAnalysis {
            direction: trend_direction,
            slope,
            r_squared,
            volatility,
            confidence: r_squared,
            start_value: data_points.first().unwrap().value,
            end_value: data_points.last().unwrap().value,
            min_value: y_values.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
            max_value: y_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AggregationType {
    Average,
    Sum,
    Min,
    Max,
    Median,
    Count,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub direction: TrendDirection,
    pub slope: f64,
    pub r_squared: f64,
    pub volatility: f64,
    pub confidence: f64,
    pub start_value: f64,
    pub end_value: f64,
    pub min_value: f64,
    pub max_value: f64,
}

impl Default for TrendAnalysis {
    fn default() -> Self {
        Self {
            direction: TrendDirection::Stable,
            slope: 0.0,
            r_squared: 0.0,
            volatility: 0.0,
            confidence: 0.0,
            start_value: 0.0,
            end_value: 0.0,
            min_value: 0.0,
            max_value: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
}

/// Analytics event tracking
pub struct AnalyticsEventTracker {
    events: Vec<AnalyticsEvent>,
}

impl AnalyticsEventTracker {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    pub fn track_event(&mut self, event: AnalyticsEvent) {
        self.events.push(event);
    }

    pub fn get_events_by_type(&self, event_type: &str) -> Vec<&AnalyticsEvent> {
        self.events
            .iter()
            .filter(|e| e.event_type == event_type)
            .collect()
    }

    pub fn get_events_in_range(&self, time_range: &TimeRange) -> Vec<&AnalyticsEvent> {
        self.events
            .iter()
            .filter(|e| e.timestamp >= time_range.start && e.timestamp <= time_range.end)
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub system: String,
    pub description: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub severity: EventSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_range_creation() {
        let now = Utc::now();
        let start = now - Duration::hours(1);
        let range = TimeRange::new(start, now);
        
        assert_eq!(range.start, start);
        assert_eq!(range.end, now);
        assert_eq!(range.duration(), Duration::hours(1));
    }

    #[test]
    fn test_time_range_presets() {
        let last_hour = TimeRange::last_hour();
        let last_day = TimeRange::last_day();
        let last_week = TimeRange::last_week();
        
        assert!(last_hour.duration() <= Duration::hours(1) + Duration::seconds(1));
        assert!(last_day.duration() <= Duration::days(1) + Duration::seconds(1));
        assert!(last_week.duration() <= Duration::weeks(1) + Duration::seconds(1));
    }

    #[test]
    fn test_dashboard_data_freshness() {
        let mut data = RealtimeDashboardData::new();
        assert!(data.is_fresh(3600)); // Fresh for 1 hour
        
        data.generated_at = Utc::now() - Duration::hours(2);
        assert!(!data.is_fresh(3600)); // Not fresh after 2 hours
    }

    #[test]
    fn test_metric_aggregation() {
        let data_points = vec![
            TrendPoint {
                timestamp: Utc::now(),
                value: 10.0,
                metadata: None,
            },
            TrendPoint {
                timestamp: Utc::now() + Duration::minutes(1),
                value: 20.0,
                metadata: None,
            },
            TrendPoint {
                timestamp: Utc::now() + Duration::minutes(2),
                value: 30.0,
                metadata: None,
            },
        ];

        let aggregated = MetricAggregator::aggregate_by_time_window(
            &data_points,
            5,
            AggregationType::Average,
        );

        assert!(!aggregated.is_empty());
    }

    #[test]
    fn test_trend_detection() {
        let data_points = vec![
            TrendPoint { timestamp: Utc::now(), value: 10.0, metadata: None },
            TrendPoint { timestamp: Utc::now() + Duration::minutes(1), value: 15.0, metadata: None },
            TrendPoint { timestamp: Utc::now() + Duration::minutes(2), value: 20.0, metadata: None },
            TrendPoint { timestamp: Utc::now() + Duration::minutes(3), value: 25.0, metadata: None },
        ];

        let trend = MetricAggregator::detect_trends(&data_points);
        assert!(matches!(trend.direction, TrendDirection::Increasing));
        assert!(trend.slope > 0.0);
    }

    #[test]
    fn test_moving_average() {
        let data_points = vec![
            TrendPoint { timestamp: Utc::now(), value: 10.0, metadata: None },
            TrendPoint { timestamp: Utc::now() + Duration::minutes(1), value: 20.0, metadata: None },
            TrendPoint { timestamp: Utc::now() + Duration::minutes(2), value: 30.0, metadata: None },
            TrendPoint { timestamp: Utc::now() + Duration::minutes(3), value: 40.0, metadata: None },
        ];

        let moving_avg = MetricAggregator::calculate_moving_average(&data_points, 2);
        assert_eq!(moving_avg.len(), 3);
        assert_eq!(moving_avg[0].value, 15.0); // Average of 10 and 20
    }

    #[test]
    fn test_analytics_event_tracker() {
        let mut tracker = AnalyticsEventTracker::new();
        let event = AnalyticsEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: "test_event".to_string(),
            system: "test_system".to_string(),
            description: "Test event".to_string(),
            metadata: HashMap::new(),
            severity: EventSeverity::Info,
        };

        tracker.track_event(event.clone());
        let events = tracker.get_events_by_type("test_event");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].id, event.id);
    }
}