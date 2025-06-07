-- mutsea-database/src/sql/postgresql/world_state/world_state_statistics.sql
SELECT 
    COUNT(*) as total_records,
    MIN(timestamp) as earliest_timestamp,
    MAX(timestamp) as latest_timestamp,
    AVG(EXTRACT(EPOCH FROM (updated_at - created_at))) as avg_processing_time_seconds,
    COUNT(DISTINCT DATE_TRUNC('hour', timestamp)) as unique_hours_covered,
    COUNT(DISTINCT DATE_TRUNC('day', timestamp)) as unique_days_covered,
    pg_size_pretty(pg_total_relation_size('world_states')) as table_size
FROM world_states 
WHERE timestamp BETWEEN :start_time AND :end_time;