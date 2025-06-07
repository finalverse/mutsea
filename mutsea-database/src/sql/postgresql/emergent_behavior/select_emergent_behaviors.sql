-- mutsea-database/src/sql/postgresql/emergent_behavior/select_emergent_behaviors.sql
SELECT 
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
FROM emergent_behaviors 
WHERE detection_timestamp BETWEEN :start_time AND :end_time
  AND (:behavior_type IS NULL OR behavior_type = :behavior_type)
  AND complexity_score >= :min_complexity_score
ORDER BY detection_timestamp DESC, complexity_score DESC
LIMIT :limit_count;