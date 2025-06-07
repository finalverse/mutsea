-- mutsea-database/src/sql/postgresql/emergent_behavior/emergent_analysis.sql
WITH behavior_trends AS (
    SELECT 
        behavior_type,
        DATE_TRUNC(:time_bucket, detection_timestamp) as time_period,
        COUNT(*) as occurrence_count,
        AVG(complexity_score) as avg_complexity,
        MAX(complexity_score) as max_complexity,
        AVG(duration_seconds) as avg_duration,
        MAX(duration_seconds) as max_duration,
        AVG(json_array_length(participants)) as avg_participant_count,
        MAX(json_array_length(participants)) as max_participant_count,
        AVG((impact_metrics->>'ecosystem_impact')::numeric) as avg_ecosystem_impact,
        AVG((impact_metrics->>'player_impact')::numeric) as avg_player_impact
    FROM emergent_behaviors
    WHERE detection_timestamp >= NOW() - INTERVAL ':analysis_window_hours hours'
    GROUP BY behavior_type, DATE_TRUNC(:time_bucket, detection_timestamp)
),
complexity_distribution AS (
    SELECT 
        behavior_type,
        CASE 
            WHEN complexity_score < 0.3 THEN 'Low'
            WHEN complexity_score < 0.7 THEN 'Medium'
            ELSE 'High'
        END as complexity_category,
        COUNT(*) as count
    FROM emergent_behaviors
    WHERE detection_timestamp >= NOW() - INTERVAL ':analysis_window_hours hours'
    GROUP BY behavior_type, complexity_category
),
participant_analysis AS (
    SELECT 
        behavior_type,
        json_array_length(participants) as participant_count,
        COUNT(*) as frequency,
        AVG(complexity_score) as avg_complexity_for_participant_count
    FROM emergent_behaviors
    WHERE detection_timestamp >= NOW() - INTERVAL ':analysis_window_hours hours'
    GROUP BY behavior_type, json_array_length(participants)
)
SELECT 
    bt.behavior_type,
    bt.time_period,
    bt.occurrence_count,
    bt.avg_complexity,
    bt.max_complexity,
    bt.avg_duration,
    bt.avg_participant_count,
    bt.avg_ecosystem_impact,
    bt.avg_player_impact,
    -- Growth trend
    LAG(bt.occurrence_count) OVER (PARTITION BY bt.behavior_type ORDER BY bt.time_period) as prev_occurrence_count,
    CASE 
        WHEN LAG(bt.occurrence_count) OVER (PARTITION BY bt.behavior_type ORDER BY bt.time_period) > 0 THEN
            ((bt.occurrence_count - LAG(bt.occurrence_count) OVER (PARTITION BY bt.behavior_type ORDER BY bt.time_period))::numeric / 
             LAG(bt.occurrence_count) OVER (PARTITION BY bt.behavior_type ORDER BY bt.time_period) * 100)
        ELSE NULL
    END as growth_rate_percent,
    -- Complexity distribution for this behavior type
    (SELECT COUNT(*) FROM complexity_distribution cd WHERE cd.behavior_type = bt.behavior_type AND cd.complexity_category = 'High') as high_complexity_count,
    (SELECT COUNT(*) FROM complexity_distribution cd WHERE cd.behavior_type = bt.behavior_type AND cd.complexity_category = 'Medium') as medium_complexity_count,
    (SELECT COUNT(*) FROM complexity_distribution cd WHERE cd.behavior_type = bt.behavior_type AND cd.complexity_category = 'Low') as low_complexity_count
FROM behavior_trends bt
ORDER BY bt.behavior_type, bt.time_period DESC;