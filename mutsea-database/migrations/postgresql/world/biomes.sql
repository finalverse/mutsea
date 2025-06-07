CREATE TABLE IF NOT EXISTS biomes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    biome_type VARCHAR(50) NOT NULL,
    climate_data JSONB NOT NULL,
    terrain_data JSONB NOT NULL,
    species_data JSONB DEFAULT '{}',
    resource_distribution JSONB DEFAULT '{}',
    carrying_capacity_base INTEGER CHECK (carrying_capacity_base >= 0),
    environmental_factors JSONB DEFAULT '{}',
    generation_parameters JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_biomes_name ON biomes(name);
CREATE INDEX IF NOT EXISTS idx_biomes_biome_type ON biomes(biome_type);
CREATE INDEX IF NOT EXISTS idx_biomes_climate_data ON biomes USING GIN(climate_data);
CREATE INDEX IF NOT EXISTS idx_biomes_terrain_data ON biomes USING GIN(terrain_data);
