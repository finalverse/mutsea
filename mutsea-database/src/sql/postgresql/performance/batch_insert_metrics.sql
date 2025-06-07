-- mutsea-database/src/sql/postgresql/performance/batch_insert_metrics.sql
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
) VALUES 
    (:id, :timestamp, :component_name, :metric_type, :metric_value, :unit_of_measure, :context_data, :threshold_data, :alert_level, :metadata, NOW(), NOW())
ON CONFLICT (id) DO UPDATE SET
    metric_value = EXCLUDED.metric_value,
    context_data = EXCLUDED.context_data,
    threshold_data = EXCLUDED.threshold_data,
    alert_level = EXCLUDED.alert_level,
    metadata = EXCLUDED.metadata,
    updated_at = NOW()
RETURNING id;