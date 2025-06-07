-- mutsea-database/src/sql/postgresql/player_behavior/select_player_behaviors.sql
SELECT 
    id,
    player_id,
    session_id,
    timestamp,
    action_type,
    action_data,
    context_data,
    performance_metrics,
    metadata,
    created_at,
    updated_at
FROM player_behaviors 
WHERE player_id = :player_id
  AND timestamp BETWEEN :start_time AND :end_time
ORDER BY timestamp DESC
LIMIT :limit_count;