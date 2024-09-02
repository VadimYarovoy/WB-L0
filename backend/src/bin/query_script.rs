use clap::{Parser, Subcommand};
use reqwest::Client;
use serde_json::Value;
use std::fs;

#[derive(Parser)]
#[command(name = "test_client")]
#[command(about = "A simple HTTP client for testing API endpoints")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get {
        id: i32,
    },
    Post{
        file: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.command {
        Commands::Get { id } => {
            let url = format!("http://localhost:3000/api/orders/get/{}", id);
            match client.get(&url).send().await {
                Ok(response) => {
                    let body = response.text().await.unwrap();
                    println!("Response: {}", body);
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }
        Commands::Post { file } => {
            let json_data = fs::read_to_string(format!("../models/{file}.json")).expect("Unable to read file");
            let order: Value = serde_json::from_str(&json_data).expect("Unable to parse JSON");

            let url = "http://localhost:3000/api/orders/create";

            match client.post(url).json(&order).send().await {
                Ok(response) => {
                    let body = response.text().await.unwrap();
                    println!("Response: {}", body);
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }
    }
}
