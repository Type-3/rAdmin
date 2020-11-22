-- Your SQL goes here-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE accounts (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  username VARCHAR UNIQUE NOT NULL,
  email VARCHAR UNIQUE NOT NULL,
  password_type SMALLINT NOT NULL,
  password_hash BYTEA NOT NULL,
  password_salt BYTEA NOT NULL,
  auth_token VARCHAR,
  email_verified_at TIMESTAMP,
  avatar UUID,
  roles UUID[],
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE roles(
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name VARCHAR UNIQUE NOT NULL,
  label VARCHAR,
  description TEXT,
  is_super BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('accounts');
SELECT diesel_manage_updated_at('roles');

CREATE UNIQUE INDEX account_username_idx ON accounts(username);
CREATE UNIQUE INDEX account_auth_token_idx ON accounts(auth_token);
CREATE UNIQUE INDEX role_name_idx ON roles(name);
