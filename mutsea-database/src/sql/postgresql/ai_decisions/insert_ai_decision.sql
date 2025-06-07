-- mutsea-database/src/sql/postgresql/ai_decisions/insert_ai_decision.sql
INSERT INTO ai_decisions (
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
) VALUES (
    :id,
    :decision_type,
    :context_hash,
    :input_data,
    :decision_data,
    :confidence_score,
    :execution_time_ms,
    :model_version,
    :feedback_score,
    :outcome_data,
    :metadata,
    NOW(),
    NOW()
) RETURNING id, created_at;