-- mutsea-database/src/sql/postgresql/player_behavior/batch_insert_behaviors.sql
INSERT INTO player_behaviors (
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
) VALUES 
    (:id, :player_id, :session_id, :timestamp, :action_type, :action_data, :context_data, :performance_metrics, :metadata, NOW(), NOW())
ON CONFLICT (id) DO UPDATE SET
    action_data = EXCLUDED.action_data,
    context_data = EXCLUDED.context_data,
    performance_metrics = EXCLUDED.performance_metrics,
    metadata = EXCLUDED.metadata,
    updated_at = NOW()
RETURNING id;