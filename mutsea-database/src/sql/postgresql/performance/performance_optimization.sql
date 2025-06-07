-- mutsea-database/src/sql/postgresql/performance/performance_optimization.sql
WITH component_performance AS (
    SELECT 
        component_name,
        metric_type,
        DATE_TRUNC(:time_interval, timestamp) as time_bucket,
        AVG(metric_value) as avg_value,
        MIN(metric_value) as min_value,
        MAX(metric_value) as max_value,
        STDDEV(metric_value) as value_stddev,
        COUNT(*) as measurement_count,
        COUNT(CASE WHEN alert_level = 'critical' THEN 1 END) as critical_alerts,
        COUNT(CASE WHEN alert_level = 'warning' THEN 1 END) as warning_alerts,
        AVG((threshold_data->>'target_value')::numeric) as target_value,
        AVG((threshold_data->>'warning_threshold')::numeric) as warning_threshold,
        AVG((threshold_data->>'critical_threshold')::numeric) as critical_threshold
    FROM performance_metrics
    WHERE timestamp >= NOW() - INTERVAL ':analysis_window_hours hours'
    GROUP BY component_name, metric_type, DATE_TRUNC(:time_interval, timestamp)
),
performance_trends AS (
    SELECT 
        *,
        LAG(avg_value) OVER (PARTITION BY component_name, metric_type ORDER BY time_bucket) as prev_avg_value,
        LAG(critical_alerts) OVER (PARTITION BY component_name, metric_type ORDER BY time_bucket) as prev_critical_alerts,
        LAG(warning_alerts) OVER (PARTITION BY component_name, metric_type ORDER BY time_bucket) as prev_warning_alerts
    FROM component_performance
),
optimization_opportunities AS (
    SELECT 
        component_name,
        metric_type,
        time_bucket,
        avg_value,
        target_value,
        warning_threshold,
        critical_threshold,
        measurement_count,
        critical_alerts,
        warning_alerts,
        value_stddev,
        -- Performance vs target analysis
        CASE 
            WHEN target_value IS NOT NULL THEN
                ABS(avg_value - target_value) / NULLIF(target_value, 0) * 100
            ELSE NULL
        END as deviation_from_target_percent,
        -- Trend analysis
        CASE 
            WHEN prev_avg_value IS NOT NULL AND prev_avg_value != 0 THEN
                (avg_value - prev_avg_value) / prev_avg_value * 100
            ELSE NULL
        END as performance_change_percent,
        -- Alert trend
        (critical_alerts - COALESCE(prev_critical_alerts, 0)) as critical_alert_change,
        (warning_alerts - COALESCE(prev_warning_alerts, 0)) as warning_alert_change,
        -- Stability score (lower is more stable)
        value_stddev / NULLIF(avg_value, 0) as coefficient_of_variation,
        -- Optimization priority
        CASE 
            WHEN critical_alerts > 0 THEN 1
            WHEN warning_alerts > 3 THEN 2
            WHEN ABS(avg_value - COALESCE(target_value, avg_value)) / NULLIF(COALESCE(target_value, avg_value), 0) > 0.2 THEN 3
            ELSE 4
        END as optimization_priority
    FROM performance_trends
)
SELECT 
    component_name,
    metric_type,
    time_bucket,
    avg_value,
    min_value,
    max_value,
    target_value,
    deviation_from_target_percent,
    performance_change_percent,
    critical_alerts,
    warning_alerts,
    critical_alert_change,
    warning_alert_change,
    coefficient_of_variation,
    optimization_priority,
    CASE optimization_priority
        WHEN 1 THEN 'Critical - High Alert Count'
        WHEN 2 THEN 'High - Frequent Warnings'
        WHEN 3 THEN 'Medium - Off Target'
        ELSE 'Low - Within Acceptable Range'
    END as optimization_recommendation,
    measurement_count
FROM optimization_opportunities
ORDER BY optimization_priority ASC, critical_alerts DESC, deviation_from_target_percent DESC;
