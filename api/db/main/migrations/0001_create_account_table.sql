-- SQLite
-- create account table
CREATE TABLE account (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  email TEXT UNIQUE NOT NULL,
  slug TEXT NOT NULL,
  type TEXT NOT NULL,
  first_name TEXT,
  last_name TEXT,
  company_name TEXT,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);