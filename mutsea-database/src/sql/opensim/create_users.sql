-- src/sql/opensim/create_users.sql
-- OpenSim user accounts table
CREATE TABLE IF NOT EXISTS user_accounts (
    principal_id VARCHAR(36) NOT NULL PRIMARY KEY,
    scope_id VARCHAR(36) NOT NULL,
    first_name VARCHAR(64) NOT NULL,
    last_name VARCHAR(64) NOT NULL,
    email VARCHAR(64) DEFAULT NULL,
    service_urls TEXT DEFAULT NULL,
    created INTEGER DEFAULT NULL,
    user_level INTEGER NOT NULL DEFAULT 0,
    user_flags INTEGER NOT NULL DEFAULT 0,
    user_title VARCHAR(64) DEFAULT NULL,
    active INTEGER DEFAULT 1,
    UNIQUE KEY name (first_name, last_name),
    KEY email (email),
    KEY scope_id (scope_id)
);
