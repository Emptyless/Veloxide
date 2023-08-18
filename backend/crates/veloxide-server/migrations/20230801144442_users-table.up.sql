CREATE TABLE users (
  id UUID PRIMARY KEY,
  email VARCHAR(255) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL default statement_timestamp(),
  updated_at TIMESTAMPTZ NOT NULL default statement_timestamp(),
  token_salt UUID NOT NULL
);