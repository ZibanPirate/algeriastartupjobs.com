-- SQLite
CREATE TABLE post (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  slug TEXT NOT NULL,
  title TEXT NOT NULL,
  poster_id INTEGER NOT NULL,
  short_description TEXT NOT NULL,
  description TEXT NOT NULL,
  tag_ids TEXT NOT NULL,
  published_at TEXT NOT NULL DEFAULT '',
  is_published BOOLEAN GENERATED ALWAYS AS (published_at <> '') STORED,
  deleted_at TEXT NOT NULL DEFAULT '',
  is_deleted BOOLEAN GENERATED ALWAYS AS (deleted_at <> '') STORED,
  created_at TEXT NOT NULL
);
CREATE INDEX idx_post_published_at ON post (published_at);
CREATE INDEX idx_post_is_published ON post (is_published);
CREATE INDEX idx_post_deleted_at ON post (deleted_at);
CREATE INDEX idx_post_is_deleted ON post (is_deleted);
CREATE INDEX idx_post_created_at ON post (created_at);