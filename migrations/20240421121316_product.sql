-- Add migration script here
CREATE TABLE IF NOT EXISTS product
(
    id INTEGER PRIMARY KEY NOT NULL,
    slug VARCHAR(64) UNIQUE NOT NULL,
    name TEXT NOT NULL DEFAULT ""
)