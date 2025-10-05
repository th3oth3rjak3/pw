CREATE TABLE IF NOT EXISTS password_entries (
    id INTEGER PRIMARY KEY NOT NULL,
    site TEXT NOT NULL,
    username TEXT NOT NULL,
    password_hash TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_password_entries_site ON password_entries(site);
CREATE INDEX IF NOT EXISTS idx_password_entries_username ON password_entries(username);

CREATE TABLE IF NOT EXISTS master_password (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    password_hash TEXT NOT NULL,
    key_salt TEXT NOT NULL DEFAULT ''
);

INSERT INTO master_password (id, password_hash, key_salt)
VALUES (1, '', '');