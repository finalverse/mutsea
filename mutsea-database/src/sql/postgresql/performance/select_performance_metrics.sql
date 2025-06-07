-- mutsea-database/src/sql/postgresql/performance/select_performance_metrics.sql
SELECT 
    id,
    timestamp,
    component_name,
    metric_type,
    metric_value,
    unit_of_measure,
    context_data,
    threshold_data,
    alert_level,
    metadata,
    created_at,
    updated_at
FROM performance_metrics 
WHERE component_name = :component_name
  AND metric_type = :metric_type
  AND timestamp BETWEEN :start_time AND :end_time
ORDER BY timestamp DESC
LIMIT :limit_count;