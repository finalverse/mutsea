-- src/sql/opensim/create_terrain.sql
-- OpenSim terrain table
CREATE TABLE IF NOT EXISTS terrain (
    region_uuid VARCHAR(36) DEFAULT NULL,
    revision INTEGER DEFAULT NULL,
    heightfield LONGBLOB,
    KEY region_uuid (region_uuid)
);