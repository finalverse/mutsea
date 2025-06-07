CREATE TABLE IF NOT EXISTS ai_decisions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    decision_type VARCHAR(100) NOT NULL,
    context_hash VARCHAR(64) NOT NULL,
    input_data JSONB NOT NULL,
    decision_data JSONB NOT NULL,
    confidence_score DECIMAL(5,4) CHECK (confidence_score >= 0 AND confidence_score <= 1),
    execution_time_ms INTEGER NOT NULL CHECK (execution_time_ms >= 0),
    model_version VARCHAR(50) NOT NULL,
    feedback_score DECIMAL(5,4) CHECK (feedback_score >= 0 AND feedback_score <= 1),
    outcome_data JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ai_decisions_decision_type ON ai_decisions(decision_type);
CREATE INDEX IF NOT EXISTS idx_ai_decisions_context_hash ON ai_decisions(context_hash);
CREATE INDEX IF NOT EXISTS idx_ai_decisions_created_at ON ai_decisions(created_at);
CREATE INDEX IF NOT EXISTS idx_ai_decisions_confidence_score ON ai_decisions(confidence_score);
CREATE INDEX IF NOT EXISTS idx_ai_decisions_feedback_score ON ai_decisions(feedback_score);
CREATE INDEX IF NOT EXISTS idx_ai_decisions_model_version ON ai_decisions(model_version);
