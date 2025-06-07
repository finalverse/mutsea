-- mutsea-database/src/sql/postgresql/world_state/select_world_states_by_time_range.sql
SELECT 
    id,
    timestamp,
    simulation_state,
    environmental_state,
    ecosystem_state,
    metadata,
    created_at,
    updated_at
FROM world_states 
WHERE timestamp BETWEEN :start_time AND :end_time
ORDER BY timestamp DESC
LIMIT :limit OFFSET :offset;