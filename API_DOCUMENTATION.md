# PM History Tracker - Public API

> **Public read-only API for querying prediction market data from Polymarket and Kalshi**

[![API Status](https://img.shields.io/badge/API-Live-green)](https://pm-history-api.onrender.com/health)
[![Markets](https://img.shields.io/badge/Markets-1000%2B-blue)](https://pm-history-api.onrender.com/api/markets)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Base URL

```
https://pm-history-api.onrender.com
```

## Quick Start

```bash
# Check API health
curl https://pm-history-api.onrender.com/health

# Search for markets
curl "https://pm-history-api.onrender.com/api/search?q=election"

# Get top 10 markets by volume
curl "https://pm-history-api.onrender.com/api/markets?limit=10&sort=volume"
```

## Endpoints

### 1. Health Check

Check if the API is running.

```http
GET /health
```

**Response:**
```
OK
```

**Example:**
```bash
curl https://pm-history-api.onrender.com/health
```

---

### 2. List Markets

Get a paginated list of prediction markets.

```http
GET /api/markets
```

**Query Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `limit` | integer | 20 | Number of results (max 100) |
| `offset` | integer | 0 | Pagination offset |
| `sort` | string | `created_at` | Sort field: `volume`, `created_at`, `close_at`, `volume_24h` |
| `order` | string | `desc` | Sort order: `asc` or `desc` |

**Response:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "source_id": "market-123",
    "source": "polymarket",
    "title": "Will Bitcoin reach $100k in 2024?",
    "description": "Market resolves YES if Bitcoin...",
    "category": "Crypto",
    "tags": ["bitcoin", "crypto"],
    "yes_price": 0.65,
    "no_price": 0.35,
    "volume": 125000.50,
    "volume_24h": 15000.25,
    "liquidity": 50000.00,
    "status": "open",
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-20T14:25:00Z",
    "close_at": "2024-12-31T23:59:59Z",
    "url": "https://polymarket.com/market/..."
  }
]
```

**Examples:**
```bash
# Get top 10 markets
curl "https://pm-history-api.onrender.com/api/markets?limit=10"

# Get top 10 by volume
curl "https://pm-history-api.onrender.com/api/markets?limit=10&sort=volume&order=desc"

# Get next page (pagination)
curl "https://pm-history-api.onrender.com/api/markets?limit=20&offset=20"
```

---

### 3. Search Markets

Search markets by title and description using full-text search.

```http
GET /api/search
```

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `q` | string | ✅ Yes | Search query |
| `limit` | integer | No | Number of results (max 100, default 10) |
| `source` | string | No | Filter by source: `polymarket` or `kalshi` |
| `status` | string | No | Filter by status: `open`, `closed`, `resolved` |

**Response:**
```json
{
  "results": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "title": "Will Bitcoin reach $100k in 2024?",
      "source": "polymarket",
      "yes_price": 0.65,
      "no_price": 0.35,
      "volume": 125000.50,
      "status": "open",
      "score": 12.5,
      ...
    }
  ],
  "total": 1
}
```

**Examples:**
```bash
# Search for election markets
curl "https://pm-history-api.onrender.com/api/search?q=election"

# Search with filters
curl "https://pm-history-api.onrender.com/api/search?q=bitcoin&source=polymarket&limit=5"

# Search for sports markets
curl "https://pm-history-api.onrender.com/api/search?q=nba+finals"
```

---

### 4. Get Market Details

Get full details for a specific market.

```http
GET /api/markets/{id}
```

**Path Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | UUID | Market ID |

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "source_id": "market-123",
  "source": "polymarket",
  "title": "Will Bitcoin reach $100k in 2024?",
  "description": "Market resolves YES if Bitcoin reaches $100,000 USD...",
  "category": "Crypto",
  "tags": ["bitcoin", "crypto", "price"],
  "yes_price": 0.65,
  "no_price": 0.35,
  "volume": 125000.50,
  "volume_24h": 15000.25,
  "liquidity": 50000.00,
  "status": "open",
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-20T14:25:00Z",
  "close_at": "2024-12-31T23:59:59Z",
  "url": "https://polymarket.com/market/market-123"
}
```

**Example:**
```bash
curl "https://pm-history-api.onrender.com/api/markets/550e8400-e29b-41d4-a716-446655440000"
```

---

### 5. Get Price History

Get historical price snapshots for a market.

```http
GET /api/markets/{id}/history
```

**Path Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | UUID | Market ID |

**Query Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `hours` | integer | - | Limit to last N hours |
| `limit` | integer | 100 | Number of snapshots (max 1000) |

**Response:**
```json
[
  {
    "id": "660e8400-e29b-41d4-a716-446655440000",
    "market_id": "550e8400-e29b-41d4-a716-446655440000",
    "yes_price": 0.65,
    "no_price": 0.35,
    "volume": 125000.50,
    "volume_24h": 15000.25,
    "liquidity": 50000.00,
    "recorded_at": "2024-01-20T14:25:00Z"
  },
  {
    "id": "770e8400-e29b-41d4-a716-446655440000",
    "market_id": "550e8400-e29b-41d4-a716-446655440000",
    "yes_price": 0.62,
    "no_price": 0.38,
    "volume": 120000.00,
    "volume_24h": 12000.00,
    "liquidity": 48000.00,
    "recorded_at": "2024-01-20T09:25:00Z"
  }
]
```

**Examples:**
```bash
# Get last 24 hours of history
curl "https://pm-history-api.onrender.com/api/markets/{id}/history?hours=24"

# Get last week (168 hours)
curl "https://pm-history-api.onrender.com/api/markets/{id}/history?hours=168"

# Get all available history (up to 1000 snapshots)
curl "https://pm-history-api.onrender.com/api/markets/{id}/history?limit=1000"
```

---

## Rate Limits

- **No authentication required** - API is public and read-only
- **Soft limit**: 100 requests per minute per IP
- **Be respectful** of resources

If you need higher rate limits, please open an issue on GitHub.

---

## Data Update Frequency

- **Markets**: Updated every 5 minutes
- **Price History**: New snapshots every 5 minutes
- **Database**: Contains 1000+ active markets

---

## Response Formats

All responses are JSON with appropriate HTTP status codes:

- `200 OK` - Successful request
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server error

**Error Response:**
```json
{
  "error": "Resource not found"
}
```

---

## Data Sources

This API aggregates data from:

- **Polymarket** - Leading decentralized prediction market
- **Kalshi** - CFTC-regulated prediction market

All data is collected from public APIs and is provided as-is.

---

## Use Cases

### Analytics Dashboard
```javascript
// Fetch top markets by volume
const response = await fetch(
  'https://pm-history-api.onrender.com/api/markets?limit=20&sort=volume'
);
const markets = await response.json();
```

### Price Tracking Bot
```python
import requests

# Get price history
response = requests.get(
    f'https://pm-history-api.onrender.com/api/markets/{market_id}/history',
    params={'hours': 24}
)
history = response.json()

# Calculate price change
price_change = history[0]['yes_price'] - history[-1]['yes_price']
```

### Market Monitor
```bash
#!/bin/bash
# Monitor specific market every 5 minutes
while true; do
  curl -s "https://pm-history-api.onrender.com/api/markets/{id}" | jq '.yes_price'
  sleep 300
done
```

---

## Example Integrations

### React
```jsx
import { useEffect, useState } from 'react';

function MarketList() {
  const [markets, setMarkets] = useState([]);

  useEffect(() => {
    fetch('https://pm-history-api.onrender.com/api/markets?limit=10')
      .then(res => res.json())
      .then(data => setMarkets(data));
  }, []);

  return (
    <div>
      {markets.map(market => (
        <div key={market.id}>
          <h3>{market.title}</h3>
          <p>Yes: {(market.yes_price * 100).toFixed(1)}%</p>
        </div>
      ))}
    </div>
  );
}
```

### Python
```python
import requests
import pandas as pd

# Get markets
response = requests.get(
    'https://pm-history-api.onrender.com/api/markets',
    params={'limit': 100, 'sort': 'volume'}
)
markets = response.json()

# Convert to DataFrame
df = pd.DataFrame(markets)
print(df[['title', 'yes_price', 'volume']].head())
```

---

## Visualization

Use with the provided Python visualization tool:

```bash
# Install dependencies
pip install -r viz/requirements.txt

# Visualize market history
python3 viz/terminal_viz.py --market-id {uuid} --api-url https://pm-history-api.onrender.com
```

---

## Technical Details

**Stack:**
- **Language**: Rust
- **Framework**: Axum (async web framework)
- **Database**: PostgreSQL (Neon serverless)
- **Deployment**: Render
- **Region**: Oregon, USA

**Performance:**
- Average response time: < 200ms
- Database connection pooling
- Full-text search with PostgreSQL GIN indexes
- Time-series optimized queries

---

## Credits

Built with inspiration from:
- **[pm-indexer](https://github.com/0xqtpie/pm-indexer)** by 0xqtpie - Original API patterns and architecture

**Technology:**
- Data from [Polymarket](https://polymarket.com) and [Kalshi](https://kalshi.com) public APIs
- Built with Rust + Axum + PostgreSQL
- Hosted on Render + Neon

---

## Support & Contributing

- **GitHub**: [alejandorosumah-mansa/pm-history-tracker](https://github.com/alejandorosumah-mansa/pm-history-tracker)
- **Issues**: Create an issue on GitHub
- **Feature Requests**: Open a GitHub issue with your idea

---

## License

MIT License - See [LICENSE](LICENSE) file for details

---

## Disclaimer

This API provides historical prediction market data for informational purposes only. It is not financial advice. Always do your own research before making any trading decisions.

Market data is collected from public APIs and may contain errors or delays. Use at your own risk.

---

**Last Updated**: January 2026
**API Version**: 1.0.0
**Status**: ✅ Production
