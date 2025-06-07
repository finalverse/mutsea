-- mutsea-database/src/sql/postgresql/player_behavior/player_behavior_patterns.sql
WITH player_sessions AS (
    SELECT 
        player_id,
        session_id,
        MIN(timestamp) as session_start,
        MAX(timestamp) as session_end,
        COUNT(*) as action_count,
        COUNT(DISTINCT action_type) as unique_actions,
        EXTRACT(EPOCH FROM (MAX(timestamp) - MIN(timestamp))) / 60.0 as session_duration_minutes
    FROM player_behaviors
    WHERE timestamp >= NOW() - INTERVAL ':time_window_hours hours'
    GROUP BY player_id, session_id
),
action_patterns AS (
    SELECT 
        player_id,
        action_type,
        COUNT(*) as action_frequency,
        AVG(EXTRACT(EPOCH FROM (timestamp - LAG(timestamp) OVER (PARTITION BY player_id ORDER BY timestamp)))) as avg_time_between_actions,
        AVG((performance_metrics->>'response_time')::numeric) as avg_response_time,
        AVG((performance_metrics->>'accuracy')::numeric) as avg_accuracy
    FROM player_behaviors
    WHERE timestamp >= NOW() - INTERVAL ':time_window_hours hours'
    GROUP BY player_id, action_type
)
SELECT 
    ps.player_id,
    COUNT(ps.session_id) as total_sessions,
    AVG(ps.session_duration_minutes) as avg_session_duration,
    AVG(ps.action_count) as avg_actions_per_session,
    AVG(ps.unique_actions) as avg_unique_actions_per_session,
    SUM(ps.action_count) as total_actions,
    -- Most common action type
    (SELECT ap.action_type FROM action_patterns ap 
     WHERE ap.player_id = ps.player_id 
     ORDER BY ap.action_frequency DESC LIMIT 1) as most_common_action,
    -- Overall performance metrics
    AVG((SELECT ap.avg_response_time FROM action_patterns ap WHERE ap.player_id = ps.player_id)) as overall_avg_response_time,
    AVG((SELECT ap.avg_accuracy FROM action_patterns ap WHERE ap.player_id = ps.player_id)) as overall_avg_accuracy,
    -- Behavior consistency score (lower = more consistent)
    STDDEV(ps.session_duration_minutes) / NULLIF(AVG(ps.session_duration_minutes), 0) as session_consistency_score
FROM player_sessions ps
GROUP BY ps.player_id
ORDER BY total_actions DESC;