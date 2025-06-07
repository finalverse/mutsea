-- src/sql/opensim/insert_asset.sql
INSERT INTO assets (
    id, name, description, asset_type, local, temporary, 
    data, create_time, access_time, asset_flags, creator_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);