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
  created_at TEXT NOT NULL
);
