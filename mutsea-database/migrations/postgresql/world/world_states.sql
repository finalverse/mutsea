CREATE TABLE IF NOT EXISTS world_states (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    timestamp TIMESTAMPTZ NOT NULL,
    simulation_state JSONB NOT NULL,
    environmental_state JSONB NOT NULL,
    ecosystem_state JSONB NOT NULL,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_world_states_timestamp ON world_states(timestamp);
CREATE INDEX IF NOT EXISTS idx_world_states_created_at ON world_states(created_at);
CREATE INDEX IF NOT EXISTS idx_world_states_simulation_state ON world_states USING GIN(simulation_state);
CREATE INDEX IF NOT EXISTS idx_world_states_environmental_state ON world_states USING GIN(environmental_state);
CREATE INDEX IF NOT EXISTS idx_world_states_ecosystem_state ON world_states USING GIN(ecosystem_state);
