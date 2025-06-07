-- mutsea-database/src/sql/postgresql/ecosystem/insert_ecosystem_state.sql
INSERT INTO ecosystem_states (
    id,
    timestamp,
    biome_id,
    population_data,
    resource_data,
    interaction_data,
    health_metrics,
    biodiversity_index,
    carrying_capacity,
    metadata,
    created_at,
    updated_at
) VALUES (
    :id,
    :timestamp,
    :biome_id,
    :population_data,
    :resource_data,
    :interaction_data,
    :health_metrics,
    :biodiversity_index,
    :carrying_capacity,
    :metadata,
    NOW(),
    NOW()
) RETURNING id, created_at;