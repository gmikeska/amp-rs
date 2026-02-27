use amp_rs::model::CategoryAdd;
use amp_rs::ApiClient;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let client = ApiClient::new().await.unwrap();

    let categories = vec![
        CategoryAdd {
            name: "SecondIssuer Category".to_string(),
            description: Some("".to_string()),
        },
        CategoryAdd {
            name: "Blah Category 2".to_string(),
            description: Some("".to_string()),
        },
    ];

    for cat in &categories {
        match client.add_category(cat).await {
            Ok(resp) => println!("Registered '{}' -> id: {}", resp.name, resp.id),
            Err(e) => eprintln!("Failed to register '{}': {}", cat.name, e),
        }
    }
}
