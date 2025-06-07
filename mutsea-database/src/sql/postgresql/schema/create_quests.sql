-- mutsea-database/src/sql/postgresql/schema/create_quests.sql
CREATE TABLE IF NOT EXISTS quests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    quest_type VARCHAR(100) NOT NULL,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    objectives JSONB NOT NULL,
    rewards JSONB DEFAULT '{}',
    prerequisites JSONB DEFAULT '{}',
    difficulty_level INTEGER CHECK (difficulty_level >= 1 AND difficulty_level <= 10),
    estimated_duration_minutes INTEGER CHECK (estimated_duration_minutes > 0),
    target_players JSONB DEFAULT '{}', -- Player types or specific IDs
    generation_context JSONB DEFAULT '{}',
    status VARCHAR(20) DEFAULT 'available' CHECK (status IN ('available', 'active', 'completed', 'failed', 'expired')),
    completion_metrics JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_quests_quest_type ON quests(quest_type);
CREATE INDEX IF NOT EXISTS idx_quests_status ON quests(status);
CREATE INDEX IF NOT EXISTS idx_quests_difficulty_level ON quests(difficulty_level);
CREATE INDEX IF NOT EXISTS idx_quests_target_players ON quests USING GIN(target_players);
