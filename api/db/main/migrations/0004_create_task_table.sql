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
