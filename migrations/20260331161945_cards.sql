-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE TYPE card_status AS ENUM ('active', 'blocked');

CREATE TABLE cards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    cardname varchar(255) NOT NULL,
    phonenumber varchar(255) NOT NULL,

    card_status card_status NOT NULL DEFAULT 'active',
    card_qr_code varchar(255),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cards_parent_id ON cards(parent_id);  