CREATE TABLE IF NOT EXISTS ecosystem_states (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    timestamp TIMESTAMPTZ NOT NULL,
    biome_id UUID NOT NULL,
    population_data JSONB NOT NULL,
    resource_data JSONB NOT NULL,
    interaction_data JSONB DEFAULT '{}',
    health_metrics JSONB DEFAULT '{}',
    biodiversity_index DECIMAL(5,4) CHECK (biodiversity_index >= 0 AND biodiversity_index <= 1),
    carrying_capacity INTEGER CHECK (carrying_capacity >= 0),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ecosystem_states_biome_id ON ecosystem_states(biome_id);
CREATE INDEX IF NOT EXISTS idx_ecosystem_states_timestamp ON ecosystem_states(timestamp);
CREATE INDEX IF NOT EXISTS idx_ecosystem_states_biodiversity_index ON ecosystem_states(biodiversity_index);
CREATE INDEX IF NOT EXISTS idx_ecosystem_states_population_data ON ecosystem_states USING GIN(population_data);
