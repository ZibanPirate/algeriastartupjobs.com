-- SQLite
CREATE TABLE account (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  email TEXT UNIQUE NOT NULL,
  slug TEXT NOT NULL,
  type TEXT NOT NULL,
  first_name TEXT,
  last_name TEXT,
  company_name TEXT,
  created_at TEXT NOT NULL
);
CREATE INDEX idx_account_type ON account (type);
CREATE INDEX idx_account_created_at ON account (created_at);
