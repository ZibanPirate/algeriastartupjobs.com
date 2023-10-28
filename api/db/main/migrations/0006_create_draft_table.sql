-- SQLite
CREATE TABLE imported_content (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  source_url TEXT,
  type TEXT NOT NULL,
  -- name = JobPost
  json_data TEXT NOT NULL,
  status TEXT NOT NULL,
  -- status = Failure
  failure_reason TEXT,
  --
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL DEFAULT ''
);
CREATE INDEX idx_imported_content_source_url ON imported_content (source_url);
CREATE INDEX idx_imported_content_type ON imported_content (type);
CREATE INDEX idx_imported_content_status ON imported_content (status);
CREATE INDEX idx_imported_content_created_at ON imported_content (created_at);
CREATE INDEX idx_imported_content_updated_at ON imported_content (updated_at);
