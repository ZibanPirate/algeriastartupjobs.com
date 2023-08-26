-- SQLite
CREATE TABLE word (
  id INTEGER PRIMARY KEY,
  word TEXT NOT NULL,
  model_type TEXT NOT NULL,
  model_id INTEGER NOT NULL,
  appear_in TEXT NOT NULL
);
-- @TODO-ZM: btw add indexes to other models, such as task.status, type ...etc.
