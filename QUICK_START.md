# Quick Start Guide

## Prerequisites Check
```bash
# Check Rust
rustc --version  # Should be 1.75+

# Check Python
python3 --version  # Should be 3.8+

# Check PostgreSQL client
psql --version
```

## 5-Minute Setup

### 1. Initial Setup
```bash
# Run setup script
./scripts/setup.sh

# This will:
# - Create .env file
# - Install Python dependencies
# - Build Rust binaries
```

### 2. Configure Database
```bash
# Edit .env file
nano .env

# Add your Neon PostgreSQL URL:
# DATABASE_URL=postgresql://user:pass@neon.tech/dbname?sslmode=require
```

### 3. Run Migrations
```bash
./scripts/migrate.sh
```

### 4. Start Services

**Terminal 1 - API**:
```bash
cargo run --release --bin pm-api
```

**Terminal 2 - Worker**:
```bash
# For testing (collects every minute)
COLLECTION_INTERVAL_SECONDS=60 cargo run --release --bin pm-worker
```

Wait 1-2 minutes for the worker to collect initial data.

### 5. Test CLI

**Terminal 3**:
```bash
# List markets
./target/release/pm-cli list

# Search
./target/release/pm-cli search "election"

# Get first market ID from list, then:
./target/release/pm-cli detail <market-id>
./target/release/pm-cli history <market-id>
```

### 6. Test Visualization
```bash
# Get a market ID from the list command, then:
python3 viz/terminal_viz.py --market-id <uuid>
```

## Docker Alternative

```bash
# Start everything with Docker
docker-compose up

# API will be at http://localhost:3000
# Check with: curl http://localhost:3000/health
```

## Verify It's Working

1. **API Health**: `curl http://localhost:3000/health` → "OK"
2. **Markets Exist**: `curl http://localhost:3000/api/markets` → JSON array
3. **Worker Logs**: Should see "Collected X markets" messages
4. **CLI Works**: `./target/release/pm-cli list` → Shows markets
5. **Viz Works**: Python script shows tables and charts

## Common Issues

### "Database connection failed"
- Check DATABASE_URL in .env
- Verify Neon database is accessible
- Test with: `psql $DATABASE_URL -c "SELECT 1;"`

### "No markets found"
- Worker needs 1-2 minutes to collect initial data
- Check worker logs for errors
- Verify APIs are accessible: `curl https://gamma-api.polymarket.com/markets?limit=1`

### "Port 3000 already in use"
- Change API_PORT in .env
- Or kill existing process: `lsof -ti:3000 | xargs kill`

### "Cargo not found"
- Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Production Deployment

See README.md section "Deployment" for detailed instructions on deploying to Render + Neon.

## Next Steps

- Read full README.md for detailed documentation
- Explore API endpoints: http://localhost:3000/api/*
- Customize collection interval in .env
- Add more markets by increasing TRACKED_MARKETS

## Support

Check logs:
```bash
# Detailed API logs
RUST_LOG=debug cargo run --bin pm-api

# Detailed worker logs
RUST_LOG=debug cargo run --bin pm-worker
```

View database:
```bash
# Check tables
psql $DATABASE_URL -c "\dt"

# Check market count
psql $DATABASE_URL -c "SELECT COUNT(*) FROM markets;"

# Check price history count
psql $DATABASE_URL -c "SELECT COUNT(*) FROM price_history;"
```
