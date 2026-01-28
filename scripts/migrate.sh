#!/bin/bash
set -e

# Load environment variables
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

if [ -z "$DATABASE_URL" ]; then
    echo "❌ DATABASE_URL not set. Please configure .env file."
    exit 1
fi

echo "🗄️  Running database migrations..."
echo "===================================="

for migration in migrations/*.sql; do
    echo ""
    echo "Running $(basename $migration)..."
    psql "$DATABASE_URL" -f "$migration"
    echo "✓ $(basename $migration) completed"
done

echo ""
echo "===================================="
echo "✅ All migrations completed successfully!"
