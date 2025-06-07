// mutsea-database/src/utils/parameter_binding.rs

use crate::error::{DatabaseError, Result};
use serde_json::Value;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Parameter binding utilities for SQL queries
#[derive(Debug, Clone)]
pub struct ParameterBinder {
    parameters: HashMap<String, ParameterValue>,
}

#[derive(Debug, Clone)]
pub enum ParameterValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Uuid(Uuid),
    DateTime(DateTime<Utc>),
    Json(Value),
    Binary(Vec<u8>),
    Null,
}

impl ParameterBinder {
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
        }
    }

    pub fn bind_string<S: Into<String>>(&mut self, key: S, value: S) -> &mut Self {
        self.parameters.insert(key.into(), ParameterValue::String(value.into()));
        self
    }

    pub fn bind_i64<S: Into<String>>(&mut self, key: S, value: i64) -> &mut Self {
        self.parameters.insert(key.into(), ParameterValue::Integer(value));
        self
    }

    pub fn bind_f64<S: Into<String>>(&mut self, key: S, value: f64) -> &mut Self {
        self.parameters.insert(key.into(), ParameterValue::Float(value));
        self
    }

    pub fn bind_bool<S: Into<String>>(&mut self, key: S, value: bool) -> &mut Self {
        self.parameters.insert(key.into(), ParameterValue::Boolean(value));
        self
    }

    pub fn bind_uuid<S: Into<String>>(&mut self, key: S, value: Uuid) -> &mut Self {
        self.parameters.insert(key.into(), ParameterValue::Uuid(value));
        self
    }

    pub fn bind_datetime<S: Into<String>>(&mut self, key: S, value: DateTime<Utc>) -> &mut Self {
        self.parameters.insert(key.into(), ParameterValue::DateTime(value));
        self
    }

    pub fn bind_json<S: Into<String>>(&mut self, key: S, value: Value) -> &mut Self {
        self.parameters.insert(key.into(), ParameterValue::Json(value));
        self
    }

    pub fn bind_binary<S: Into<String>>(&mut self, key: S, value: Vec<u8>) -> &mut Self {
        self.parameters.insert(key.into(), ParameterValue::Binary(value));
        self
    }

    pub fn bind_null<S: Into<String>>(&mut self, key: S) -> &mut Self {
        self.parameters.insert(key.into(), ParameterValue::Null);
        self
    }

    pub fn get_parameter(&self, key: &str) -> Option<&ParameterValue> {
        self.parameters.get(key)
    }

    pub fn get_parameters(&self) -> &HashMap<String, ParameterValue> {
        &self.parameters
    }

    pub fn clear(&mut self) {
        self.parameters.clear();
    }

    /// Replace named parameters in SQL with indexed parameters
    pub fn replace_named_parameters(&self, sql: &str) -> Result<(String, Vec<&ParameterValue>)> {
        let mut result_sql = sql.to_string();
        let mut ordered_params = Vec::new();
        let mut param_index = 1;

        // Find all named parameters in the format :param_name
        let re = regex::Regex::new(r":([a-zA-Z_][a-zA-Z0-9_]*)")
            .map_err(|e| DatabaseError::QueryError(format!("Regex error: {}", e)))?;

        let mut replacements = Vec::new();
        
        for cap in re.captures_iter(sql) {
            let full_match = cap.get(0).unwrap();
            let param_name = cap.get(1).unwrap().as_str();
            
            let param_value = self.parameters.get(param_name)
                .ok_or_else(|| DatabaseError::QueryError(
                    format!("Parameter '{}' not found", param_name)
                ))?;

            replacements.push((full_match.start(), full_match.end(), param_index, param_value));
            param_index += 1;
        }

        // Apply replacements in reverse order to maintain positions
        replacements.reverse();
        for (start, end, index, param_value) in replacements {
            result_sql.replace_range(start..end, &format!("${}", index));
            ordered_params.insert(0, param_value);
        }

        ordered_params.reverse();
        Ok((result_sql, ordered_params))
    }

    /// Convert to PostgreSQL-style parameters
    pub fn to_postgres_params(&self, sql: &str) -> Result<(String, Vec<&(dyn tokio_postgres::types::ToSql + Sync)>)> {
        let (indexed_sql, param_values) = self.replace_named_parameters(sql)?;
        
        // This would need actual implementation based on the specific database driver
        // For now, return the indexed SQL and empty params
        Ok((indexed_sql, vec![]))
    }

    /// Batch parameter binding for bulk operations
    pub fn bind_batch<T>(&mut self, items: &[T]) -> Result<Vec<ParameterBinder>>
    where
        T: BatchBindable,
    {
        items.iter()
            .map(|item| item.to_parameter_binder())
            .collect()
    }
}

impl Default for ParameterBinder {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for types that can be converted to parameter binders
pub trait BatchBindable {
    fn to_parameter_binder(&self) -> Result<ParameterBinder>;
}

/// Builder pattern for complex parameter binding
#[derive(Debug)]
pub struct ParameterBinderBuilder {
    binder: ParameterBinder,
}

impl ParameterBinderBuilder {
    pub fn new() -> Self {
        Self {
            binder: ParameterBinder::new(),
        }
    }

    pub fn with_string<S: Into<String>>(mut self, key: S, value: S) -> Self {
        self.binder.bind_string(key, value);
        self
    }

    pub fn with_i64<S: Into<String>>(mut self, key: S, value: i64) -> Self {
        self.binder.bind_i64(key, value);
        self
    }

    pub fn with_f64<S: Into<String>>(mut self, key: S, value: f64) -> Self {
        self.binder.bind_f64(key, value);
        self
    }

    pub fn with_bool<S: Into<String>>(mut self, key: S, value: bool) -> Self {
        self.binder.bind_bool(key, value);
        self
    }

    pub fn with_uuid<S: Into<String>>(mut self, key: S, value: Uuid) -> Self {
        self.binder.bind_uuid(key, value);
        self
    }

    pub fn with_datetime<S: Into<String>>(mut self, key: S, value: DateTime<Utc>) -> Self {
        self.binder.bind_datetime(key, value);
        self
    }

    pub fn with_json<S: Into<String>>(mut self, key: S, value: Value) -> Self {
        self.binder.bind_json(key, value);
        self
    }

    pub fn with_optional_string<S: Into<String>>(mut self, key: S, value: Option<S>) -> Self {
        match value {
            Some(v) => self.binder.bind_string(key, v),
            None => self.binder.bind_null(key),
        };
        self
    }

    pub fn build(self) -> ParameterBinder {
        self.binder
    }
}

impl Default for ParameterBinderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for common parameter binding patterns
pub mod utils {
    use super::*;
    use crate::models::*;

    pub fn bind_world_state_params(world_state: &world_state::WorldState) -> ParameterBinder {
        ParameterBinderBuilder::new()
            .with_uuid("id", world_state.id)
            .with_datetime("timestamp", world_state.timestamp)
            .with_json("simulation_state", world_state.simulation_state.clone())
            .with_json("environmental_state", world_state.environmental_state.clone())
            .with_json("ecosystem_state", world_state.ecosystem_state.clone())
            .build()
    }

    pub fn bind_player_behavior_params(behavior: &player_behavior::PlayerBehavior) -> ParameterBinder {
        ParameterBinderBuilder::new()
            .with_uuid("id", behavior.id)
            .with_uuid("player_id", behavior.player_id)
            .with_datetime("timestamp", behavior.timestamp)
            .with_string("action_type", &behavior.action_type)
            .with_json("action_data", behavior.action_data.clone())
            .with_json("context", behavior.context.clone())
            .build()
    }

    pub fn bind_ai_decision_params(decision: &ai_decision::AIDecision) -> ParameterBinder {
        ParameterBinderBuilder::new()
            .with_uuid("id", decision.id)
            .with_datetime("timestamp", decision.timestamp)
            .with_string("decision_type", &decision.decision_type)
            .with_json("input_data", decision.input_data.clone())
            .with_json("output_data", decision.output_data.clone())
            .with_f64("confidence_score", decision.confidence_score)
            .with_json("context", decision.context.clone())
            .build()
    }

    pub fn bind_npc_state_params(npc_state: &npc_state::NPCState) -> ParameterBinder {
        ParameterBinderBuilder::new()
            .with_uuid("id", npc_state.id)
            .with_uuid("npc_id", npc_state.npc_id)
            .with_datetime("timestamp", npc_state.timestamp)
            .with_json("physical_state", serde_json::to_value(&npc_state.physical_state).unwrap_or_default())
            .with_json("cognitive_state", serde_json::to_value(&npc_state.cognitive_state).unwrap_or_default())
            .with_json("behavioral_state", serde_json::to_value(&npc_state.behavioral_state).unwrap_or_default())
            .with_json("social_state", serde_json::to_value(&npc_state.social_state).unwrap_or_default())
            .with_json("health_state", serde_json::to_value(&npc_state.health_state).unwrap_or_default())
            .with_json("economic_state", serde_json::to_value(&npc_state.economic_state).unwrap_or_default())
            .build()
    }

    pub fn bind_ecosystem_state_params(ecosystem: &ecosystem_state::EcosystemState) -> ParameterBinder {
        ParameterBinderBuilder::new()
            .with_uuid("id", ecosystem.id)
            .with_datetime("timestamp", ecosystem.timestamp)
            .with_json("environmental_data", serde_json::to_value(&ecosystem.environmental).unwrap_or_default())
            .with_json("population_data", ecosystem.population_data.clone())
            .with_json("resource_data", ecosystem.resource_data.clone())
            .with_json("interaction_data", ecosystem.interaction_data.clone())
            .build()
    }

    pub fn bind_performance_metrics_params(metrics: &performance_metrics::PerformanceMetrics) -> ParameterBinder {
        ParameterBinderBuilder::new()
            .with_uuid("id", metrics.id)
            .with_datetime("timestamp", metrics.timestamp)
            .with_json("core_metrics", serde_json::to_value(&metrics.core_metrics).unwrap_or_default())
            .with_json("system_metrics", serde_json::to_value(&metrics.system_metrics).unwrap_or_default())
            .with_json("ai_metrics", serde_json::to_value(&metrics.ai_metrics).unwrap_or_default())
            .with_json("health_metrics", serde_json::to_value(&metrics.health_metrics).unwrap_or_default())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_binding() {
        let mut binder = ParameterBinder::new();
        binder.bind_string("name", "test")
              .bind_i64("count", 42)
              .bind_bool("active", true);

        assert_eq!(binder.parameters.len(), 3);
        
        match binder.get_parameter("name") {
            Some(ParameterValue::String(s)) => assert_eq!(s, "test"),
            _ => panic!("Expected string parameter"),
        }
    }

    #[test]
    fn test_named_parameter_replacement() {
        let binder = ParameterBinderBuilder::new()
            .with_string("name", "test")
            .with_i64("id", 123)
            .build();

        let sql = "SELECT * FROM users WHERE name = :name AND id = :id";
        let (result_sql, params) = binder.replace_named_parameters(sql).unwrap();
        
        assert_eq!(result_sql, "SELECT * FROM users WHERE name = $1 AND id = $2");
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_builder_pattern() {
        let binder = ParameterBinderBuilder::new()
            .with_string("test", "value")
            .with_i64("number", 42)
            .build();

        assert_eq!(binder.parameters.len(), 2);
    }
}