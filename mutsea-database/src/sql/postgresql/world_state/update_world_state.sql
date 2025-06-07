-- mutsea-database/src/sql/postgresql/world_state/update_world_state.sql
UPDATE world_states 
SET 
    timestamp = :timestamp,
    simulation_state = :simulation_state,
    environmental_state = :environmental_state,
    ecosystem_state = :ecosystem_state,
    metadata = :metadata,
    updated_at = NOW()
WHERE id = :id
RETURNING id, updated_at;