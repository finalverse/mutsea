CREATE TABLE IF NOT EXISTS learning_data (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    subject_type VARCHAR(50) NOT NULL, -- 'player', 'npc', 'ecosystem', 'ai_system'
    subject_id UUID NOT NULL,
    learning_session_id UUID NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    input_data JSONB NOT NULL,
    output_data JSONB NOT NULL,
    feedback_data JSONB DEFAULT '{}',
    learning_algorithm VARCHAR(100) NOT NULL,
    performance_metrics JSONB DEFAULT '{}',
    convergence_metrics JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_learning_data_subject_type ON learning_data(subject_type);
CREATE INDEX IF NOT EXISTS idx_learning_data_subject_id ON learning_data(subject_id);
CREATE INDEX IF NOT EXISTS idx_learning_data_learning_session_id ON learning_data(learning_session_id);
CREATE INDEX IF NOT EXISTS idx_learning_data_timestamp ON learning_data(timestamp);
CREATE INDEX IF NOT EXISTS idx_learning_data_learning_algorithm ON learning_data(learning_algorithm);
CREATE INDEX IF NOT EXISTS idx_learning_data_composite ON learning_data(subject_type, subject_id, timestamp);
