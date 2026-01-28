# Prediction Market History Tracker

A production-ready prediction market history tracking system with Neon PostgreSQL backend, Rust API/CLI, and Python visualization. Track price movements, volume, and liquidity across multiple prediction market platforms.

## Credits

**Original Project**: [pm-indexer by 0xqtpie](https://github.com/0xqtpie/pm-indexer)
- API ingestion patterns with retry logic
- Database schema inspiration
- Sync scheduler architecture

**This Fork**: Focus on historical price tracking, Rust-based architecture, and terminal visualization

## Features

- 📊 **Historical Price Tracking**: Record price snapshots at configurable intervals
- 🔍 **Fuzzy Search**: Fast text-based search across market titles and descriptions
- 📈 **Interactive Charts**: Terminal-based visualization with Rich and Plotly
- 🔄 **Multi-Platform**: Supports Polymarket and Kalshi
- 🚀 **Production Ready**: Built with Rust for performance and reliability
- 🎯 **RESTful API**: Query markets and price history via HTTP
- 💻 **CLI Tools**: Command-line interface for quick market lookups

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Rust Axum API Server                     │
│  /api/search  │  /api/markets/:id  │  /api/markets/:id/history │
└─────────────────────────────────────────────────────────────┘
                                │
┌───────────────────────────────┼───────────────────────────────┐
│                  Background Worker (Tokio)                     │
│  Scheduled Collection → Polymarket/Kalshi → Database          │
└───────────────────────────────────────────────────────────────┘
                                │
┌───────────────────────────────┼───────────────────────────────┐
│                    Neon PostgreSQL Database                    │
│  markets (metadata) → price_history (time-series)             │
└───────────────────────────────────────────────────────────────┘
                                │
┌───────────────────────────────┼───────────────────────────────┐
│              CLI + Visualization Layer                         │
│  Rust CLI + Python Viz (Rich + Plotly)                        │
└───────────────────────────────────────────────────────────────┘
```

## Quick Start

### Prerequisites

- Rust 1.75+ ([Install](https://rustup.rs/))
- Python 3.8+ ([Install](https://www.python.org/downloads/))
- PostgreSQL client (`psql`)
- Neon PostgreSQL database ([Create free account](https://neon.tech/))

### Installation

1. **Clone and setup**:
   ```bash
   cd pm-history-tracker
   ./scripts/setup.sh
   ```

2. **Configure database**:
   ```bash
   # Edit .env and add your Neon database URL
   vim .env
   # DATABASE_URL=postgresql://user:pass@neon.tech/dbname?sslmode=require
   ```

3. **Run migrations**:
   ```bash
   ./scripts/migrate.sh
   ```

4. **Seed initial data** (optional):
   ```bash
   ./scripts/seed.sh
   ```

### Running the System

**Terminal 1 - API Server**:
```bash
cargo run --release --bin pm-api
```

**Terminal 2 - Background Worker**:
```bash
cargo run --release --bin pm-worker
```

**Terminal 3 - CLI**:
```bash
# Search markets
./target/release/pm-cli search "bitcoin"

# Get market details
./target/release/pm-cli detail <market-id>

# View price history
./target/release/pm-cli history <market-id> --hours 24

# List top markets
./target/release/pm-cli list --limit 20
```

**Python Visualization**:
```bash
# Interactive terminal charts
python3 viz/terminal_viz.py --market-id <uuid> --api-url http://localhost:3000

# Save chart to HTML
python3 viz/terminal_viz.py --market-id <uuid> --output chart.html
```

## API Endpoints

### Search Markets
```
GET /api/search?q=bitcoin&limit=10&source=polymarket&status=open
```

Returns markets matching the search query with relevance scores.

### List Markets
```
GET /api/markets?limit=20&sort=volume&order=desc
```

Returns paginated list of markets sorted by specified field.

### Get Market Detail
```
GET /api/markets/{id}
```

Returns full metadata for a specific market.

### Get Price History
```
GET /api/markets/{id}/history?hours=24&limit=100
```

Returns time-series price snapshots for a market.

## Database Schema

### `markets` Table
Stores prediction market metadata and current state:
- Market identification (source, source_id)
- Content (title, description, category, tags)
- Current pricing (yes_price, no_price)
- Volume and liquidity metrics
- Status and timestamps

### `price_history` Table
Stores time-series snapshots:
- Market reference (market_id)
- Price snapshot (yes_price, no_price)
- Volume and liquidity at snapshot time
- Recording timestamp

### Indexes
- Full-text search on title/description
- Time-series optimized for recent queries
- Partial index for last 30 days

## Configuration

### Environment Variables

```bash
# Database
DATABASE_URL=postgresql://...

# API Server
API_PORT=3000
API_HOST=0.0.0.0

# Worker
WORKER_ENABLED=true
COLLECTION_INTERVAL_SECONDS=3600  # 1 hour production, 60 for testing
TRACKED_MARKETS=10

# Logging
RUST_LOG=info
```

### Collection Intervals

- **Production**: 3600 seconds (1 hour)
- **Testing**: 60 seconds (1 minute)
- **Aggressive**: 300 seconds (5 minutes)

## Project Structure

```
pm-history-tracker/
├── Cargo.toml              # Workspace manifest
├── README.md               # This file
├── .env.example            # Example configuration
├── .gitignore
│
├── crates/
│   ├── api/                # Axum API server
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── routes/     # API endpoints
│   │   │   ├── db/         # Database queries
│   │   │   ├── error.rs
│   │   │   └── config.rs
│   │
│   ├── worker/             # Background data collector
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── scheduler.rs
│   │   │   ├── collectors/ # Polymarket, Kalshi
│   │   │   └── recorder.rs
│   │
│   ├── cli/                # Command-line interface
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── api_client.rs
│   │   │   └── commands/
│   │
│   └── shared/             # Common types
│       └── src/models.rs
│
├── viz/                    # Python visualization
│   ├── requirements.txt
│   └── terminal_viz.py
│
├── migrations/             # Database schema
│   ├── 001_create_markets.sql
│   └── 002_create_price_history.sql
│
└── scripts/
    ├── setup.sh            # Initial setup
    ├── migrate.sh          # Run migrations
    └── seed.sh             # Seed test data
```

## CLI Usage Examples

### Search Markets
```bash
pm-cli search "election" --limit 5
pm-cli search "crypto" --limit 10
```

### View Market Details
```bash
pm-cli detail 550e8400-e29b-41d4-a716-446655440000
```

### View Price History
```bash
# Last 24 hours
pm-cli history <market-id> --hours 24

# Last week
pm-cli history <market-id> --hours 168

# All available data
pm-cli history <market-id>
```

### List Top Markets
```bash
pm-cli list --limit 10
```

## Python Visualization Features

### Terminal Output
- Rich tables with colored price data
- ASCII sparkline charts for quick trends
- Market metadata display

### Interactive Charts
- Plotly line charts for price history
- Volume bar charts
- Dual-axis subplots
- HTML export for sharing

### Example
```bash
# View in terminal with interactive chart
python3 viz/terminal_viz.py \
  --market-id 550e8400-e29b-41d4-a716-446655440000 \
  --api-url http://localhost:3000

# Save chart to file
python3 viz/terminal_viz.py \
  --market-id 550e8400-e29b-41d4-a716-446655440000 \
  --output market_chart.html

# Last 48 hours only
python3 viz/terminal_viz.py \
  --market-id 550e8400-e29b-41d4-a716-446655440000 \
  --hours 48
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Specific binary
cargo build --release --bin pm-api
```

### Testing

```bash
# Run all tests
cargo test

# Specific crate
cargo test -p pm-shared
```

### Running in Development

```bash
# API with auto-reload (requires cargo-watch)
cargo watch -x 'run --bin pm-api'

# Worker with logs
RUST_LOG=debug cargo run --bin pm-worker
```

## Deployment

### Local Development
```bash
# Terminal 1
cargo run --bin pm-api

# Terminal 2
cargo run --bin pm-worker
```

### Production (Render + Neon)

1. **Create Neon Database**:
   - Sign up at [neon.tech](https://neon.tech/)
   - Create new project
   - Copy connection string

2. **Deploy API to Render**:
   - Create new Web Service
   - Connect GitHub repository
   - Build command: `cargo build --release --bin pm-api`
   - Start command: `./target/release/pm-api`
   - Add environment variables

3. **Deploy Worker to Render**:
   - Create new Background Worker
   - Build command: `cargo build --release --bin pm-worker`
   - Start command: `./target/release/pm-worker`
   - Add environment variables

4. **Run Migrations**:
   ```bash
   psql $DATABASE_URL -f migrations/001_create_markets.sql
   psql $DATABASE_URL -f migrations/002_create_price_history.sql
   ```

## Performance Optimizations

### Database
- GIN indexes for full-text search
- Partial indexes for recent data (30 days)
- Connection pooling (10-20 connections)
- Batch inserts for history snapshots

### API
- Pagination limits (max 100 per page)
- Query timeouts (30 seconds)
- CORS and rate limiting ready

### Worker
- Configurable collection intervals
- Rate limiting between API calls
- Exponential backoff on errors
- Batch recording of snapshots

## Troubleshooting

### Database Connection Issues
```bash
# Test connection
psql $DATABASE_URL -c "SELECT 1;"

# Check migrations
psql $DATABASE_URL -c "\dt"
```

### API Not Starting
```bash
# Check port availability
lsof -i :3000

# Verify environment
cargo run --bin pm-api -- --help
```

### Worker Not Collecting
```bash
# Check logs
RUST_LOG=debug cargo run --bin pm-worker

# Verify API access
curl https://gamma-api.polymarket.com/markets?limit=1
```

## Contributing

This is a personal project based on pm-indexer. If you find it useful and want to contribute:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT License - See LICENSE file for details

## Acknowledgments

- **pm-indexer** by 0xqtpie: Original inspiration and architecture patterns
- **Polymarket** and **Kalshi**: Public APIs for prediction market data
- **Neon**: Serverless PostgreSQL platform
- **Rust Community**: Amazing ecosystem and tools

## Resources

- [pm-indexer Original Project](https://github.com/0xqtpie/pm-indexer)
- [Polymarket API](https://docs.polymarket.com/)
- [Kalshi API](https://docs.kalshi.com/)
- [Neon Database](https://neon.tech/)
- [Rust Documentation](https://doc.rust-lang.org/)

---

**Built with**: Rust 🦀 | Axum | PostgreSQL | Python | Rich | Plotly
