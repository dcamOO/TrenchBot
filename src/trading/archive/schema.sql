CREATE TABLE IF NOT EXISTS identity (
    version TEXT PRIMARY KEY CHECK(version = 'helius-mainnet-pump-create-v1')
);
INSERT OR IGNORE INTO identity VALUES ('helius-mainnet-pump-create-v1');
CREATE TABLE IF NOT EXISTS deployments (
    mint TEXT PRIMARY KEY,
    creator TEXT NOT NULL,
    launched_at INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS creator_tokens ON deployments(creator);
CREATE TABLE IF NOT EXISTS progress (
    wallet TEXT PRIMARY KEY,
    state TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS requests (
    day INTEGER PRIMARY KEY,
    used INTEGER NOT NULL
);
