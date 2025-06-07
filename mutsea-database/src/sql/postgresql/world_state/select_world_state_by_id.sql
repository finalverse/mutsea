-- mutsea-database/src/sql/postgresql/world_state/select_world_state_by_id.sql
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
WHERE id = :id;