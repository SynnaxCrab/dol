CREATE TABLE discoveries
(
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  jp_name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  level INTEGER NOT NULL
)