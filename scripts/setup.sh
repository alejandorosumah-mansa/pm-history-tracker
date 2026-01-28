#!/bin/bash
set -e

echo "🚀 PM History Tracker - Setup Script"
echo "===================================="

# Check for required tools
echo ""
echo "Checking prerequisites..."

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install from https://rustup.rs/"
    exit 1
fi
echo "✓ Rust/Cargo found"

if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 not found. Please install Python 3.8+"
    exit 1
fi
echo "✓ Python 3 found"

if ! command -v psql &> /dev/null; then
    echo "⚠️  PostgreSQL client (psql) not found. You'll need it to run migrations."
else
    echo "✓ PostgreSQL client found"
fi

# Create .env if it doesn't exist
if [ ! -f .env ]; then
    echo ""
    echo "Creating .env file from .env.example..."
    cp .env.example .env
    echo "✓ .env created - please update with your database URL"
else
    echo "✓ .env file exists"
fi

# Install Python dependencies
echo ""
echo "Installing Python dependencies..."
python3 -m pip install --quiet -r viz/requirements.txt
echo "✓ Python dependencies installed"

# Build Rust projects
echo ""
echo "Building Rust projects (this may take a while)..."
cargo build --release
echo "✓ Rust projects built"

echo ""
echo "===================================="
echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "1. Update .env with your Neon PostgreSQL database URL"
echo "2. Run migrations: ./scripts/migrate.sh"
echo "3. Start the API: cargo run --release --bin pm-api"
echo "4. Start the worker: cargo run --release --bin pm-worker"
echo "5. Use the CLI: ./target/release/pm-cli --help"
