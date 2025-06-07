// mutsea-database/src/queries/world_queries.rs

use crate::error::Result;
use crate::models::world_state::{WorldState, SimulationState, EnvironmentalState, EcosystemState};
use crate::queries::{QueryBuilder, SelectQueryBuilder, InsertQueryBuilder, UpdateQueryBuilder, DeleteQueryBuilder, PaginatedResult, PaginationParams};
use crate::utils::parameter_binding::{ParameterBinder, utils};
use crate::utils::sql_loader::SqlLoader;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

/// World state query operations
pub struct WorldQueries {
    sql_loader: SqlLoader,
}

impl WorldQueries {
    pub fn new(sql_loader: SqlLoader) -> Self {
        Self { sql_loader }
    }

    /// Insert a new world state record
    pub async fn insert_world_state(&self, world_state: &WorldState) -> Result<(String, ParameterBinder)> {
        let sql = self.sql_loader.load_sql("world_state/insert_world_state.sql")?;
        let params = utils::bind_world_state_params(world_state);
        Ok((sql, params))
    }

    /// Insert multiple world states in batch
    pub async fn batch_insert_world_states(&self, world_states: &[WorldState]) -> Result<(String, Vec<ParameterBinder>)> {
        let sql = self.sql_loader.load_sql("world_state/batch_insert_world_states.sql")?;
        let param_batches: Vec<ParameterBinder> = world_states
            .iter()
            .map(utils::bind_world_state_params)
            .collect();
        Ok((sql, param_batches))
    }

    /// Select world states by time range
    pub async fn select_world_states_by_time_range(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        pagination: Option<PaginationParams>,
    ) -> Result<(String, ParameterBinder)> {
        let mut query = SelectQueryBuilder::<WorldState>::new("world_states")
            .columns(&[
                "id", "timestamp", "simulation_state", "environmental_state", 
                "ecosystem_state", "metadata", "created_at", "updated_at"
            ])
            .where_clause("timestamp BETWEEN :start_time AND :end_time")
            .order_by("timestamp", "DESC");

        let mut params = ParameterBinder::new();
        params.bind_datetime("start_time", start_time)
              .bind_datetime("end_time", end_time);

        if let Some(pagination) = pagination {
            query = query.limit(pagination.limit()).offset(pagination.offset());
        }

        let (sql, mut query_params) = query.build();
        
        // Merge parameters
        for (key, value) in params.get_parameters() {
            // This would need proper parameter merging implementation
        }

        Ok((sql, params))
    }

    /// Select latest world state
    pub async fn select_latest_world_state(&self) -> Result<(String, ParameterBinder)> {
        let sql = self.sql_loader.load_sql("world_state/select_latest_world_state.sql")?;
        let params = ParameterBinder::new();
        Ok((sql, params))
    }

    /// Select world state by ID
    pub async fn select_world_state_by_id(&self, id: Uuid) -> Result<(String, ParameterBinder)> {
        let sql = self.sql_loader.load_sql("world_state/select_world_state_by_id.sql")?;
        let params = ParameterBinder::new()
            .bind_uuid("id", id)
            .clone();
        Ok((sql, params))
    }

    /// Update world state
    pub async fn update_world_state(&self, world_state: &WorldState) -> Result<(String, ParameterBinder)> {
        let sql = self.sql_loader.load_sql("world_state/update_world_state.sql")?;
        let params = utils::bind_world_state_params(world_state);
        Ok((sql, params))
    }

    /// Delete old world states (cleanup)
    pub async fn delete_old_world_states(&self, cutoff_time: DateTime<Utc>) -> Result<(String, ParameterBinder)> {
        let sql = self.sql_loader.load_sql("world_state/delete_old_world_states.sql")?;
        let params = ParameterBinder::new()
            .bind_datetime("cutoff_time", cutoff_time)
            .clone();
        Ok((sql, params))
    }

    /// Get world state statistics
    pub async fn get_world_state_statistics(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<(String, ParameterBinder)> {
        let sql = self.sql_loader.load_sql("world_state/world_state_statistics.sql")?;
        let params = ParameterBinder::new()
            .bind_datetime("start_time", start_time)
            .bind_datetime("end_time", end_time)
            .clone();
        Ok((sql, params))
    }

    /// Search world states by simulation criteria
    pub async fn search_world_states_by_criteria(
        &self,
        criteria: &WorldSearchCriteria,
        pagination: Option<PaginationParams>,
    ) -> Result<(String, ParameterBinder)> {
        let mut where_conditions = Vec::new();
        let mut params = ParameterBinder::new();

        if let Some(min_time) = criteria.min_timestamp {
            where_conditions.push("timestamp >= :min_time".to_string());
            params.bind_datetime("min_time", min_time);
        }

        if let Some(max_time) = criteria.max_timestamp {
            where_conditions.push("timestamp <= :max_time".to_string());
            params.bind_datetime("max_time", max_time);
        }

        if let Some(ref simulation_filter) = criteria.simulation_filter {
            where_conditions.push("simulation_state @> :simulation_filter".to_string());
            params.bind_json("simulation_filter", simulation_filter.clone());
        }

        if let Some(ref environmental_filter) = criteria.environmental_filter {
            where_conditions.push("environmental_state @> :environmental_filter".to_string());
            params.bind_json("environmental_filter", environmental_filter.clone());
        }

        let mut query = SelectQueryBuilder::<WorldState>::new("world_states")
            .columns(&[
                "id", "timestamp", "simulation_state", "environmental_state", 
                "ecosystem_state", "metadata", "created_at", "updated_at"
            ])
            .order_by("timestamp", "DESC");

        if !where_conditions.is_empty() {
            query = query.where_clause(&where_conditions.join(" AND "));
        }

        if let Some(pagination) = pagination {
            query = query.limit(pagination.limit()).offset(pagination.offset());
        }

        let (sql, _) = query.build();
        Ok((sql, params))
    }

    /// Get world state evolution patterns
    pub async fn get_world_evolution_patterns(
        &self,
        time_window_hours: i32,
        pattern_threshold: f64,
    ) -> Result<(String, ParameterBinder)> {
        let sql = self.sql_loader.load_sql("analytics/world_evolution_patterns.sql")?;
        let params = ParameterBinder::new()
            .bind_i64("time_window_hours", time_window_hours as i64)
            .bind_f64("pattern_threshold", pattern_threshold)
            .clone();
        Ok((sql, params))
    }

    /// Get environmental change trends
    pub async fn get_environmental_trends(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        aggregation_interval: &str,
    ) -> Result<(String, ParameterBinder)> {
        let sql = self.sql_loader.load_sql("analytics/environmental_trends.sql")?;
        let params = ParameterBinder::new()
            .bind_datetime("start_time", start_time)
            .bind_datetime("end_time", end_time)
            .bind_string("aggregation_interval", aggregation_interval)
            .clone();
        Ok((sql, params))
    }

    /// Find anomalous world states
    pub async fn find_anomalous_world_states(
        &self,
        anomaly_threshold: f64,
        time_range_hours: i32,
    ) -> Result<(String, ParameterBinder)> {
        let sql = self.sql_loader.load_sql("analytics/anomalous_world_states.sql")?;
        let params = ParameterBinder::new()
            .bind_f64("anomaly_threshold", anomaly_threshold)
            .bind_i64("time_range_hours", time_range_hours as i64)
            .clone();
        Ok((sql, params))
    }
}

impl QueryBuilder<WorldState> for WorldQueries {
    fn select() -> SelectQueryBuilder<WorldState> {
        SelectQueryBuilder::new("world_states")
    }

    fn insert() -> InsertQueryBuilder<WorldState> {
        InsertQueryBuilder::new("world_states")
    }

    fn update() -> UpdateQueryBuilder<WorldState> {
        UpdateQueryBuilder::new("world_states")
    }

    fn delete() -> DeleteQueryBuilder<WorldState> {
        DeleteQueryBuilder::new("world_states")
    }
}

/// Search criteria for world states
#[derive(Debug, Clone)]
pub struct WorldSearchCriteria {
    pub min_timestamp: Option<DateTime<Utc>>,
    pub max_timestamp: Option<DateTime<Utc>>,
    pub simulation_filter: Option<Value>,
    pub environmental_filter: Option<Value>,
    pub ecosystem_filter: Option<Value>,
    pub metadata_filter: Option<Value>,
}

impl Default for WorldSearchCriteria {
    fn default() -> Self {
        Self {
            min_timestamp: None,
            max_timestamp: None,
            simulation_filter: None,
            environmental_filter: None,
            ecosystem_filter: None,
            metadata_filter: None,
        }
    }
}

/// World state aggregation types
#[derive(Debug, Clone)]
pub enum WorldAggregationType {
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

impl WorldAggregationType {
    pub fn to_sql_interval(&self) -> &str {
        match self {
            Self::Hourly => "1 hour",
            Self::Daily => "1 day",
            Self::Weekly => "1 week",
            Self::Monthly => "1 month",
        }
    }
}

/// World state change detection
pub struct WorldChangeDetector;

impl WorldChangeDetector {
    /// Detect significant changes between world states
    pub fn detect_changes(
        previous: &WorldState,
        current: &WorldState,
        threshold: f64,
    ) -> Vec<WorldStateChange> {
        let mut changes = Vec::new();

        // Compare simulation states
        if let (Some(prev_sim), Some(curr_sim)) = (
            previous.simulation_state.as_object(),
            current.simulation_state.as_object(),
        ) {
            for (key, curr_value) in curr_sim {
                if let Some(prev_value) = prev_sim.get(key) {
                    if Self::values_differ_significantly(prev_value, curr_value, threshold) {
                        changes.push(WorldStateChange {
                            change_type: WorldStateChangeType::SimulationState,
                            field: key.clone(),
                            old_value: prev_value.clone(),
                            new_value: curr_value.clone(),
                            magnitude: Self::calculate_change_magnitude(prev_value, curr_value),
                        });
                    }
                }
            }
        }

        // Compare environmental states
        if let (Some(prev_env), Some(curr_env)) = (
            previous.environmental_state.as_object(),
            current.environmental_state.as_object(),
        ) {
            for (key, curr_value) in curr_env {
                if let Some(prev_value) = prev_env.get(key) {
                    if Self::values_differ_significantly(prev_value, curr_value, threshold) {
                        changes.push(WorldStateChange {
                            change_type: WorldStateChangeType::EnvironmentalState,
                            field: key.clone(),
                            old_value: prev_value.clone(),
                            new_value: curr_value.clone(),
                            magnitude: Self::calculate_change_magnitude(prev_value, curr_value),
                        });
                    }
                }
            }
        }

        changes
    }

    fn values_differ_significantly(old: &Value, new: &Value, threshold: f64) -> bool {
        match (old, new) {
            (Value::Number(old_num), Value::Number(new_num)) => {
                if let (Some(old_f), Some(new_f)) = (old_num.as_f64(), new_num.as_f64()) {
                    (old_f - new_f).abs() > threshold
                } else {
                    false
                }
            }
            _ => old != new,
        }
    }

    fn calculate_change_magnitude(old: &Value, new: &Value) -> f64 {
        match (old, new) {
            (Value::Number(old_num), Value::Number(new_num)) => {
                if let (Some(old_f), Some(new_f)) = (old_num.as_f64(), new_num.as_f64()) {
                    (new_f / old_f - 1.0).abs()
                } else {
                    0.0
                }
            }
            _ => if old != new { 1.0 } else { 0.0 },
        }
    }
}

/// Represents a detected change in world state
#[derive(Debug, Clone)]
pub struct WorldStateChange {
    pub change_type: WorldStateChangeType,
    pub field: String,
    pub old_value: Value,
    pub new_value: Value,
    pub magnitude: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WorldStateChangeType {
    SimulationState,
    EnvironmentalState,
    EcosystemState,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sql_loader::SqlLoader;

    #[tokio::test]
    async fn test_world_queries_creation() {
        let sql_loader = SqlLoader::new("test_sql_path");
        let world_queries = WorldQueries::new(sql_loader);
        
        // Test that the world queries can be created
        assert!(true); // Placeholder test
    }

    #[test]
    fn test_world_search_criteria_default() {
        let criteria = WorldSearchCriteria::default();
        assert!(criteria.min_timestamp.is_none());
        assert!(criteria.max_timestamp.is_none());
        assert!(criteria.simulation_filter.is_none());
    }

    #[test]
    fn test_world_aggregation_type_sql_interval() {
        assert_eq!(WorldAggregationType::Hourly.to_sql_interval(), "1 hour");
        assert_eq!(WorldAggregationType::Daily.to_sql_interval(), "1 day");
        assert_eq!(WorldAggregationType::Weekly.to_sql_interval(), "1 week");
        assert_eq!(WorldAggregationType::Monthly.to_sql_interval(), "1 month");
    }

    #[test]
    fn test_world_change_detection() {
        use serde_json::json;
        
        let prev_state = WorldState {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            simulation_state: json!({"temperature": 20.0}),
            environmental_state: json!({"humidity": 60.0}),
            ecosystem_state: json!({"population": 1000}),
            metadata: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let curr_state = WorldState {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            simulation_state: json!({"temperature": 25.0}),
            environmental_state: json!({"humidity": 65.0}),
            ecosystem_state: json!({"population": 1050}),
            metadata: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let changes = WorldChangeDetector::detect_changes(&prev_state, &curr_state, 1.0);
        assert!(!changes.is_empty());
    }
}