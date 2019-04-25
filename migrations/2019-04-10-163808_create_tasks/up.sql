-- Your SQL goes here
CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  description VARCHAR NOT NULL,
  finished BOOLEAN NOT NULL DEFAULT 'f'
)
