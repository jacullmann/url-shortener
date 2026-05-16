CREATE TABLE IF NOT EXISTS urls (
    id           TEXT NOT NULL PRIMARY KEY,
    original_url TEXT NOT NULL UNIQUE
);

CREATE INDEX IF NOT EXISTS idx_original_url ON urls (original_url);