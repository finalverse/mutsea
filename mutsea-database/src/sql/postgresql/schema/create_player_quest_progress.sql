-- mutsea-database/src/sql/postgresql/schema/create_player_quest_progress.sql
CREATE TABLE IF NOT EXISTS player_quest_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID NOT NULL,
    quest_id UUID NOT NULL REFERENCES quests(id) ON DELETE CASCADE,
    started_at TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) DEFAULT 'active' CHECK (status IN ('active', 'completed', 'failed', 'abandoned')),
    progress_data JSONB DEFAULT '{}',
    completed_objectives JSONB DEFAULT '{}',
    choices_made JSONB DEFAULT '{}',
    time_spent_minutes INTEGER DEFAULT 0,
    difficulty_adjustments JSONB DEFAULT '{}',
    ai_assistance_used JSONB DEFAULT '{}',
    completion_metrics JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(player_id, quest_id)
);

CREATE INDEX IF NOT EXISTS idx_player_quest_progress_player_id ON player_quest_progress(player_id);
CREATE INDEX IF NOT EXISTS idx_player_quest_progress_quest_id ON player_quest_progress(quest_id);
CREATE INDEX IF NOT EXISTS idx_player_quest_progress_status ON player_quest_progress(status);
CREATE INDEX IF NOT EXISTS idx_player_quest_progress_started_at ON player_quest_progress(started_at);
