-- src/sql/opensim/create_assets.sql
-- OpenSim assets table
CREATE TABLE IF NOT EXISTS assets (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    description VARCHAR(64) NOT NULL,
    asset_type TINYINT(4) NOT NULL,
    local BOOLEAN NOT NULL,
    temporary BOOLEAN NOT NULL,
    data LONGBLOB NOT NULL,
    create_time INTEGER DEFAULT 0,
    access_time INTEGER DEFAULT 0,
    asset_flags INTEGER NOT NULL DEFAULT 0,
    creator_id VARCHAR(36) NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000'
);