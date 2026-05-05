-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE TYPE types AS ENUM ('recharge', 'payment', 'withdrawal');
CREATE TYPE transaction_status AS ENUM ('pending', 'success', 'failed');

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    

    from_wallet_id UUID NOT NULL REFERENCES wallets(id),
    to_wallet_id UUID NOT NULL REFERENCES wallets(id),

    amount NUMERIC(12,2) NOT NULL CHECK (amount > 0),

    transaction_type types NOT NULL,
    transaction_status transaction_status NOT NULL DEFAULT 'pending',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_transactions_from_wallet_id ON transactions(from_wallet_id);
CREATE INDEX idx_transactions_to_wallet_id ON transactions(to_wallet_id);