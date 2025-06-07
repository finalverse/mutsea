-- mutsea-database/src/sql/postgresql/ai_decisions/select_ai_decisions.sql
SELECT 
    id,
    decision_type,
    context_hash,
    input_data,
    decision_data,
    confidence_score,
    execution_time_ms,
    model_version,
    feedback_score,
    outcome_data,
    metadata,
    created_at,
    updated_at
FROM ai_decisions 
WHERE decision_type = :decision_type
  AND created_at BETWEEN :start_time AND :end_time
ORDER BY created_at DESC
LIMIT :limit_count;