CREATE TABLE IF NOT EXISTS story_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    participants JSONB NOT NULL, -- Array of participant IDs
    location_data JSONB DEFAULT '{}',
    narrative_data JSONB NOT NULL,
    choices_presented JSONB DEFAULT '{}',
    player_decisions JSONB DEFAULT '{}',
    outcomes JSONB DEFAULT '{}',
    impact_metrics JSONB DEFAULT '{}',
    generation_source VARCHAR(50) NOT NULL, -- 'ai_generated', 'player_triggered', 'emergent'
    story_arc_id UUID,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_story_events_event_type ON story_events(event_type);
CREATE INDEX IF NOT EXISTS idx_story_events_timestamp ON story_events(timestamp);
CREATE INDEX IF NOT EXISTS idx_story_events_participants ON story_events USING GIN(participants);
CREATE INDEX IF NOT EXISTS idx_story_events_generation_source ON story_events(generation_source);
CREATE INDEX IF NOT EXISTS idx_story_events_story_arc_id ON story_events(story_arc_id);
