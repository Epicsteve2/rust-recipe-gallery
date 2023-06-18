CREATE TABLE comments (
  id UUID PRIMARY KEY,
  recipe_id UUID NOT NULL REFERENCES recipes (id),
  comment TEXT NOT NULL
)
