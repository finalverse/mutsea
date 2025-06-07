-- mutsea-database/src/sql/postgresql/ecosystem/ecosystem_health_metrics.sql
WITH ecosystem_trends AS (
    SELECT 
        biome_id,
        DATE_TRUNC(:time_interval, timestamp) as time_bucket,
        AVG(biodiversity_index) as avg_biodiversity,
        AVG(carrying_capacity) as avg_carrying_capacity,
        AVG((health_metrics->>'stability_score')::numeric) as avg_stability,
        AVG((health_metrics->>'resilience_score')::numeric) as avg_resilience,
        AVG((health_metrics->>'productivity_score')::numeric) as avg_productivity,
        -- Population metrics
        AVG((population_data->>'total_population')::numeric) as avg_total_population,
        AVG((population_data->>'predator_population')::numeric) as avg_predator_population,
        AVG((population_data->>'prey_population')::numeric) as avg_prey_population,
        AVG((population_data->>'producer_population')::numeric) as avg_producer_population,
        -- Resource metrics
        AVG((resource_data->>'food_availability')::numeric) as avg_food_availability,
        AVG((resource_data->>'water_availability')::numeric) as avg_water_availability,
        AVG((resource_data->>'shelter_availability')::numeric) as avg_shelter_availability,
        COUNT(*) as measurement_count
    FROM ecosystem_states
    WHERE timestamp >= NOW() - INTERVAL ':analysis_window_hours hours'
    GROUP BY biome_id, DATE_TRUNC(:time_interval, timestamp)
),
population_stability AS (
    SELECT 
        biome_id,
        STDDEV((population_data->>'total_population')::numeric) / 
        NULLIF(AVG((population_data->>'total_population')::numeric), 0) as population_volatility,
        STDDEV(biodiversity_index) / NULLIF(AVG(biodiversity_index), 0) as biodiversity_volatility
    FROM ecosystem_states
    WHERE timestamp >= NOW() - INTERVAL ':analysis_window_hours hours'
    GROUP BY biome_id
)
SELECT 
    et.biome_id,
    et.time_bucket,
    et.avg_biodiversity,
    et.avg_carrying_capacity,
    et.avg_stability,
    et.avg_resilience,
    et.avg_productivity,
    et.avg_total_population,
    et.avg_predator_population,
    et.avg_prey_population,
    et.avg_producer_population,
    et.avg_food_availability,
    et.avg_water_availability,
    et.avg_shelter_availability,
    et.measurement_count,
    ps.population_volatility,
    ps.biodiversity_volatility,
    -- Health score calculation
    (et.avg_stability + et.avg_resilience + et.avg_productivity) / 3.0 as composite_health_score,
    -- Resource pressure indicators
    CASE 
        WHEN et.avg_total_population > et.avg_carrying_capacity * 0.9 THEN 'High Pressure'
        WHEN et.avg_total_population > et.avg_carrying_capacity * 0.7 THEN 'Medium Pressure'
        ELSE 'Low Pressure'
    END as resource_pressure_level,
    -- Trend analysis
    LAG(et.avg_biodiversity) OVER (PARTITION BY et.biome_id ORDER BY et.time_bucket) as prev_biodiversity,
    LAG(et.avg_total_population) OVER (PARTITION BY et.biome_id ORDER BY et.time_bucket) as prev_population
FROM ecosystem_trends et
LEFT JOIN population_stability ps ON et.biome_id = ps.biome_id
ORDER BY et.biome_id, et.time_bucket DESC;