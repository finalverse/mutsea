-- mutsea-database/src/sql/postgresql/analytics/environmental_trends.sql
WITH environmental_metrics AS (
    SELECT 
        timestamp,
        DATE_TRUNC(:aggregation_interval, timestamp) as time_bucket,
        (environmental_state->>'temperature')::numeric as temperature,
        (environmental_state->>'humidity')::numeric as humidity,
        (environmental_state->>'pressure')::numeric as pressure,
        (environmental_state->>'wind_speed')::numeric as wind_speed,
        (environmental_state->>'weather_severity')::numeric as weather_severity
    FROM world_states 
    WHERE timestamp BETWEEN :start_time AND :end_time
      AND environmental_state IS NOT NULL
)
SELECT 
    time_bucket,
    AVG(temperature) as avg_temperature,
    MIN(temperature) as min_temperature,
    MAX(temperature) as max_temperature,
    STDDEV(temperature) as temp_volatility,
    AVG(humidity) as avg_humidity,
    MIN(humidity) as min_humidity,
    MAX(humidity) as max_humidity,
    STDDEV(humidity) as humidity_volatility,
    AVG(pressure) as avg_pressure,
    AVG(wind_speed) as avg_wind_speed,
    MAX(wind_speed) as max_wind_speed,
    AVG(weather_severity) as avg_weather_severity,
    MAX(weather_severity) as max_weather_severity,
    COUNT(*) as measurement_count,
    -- Calculate trend indicators
    CASE 
        WHEN LAG(AVG(temperature)) OVER (ORDER BY time_bucket) IS NOT NULL THEN
            (AVG(temperature) - LAG(AVG(temperature)) OVER (ORDER BY time_bucket)) / 
            NULLIF(LAG(AVG(temperature)) OVER (ORDER BY time_bucket), 0) * 100
        ELSE NULL
    END as temperature_change_percent,
    CASE 
        WHEN LAG(AVG(humidity)) OVER (ORDER BY time_bucket) IS NOT NULL THEN
            (AVG(humidity) - LAG(AVG(humidity)) OVER (ORDER BY time_bucket)) / 
            NULLIF(LAG(AVG(humidity)) OVER (ORDER BY time_bucket), 0) * 100
        ELSE NULL
    END as humidity_change_percent
FROM environmental_metrics
GROUP BY time_bucket
ORDER BY time_bucket;