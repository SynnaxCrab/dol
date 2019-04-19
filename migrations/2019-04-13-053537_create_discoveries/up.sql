CREATE TABLE discoveries
(
  id SERIAL PRIMARY KEY,
  uid INTEGER NOT NULL,
  category VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  level INTEGER,
  points INTEGER,
  era VARCHAR,
  difficulty INTEGER,
  exp INTEGER,
  note VARCHAR,
  link VARCHAR
)