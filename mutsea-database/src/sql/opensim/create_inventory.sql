-- src/sql/opensim/create_inventory.sql
-- OpenSim inventory tables
CREATE TABLE IF NOT EXISTS inventoryfolders (
    folder_id VARCHAR(36) NOT NULL PRIMARY KEY,
    agent_id VARCHAR(36) DEFAULT NULL,
    parent_folder_id VARCHAR(36) DEFAULT NULL,
    folder_name VARCHAR(64) DEFAULT NULL,
    type SMALLINT(6) DEFAULT NULL,
    version INTEGER DEFAULT NULL,
    KEY agent_id (agent_id),
    KEY parent_folder_id (parent_folder_id)
);

CREATE TABLE IF NOT EXISTS inventoryitems (
    inventory_id VARCHAR(36) NOT NULL PRIMARY KEY,
    asset_id VARCHAR(36) DEFAULT NULL,
    asset_type INTEGER DEFAULT NULL,
    parent_folder_id VARCHAR(36) DEFAULT NULL,
    avatar_id VARCHAR(36) DEFAULT NULL,
    inventory_name VARCHAR(64) DEFAULT NULL,
    inventory_description VARCHAR(128) DEFAULT NULL,
    inventory_next_permissions INTEGER DEFAULT NULL,
    inventory_current_permissions INTEGER DEFAULT NULL,
    inv_type INTEGER DEFAULT NULL,
    creator_id VARCHAR(36) DEFAULT NULL,
    inventory_base_permissions INTEGER DEFAULT NULL,
    inventory_everyone_permissions INTEGER DEFAULT NULL,
    sale_price INTEGER DEFAULT NULL,
    sale_type TINYINT(4) DEFAULT NULL,
    creation_date INTEGER DEFAULT NULL,
    group_id VARCHAR(36) DEFAULT NULL,
    group_owned BOOLEAN DEFAULT NULL,
    last_owner_id VARCHAR(36) DEFAULT NULL,
    inventory_group_permissions INTEGER DEFAULT NULL,
    KEY avatar_id (avatar_id),
    KEY parent_folder_id (parent_folder_id)
);
