#!/usr/bin/env python3
"""
PM History Tracker - Terminal Visualization
Interactive price history charts using Rich and Plotly
"""

import argparse
import sys
from datetime import datetime
from typing import List, Dict, Any

import requests
from rich.console import Console
from rich.table import Table
from rich.panel import Panel
from rich import box
import plotly.graph_objects as go
from plotly.subplots import make_subplots


console = Console()


def fetch_market(api_url: str, market_id: str) -> Dict[str, Any]:
    """Fetch market details from API"""
    url = f"{api_url}/api/markets/{market_id}"

    try:
        response = requests.get(url, timeout=10)
        response.raise_for_status()
        return response.json()
    except requests.RequestException as e:
        console.print(f"[red]Error fetching market: {e}[/red]")
        sys.exit(1)


def fetch_history(api_url: str, market_id: str, hours: int = None) -> List[Dict[str, Any]]:
    """Fetch price history from API"""
    url = f"{api_url}/api/markets/{market_id}/history?limit=1000"

    if hours:
        url += f"&hours={hours}"

    try:
        response = requests.get(url, timeout=10)
        response.raise_for_status()
        return response.json()
    except requests.RequestException as e:
        console.print(f"[red]Error fetching history: {e}[/red]")
        sys.exit(1)


def display_market_info(market: Dict[str, Any]):
    """Display market information in a rich table"""

    table = Table(title=market['title'], box=box.ROUNDED, show_header=False)
    table.add_column("Field", style="cyan", width=15)
    table.add_column("Value", style="white")

    table.add_row("Source", market['source'])
    table.add_row("Status", market['status'])
    table.add_row("Yes Price", f"{market['yes_price'] * 100:.2f}%")
    table.add_row("No Price", f"{market['no_price'] * 100:.2f}%")
    table.add_row("Volume", f"${market['volume']:,.0f}")
    table.add_row("24h Volume", f"${market['volume_24h']:,.0f}")

    if market.get('liquidity'):
        table.add_row("Liquidity", f"${market['liquidity']:,.0f}")

    console.print(table)


def display_history_table(history: List[Dict[str, Any]], limit: int = 10):
    """Display recent history in a table"""

    table = Table(title="Recent Price Snapshots", box=box.ROUNDED)
    table.add_column("Timestamp", style="cyan")
    table.add_column("Yes Price", justify="right", style="green")
    table.add_column("No Price", justify="right", style="red")
    table.add_column("Volume", justify="right")
    table.add_column("24h Volume", justify="right")

    for snapshot in history[:limit]:
        timestamp = datetime.fromisoformat(snapshot['recorded_at'].replace('Z', '+00:00'))
        table.add_row(
            timestamp.strftime("%Y-%m-%d %H:%M"),
            f"{snapshot['yes_price'] * 100:.2f}%",
            f"{snapshot['no_price'] * 100:.2f}%",
            f"${snapshot['volume']:,.0f}",
            f"${snapshot['volume_24h']:,.0f}"
        )

    console.print(table)


def create_ascii_sparkline(values: List[float], width: int = 40) -> str:
    """Create a simple ASCII sparkline"""
    if not values or len(values) < 2:
        return ""

    min_val = min(values)
    max_val = max(values)

    if max_val == min_val:
        return "─" * width

    chars = "▁▂▃▄▅▆▇█"

    normalized = [(v - min_val) / (max_val - min_val) for v in values]

    # Sample to fit width
    if len(normalized) > width:
        step = len(normalized) / width
        sampled = [normalized[int(i * step)] for i in range(width)]
    else:
        sampled = normalized

    return "".join(chars[min(int(v * (len(chars) - 1)), len(chars) - 1)] for v in sampled)


def display_sparklines(history: List[Dict[str, Any]]):
    """Display ASCII sparklines for quick visualization"""

    # Reverse to show oldest to newest
    history_reversed = list(reversed(history))

    yes_prices = [h['yes_price'] * 100 for h in history_reversed]
    volumes = [h['volume_24h'] for h in history_reversed]

    table = Table(title="Trend Sparklines", box=box.ROUNDED, show_header=False)
    table.add_column("Metric", style="cyan", width=15)
    table.add_column("Sparkline", style="green")

    table.add_row("Yes Price", create_ascii_sparkline(yes_prices, 60))
    table.add_row("24h Volume", create_ascii_sparkline(volumes, 60))

    console.print(table)


def create_plotly_chart(market: Dict[str, Any], history: List[Dict[str, Any]], output_file: str = None):
    """Create interactive Plotly chart"""

    # Reverse to show oldest to newest
    history_reversed = list(reversed(history))

    timestamps = [datetime.fromisoformat(h['recorded_at'].replace('Z', '+00:00')) for h in history_reversed]
    yes_prices = [h['yes_price'] * 100 for h in history_reversed]
    no_prices = [h['no_price'] * 100 for h in history_reversed]
    volumes = [h['volume_24h'] for h in history_reversed]

    # Create subplots
    fig = make_subplots(
        rows=2, cols=1,
        shared_xaxes=True,
        vertical_spacing=0.1,
        subplot_titles=('Price History', '24h Volume'),
        row_heights=[0.7, 0.3]
    )

    # Add price traces
    fig.add_trace(
        go.Scatter(
            x=timestamps,
            y=yes_prices,
            mode='lines',
            name='Yes Price',
            line=dict(color='green', width=2)
        ),
        row=1, col=1
    )

    fig.add_trace(
        go.Scatter(
            x=timestamps,
            y=no_prices,
            mode='lines',
            name='No Price',
            line=dict(color='red', width=2)
        ),
        row=1, col=1
    )

    # Add volume trace
    fig.add_trace(
        go.Bar(
            x=timestamps,
            y=volumes,
            name='24h Volume',
            marker=dict(color='lightblue')
        ),
        row=2, col=1
    )

    # Update layout
    fig.update_layout(
        title=dict(
            text=f"{market['title']}<br><sub>Source: {market['source']} | Status: {market['status']}</sub>",
            x=0.5,
            xanchor='center'
        ),
        hovermode='x unified',
        height=800
    )

    fig.update_yaxes(title_text="Price (%)", row=1, col=1)
    fig.update_yaxes(title_text="Volume ($)", row=2, col=1)
    fig.update_xaxes(title_text="Date", row=2, col=1)

    if output_file:
        fig.write_html(output_file)
        console.print(f"\n[green]Chart saved to:[/green] {output_file}")
    else:
        # Try to open in browser
        try:
            fig.show()
        except Exception as e:
            # Fallback to saving HTML
            fallback_file = f"market_{market['id']}_chart.html"
            fig.write_html(fallback_file)
            console.print(f"\n[yellow]Could not open browser. Chart saved to:[/yellow] {fallback_file}")


def main():
    parser = argparse.ArgumentParser(description="PM History Tracker - Terminal Visualization")
    parser.add_argument("--market-id", required=True, help="Market UUID")
    parser.add_argument("--api-url", default="http://localhost:3000", help="API base URL")
    parser.add_argument("--hours", type=int, help="Limit to last N hours")
    parser.add_argument("--output", help="Output HTML file for chart")
    parser.add_argument("--no-chart", action="store_true", help="Skip interactive chart")

    args = parser.parse_args()

    console.print(Panel.fit(
        "[bold cyan]PM History Tracker[/bold cyan]\n"
        "[dim]Terminal Visualization Tool[/dim]",
        border_style="cyan"
    ))

    # Fetch data
    with console.status("[cyan]Fetching market data..."):
        market = fetch_market(args.api_url, args.market_id)
        history = fetch_history(args.api_url, args.market_id, args.hours)

    if not history:
        console.print("[yellow]No price history available for this market.[/yellow]")
        return

    console.print(f"\n[green]✓[/green] Fetched {len(history)} price snapshots\n")

    # Display information
    display_market_info(market)
    console.print()
    display_history_table(history, limit=10)
    console.print()
    display_sparklines(history)

    # Create interactive chart
    if not args.no_chart:
        console.print("\n[cyan]Generating interactive chart...[/cyan]")
        create_plotly_chart(market, history, args.output)

    console.print("\n[dim]Tip: Use --output chart.html to save the chart to a file[/dim]")


if __name__ == "__main__":
    main()
