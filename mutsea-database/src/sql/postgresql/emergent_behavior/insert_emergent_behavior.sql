-- mutsea-database/src/sql/postgresql/emergent_behavior/insert_emergent_behavior.sql
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
) VALUES (
    :id,
    :behavior_type,
    :detection_timestamp,
    :participants,
    :trigger_conditions,
    :behavior_data,
    :complexity_score,
    :impact_metrics,
    :duration_seconds,
    :metadata,
    NOW(),
    NOW()
) RETURNING id, created_at;