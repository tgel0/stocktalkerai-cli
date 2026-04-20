mod cli;
mod client;

use clap::Parser;
use cli::{Cli, Commands, AlertCommand};
use client::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Optionally load from .env file
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();
    let client = Client::new(cli.api_key.clone());

    let result = match &cli.command {
        Commands::Status => {
            let res = client.get("/status").await?;
            res
        }
        Commands::Price { ticker, history } => {
            let path = if *history {
                format!("/price/{}/history", ticker)
            } else {
                format!("/price/{}", ticker)
            };
            client.get(&path).await?
        }
        Commands::Indicators { ticker } => {
            client.get(&format!("/indicators/{}", ticker)).await?
        }
        Commands::Account => {
            client.get("/account").await?
        }
        Commands::Alert { action } => match action {
            AlertCommand::List { status } => {
                let path = match status {
                    Some(s) => format!("/alerts?status={}", s),
                    None => "/alerts".to_string()
                };
                client.get(&path).await?
            }
            AlertCommand::Get { id } => {
                client.get(&format!("/alerts/{}", id)).await?
            }
            AlertCommand::Create { prompt } => {
                let body = serde_json::json!({ "prompt": prompt });
                client.post("/alerts", &body).await?
            }
            AlertCommand::Delete { id } => {
                client.delete(&format!("/alerts/{}", id)).await?
            }
        }
    };

    if cli.output == "json" {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        // Human readable output
        // Based on the data returned, we'd implement comfy-table matching here.
        // For brevity in scaffold, default to pretty JSON if table formatter not implemented for that response.
        println!("{}", serde_json::to_string_pretty(&result)?);
    }

    Ok(())
}
