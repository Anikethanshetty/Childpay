-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE card_pins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    card_id UUID NOT NULL REFERENCES cards(id) ON DELETE CASCADE,

    hashed_pin TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_card_pins_card_id ON card_pins(card_id);