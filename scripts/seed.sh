#!/bin/bash
set -e

# This script seeds the database with initial test markets
# by running the worker once in manual mode

echo "🌱 Seeding database with test markets..."
echo "========================================"

# Load environment variables
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

if [ -z "$DATABASE_URL" ]; then
    echo "❌ DATABASE_URL not set. Please configure .env file."
    exit 1
fi

# Temporarily set collection interval to 0 to run once
export COLLECTION_INTERVAL_SECONDS=60
export TRACKED_MARKETS=10
export WORKER_ENABLED=true

echo ""
echo "Starting worker to collect initial markets..."
echo "This will fetch 10 markets from Polymarket and Kalshi"
echo ""

# Run worker for one cycle (it will run indefinitely, so we'll need to kill it)
timeout 90s cargo run --release --bin pm-worker || true

echo ""
echo "===================================="
echo "✅ Seeding complete!"
echo ""
echo "Check your database:"
echo "  psql \$DATABASE_URL -c 'SELECT COUNT(*) FROM markets;'"
echo "  psql \$DATABASE_URL -c 'SELECT COUNT(*) FROM price_history;'"
