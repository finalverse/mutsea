-- mutsea-database/src/sql/postgresql/schema/create_ai_global_mind_state.sql
CREATE TABLE IF NOT EXISTS ai_global_mind_state (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    timestamp TIMESTAMPTZ NOT NULL,
    world_consciousness JSONB NOT NULL,
    player_psychology_state JSONB DEFAULT '{}',
    story_director_state JSONB DEFAULT '{}',
    resource_orchestrator_state JSONB DEFAULT '{}',
    emergence_detection_state JSONB DEFAULT '{}',
    decision_networks_state JSONB DEFAULT '{}',
    global_objectives JSONB DEFAULT '{}',
    adaptation_metrics JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ai_global_mind_state_timestamp ON ai_global_mind_state(timestamp);
CREATE INDEX IF NOT EXISTS idx_ai_global_mind_state_world_consciousness ON ai_global_mind_state USING GIN(world_consciousness);
CREATE INDEX IF NOT EXISTS idx_ai_global_mind_state_global_objectives ON ai_global_mind_state USING GIN(global_objectives);
