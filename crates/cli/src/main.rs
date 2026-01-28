mod api_client;
mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "pm-cli")]
#[command(about = "Prediction Market History Tracker CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// API base URL
    #[arg(long, env = "PM_API_URL", default_value = "http://localhost:3000")]
    api_url: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for markets
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Get market details
    Detail {
        /// Market ID
        id: Uuid,
    },

    /// Get price history for a market
    History {
        /// Market ID
        id: Uuid,

        /// Limit to last N hours
        #[arg(long)]
        hours: Option<i64>,
    },

    /// List markets
    List {
        /// Maximum number of results
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search { query, limit } => {
            commands::search_command(&cli.api_url, &query, limit).await?;
        }
        Commands::Detail { id } => {
            commands::detail_command(&cli.api_url, id).await?;
        }
        Commands::History { id, hours } => {
            commands::history_command(&cli.api_url, id, hours).await?;
        }
        Commands::List { limit } => {
            commands::list_command(&cli.api_url, limit).await?;
        }
    }

    Ok(())
}
