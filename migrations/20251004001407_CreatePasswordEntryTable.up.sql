-- Migration: create_password_entries_table
-- Description: Creates the password_entries table for storing encrypted passwords

CREATE TABLE IF NOT EXISTS password_entries (
    id INTEGER PRIMARY KEY NOT NULL,
    site TEXT NOT NULL,
    username TEXT NOT NULL,
    password_hash TEXT NOT NULL
);

-- Index for faster lookups by site
CREATE INDEX IF NOT EXISTS idx_password_entries_site ON password_entries(site);

-- Index for faster lookups by username
CREATE INDEX IF NOT EXISTS idx_password_entries_username ON password_entries(username);