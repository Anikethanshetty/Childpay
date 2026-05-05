-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE TYPE roles AS ENUM ('parent', 'vendor', 'both');

CREATE TABLE if not exists users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username varchar(255) NOT NULL UNIQUE,
    password varchar(255) NOT NULL,
    email varchar(255) NOT NULL UNIQUE,
    phonenumber varchar(255) NOT NULL UNIQUE,

    user_role roles NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_users_email ON users(email);
