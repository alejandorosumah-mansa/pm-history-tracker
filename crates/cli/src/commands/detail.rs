use anyhow::Result;
use colored::*;
use uuid::Uuid;

use crate::api_client::ApiClient;

pub async fn detail_command(api_url: &str, market_id: Uuid) -> Result<()> {
    let client = ApiClient::new(api_url.to_string());

    println!("{}", "Fetching market details...".cyan());

    let market = client.get_market(market_id).await?;

    println!("\n{}", "=".repeat(80).green());
    println!("{}", market.title.white().bold());
    println!("{}", "=".repeat(80).green());

    println!("\n{}", "Description:".yellow().bold());
    println!("{}", market.description);

    println!("\n{}", "Market Details:".yellow().bold());
    println!("  {} {}", "ID:".bright_black(), market.id);
    println!("  {} {}", "Source:".bright_black(), market.source.blue());
    println!("  {} {}", "Source ID:".bright_black(), market.source_id);
    println!(
        "  {} {}",
        "Status:".bright_black(),
        if market.status == "open" {
            market.status.green()
        } else {
            market.status.yellow()
        }
    );

    if let Some(category) = &market.category {
        println!("  {} {}", "Category:".bright_black(), category);
    }

    println!("\n{}", "Current Prices:".yellow().bold());
    println!("  {} {:.2}%", "Yes:".bright_black(), market.yes_price * 100.0);
    println!("  {} {:.2}%", "No:".bright_black(), market.no_price * 100.0);

    println!("\n{}", "Volume & Liquidity:".yellow().bold());
    println!("  {} ${:.2}", "Total Volume:".bright_black(), market.volume);
    println!("  {} ${:.2}", "24h Volume:".bright_black(), market.volume_24h);
    if let Some(liq) = market.liquidity {
        println!("  {} ${:.2}", "Liquidity:".bright_black(), liq);
    }

    println!("\n{}", "Timestamps:".yellow().bold());
    println!("  {} {}", "Created:".bright_black(), market.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("  {} {}", "Updated:".bright_black(), market.updated_at.format("%Y-%m-%d %H:%M:%S UTC"));
    if let Some(close) = market.close_at {
        println!("  {} {}", "Closes:".bright_black(), close.format("%Y-%m-%d %H:%M:%S UTC"));
    }

    println!("\n{}", "Link:".yellow().bold());
    println!("  {}", market.url.bright_blue());

    println!("\n{}", "=".repeat(80).green());
    println!("\n{}", format!("Use 'pm-cli history {}' to see price history", market.id).bright_black());

    Ok(())
}
