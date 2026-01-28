use anyhow::Result;
use colored::*;
use uuid::Uuid;

use crate::api_client::ApiClient;

pub async fn history_command(api_url: &str, market_id: Uuid, hours: Option<i64>) -> Result<()> {
    let client = ApiClient::new(api_url.to_string());

    println!("{}", "Fetching price history...".cyan());

    let history = client.get_history(market_id, hours).await?;

    if history.is_empty() {
        println!("{}", "No price history available for this market.".yellow());
        return Ok(());
    }

    // Get market details for title
    let market = client.get_market(market_id).await?;

    println!("\n{}", "=".repeat(80).green());
    println!("{}", format!("Price History: {}", market.title).white().bold());
    println!("{}", "=".repeat(80).green());

    println!(
        "\n{} {} snapshots",
        "Total:".bright_black(),
        history.len().to_string().cyan()
    );

    if let Some(h) = hours {
        println!("{} Last {} hours", "Period:".bright_black(), h);
    } else {
        println!("{} All available data", "Period:".bright_black());
    }

    // Show most recent 10 snapshots
    println!("\n{}", "Recent Snapshots:".yellow().bold());
    println!(
        "{:^20} | {:^10} | {:^10} | {:^12} | {:^12}",
        "Timestamp", "Yes Price", "No Price", "Volume", "24h Volume"
    );
    println!("{}", "-".repeat(80));

    for snapshot in history.iter().take(10) {
        println!(
            "{:^20} | {:>9.2}% | {:>9.2}% | ${:>10.0} | ${:>10.0}",
            snapshot.recorded_at.format("%Y-%m-%d %H:%M"),
            snapshot.yes_price * 100.0,
            snapshot.no_price * 100.0,
            snapshot.volume,
            snapshot.volume_24h
        );
    }

    // Calculate price change
    if history.len() >= 2 {
        let latest = &history[0];
        let oldest = &history[history.len() - 1];
        let yes_change = (latest.yes_price - oldest.yes_price) * 100.0;
        let no_change = (latest.no_price - oldest.no_price) * 100.0;

        println!("\n{}", "Price Change:".yellow().bold());
        println!(
            "  {} {:+.2}%",
            "Yes:".bright_black(),
            yes_change
        );
        println!(
            "  {} {:+.2}%",
            "No:".bright_black(),
            no_change
        );
    }

    println!("\n{}", "=".repeat(80).green());

    Ok(())
}
