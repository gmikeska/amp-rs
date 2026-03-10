use amp_rs::ApiClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get GAID from command line arguments
    let args: Vec<String> = env::args().collect();
    let gaid = if args.len() > 1 {
        args[1].clone()
    } else {
        eprintln!("Usage: cargo run --example get_gaid_balance <GAID>");
        std::process::exit(1);
    };

    // Create API client
    let client = ApiClient::new().await?;

    println!("Fetching balance for GAID: {}", gaid);

    match client.get_gaid_balance(&gaid).await {
        Ok(entries) => {
            if entries.is_empty() {
                println!("No asset balances found for this GAID.");
            } else {
                println!("\n{:<40} {:<68} {:>12}", "Asset UUID", "Asset ID", "Balance");
                println!("{}", "-".repeat(120));
                for entry in &entries {
                    println!(
                        "{:<40} {:<68} {:>12}",
                        entry.asset_uuid, entry.asset_id, entry.balance
                    );
                }
                println!(
                    "\nTotal: {} asset(s)",
                    entries.len()
                );
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to get balance for GAID '{}': {}", gaid, e);
            std::process::exit(1);
        }
    }

    Ok(())
}
