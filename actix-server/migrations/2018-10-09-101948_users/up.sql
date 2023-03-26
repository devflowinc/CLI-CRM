-- Your SQL goes here
CREATE TABLE users (
  id UUID NOT NULL UNIQUE PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  name TEXT DEFAULT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
);