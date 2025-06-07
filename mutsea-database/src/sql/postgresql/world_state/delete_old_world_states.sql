-- mutsea-database/src/sql/postgresql/world_state/delete_old_world_states.sql
DELETE FROM world_states 
WHERE timestamp < :cutoff_time
RETURNING id, timestamp;