-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE wallets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    user_id UUID UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    card_id UUID UNIQUE REFERENCES cards(id) ON DELETE CASCADE,
    
    balance NUMERIC(12,2) NOT NULL DEFAULT 0 CHECK (balance >= 0),
    locked_balance NUMERIC(12,2) NOT NULL DEFAULT 0 CHECK (locked_balance >= 0),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CHECK (
        (user_id IS NOT NULL AND card_id IS NULL) OR
        (user_id IS NULL AND card_id IS NOT NULL)
    )
);

CREATE UNIQUE INDEX idx_wallets_user ON wallets(user_id);
CREATE UNIQUE INDEX idx_wallets_card ON wallets(card_id);

