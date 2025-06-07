-- mutsea-database/src/sql/postgresql/analytics/world_evolution_patterns.sql
WITH time_windows AS (
    SELECT 
        id,
        timestamp,
        simulation_state,
        environmental_state,
        ecosystem_state,
        LAG(simulation_state) OVER (ORDER BY timestamp) as prev_simulation_state,
        LAG(environmental_state) OVER (ORDER BY timestamp) as prev_environmental_state,
        LAG(ecosystem_state) OVER (ORDER BY timestamp) as prev_ecosystem_state,
        LAG(timestamp) OVER (ORDER BY timestamp) as prev_timestamp
    FROM world_states 
    WHERE timestamp >= NOW() - INTERVAL ':time_window_hours hours'
    ORDER BY timestamp
),
state_changes AS (
    SELECT 
        timestamp,
        CASE 
            WHEN prev_simulation_state IS NOT NULL THEN
                json_diff_magnitude(simulation_state, prev_simulation_state)
            ELSE 0
        END as simulation_change_magnitude,
        CASE 
            WHEN prev_environmental_state IS NOT NULL THEN
                json_diff_magnitude(environmental_state, prev_environmental_state)
            ELSE 0
        END as environmental_change_magnitude,
        CASE 
            WHEN prev_ecosystem_state IS NOT NULL THEN
                json_diff_magnitude(ecosystem_state, prev_ecosystem_state)
            ELSE 0
        END as ecosystem_change_magnitude,
        EXTRACT(EPOCH FROM (timestamp - prev_timestamp)) / 3600.0 as time_delta_hours
    FROM time_windows
    WHERE prev_timestamp IS NOT NULL
)
SELECT 
    DATE_TRUNC('hour', timestamp) as time_bucket,
    AVG(simulation_change_magnitude) as avg_simulation_change,
    AVG(environmental_change_magnitude) as avg_environmental_change,
    AVG(ecosystem_change_magnitude) as avg_ecosystem_change,
    MAX(simulation_change_magnitude) as max_simulation_change,
    MAX(environmental_change_magnitude) as max_environmental_change,
    MAX(ecosystem_change_magnitude) as max_ecosystem_change,
    COUNT(*) as change_events,
    AVG(time_delta_hours) as avg_time_between_changes
FROM state_changes
WHERE simulation_change_magnitude > :pattern_threshold 
   OR environmental_change_magnitude > :pattern_threshold
   OR ecosystem_change_magnitude > :pattern_threshold
GROUP BY DATE_TRUNC('hour', timestamp)
ORDER BY time_bucket DESC;