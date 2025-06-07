-- mutsea-database/src/sql/postgresql/ai_decisions/batch_update_feedback.sql
UPDATE ai_decisions 
SET 
    feedback_score = data_table.feedback_score,
    outcome_data = data_table.outcome_data,
    updated_at = NOW()
FROM (VALUES 
    (:id, :feedback_score, :outcome_data)
) AS data_table(id, feedback_score, outcome_data)
WHERE ai_decisions.id = data_table.id
RETURNING ai_decisions.id, ai_decisions.updated_at;