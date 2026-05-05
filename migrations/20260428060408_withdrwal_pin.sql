-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE withdrawal_pins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vendor_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    hashed_pin TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_card_pins_vendor_id ON withdrawal_pins(vendor_id);