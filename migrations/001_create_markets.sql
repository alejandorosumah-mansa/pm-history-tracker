-- Migration: Create markets table
-- Stores prediction market metadata and current state
-- Inspired by pm-indexer schema (https://github.com/0xqtpie/pm-indexer)

CREATE TABLE IF NOT EXISTS markets (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Source identification
    source_id VARCHAR(255) NOT NULL,
    source VARCHAR(50) NOT NULL,

    -- Market content
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    category VARCHAR(255),
    tags TEXT[],

    -- Current pricing
    yes_price REAL NOT NULL,
    no_price REAL NOT NULL,

    -- Volume and liquidity
    volume REAL NOT NULL DEFAULT 0,
    volume_24h REAL NOT NULL DEFAULT 0,
    liquidity REAL,

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'open',

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    close_at TIMESTAMPTZ,

    -- External link
    url TEXT NOT NULL,

    -- Ensure unique markets per source
    UNIQUE(source, source_id)
);

-- Performance indexes
CREATE INDEX idx_markets_source ON markets(source);
CREATE INDEX idx_markets_status ON markets(status);
CREATE INDEX idx_markets_volume ON markets(volume DESC);
CREATE INDEX idx_markets_created ON markets(created_at DESC);

-- Full-text search index for fuzzy matching
CREATE INDEX idx_markets_title_search ON markets USING GIN(to_tsvector('english', title));
CREATE INDEX idx_markets_desc_search ON markets USING GIN(to_tsvector('english', description));

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger to automatically update updated_at
CREATE TRIGGER update_markets_updated_at
    BEFORE UPDATE ON markets
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Comments for documentation
COMMENT ON TABLE markets IS 'Prediction market metadata and current state';
COMMENT ON COLUMN markets.source IS 'Platform: polymarket, kalshi, etc.';
COMMENT ON COLUMN markets.source_id IS 'Platform-specific market identifier';
COMMENT ON COLUMN markets.yes_price IS 'Current YES outcome price (0.0-1.0)';
COMMENT ON COLUMN markets.no_price IS 'Current NO outcome price (0.0-1.0)';
COMMENT ON COLUMN markets.volume IS 'Total trading volume in USD';
COMMENT ON COLUMN markets.volume_24h IS '24-hour trading volume in USD';
COMMENT ON COLUMN markets.liquidity IS 'Available liquidity in USD';
COMMENT ON COLUMN markets.status IS 'Market status: open, closed, resolved';
