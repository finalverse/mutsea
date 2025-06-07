-- mutsea-database/src/sql/postgresql/world_state/batch_insert_world_states.sql
INSERT INTO world_states (
    id,
    timestamp,
    simulation_state,
    environmental_state,
    ecosystem_state,
    metadata,
    created_at,
    updated_at
) VALUES 
    (:id, :timestamp, :simulation_state, :environmental_state, :ecosystem_state, :metadata, NOW(), NOW())
ON CONFLICT (id) DO UPDATE SET
    timestamp = EXCLUDED.timestamp,
    simulation_state = EXCLUDED.simulation_state,
    environmental_state = EXCLUDED.environmental_state,
    ecosystem_state = EXCLUDED.ecosystem_state,
    metadata = EXCLUDED.metadata,
    updated_at = NOW()
RETURNING id;