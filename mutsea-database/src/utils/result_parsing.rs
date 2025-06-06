// mutsea-database/src/utils/result_parsing.rs

use crate::error::{DatabaseError, DatabaseResult};
use crate::models::*;
use crate::traits::query_builder::QueryParam;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;
use uuid::Uuid;

/// Trait for parsing database row results into structured types
pub trait ResultParser<T> {
    fn parse_row(&self, row: &DatabaseRow) -> DatabaseResult<T>;
    fn parse_rows(&self, rows: Vec<DatabaseRow>) -> DatabaseResult<Vec<T>> {
        rows.into_iter()
            .map(|row| self.parse_row(&row))
            .collect()
    }
}

/// Represents a single database row with column access
#[derive(Debug, Clone)]
pub struct DatabaseRow {
    columns: HashMap<String, DatabaseValue>,
}

impl DatabaseRow {
    pub fn new(columns: HashMap<String, DatabaseValue>) -> Self {
        Self { columns }
    }

    /// Get a value by column name
    pub fn get<T>(&self, column: &str) -> DatabaseResult<T>
    where
        T: TryFrom<DatabaseValue>,
        T::Error: Into<DatabaseError>,
    {
        self.columns
            .get(column)
            .ok_or_else(|| DatabaseError::ColumnNotFound(column.to_string()))
            .and_then(|value| T::try_from(value.clone()).map_err(Into::into))
    }

    /// Get an optional value by column name
    pub fn get_optional<T>(&self, column: &str) -> DatabaseResult<Option<T>>
    where
        T: TryFrom<DatabaseValue>,
        T::Error: Into<DatabaseError>,
    {
        match self.columns.get(column) {
            Some(DatabaseValue::Null) | None => Ok(None),
            Some(value) => T::try_from(value.clone())
                .map(Some)
                .map_err(Into::into),
        }
    }

    /// Get a value with a default fallback
    pub fn get_or_default<T>(&self, column: &str, default: T) -> T
    where
        T: TryFrom<DatabaseValue>,
        T::Error: Into<DatabaseError>,
    {
        self.get(column).unwrap_or(default)
    }

    /// Check if column exists and is not null
    pub fn has_value(&self, column: &str) -> bool {
        matches!(
            self.columns.get(column),
            Some(value) if !matches!(value, DatabaseValue::Null)
        )
    }

    /// Get all column names
    pub fn column_names(&self) -> Vec<&String> {
        self.columns.keys().collect()
    }
}

/// Represents a database value that can be converted to Rust types
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Uuid(Uuid),
    Json(JsonValue),
    Binary(Vec<u8>),
    DateTime(DateTime<Utc>),
    Null,
}

impl DatabaseValue {
    /// Convert to query parameter
    pub fn to_query_param(&self) -> QueryParam {
        match self {
            DatabaseValue::String(s) => QueryParam::String(s.clone()),
            DatabaseValue::Integer(i) => QueryParam::Integer(*i),
            DatabaseValue::Float(f) => QueryParam::Float(*f),
            DatabaseValue::Boolean(b) => QueryParam::Boolean(*b),
            DatabaseValue::Uuid(u) => QueryParam::Uuid(*u),
            DatabaseValue::Json(j) => QueryParam::Json(j.clone()),
            DatabaseValue::Binary(b) => QueryParam::Binary(b.clone()),
            DatabaseValue::DateTime(_) => QueryParam::String(self.to_string()),
            DatabaseValue::Null => QueryParam::Null,
        }
    }
}

impl std::fmt::Display for DatabaseValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseValue::String(s) => write!(f, "{}", s),
            DatabaseValue::Integer(i) => write!(f, "{}", i),
            DatabaseValue::Float(fl) => write!(f, "{}", fl),
            DatabaseValue::Boolean(b) => write!(f, "{}", b),
            DatabaseValue::Uuid(u) => write!(f, "{}", u),
            DatabaseValue::Json(j) => write!(f, "{}", j),
            DatabaseValue::Binary(b) => write!(f, "{:?}", b),
            DatabaseValue::DateTime(dt) => write!(f, "{}", dt.to_rfc3339()),
            DatabaseValue::Null => write!(f, "NULL"),
        }
    }
}

/// Conversion implementations for common Rust types
impl TryFrom<DatabaseValue> for String {
    type Error = DatabaseError;

    fn try_from(value: DatabaseValue) -> Result<Self, Self::Error> {
        match value {
            DatabaseValue::String(s) => Ok(s),
            DatabaseValue::Integer(i) => Ok(i.to_string()),
            DatabaseValue::Float(f) => Ok(f.to_string()),
            DatabaseValue::Boolean(b) => Ok(b.to_string()),
            DatabaseValue::Uuid(u) => Ok(u.to_string()),
            DatabaseValue::DateTime(dt) => Ok(dt.to_rfc3339()),
            other => Err(DatabaseError::TypeConversion {
                from: format!("{:?}", other),
                to: "String".to_string(),
            }),
        }
    }
}

impl TryFrom<DatabaseValue> for i64 {
    type Error = DatabaseError;

    fn try_from(value: DatabaseValue) -> Result<Self, Self::Error> {
        match value {
            DatabaseValue::Integer(i) => Ok(i),
            DatabaseValue::Float(f) => Ok(f as i64),
            DatabaseValue::String(s) => s.parse().map_err(|_| DatabaseError::TypeConversion {
                from: format!("String({})", s),
                to: "i64".to_string(),
            }),
            other => Err(DatabaseError::TypeConversion {
                from: format!("{:?}", other),
                to: "i64".to_string(),
            }),
        }
    }
}

impl TryFrom<DatabaseValue> for f64 {
    type Error = DatabaseError;

    fn try_from(value: DatabaseValue) -> Result<Self, Self::Error> {
        match value {
            DatabaseValue::Float(f) => Ok(f),
            DatabaseValue::Integer(i) => Ok(i as f64),
            DatabaseValue::String(s) => s.parse().map_err(|_| DatabaseError::TypeConversion {
                from: format!("String({})", s),
                to: "f64".to_string(),
            }),
            other => Err(DatabaseError::TypeConversion {
                from: format!("{:?}", other),
                to: "f64".to_string(),
            }),
        }
    }
}

impl TryFrom<DatabaseValue> for bool {
    type Error = DatabaseError;

    fn try_from(value: DatabaseValue) -> Result<Self, Self::Error> {
        match value {
            DatabaseValue::Boolean(b) => Ok(b),
            DatabaseValue::Integer(i) => Ok(i != 0),
            DatabaseValue::String(s) => match s.to_lowercase().as_str() {
                "true" | "t" | "yes" | "y" | "1" => Ok(true),
                "false" | "f" | "no" | "n" | "0" => Ok(false),
                _ => Err(DatabaseError::TypeConversion {
                    from: format!("String({})", s),
                    to: "bool".to_string(),
                }),
            },
            other => Err(DatabaseError::TypeConversion {
                from: format!("{:?}", other),
                to: "bool".to_string(),
            }),
        }
    }
}

impl TryFrom<DatabaseValue> for Uuid {
    type Error = DatabaseError;

    fn try_from(value: DatabaseValue) -> Result<Self, Self::Error> {
        match value {
            DatabaseValue::Uuid(u) => Ok(u),
            DatabaseValue::String(s) => Uuid::from_str(&s).map_err(|_| DatabaseError::TypeConversion {
                from: format!("String({})", s),
                to: "Uuid".to_string(),
            }),
            other => Err(DatabaseError::TypeConversion {
                from: format!("{:?}", other),
                to: "Uuid".to_string(),
            }),
        }
    }
}

impl TryFrom<DatabaseValue> for DateTime<Utc> {
    type Error = DatabaseError;

    fn try_from(value: DatabaseValue) -> Result<Self, Self::Error> {
        match value {
            DatabaseValue::DateTime(dt) => Ok(dt),
            DatabaseValue::String(s) => DateTime::parse_from_rfc3339(&s)
                .map(|dt| dt.with_timezone(&Utc))
                .map_err(|_| DatabaseError::TypeConversion {
                    from: format!("String({})", s),
                    to: "DateTime<Utc>".to_string(),
                }),
            other => Err(DatabaseError::TypeConversion {
                from: format!("{:?}", other),
                to: "DateTime<Utc>".to_string(),
            }),
        }
    }
}

impl TryFrom<DatabaseValue> for JsonValue {
    type Error = DatabaseError;

    fn try_from(value: DatabaseValue) -> Result<Self, Self::Error> {
        match value {
            DatabaseValue::Json(j) => Ok(j),
            DatabaseValue::String(s) => serde_json::from_str(&s).map_err(|e| DatabaseError::TypeConversion {
                from: format!("String({})", s),
                to: format!("Json ({})", e),
            }),
            other => Err(DatabaseError::TypeConversion {
                from: format!("{:?}", other),
                to: "Json".to_string(),
            }),
        }
    }
}

impl TryFrom<DatabaseValue> for Vec<u8> {
    type Error = DatabaseError;

    fn try_from(value: DatabaseValue) -> Result<Self, Self::Error> {
        match value {
            DatabaseValue::Binary(b) => Ok(b),
            DatabaseValue::String(s) => Ok(s.into_bytes()),
            other => Err(DatabaseError::TypeConversion {
                from: format!("{:?}", other),
                to: "Vec<u8>".to_string(),
            }),
        }
    }
}

/// Specialized result parsers for AI-driven components

/// Parser for AI decision results
pub struct AIDecisionParser;

impl ResultParser<AIDecision> for AIDecisionParser {
    fn parse_row(&self, row: &DatabaseRow) -> DatabaseResult<AIDecision> {
        Ok(AIDecision {
            id: row.get("id")?,
            decision_type: row.get::<String>("decision_type")?.parse().map_err(|_| {
                DatabaseError::TypeConversion {
                    from: "String".to_string(),
                    to: "AIDecisionType".to_string(),
                }
            })?,
            context: row.get("context")?,
            confidence: row.get("confidence")?,
            outcome: row.get_optional("outcome")?,
            feedback_score: row.get_optional("feedback_score")?,
            timestamp: row.get("timestamp")?,
            ai_system_id: row.get("ai_system_id")?,
            player_id: row.get_optional("player_id")?,
            world_state_id: row.get_optional("world_state_id")?,
            parameters: row.get("parameters")?,
        })
    }
}

/// Parser for world state results
pub struct WorldStateParser;

impl ResultParser<WorldState> for WorldStateParser {
    fn parse_row(&self, row: &DatabaseRow) -> DatabaseResult<WorldState> {
        Ok(WorldState {
            id: row.get("id")?,
            region_id: row.get("region_id")?,
            state_data: row.get("state_data")?,
            timestamp: row.get("timestamp")?,
            version: row.get("version")?,
            generation_source: row.get::<String>("generation_source")?.parse().map_err(|_| {
                DatabaseError::TypeConversion {
                    from: "String".to_string(),
                    to: "GenerationSource".to_string(),
                }
            })?,
            biome_distribution: row.get("biome_distribution")?,
            resource_density: row.get("resource_density")?,
            ecosystem_health: row.get("ecosystem_health")?,
            player_impact_score: row.get_optional("player_impact_score")?,
        })
    }
}

/// Parser for player behavior results
pub struct PlayerBehaviorParser;

impl ResultParser<PlayerBehavior> for PlayerBehaviorParser {
    fn parse_row(&self, row: &DatabaseRow) -> DatabaseResult<PlayerBehavior> {
        Ok(PlayerBehavior {
            id: row.get("id")?,
            player_id: row.get("player_id")?,
            action_type: row.get::<String>("action_type")?.parse().map_err(|_| {
                DatabaseError::TypeConversion {
                    from: "String".to_string(),
                    to: "ActionType".to_string(),
                }
            })?,
            context: row.get("context")?,
            timestamp: row.get("timestamp")?,
            duration: row.get_optional::<i64>("duration_seconds")?.map(chrono::Duration::seconds),
            success: row.get("success")?,
            difficulty_level: row.get("difficulty_level")?,
            emotional_state: row.get_optional("emotional_state")?,
            social_context: row.get_optional("social_context")?,
            performance_metrics: row.get("performance_metrics")?,
        })
    }
}

/// Parser for NPC state results
pub struct NPCStateParser;

impl ResultParser<NPCState> for NPCStateParser {
    fn parse_row(&self, row: &DatabaseRow) -> DatabaseResult<NPCState> {
        Ok(NPCState {
            id: row.get("id")?,
            npc_id: row.get("npc_id")?,
            personality_traits: row.get("personality_traits")?,
            current_goals: row.get("current_goals")?,
            memory_data: row.get("memory_data")?,
            relationship_network: row.get("relationship_network")?,
            learning_progress: row.get("learning_progress")?,
            behavioral_patterns: row.get("behavioral_patterns")?,
            emotional_state: row.get("emotional_state")?,
            adaptation_history: row.get("adaptation_history")?,
            timestamp: row.get("timestamp")?,
        })
    }
}

/// Parser for ecosystem state results
pub struct EcosystemStateParser;

impl ResultParser<EcosystemState> for EcosystemStateParser {
    fn parse_row(&self, row: &DatabaseRow) -> DatabaseResult<EcosystemState> {
        Ok(EcosystemState {
            id: row.get("id")?,
            ecosystem_id: row.get("ecosystem_id")?,
            species_populations: row.get("species_populations")?,
            resource_levels: row.get("resource_levels")?,
            environmental_factors: row.get("environmental_factors")?,
            interaction_networks: row.get("interaction_networks")?,
            health_metrics: row.get("health_metrics")?,
            stability_indicators: row.get("stability_indicators")?,
            adaptation_events: row.get("adaptation_events")?,
            timestamp: row.get("timestamp")?,
        })
    }
}

/// Parser for emergent behavior results
pub struct EmergentBehaviorParser;

impl ResultParser<EmergentBehavior> for EmergentBehaviorParser {
    fn parse_row(&self, row: &DatabaseRow) -> DatabaseResult<EmergentBehavior> {
        Ok(EmergentBehavior {
            id: row.get("id")?,
            behavior_type: row.get::<String>("behavior_type")?.parse().map_err(|_| {
                DatabaseError::TypeConversion {
                    from: "String".to_string(),
                    to: "EmergentBehaviorType".to_string(),
                }
            })?,
            participants: row.get::<JsonValue>("participants")?
                .as_array()
                .ok_or_else(|| DatabaseError::TypeConversion {
                    from: "JsonValue".to_string(),
                    to: "Vec<Uuid>".to_string(),
                })?
                .iter()
                .map(|v| v.as_str()
                    .ok_or_else(|| DatabaseError::TypeConversion {
                        from: "JsonValue".to_string(),
                        to: "String".to_string(),
                    })
                    .and_then(|s| Uuid::from_str(s).map_err(|_| DatabaseError::TypeConversion {
                        from: "String".to_string(),
                        to: "Uuid".to_string(),
                    }))
                )
                .collect::<Result<Vec<_>, _>>()?,
            emergence_strength: row.get("emergence_strength")?,
            duration: row.get::<i64>("duration_seconds")?.pipe(chrono::Duration::seconds),
            impact_radius: row.get("impact_radius")?,
            novelty_score: row.get("novelty_score")?,
            context_data: row.get("context_data")?,
            detection_algorithm: row.get("detection_algorithm")?,
            timestamp: row.get("timestamp")?,
        })
    }
}

/// Parser for performance metrics results
pub struct PerformanceMetricsParser;

impl ResultParser<PerformanceMetrics> for PerformanceMetricsParser {
    fn parse_row(&self, row: &DatabaseRow) -> DatabaseResult<PerformanceMetrics> {
        Ok(PerformanceMetrics {
            id: row.get("id")?,
            component_name: row.get("component_name")?,
            metric_type: row.get::<String>("metric_type")?.parse().map_err(|_| {
                DatabaseError::TypeConversion {
                    from: "String".to_string(),
                    to: "PerformanceMetricType".to_string(),
                }
            })?,
            value: row.get("value")?,
            unit: row.get("unit")?,
            threshold: row.get_optional("threshold")?,
            is_critical: row.get("is_critical")?,
            optimization_suggestions: row.get_optional("optimization_suggestions")?,
            timestamp: row.get("timestamp")?,
        })
    }
}

/// Parser for learning data results
pub struct LearningDataParser;

impl ResultParser<LearningData> for LearningDataParser {
    fn parse_row(&self, row: &DatabaseRow) -> DatabaseResult<LearningData> {
        Ok(LearningData {
            id: row.get("id")?,
            learning_type: row.get::<String>("learning_type")?.parse().map_err(|_| {
                DatabaseError::TypeConversion {
                    from: "String".to_string(),
                    to: "LearningType".to_string(),
                }
            })?,
            input_data: row.get("input_data")?,
            target_output: row.get_optional("target_output")?,
            actual_output: row.get_optional("actual_output")?,
            reward_signal: row.get_optional("reward_signal")?,
            context_features: row.get("context_features")?,
            quality_score: row.get_optional("quality_score")?,
            is_validated: row.get("is_validated")?,
            timestamp: row.get("timestamp")?,
        })
    }
}

/// Generic result parser factory
pub struct ResultParserFactory;

impl ResultParserFactory {
    /// Create a parser for a specific type
    pub fn create_parser<T>() -> Box<dyn ResultParser<T>>
    where
        T: 'static,
    {
        // This would be implemented with type matching
        // For now, we'll use a placeholder that panics
        panic!("Parser not implemented for type")
    }

    /// Create AI decision parser
    pub fn ai_decision_parser() -> AIDecisionParser {
        AIDecisionParser
    }

    /// Create world state parser
    pub fn world_state_parser() -> WorldStateParser {
        WorldStateParser
    }

    /// Create player behavior parser
    pub fn player_behavior_parser() -> PlayerBehaviorParser {
        PlayerBehaviorParser
    }

    /// Create NPC state parser
    pub fn npc_state_parser() -> NPCStateParser {
        NPCStateParser
    }

    /// Create ecosystem state parser
    pub fn ecosystem_state_parser() -> EcosystemStateParser {
        EcosystemStateParser
    }

    /// Create emergent behavior parser
    pub fn emergent_behavior_parser() -> EmergentBehaviorParser {
        EmergentBehaviorParser
    }

    /// Create performance metrics parser
    pub fn performance_metrics_parser() -> PerformanceMetricsParser {
        PerformanceMetricsParser
    }

    /// Create learning data parser
    pub fn learning_data_parser() -> LearningDataParser {
        LearningDataParser
    }
}

/// Utility functions for result processing

/// Convert a vector of database rows to a specific type
pub fn parse_rows_to_type<T>(
    rows: Vec<DatabaseRow>,
    parser: &dyn ResultParser<T>,
) -> DatabaseResult<Vec<T>> {
    parser.parse_rows(rows)
}

/// Parse a single row to a specific type
pub fn parse_row_to_type<T>(
    row: DatabaseRow,
    parser: &dyn ResultParser<T>,
) -> DatabaseResult<T> {
    parser.parse_row(&row)
}

/// Convert query results to a hashmap grouped by a specific column
pub fn group_results_by_column<T>(
    rows: Vec<DatabaseRow>,
    group_column: &str,
    parser: &dyn ResultParser<T>,
) -> DatabaseResult<HashMap<String, Vec<T>>> {
    let mut groups: HashMap<String, Vec<T>> = HashMap::new();
    
    for row in rows {
        let group_key: String = row.get(group_column)?;
        let parsed_item = parser.parse_row(&row)?;
        
        groups.entry(group_key)
            .or_insert_with(Vec::new)
            .push(parsed_item);
    }
    
    Ok(groups)
}

/// Aggregate numeric values from parsed results
pub fn aggregate_numeric_column<T, F>(
    rows: &[DatabaseRow],
    column: &str,
    aggregator: F,
) -> DatabaseResult<f64>
where
    F: Fn(&[f64]) -> f64,
{
    let values: Result<Vec<f64>, _> = rows
        .iter()
        .map(|row| row.get::<f64>(column))
        .collect();
    
    let values = values?;
    Ok(aggregator(&values))
}

/// Helper trait for pipeline operations
trait PipeExt<T> {
    fn pipe<U, F>(self, f: F) -> U
    where
        F: FnOnce(T) -> U;
}

impl<T> PipeExt<T> for T {
    fn pipe<U, F>(self, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        f(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::DatabaseError;

    #[test]
    fn test_database_value_string_conversion() {
        let value = DatabaseValue::String("test".to_string());
        let converted: String = value.try_into().unwrap();
        assert_eq!(converted, "test");
    }

    #[test]
    fn test_database_value_integer_conversion() {
        let value = DatabaseValue::Integer(42);
        let converted: i64 = value.try_into().unwrap();
        assert_eq!(converted, 42);
    }

    #[test]
    fn test_database_value_float_conversion() {
        let value = DatabaseValue::Float(3.14);
        let converted: f64 = value.try_into().unwrap();
        assert_eq!(converted, 3.14);
    }

    #[test]
    fn test_database_value_boolean_conversion() {
        let value = DatabaseValue::Boolean(true);
        let converted: bool = value.try_into().unwrap();
        assert_eq!(converted, true);
    }

    #[test]
    fn test_database_row_get_value() {
        let mut columns = HashMap::new();
        columns.insert("test_col".to_string(), DatabaseValue::String("test_value".to_string()));
        let row = DatabaseRow::new(columns);
        
        let value: String = row.get("test_col").unwrap();
        assert_eq!(value, "test_value");
    }

    #[test]
    fn test_database_row_get_optional_value() {
        let mut columns = HashMap::new();
        columns.insert("existing_col".to_string(), DatabaseValue::String("value".to_string()));
        columns.insert("null_col".to_string(), DatabaseValue::Null);
        let row = DatabaseRow::new(columns);
        
        let existing: Option<String> = row.get_optional("existing_col").unwrap();
        assert_eq!(existing, Some("value".to_string()));
        
        let null_value: Option<String> = row.get_optional("null_col").unwrap();
        assert_eq!(null_value, None);
        
        let missing: Option<String> = row.get_optional("missing_col").unwrap();
        assert_eq!(missing, None);
    }

    #[test]
    fn test_invalid_type_conversion() {
        let value = DatabaseValue::String("not_a_number".to_string());
        let result: Result<i64, DatabaseError> = value.try_into();
        assert!(result.is_err());
    }
}