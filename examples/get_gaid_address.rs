//! GAID Address Lookup Example
//!
//! This example demonstrates how to get the address for a GAID using the AMP API.
//!
//! Usage:
//!   cargo run --example get_gaid_address <GAID>
//!
//! Example:
//!   cargo run --example get_gaid_address GAbYScu6jkWUND2jo3L4KJxyvo55d

use amp_rs::ApiClient;
use std::env;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get GAID from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <GAID>", args[0]);
        eprintln!("Example: {} GAbYScu6jkWUND2jo3L4KJxyvo55d", args[0]);
        std::process::exit(1);
    }

    let gaid = &args[1];
    println!("Looking up address for GAID: {}", gaid);

    let client = ApiClient::new().await.expect("Failed to create API client");

    match client.get_gaid_address(gaid).await {
        Ok(response) => {
            if let Some(error) = &response.error {
                if !error.is_empty() {
                    eprintln!("Error from API: {}", error);
                    std::process::exit(1);
                }
            }

            println!("Address: {}", response.address);
        }
        Err(e) => {
            eprintln!("Error getting address for GAID: {:?}", e);
            std::process::exit(1);
        }
    }
}
