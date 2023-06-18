-- Your SQL goes here
CREATE TABLE recipes (
  id UUID PRIMARY KEY,
  title VARCHAR NOT NULL,
  ingredients TEXT NOT NULL,
  body TEXT NOT NULL
)
