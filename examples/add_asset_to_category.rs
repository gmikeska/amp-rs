use amp_rs::ApiClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get asset UUID and category ID from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <asset_uuid> <category_id>", args[0]);
        eprintln!("Example: {} 550e8400-e29b-41d4-a716-446655440000 1", args[0]);
        std::process::exit(1);
    }

    let asset_uuid = &args[1];
    let category_id: i64 = match args[2].parse() {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Error: category_id must be a valid integer");
            std::process::exit(1);
        }
    };

    // Create API client
    let client = ApiClient::new().await?;

    // Associate the asset with the category
    let updated_category = client.add_asset_to_category(category_id, asset_uuid).await?;
    println!(
        "Successfully added asset {} to category '{}' (ID: {})",
        asset_uuid, updated_category.name, updated_category.id
    );
    println!("Category now has {} asset(s)", updated_category.assets.len());

    Ok(())
}
