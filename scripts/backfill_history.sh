#!/bin/bash
# Backfill script - triggers worker to collect more snapshots
# This will run the worker multiple times to build up historical data

set -e

echo "🔄 Backfill Historical Data"
echo "=============================="
echo ""

# Check if worker binary exists
if [ ! -f "./target/release/pm-worker" ]; then
    echo "❌ Worker binary not found. Building..."
    cargo build --release --bin pm-worker
fi

# Get number of iterations (default 10 = 10 snapshot cycles)
ITERATIONS=${1:-10}
SLEEP_SECONDS=${2:-30}

echo "📊 Will collect $ITERATIONS snapshot cycles"
echo "⏱️  Waiting $SLEEP_SECONDS seconds between cycles"
echo ""

for i in $(seq 1 $ITERATIONS); do
    echo "[$i/$ITERATIONS] Running collection cycle..."

    # Run worker for one cycle
    timeout 60s ./target/release/pm-worker || true

    # Check database growth
    COUNT=$(psql "$DATABASE_URL" -t -c "SELECT COUNT(*) FROM price_history;")
    echo "  📈 Total snapshots: $COUNT"

    if [ $i -lt $ITERATIONS ]; then
        echo "  ⏳ Waiting $SLEEP_SECONDS seconds..."
        sleep $SLEEP_SECONDS
    fi
    echo ""
done

echo "✅ Backfill complete!"
echo ""
echo "📊 Final Statistics:"
psql "$DATABASE_URL" -c "
SELECT
  (SELECT COUNT(*) FROM markets) as markets,
  (SELECT COUNT(*) FROM price_history) as snapshots,
  (SELECT AVG(snapshot_count) FROM (
    SELECT market_id, COUNT(*) as snapshot_count
    FROM price_history
    GROUP BY market_id
  ) AS counts) as avg_snapshots_per_market;
"
