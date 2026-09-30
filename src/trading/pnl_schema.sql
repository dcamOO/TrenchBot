-- Schema PostgreSQL para histórico de transações, configurações e PnL (TrenchBot)

CREATE TABLE IF NOT EXISTS trades (
    id SERIAL PRIMARY KEY,
    mint VARCHAR(64) NOT NULL,
    opened_at BIGINT NOT NULL,
    closed_at BIGINT NOT NULL,
    cost_lamports BIGINT NOT NULL,
    received_lamports BIGINT NOT NULL,
    pnl_lamports BIGINT NOT NULL,
    pnl_sol NUMERIC(18, 9) NOT NULL,
    pnl_percent NUMERIC(8, 2) NOT NULL,
    reason VARCHAR(32) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_trades_mint ON trades(mint);
CREATE INDEX IF NOT EXISTS idx_trades_closed_at ON trades(closed_at);

CREATE TABLE IF NOT EXISTS user_configs (
    id SERIAL PRIMARY KEY,
    user_wallet VARCHAR(64) UNIQUE NOT NULL,
    stop_loss_percent NUMERIC(5, 2) NOT NULL,
    take_profit_percent NUMERIC(5, 2) NOT NULL,
    sol_per_trade VARCHAR(32) NOT NULL,
    sniping_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    max_creator_launches INT NOT NULL DEFAULT 3,
    min_ath_market_cap_usd NUMERIC(18, 2) NOT NULL DEFAULT 1000000.0,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
