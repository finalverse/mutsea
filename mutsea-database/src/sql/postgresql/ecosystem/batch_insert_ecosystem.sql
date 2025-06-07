-- mutsea-database/src/sql/postgresql/ecosystem/batch_insert_ecosystem.sql
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
) VALUES 
    (:id, :timestamp, :biome_id, :population_data, :resource_data, :interaction_data, :health_metrics, :biodiversity_index, :carrying_capacity, :metadata, NOW(), NOW())
ON CONFLICT (id) DO UPDATE SET
    population_data = EXCLUDED.population_data,
    resource_data = EXCLUDED.resource_data,
    interaction_data = EXCLUDED.interaction_data,
    health_metrics = EXCLUDED.health_metrics,
    biodiversity_index = EXCLUDED.biodiversity_index,
    carrying_capacity = EXCLUDED.carrying_capacity,
    metadata = EXCLUDED.metadata,
    updated_at = NOW()
RETURNING id;