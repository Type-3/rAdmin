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
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE permissions (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name VARCHAR UNIQUE NOT NULL,
  label VARCHAR,
  description TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE roles(
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name VARCHAR UNIQUE NOT NULL,
  label VARCHAR,
  description TEXT,
  is_super BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE account_permissions (
  account_id UUID REFERENCES accounts(id) NOT NULL,
  permission_id UUID REFERENCES permissions(id) NOT NULL,
  PRIMARY KEY(account_id, permission_id)
);

CREATE TABLE account_roles (
  account_id UUID REFERENCES accounts(id) NOT NULL,
  role_id UUID REFERENCES roles(id) NOT NULL,
  PRIMARY KEY(account_id, role_id)
);

CREATE TABLE role_permissions (
  role_id UUID REFERENCES roles(id) NOT NULL,
  permission_id UUID REFERENCES permissions(id) NOT NULL,
  PRIMARY KEY (role_id, permission_id)
);

SELECT diesel_manage_updated_at('accounts');
SELECT diesel_manage_updated_at('permissions');
SELECT diesel_manage_updated_at('roles');

CREATE UNIQUE INDEX account_username_idx ON accounts(username);
CREATE UNIQUE INDEX account_auth_token_idx ON accounts(auth_token);
CREATE UNIQUE INDEX permission_name_idx ON permissions(name);
CREATE UNIQUE INDEX role_name_idx ON roles(name);
