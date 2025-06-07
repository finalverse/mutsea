-- mutsea-database/src/sql/postgresql/analytics/anomalous_world_states.sql
WITH recent_states AS (
    SELECT 
        id,
        timestamp,
        simulation_state,
        environmental_state,
        ecosystem_state
    FROM world_states 
    WHERE timestamp >= NOW() - INTERVAL ':time_range_hours hours'
),
state_metrics AS (
    SELECT 
        id,
        timestamp,
        (simulation_state->>'entity_count')::numeric as entity_count,
        (environmental_state->>'temperature')::numeric as temperature,
        (environmental_state->>'weather_severity')::numeric as weather_severity,
        (ecosystem_state->>'total_population')::numeric as total_population,
        (ecosystem_state->>'resource_scarcity')::numeric as resource_scarcity,
        (ecosystem_state->>'biodiversity_index')::numeric as biodiversity_index
    FROM recent_states
    WHERE simulation_state IS NOT NULL 
      AND environmental_state IS NOT NULL 
      AND ecosystem_state IS NOT NULL
),
metric_stats AS (
    SELECT 
        AVG(entity_count) as avg_entity_count,
        STDDEV(entity_count) as stddev_entity_count,
        AVG(temperature) as avg_temperature,
        STDDEV(temperature) as stddev_temperature,
        AVG(weather_severity) as avg_weather_severity,
        STDDEV(weather_severity) as stddev_weather_severity,
        AVG(total_population) as avg_total_population,
        STDDEV(total_population) as stddev_total_population,
        AVG(resource_scarcity) as avg_resource_scarcity,
        STDDEV(resource_scarcity) as stddev_resource_scarcity,
        AVG(biodiversity_index) as avg_biodiversity_index,
        STDDEV(biodiversity_index) as stddev_biodiversity_index
    FROM state_metrics
),
anomaly_scores AS (
    SELECT 
        sm.id,
        sm.timestamp,
        -- Calculate z-scores for each metric
        ABS((sm.entity_count - ms.avg_entity_count) / NULLIF(ms.stddev_entity_count, 0)) as entity_count_zscore,
        ABS((sm.temperature - ms.avg_temperature) / NULLIF(ms.stddev_temperature, 0)) as temperature_zscore,
        ABS((sm.weather_severity - ms.avg_weather_severity) / NULLIF(ms.stddev_weather_severity, 0)) as weather_severity_zscore,
        ABS((sm.total_population - ms.avg_total_population) / NULLIF(ms.stddev_total_population, 0)) as population_zscore,
        ABS((sm.resource_scarcity - ms.avg_resource_scarcity) / NULLIF(ms.stddev_resource_scarcity, 0)) as resource_scarcity_zscore,
        ABS((sm.biodiversity_index - ms.avg_biodiversity_index) / NULLIF(ms.stddev_biodiversity_index, 0)) as biodiversity_zscore,
        sm.entity_count,
        sm.temperature,
        sm.weather_severity,
        sm.total_population,
        sm.resource_scarcity,
        sm.biodiversity_index
    FROM state_metrics sm
    CROSS JOIN metric_stats ms
),
composite_anomalies AS (
    SELECT 
        *,
        -- Calculate composite anomaly score
        (COALESCE(entity_count_zscore, 0) + 
         COALESCE(temperature_zscore, 0) + 
         COALESCE(weather_severity_zscore, 0) + 
         COALESCE(population_zscore, 0) + 
         COALESCE(resource_scarcity_zscore, 0) + 
         COALESCE(biodiversity_zscore, 0)) / 6.0 as composite_anomaly_score,
        -- Find the most anomalous metric
        GREATEST(
            COALESCE(entity_count_zscore, 0),
            COALESCE(temperature_zscore, 0),
            COALESCE(weather_severity_zscore, 0),
            COALESCE(population_zscore, 0),
            COALESCE(resource_scarcity_zscore, 0),
            COALESCE(biodiversity_zscore, 0)
        ) as max_individual_zscore
    FROM anomaly_scores
),
anomaly_classification AS (
    SELECT 
        id,
        timestamp,
        composite_anomaly_score,
        max_individual_zscore,
        entity_count,
        temperature,
        weather_severity,
        total_population,
        resource_scarcity,
        biodiversity_index,
        entity_count_zscore,
        temperature_zscore,
        weather_severity_zscore,
        population_zscore,
        resource_scarcity_zscore,
        biodiversity_zscore,
        CASE 
            WHEN entity_count_zscore = max_individual_zscore THEN 'entity_count'
            WHEN temperature_zscore = max_individual_zscore THEN 'temperature'
            WHEN weather_severity_zscore = max_individual_zscore THEN 'weather_severity'
            WHEN population_zscore = max_individual_zscore THEN 'total_population'
            WHEN resource_scarcity_zscore = max_individual_zscore THEN 'resource_scarcity'
            WHEN biodiversity_zscore = max_individual_zscore THEN 'biodiversity_index'
            ELSE 'multiple_metrics'
        END as primary_anomaly_metric,
        -- Anomaly severity classification
        CASE 
            WHEN composite_anomaly_score > 3.0 THEN 'Critical'
            WHEN composite_anomaly_score > 2.0 THEN 'High'
            WHEN composite_anomaly_score > 1.5 THEN 'Medium'
            WHEN composite_anomaly_score > 1.0 THEN 'Low'
            ELSE 'Normal'
        END as anomaly_severity,
        -- Potential impact assessment
        CASE 
            WHEN entity_count_zscore > 2.5 OR population_zscore > 2.5 THEN 'Ecosystem Collapse Risk'
            WHEN temperature_zscore > 2.5 OR weather_severity_zscore > 2.5 THEN 'Environmental Crisis'
            WHEN biodiversity_zscore > 2.5 OR resource_scarcity_zscore > 2.5 THEN 'Resource Crisis'
            WHEN composite_anomaly_score > 2.0 THEN 'System Instability'
            ELSE 'Monitoring Required'
        END as impact_assessment
    FROM composite_anomalies
)
SELECT 
    id,
    timestamp,
    composite_anomaly_score,
    max_individual_zscore,
    primary_anomaly_metric,
    anomaly_severity,
    impact_assessment,
    entity_count,
    temperature,
    weather_severity,
    total_population,
    resource_scarcity,
    biodiversity_index,
    -- Detailed z-scores for investigation
    ROUND(entity_count_zscore::numeric, 3) as entity_count_zscore,
    ROUND(temperature_zscore::numeric, 3) as temperature_zscore,
    ROUND(weather_severity_zscore::numeric, 3) as weather_severity_zscore,
    ROUND(population_zscore::numeric, 3) as population_zscore,
    ROUND(resource_scarcity_zscore::numeric, 3) as resource_scarcity_zscore,
    ROUND(biodiversity_zscore::numeric, 3) as biodiversity_zscore,
    -- AI recommendation for anomaly response
    CASE 
        WHEN anomaly_severity = 'Critical' THEN 'Immediate Intervention Required'
        WHEN anomaly_severity = 'High' AND impact_assessment LIKE '%Crisis%' THEN 'Emergency Response Activated'
        WHEN anomaly_severity = 'High' THEN 'Alert Stakeholders'
        WHEN anomaly_severity = 'Medium' THEN 'Increase Monitoring Frequency'
        WHEN anomaly_severity = 'Low' THEN 'Continue Standard Monitoring'
        ELSE 'No Action Required'
    END as ai_recommendation,
    -- Calculate anomaly persistence (how long has this been anomalous)
    LAG(composite_anomaly_score) OVER (ORDER BY timestamp) as prev_anomaly_score,
    CASE 
        WHEN LAG(composite_anomaly_score) OVER (ORDER BY timestamp) > 1.5 
             AND composite_anomaly_score > 1.5 THEN 'Persistent'
        WHEN LAG(composite_anomaly_score) OVER (ORDER BY timestamp) <= 1.5 
             AND composite_anomaly_score > 1.5 THEN 'New'
        WHEN LAG(composite_anomaly_score) OVER (ORDER BY timestamp) > 1.5 
             AND composite_anomaly_score <= 1.5 THEN 'Resolving'
        ELSE 'Stable'
    END as anomaly_trend
FROM anomaly_classification
WHERE composite_anomaly_score >= :anomaly_threshold
ORDER BY composite_anomaly_score DESC, timestamp DESC
LIMIT :max_results;