-- Migration: create_master_password_table
-- Description: Creates the master_password table for storing the master password hash

CREATE TABLE IF NOT EXISTS master_password (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    password_hash TEXT NOT NULL
);

INSERT INTO master_password (id, password_hash)
VALUES (1, "");