use amp_rs::ApiClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get registered user ID from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <registered_user_id>", args[0]);
        eprintln!("Example: {} 123", args[0]);
        std::process::exit(1);
    }

    let user_id: i64 = match args[1].parse() {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Error: registered_user_id must be a valid integer");
            std::process::exit(1);
        }
    };

    // Create API client
    let client = ApiClient::new().await?;

    // Fetch user details
    let user = client.get_registered_user(user_id).await?;
    println!("User: {} (ID: {})", user.name, user.id);

    // Fetch and display GAIDs
    let gaids = client.get_registered_user_gaids(user_id).await?;
    if gaids.is_empty() {
        println!("No GAIDs associated with this user.");
    } else {
        println!("\nAssociated GAIDs ({}):", gaids.len());
        for gaid in &gaids {
            println!("  - {}", gaid);
        }
    }

    Ok(())
}
