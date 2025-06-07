-- mutsea-database/src/sql/postgresql/analytics/player_patterns_analysis.sql
WITH player_journey AS (
    SELECT 
        pb.player_id,
        pb.session_id,
        MIN(pb.timestamp) as session_start,
        MAX(pb.timestamp) as session_end,
        COUNT(*) as total_actions,
        COUNT(DISTINCT pb.action_type) as unique_actions,
        AVG((pb.performance_metrics->>'response_time')::numeric) as avg_response_time,
        AVG((pb.performance_metrics->>'accuracy')::numeric) as avg_accuracy,
        STRING_AGG(pb.action_type, ',' ORDER BY pb.timestamp) as action_sequence
    FROM player_behaviors pb
    WHERE pb.timestamp >= NOW() - INTERVAL ':analysis_window_hours hours'
    GROUP BY pb.player_id, pb.session_id
),
behavioral_clustering AS (
    SELECT 
        player_id,
        COUNT(session_id) as total_sessions,
        AVG(EXTRACT(EPOCH FROM (session_end - session_start)) / 60.0) as avg_session_duration_minutes,
        AVG(total_actions) as avg_actions_per_session,
        AVG(unique_actions) as avg_unique_actions_per_session,
        AVG(avg_response_time) as overall_avg_response_time,
        AVG(avg_accuracy) as overall_avg_accuracy,
        -- Calculate play style indicators
        STDDEV(EXTRACT(EPOCH FROM (session_end - session_start)) / 60.0) / 
        NULLIF(AVG(EXTRACT(EPOCH FROM (session_end - session_start)) / 60.0), 0) as session_duration_consistency,
        -- Most common action patterns
        MODE() WITHIN GROUP (ORDER BY action_sequence) as most_common_action_sequence
    FROM player_journey
    GROUP BY player_id
),
skill_progression AS (
    SELECT 
        player_id,
        -- Calculate learning curve by comparing early vs recent performance
        AVG(CASE WHEN timestamp >= NOW() - INTERVAL ':recent_period_hours hours' THEN (performance_metrics->>'accuracy')::numeric END) as recent_accuracy,
        AVG(CASE WHEN timestamp < NOW() - INTERVAL ':recent_period_hours hours' THEN (performance_metrics->>'accuracy')::numeric END) as early_accuracy,
        AVG(CASE WHEN timestamp >= NOW() - INTERVAL ':recent_period_hours hours' THEN (performance_metrics->>'response_time')::numeric END) as recent_response_time,
        AVG(CASE WHEN timestamp < NOW() - INTERVAL ':recent_period_hours hours' THEN (performance_metrics->>'response_time')::numeric END) as early_response_time
    FROM player_behaviors
    WHERE timestamp >= NOW() - INTERVAL ':analysis_window_hours hours'
    GROUP BY player_id
)
SELECT 
    bc.player_id,
    bc.total_sessions,
    bc.avg_session_duration_minutes,
    bc.avg_actions_per_session,
    bc.avg_unique_actions_per_session,
    bc.overall_avg_response_time,
    bc.overall_avg_accuracy,
    bc.session_duration_consistency,
    bc.most_common_action_sequence,
    sp.recent_accuracy,
    sp.early_accuracy,
    sp.recent_response_time,
    sp.early_response_time,
    -- Player type classification
    CASE 
        WHEN bc.avg_session_duration_minutes > 60 AND bc.avg_actions_per_session > 100 THEN 'Power Player'
        WHEN bc.avg_session_duration_minutes > 30 AND bc.session_duration_consistency < 0.5 THEN 'Consistent Player'
        WHEN bc.avg_session_duration_minutes < 15 THEN 'Casual Player'
        WHEN bc.avg_unique_actions_per_session > 10 THEN 'Explorer'
        ELSE 'Standard Player'
    END as player_type,
    -- Engagement score
    (bc.avg_session_duration_minutes * bc.avg_actions_per_session * bc.total_sessions) / 
    GREATEST(bc.overall_avg_response_time, 1) as engagement_score,
    -- Skill progression indicator
    CASE 
        WHEN bc.overall_avg_accuracy > 0.8 AND bc.overall_avg_response_time < 2.0 THEN 'High Skill'
        WHEN bc.overall_avg_accuracy > 0.6 AND bc.overall_avg_response_time < 5.0 THEN 'Medium Skill'
        ELSE 'Developing Skill'
    END as skill_level,
    -- Learning progress calculation
    CASE 
        WHEN sp.early_accuracy IS NOT NULL AND sp.recent_accuracy IS NOT NULL THEN
            (sp.recent_accuracy - sp.early_accuracy) / NULLIF(sp.early_accuracy, 0) * 100
        ELSE NULL
    END as accuracy_improvement_percent,
    CASE 
        WHEN sp.early_response_time IS NOT NULL AND sp.recent_response_time IS NOT NULL THEN
            (sp.early_response_time - sp.recent_response_time) / NULLIF(sp.early_response_time, 0) * 100
        ELSE NULL
    END as response_time_improvement_percent,
    -- AI recommendation for player experience optimization
    CASE 
        WHEN bc.avg_session_duration_minutes < 10 AND bc.overall_avg_accuracy < 0.5 THEN 'Provide Tutorial'
        WHEN bc.avg_session_duration_minutes > 90 AND bc.overall_avg_accuracy > 0.9 THEN 'Increase Challenge'
        WHEN bc.session_duration_consistency > 1.0 THEN 'Improve Retention'
        WHEN bc.avg_unique_actions_per_session < 3 THEN 'Encourage Exploration'
        ELSE 'Maintain Current Experience'
    END as ai_recommendation
FROM behavioral_clustering bc
LEFT JOIN skill_progression sp ON bc.player_id = sp.player_id
ORDER BY bc.total_sessions DESC, engagement_score DESC;