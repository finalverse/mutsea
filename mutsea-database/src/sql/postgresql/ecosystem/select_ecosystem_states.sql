-- mutsea-database/src/sql/postgresql/ecosystem/select_ecosystem_states.sql
SELECT 
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
FROM ecosystem_states 
WHERE biome_id = :biome_id
  AND timestamp BETWEEN :start_time AND :end_time
ORDER BY timestamp DESC
LIMIT :limit_count;