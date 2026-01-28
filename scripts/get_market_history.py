#!/usr/bin/env python3
"""
Get price history for a prediction market
"""

import requests
import sys
from datetime import datetime

API_BASE = "https://pm-history-api.onrender.com"

def get_market_history(market_id, hours=None, limit=100):
    """
    Fetch price history for a market

    Args:
        market_id: UUID of the market
        hours: Limit to last N hours (optional)
        limit: Maximum number of snapshots (default 100)
    """
    params = {"limit": limit}
    if hours:
        params["hours"] = hours

    response = requests.get(f"{API_BASE}/api/markets/{market_id}/history", params=params)

    if response.status_code == 404:
        print(f"❌ Market not found: {market_id}")
        return None

    response.raise_for_status()
    return response.json()

def get_market_details(market_id):
    """Get market metadata"""
    response = requests.get(f"{API_BASE}/api/markets/{market_id}")
    if response.status_code == 404:
        return None
    response.raise_for_status()
    return response.json()

def display_history(market, history):
    """Display market history in a nice format"""
    print(f"\n{'='*80}")
    print(f"📊 Market: {market['title'][:70]}")
    print(f"🔗 Source: {market['source'].upper()}")
    print(f"📈 Status: {market['status']}")
    print(f"{'='*80}\n")

    if not history:
        print("⚠️  No price history available yet")
        return

    print(f"📅 Price History ({len(history)} snapshots):\n")
    print(f"{'Time':<25} {'Yes Price':>12} {'No Price':>12} {'Volume':>15}")
    print(f"{'-'*70}")

    for snapshot in history:
        timestamp = datetime.fromisoformat(snapshot['recorded_at'].replace('Z', '+00:00'))
        time_str = timestamp.strftime('%Y-%m-%d %H:%M:%S')
        yes_price = snapshot['yes_price']
        no_price = snapshot['no_price']
        volume = snapshot['volume']

        print(f"{time_str:<25} {yes_price:>12.4f} {no_price:>12.4f} ${volume:>14,.2f}")

    # Summary
    if len(history) > 1:
        print(f"\n{'-'*70}")
        first = history[-1]
        last = history[0]
        price_change = last['yes_price'] - first['yes_price']
        change_pct = (price_change / first['yes_price'] * 100) if first['yes_price'] > 0 else 0

        print(f"💹 Price Change: {price_change:+.4f} ({change_pct:+.2f}%)")
        print(f"📊 Volume Change: ${first['volume']:.2f} → ${last['volume']:.2f}")

def get_sample_market():
    """Get a sample market ID"""
    response = requests.get(f"{API_BASE}/api/markets", params={"limit": 1, "sort": "volume"})
    markets = response.json()
    if markets:
        return markets[0]['id']
    return None

def main():
    if len(sys.argv) > 1:
        market_id = sys.argv[1]
        hours = int(sys.argv[2]) if len(sys.argv) > 2 else None
    else:
        print("🔍 No market ID provided, fetching a sample market...")
        market_id = get_sample_market()
        if not market_id:
            print("❌ No markets found in database")
            return
        print(f"📌 Using market ID: {market_id}\n")
        hours = 24

    # Get market details
    print(f"⏳ Fetching market details...")
    market = get_market_details(market_id)

    if not market:
        print(f"❌ Market not found: {market_id}")
        print(f"\n💡 Usage: python3 {sys.argv[0]} <market_id> [hours]")
        print(f"   Example: python3 {sys.argv[0]} 10aa9966-daa4-48b4-b838-829fc600f97e 24")
        return

    # Get history
    print(f"⏳ Fetching price history...")
    history = get_market_history(market_id, hours=hours, limit=100)

    # Display
    display_history(market, history)

    print(f"\n{'='*80}")
    print(f"✅ Done! Market URL: {market.get('url', 'N/A')}")
    print(f"{'='*80}\n")

if __name__ == "__main__":
    main()
