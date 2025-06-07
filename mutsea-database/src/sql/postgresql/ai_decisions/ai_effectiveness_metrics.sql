-- mutsea-database/src/sql/postgresql/ai_decisions/ai_effectiveness_metrics.sql
WITH decision_performance AS (
    SELECT 
        decision_type,
        model_version,
        AVG(confidence_score) as avg_confidence,
        AVG(execution_time_ms) as avg_execution_time,
        AVG(feedback_score) as avg_feedback_score,
        COUNT(*) as decision_count,
        COUNT(CASE WHEN feedback_score >= :success_threshold THEN 1 END) as successful_decisions,
        STDDEV(confidence_score) as confidence_variance,
        STDDEV(execution_time_ms) as execution_time_variance,
        MIN(created_at) as first_decision,
        MAX(created_at) as latest_decision
    FROM ai_decisions
    WHERE created_at >= NOW() - INTERVAL ':time_window_hours hours'
      AND feedback_score IS NOT NULL
    GROUP BY decision_type, model_version
),
context_effectiveness AS (
    SELECT 
        decision_type,
        context_hash,
        COUNT(*) as context_usage_count,
        AVG(feedback_score) as context_avg_feedback,
        AVG(confidence_score) as context_avg_confidence
    FROM ai_decisions
    WHERE created_at >= NOW() - INTERVAL ':time_window_hours hours'
      AND feedback_score IS NOT NULL
    GROUP BY decision_type, context_hash
    HAVING COUNT(*) >= :min_context_usage
)
SELECT 
    dp.decision_type,
    dp.model_version,
    dp.avg_confidence,
    dp.avg_execution_time,
    dp.avg_feedback_score,
    dp.decision_count,
    dp.successful_decisions,
    ROUND((dp.successful_decisions::numeric / dp.decision_count * 100), 2) as success_rate_percent,
    dp.confidence_variance,
    dp.execution_time_variance,
    -- Context diversity metrics
    (SELECT COUNT(DISTINCT ce.context_hash) FROM context_effectiveness ce WHERE ce.decision_type = dp.decision_type) as unique_contexts_used,
    (SELECT AVG(ce.context_avg_feedback) FROM context_effectiveness ce WHERE ce.decision_type = dp.decision_type) as avg_context_effectiveness,
    -- Performance trends
    EXTRACT(EPOCH FROM (dp.latest_decision - dp.first_decision)) / 3600.0 as hours_of_operation,
    dp.decision_count / GREATEST(EXTRACT(EPOCH FROM (dp.latest_decision - dp.first_decision)) / 3600.0, 1) as decisions_per_hour
FROM decision_performance dp
ORDER BY dp.avg_feedback_score DESC, dp.decision_count DESC;