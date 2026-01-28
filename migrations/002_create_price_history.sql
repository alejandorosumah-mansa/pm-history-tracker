-- Migration: Create price_history table
-- Stores time-series snapshots of market prices and metrics
-- Optimized for historical analysis and charting

CREATE TABLE IF NOT EXISTS price_history (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Foreign key to markets
    market_id UUID NOT NULL REFERENCES markets(id) ON DELETE CASCADE,

    -- Price snapshot
    yes_price REAL NOT NULL,
    no_price REAL NOT NULL,

    -- Volume and liquidity at snapshot time
    volume REAL NOT NULL DEFAULT 0,
    volume_24h REAL NOT NULL DEFAULT 0,
    liquidity REAL,

    -- Snapshot timestamp
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Ensure no duplicate snapshots for same market at same time
    UNIQUE(market_id, recorded_at)
);

-- Critical time-series indexes
-- Primary index for querying a market's history
CREATE INDEX idx_price_history_market_time ON price_history(market_id, recorded_at DESC);

-- Partial index for recent data (30 days) - most common queries
CREATE INDEX idx_price_history_recent ON price_history(market_id, recorded_at DESC)
    WHERE recorded_at > NOW() - INTERVAL '30 days';

-- Index for cleanup queries (finding old records)
CREATE INDEX idx_price_history_recorded ON price_history(recorded_at DESC);

-- Comments for documentation
COMMENT ON TABLE price_history IS 'Time-series snapshots of market prices and metrics';
COMMENT ON COLUMN price_history.market_id IS 'Reference to the market being tracked';
COMMENT ON COLUMN price_history.yes_price IS 'YES outcome price at snapshot time (0.0-1.0)';
COMMENT ON COLUMN price_history.no_price IS 'NO outcome price at snapshot time (0.0-1.0)';
COMMENT ON COLUMN price_history.volume IS 'Total volume at snapshot time in USD';
COMMENT ON COLUMN price_history.volume_24h IS '24-hour volume at snapshot time in USD';
COMMENT ON COLUMN price_history.liquidity IS 'Liquidity at snapshot time in USD';
COMMENT ON COLUMN price_history.recorded_at IS 'When this snapshot was taken';

-- View for recent price changes
CREATE OR REPLACE VIEW market_price_changes AS
SELECT
    m.id,
    m.title,
    m.source,
    m.yes_price as current_yes_price,
    m.no_price as current_no_price,
    ph_24h.yes_price as yes_price_24h_ago,
    ph_24h.no_price as no_price_24h_ago,
    (m.yes_price - ph_24h.yes_price) as yes_price_change_24h,
    (m.no_price - ph_24h.no_price) as no_price_change_24h
FROM markets m
LEFT JOIN LATERAL (
    SELECT yes_price, no_price
    FROM price_history
    WHERE market_id = m.id
      AND recorded_at <= NOW() - INTERVAL '24 hours'
    ORDER BY recorded_at DESC
    LIMIT 1
) ph_24h ON true;

COMMENT ON VIEW market_price_changes IS '24-hour price change summary for all markets';
