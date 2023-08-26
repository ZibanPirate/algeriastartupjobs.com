-- SQLite
CREATE TABLE word (
  id INTEGER PRIMARY KEY,
  word TEXT NOT NULL,
  model_type TEXT NOT NULL,
  model_id INTEGER NOT NULL,
  appear_in TEXT NOT NULL
);
CREATE INDEX idx_word_word ON word (word);
CREATE INDEX idx_word_model_type ON word (model_type);
CREATE INDEX idx_word_model_id ON word (model_id);
CREATE INDEX idx_word_appear_in ON word (appear_in);
