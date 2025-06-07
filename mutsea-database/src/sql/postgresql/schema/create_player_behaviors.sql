-- mutsea-database/src/sql/postgresql/schema/create_player_behaviors.sql
CREATE TABLE IF NOT EXISTS player_behaviors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID NOT NULL,
    session_id UUID NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    action_type VARCHAR(100) NOT NULL,
    action_data JSONB NOT NULL,
    context_data JSONB DEFAULT '{}',
    performance_metrics JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_player_behaviors_player_id ON player_behaviors(player_id);
CREATE INDEX IF NOT EXISTS idx_player_behaviors_session_id ON player_behaviors(session_id);
CREATE INDEX IF NOT EXISTS idx_player_behaviors_timestamp ON player_behaviors(timestamp);
CREATE INDEX IF NOT EXISTS idx_player_behaviors_action_type ON player_behaviors(action_type);
CREATE INDEX IF NOT EXISTS idx_player_behaviors_action_data ON player_behaviors USING GIN(action_data);
