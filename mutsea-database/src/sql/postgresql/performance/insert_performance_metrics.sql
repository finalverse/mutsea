-- mutsea-database/src/sql/postgresql/performance/insert_performance_metrics.sql
INSERT INTO performance_metrics (
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
) VALUES (
    :id,
    :timestamp,
    :component_name,
    :metric_type,
    :metric_value,
    :unit_of_measure,
    :context_data,
    :threshold_data,
    :alert_level,
    :metadata,
    NOW(),
    NOW()
) RETURNING id, created_at;