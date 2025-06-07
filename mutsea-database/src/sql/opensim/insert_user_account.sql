-- src/sql/opensim/insert_user_account.sql
INSERT INTO user_accounts (
    principal_id, scope_id, first_name, last_name, 
    email, created, user_level, user_flags, active
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);