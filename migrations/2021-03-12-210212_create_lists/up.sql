-- Your SQL goes here
CREATE TABLE lists (
  id INTEGER PRIMARY KEY NOT NULL,
  title VARCHAR NOT NULL,
  detail TEXT NOT NULL,
  done BOOLEAN NOT NULL DEFAULT 0
)