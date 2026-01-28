#!/usr/bin/env python3
"""
Backfill script to create more price history snapshots
Queries Kalshi API and stores snapshots in the database
"""

import os
import sys
import time
import requests
import psycopg2
from datetime import datetime
from urllib.parse import urlparse

def get_db_connection():
    """Get database connection from DATABASE_URL"""
    database_url = os.getenv('DATABASE_URL')
    if not database_url:
        print("❌ DATABASE_URL not set in environment")
        sys.exit(1)

    return psycopg2.connect(database_url)

def get_markets_from_db(conn, limit=100):
    """Get markets that need more snapshots"""
    with conn.cursor() as cur:
        cur.execute("""
            SELECT m.id, m.source_id, m.source
            FROM markets m
            WHERE m.source = 'kalshi'
            AND m.status = 'active'
            ORDER BY m.created_at DESC
            LIMIT %s
        """, (limit,))
        return cur.fetchall()

def fetch_kalshi_market(ticker):
    """Fetch current market data from Kalshi API"""
    url = f"https://api.elections.kalshi.com/trade-api/v2/markets/{ticker}"
    headers = {"Accept": "application/json"}

    try:
        response = requests.get(url, headers=headers, timeout=10)
        if response.status_code == 200:
            data = response.json()
            return data.get('market')
        else:
            return None
    except Exception as e:
        print(f"  ⚠️  Error fetching {ticker}: {e}")
        return None

def insert_snapshot(conn, market_id, market_data):
    """Insert a price history snapshot"""
    if not market_data:
        return False

    # Extract price data
    yes_price = market_data.get('yes_sub_title', 'N/A')
    no_price = market_data.get('no_sub_title', 'N/A')

    # Try to parse prices (they might be like "52¢" or "0.52")
    try:
        if '¢' in str(yes_price):
            yes_price = float(yes_price.replace('¢', '')) / 100
        else:
            yes_price = float(yes_price) if yes_price != 'N/A' else 0.0
    except:
        yes_price = 0.5  # Default

    try:
        if '¢' in str(no_price):
            no_price = float(no_price.replace('¢', '')) / 100
        else:
            no_price = float(no_price) if no_price != 'N/A' else 0.0
    except:
        no_price = 1.0 - yes_price  # Complement

    volume = float(market_data.get('volume', 0))
    liquidity = float(market_data.get('liquidity', 0))
    volume_24h = float(market_data.get('volume_24h', 0))

    with conn.cursor() as cur:
        cur.execute("""
            INSERT INTO price_history
            (market_id, yes_price, no_price, volume, volume_24h, liquidity, recorded_at)
            VALUES (%s, %s, %s, %s, %s, %s, NOW())
        """, (market_id, yes_price, no_price, volume, volume_24h, liquidity))

    conn.commit()
    return True

def main():
    print("🔄 Backfill Price History Snapshots")
    print("=" * 80)
    print()

    # Get parameters
    num_markets = int(sys.argv[1]) if len(sys.argv) > 1 else 100
    num_cycles = int(sys.argv[2]) if len(sys.argv) > 2 else 5
    sleep_seconds = int(sys.argv[3]) if len(sys.argv) > 3 else 60

    print(f"📊 Markets per cycle: {num_markets}")
    print(f"🔁 Number of cycles: {num_cycles}")
    print(f"⏱️  Sleep between cycles: {sleep_seconds}s")
    print()

    conn = get_db_connection()

    for cycle in range(1, num_cycles + 1):
        print(f"[Cycle {cycle}/{num_cycles}] Starting...")

        markets = get_markets_from_db(conn, num_markets)
        print(f"  📋 Found {len(markets)} markets to snapshot")

        success_count = 0
        for i, (market_id, source_id, source) in enumerate(markets, 1):
            if i % 50 == 0:
                print(f"    Progress: {i}/{len(markets)}")

            # Fetch current data
            market_data = fetch_kalshi_market(source_id)

            if market_data and insert_snapshot(conn, market_id, market_data):
                success_count += 1

            # Rate limiting
            time.sleep(0.5)

        print(f"  ✅ Created {success_count} snapshots")

        # Show stats
        with conn.cursor() as cur:
            cur.execute("SELECT COUNT(*) FROM price_history")
            total_snapshots = cur.fetchone()[0]
            print(f"  📈 Total snapshots in DB: {total_snapshots}")

        if cycle < num_cycles:
            print(f"  ⏳ Waiting {sleep_seconds}s before next cycle...")
            time.sleep(sleep_seconds)

        print()

    print("=" * 80)
    print("✅ Backfill complete!")
    print()

    # Final statistics
    with conn.cursor() as cur:
        cur.execute("""
            SELECT
                COUNT(DISTINCT market_id) as markets_with_history,
                COUNT(*) as total_snapshots,
                AVG(snapshots_per_market) as avg_snapshots
            FROM (
                SELECT market_id, COUNT(*) as snapshots_per_market
                FROM price_history
                GROUP BY market_id
            ) AS counts
        """)
        stats = cur.fetchone()
        print(f"📊 Final Statistics:")
        print(f"  Markets with history: {stats[0]}")
        print(f"  Total snapshots: {stats[1]}")
        print(f"  Avg snapshots/market: {stats[2]:.1f}")

    conn.close()

if __name__ == "__main__":
    main()
