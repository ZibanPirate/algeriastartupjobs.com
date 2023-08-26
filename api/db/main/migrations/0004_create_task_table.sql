-- SQLite
CREATE TABLE task (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  -- name = Indexing
  model_name TEXT,
  model_id INTEGER,
  --
  type TEXT NOT NULL,
  -- type = Manual
  manual_task_owner INTEGER,
  --
  status TEXT NOT NULL,
  -- status = Failure
  failure_reason TEXT,
  --
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL DEFAULT ''
);
CREATE INDEX idx_task_name ON task (name);
CREATE INDEX idx_task_model_name ON task (model_name);
CREATE INDEX idx_task_model_id ON task (model_id);
CREATE INDEX idx_task_type ON task (type);
CREATE INDEX idx_task_status ON task (status);
CREATE INDEX idx_task_created_at ON task (created_at);
CREATE INDEX idx_task_updated_at ON task (updated_at);
