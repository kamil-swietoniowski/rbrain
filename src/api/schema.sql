CREATE TABLE IF NOT EXISTS note (
  id INTEGER PRIMARY KEY,
  title TEXT,
  content TEXT,
  modified_at TEXT,
  created_at TEXT
);

CREATE TABLE IF NOT EXISTS category (
  id INTEGER PRIMARY KEY,
  category_name TEXT UNIQUE
);

CREATE TABLE IF NOT EXISTS relation (
  note_id INTEGER NOT NULL,
  category_id INTEGER NOT NULL,
  PRIMARY KEY (note_id, category_id),
  FOREIGN KEY (note_id) REFERENCES note(id) ON DELETE CASCADE,
  FOREIGN KEY (category_id) REFERENCES category(id) ON DELETE CASCADE
);
