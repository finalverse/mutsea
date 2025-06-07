-- mutsea-database/src/sql/postgresql/emergent_behavior/batch_insert_emergent.sql
INSERT INTO emergent_behaviors (
    id,
    behavior_type,
    detection_timestamp,
    participants,
    trigger_conditions,
    behavior_data,
    complexity_score,
    impact_metrics,
    duration_seconds,
    metadata,
    created_at,
    updated_at
) VALUES 
    (:id, :behavior_type, :detection_timestamp, :participants, :trigger_conditions, :behavior_data, :complexity_score, :impact_metrics, :duration_seconds, :metadata, NOW(), NOW())
ON CONFLICT (id) DO UPDATE SET
    behavior_data = EXCLUDED.behavior_data,
    complexity_score = EXCLUDED.complexity_score,
    impact_metrics = EXCLUDED.impact_metrics,
    duration_seconds = EXCLUDED.duration_seconds,
    metadata = EXCLUDED.metadata,
    updated_at = NOW()
RETURNING id;