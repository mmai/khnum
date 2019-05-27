-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(100) NOT NULL UNIQUE,
  login VARCHAR(30) NOT NULL UNIQUE,
  password VARCHAR(64) NOT NULL, --bcrypt hash
  created_at TIMESTAMP NOT NULL,
  active BOOLEAN NOT NULL DEFAULT FALSE,
  expires_at TIMESTAMP
);
