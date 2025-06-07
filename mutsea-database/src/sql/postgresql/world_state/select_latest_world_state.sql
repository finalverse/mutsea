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
ORDER BY timestamp DESC 
LIMIT 1;