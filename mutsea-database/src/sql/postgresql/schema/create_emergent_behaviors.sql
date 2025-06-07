-- mutsea-database/src/sql/postgresql/schema/create_emergent_behaviors.sql
CREATE TABLE IF NOT EXISTS emergent_behaviors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    behavior_type VARCHAR(100) NOT NULL,
    detection_timestamp TIMESTAMPTZ NOT NULL,
    participants JSONB NOT NULL, -- Array of participant IDs
    trigger_conditions JSONB NOT NULL,
    behavior_data JSONB NOT NULL,
    complexity_score DECIMAL(5,4) CHECK (complexity_score >= 0 AND complexity_score <= 1),
    impact_metrics JSONB DEFAULT '{}',
    duration_seconds INTEGER CHECK (duration_seconds >= 0),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_emergent_behaviors_behavior_type ON emergent_behaviors(behavior_type);
CREATE INDEX IF NOT EXISTS idx_emergent_behaviors_detection_timestamp ON emergent_behaviors(detection_timestamp);
CREATE INDEX IF NOT EXISTS idx_emergent_behaviors_complexity_score ON emergent_behaviors(complexity_score);
CREATE INDEX IF NOT EXISTS idx_emergent_behaviors_participants ON emergent_behaviors USING GIN(participants);
