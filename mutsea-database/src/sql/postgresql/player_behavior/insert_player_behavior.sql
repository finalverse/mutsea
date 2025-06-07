-- mutsea-database/src/sql/postgresql/player_behavior/insert_player_behavior.sql
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
) VALUES (
    :id,
    :player_id,
    :session_id,
    :timestamp,
    :action_type,
    :action_data,
    :context_data,
    :performance_metrics,
    :metadata,
    NOW(),
    NOW()
) RETURNING id, created_at;