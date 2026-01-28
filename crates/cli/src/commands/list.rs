use anyhow::Result;
use colored::*;

use crate::api_client::ApiClient;

pub async fn list_command(api_url: &str, limit: usize) -> Result<()> {
    let client = ApiClient::new(api_url.to_string());

    println!("{}", "Fetching markets...".cyan());

    let markets = client.list_markets(limit).await?;

    if markets.is_empty() {
        println!("{}", "No markets found.".yellow());
        return Ok(());
    }

    println!("\n{}", format!("Top {} Markets", markets.len()).green().bold());
    println!("{}", "=".repeat(80).green());

    for (idx, market) in markets.iter().enumerate() {
        println!(
            "\n{}. {} (ID: {})",
            (idx + 1).to_string().cyan().bold(),
            market.title.white().bold(),
            market.id.to_string().bright_black()
        );

        println!(
            "   {} {} | {} {} | {} ${:.0} | {} {:.1}%",
            "Source:".bright_black(),
            market.source.blue(),
            "Status:".bright_black(),
            if market.status == "open" {
                market.status.green()
            } else {
                market.status.yellow()
            },
            "Volume:".bright_black(),
            market.volume,
            "Yes:".bright_black(),
            market.yes_price * 100.0
        );
    }

    println!("\n{}", "=".repeat(80).green());

    Ok(())
}
