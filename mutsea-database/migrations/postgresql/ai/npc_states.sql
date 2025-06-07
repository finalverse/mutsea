CREATE TABLE IF NOT EXISTS npc_states (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    npc_id UUID NOT NULL UNIQUE,
    timestamp TIMESTAMPTZ NOT NULL,
    personality_data JSONB NOT NULL,
    memory_data JSONB DEFAULT '{}',
    goal_data JSONB DEFAULT '{}',
    relationship_data JSONB DEFAULT '{}',
    emotional_state JSONB DEFAULT '{}',
    behavioral_patterns JSONB DEFAULT '{}',
    learning_progress JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_npc_states_npc_id ON npc_states(npc_id);
CREATE INDEX IF NOT EXISTS idx_npc_states_timestamp ON npc_states(timestamp);
CREATE INDEX IF NOT EXISTS idx_npc_states_personality_data ON npc_states USING GIN(personality_data);
CREATE INDEX IF NOT EXISTS idx_npc_states_emotional_state ON npc_states USING GIN(emotional_state);
