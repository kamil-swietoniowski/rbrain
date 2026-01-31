CREATE TABLE IF NOT EXISTS record (
  id INTEGER PRIMARY KEY,
  title TEXT,
  content BLOB,
  modified_at TEXT,
  created_at TEXT
);

CREATE TABLE IF NOT EXISTS tag (
  id INTEGER PRIMARY KEY,
  tag TEXT UNIQUE
);

CREATE TABLE IF NOT EXISTS relation (
  record_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL,
  PRIMARY KEY (record_id, tag_id),
  FOREIGN KEY (record_id) REFERENCES record(id) ON DELETE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tag(id) ON DELETE CASCADE
);

