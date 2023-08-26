-- SQLite
CREATE TABLE tag (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  slug TEXT NOT NULL,
  created_at TEXT NOT NULL
);
CREATE INDEX idx_tag_name ON tag (name);
CREATE INDEX idx_tag_created_at ON tag (created_at);
