CREATE TABLE oauth2_states (
  id UUID PRIMARY KEY,
  csrf_state VARCHAR(255) NOT NULL,
  code_verifier VARCHAR(255) NOT NULL,
  return_url VARCHAR(255) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL default statement_timestamp()
)