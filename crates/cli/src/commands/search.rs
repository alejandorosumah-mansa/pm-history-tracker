use anyhow::Result;
use colored::*;

use crate::api_client::ApiClient;

pub async fn search_command(api_url: &str, query: &str, limit: usize) -> Result<()> {
    let client = ApiClient::new(api_url.to_string());

    println!("{}", "Searching markets...".cyan());

    let results = client.search(query, limit).await?;

    if results.is_empty() {
        println!("{}", "No markets found matching your query.".yellow());
        return Ok(());
    }

    println!("\n{}", "Top matching markets:".green().bold());
    println!("{}", "=".repeat(80).green());

    for (idx, (market, score)) in results.iter().enumerate() {
        println!(
            "\n{}. {} (ID: {})",
            (idx + 1).to_string().cyan().bold(),
            market.title.white().bold(),
            market.id.to_string().bright_black()
        );

        if !market.description.is_empty() && market.description.len() > 100 {
            println!(
                "   {}",
                format!("{}...", &market.description[..97]).bright_black()
            );
        } else if !market.description.is_empty() {
            println!("   {}", market.description.bright_black());
        }

        println!(
            "   {} {} | {} {} | {} ${:.0}",
            "Source:".bright_black(),
            market.source.blue(),
            "Status:".bright_black(),
            if market.status == "open" {
                market.status.green()
            } else {
                market.status.yellow()
            },
            "Volume:".bright_black(),
            market.volume
        );

        if market.yes_price > 0.0 {
            println!(
                "   {} {:.1}% | {} {:.1}%",
                "Yes:".bright_black(),
                market.yes_price * 100.0,
                "No:".bright_black(),
                market.no_price * 100.0
            );
        }

        println!("   {} {:.2}", "Match Score:".bright_black(), score);
    }

    println!("\n{}", "=".repeat(80).green());
    println!("\n{}", format!("Use 'pm-cli detail <ID>' to see full market details").bright_black());

    Ok(())
}
