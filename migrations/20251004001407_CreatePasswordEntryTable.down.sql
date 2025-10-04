-- Migration: create_password_entries_table (DOWN)
-- Description: Drops the password_entries table

DROP INDEX IF EXISTS idx_password_entries_username;
DROP INDEX IF EXISTS idx_password_entries_site;
DROP TABLE IF EXISTS password_entries;