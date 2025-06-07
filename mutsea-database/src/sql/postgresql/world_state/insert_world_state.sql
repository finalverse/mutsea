-- mutsea-database/src/sql/postgresql/world_state/insert_world_state.sql
INSERT INTO world_states (
    id,
    timestamp,
    simulation_state,
    environmental_state,
    ecosystem_state,
    metadata,
    created_at,
    updated_at
) VALUES (
    :id,
    :timestamp,
    :simulation_state,
    :environmental_state,
    :ecosystem_state,
    :metadata,
    NOW(),
    NOW()
) RETURNING id, created_at;