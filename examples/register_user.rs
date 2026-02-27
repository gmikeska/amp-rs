/// Register a local database user on AMP by their primary key ID.
///
/// Usage:
///     cargo run --example register_user -- <user_id>
///
/// This reads the user's name and is_company flag from the local database,
/// registers them on AMP via `add_registered_user`, and prints the resulting
/// AMP registered_id so you can update the local record.
use amp_rs::model::RegisteredUserAdd;
use amp_rs::ApiClient;
use std::process::Command;

const DB_URL: &str = "postgres://sats_assets:sats@localhost:5432/sats-dev";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let user_id: i32 = std::env::args()
        .nth(1)
        .expect("Usage: register_user <user_id>")
        .parse()
        .expect("user_id must be an integer");

    // Fetch the user from the local database via psql
    let output = Command::new("psql")
        .args([
            DB_URL,
            "--tuples-only",
            "--no-align",
            "--field-separator=|",
            "-c",
            &format!(
                "SELECT id, first_name, last_name, name, is_company FROM users WHERE id = {}",
                user_id
            ),
        ])
        .output()
        .expect("Failed to run psql");

    if !output.status.success() {
        eprintln!(
            "psql failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.trim();
    if line.is_empty() {
        eprintln!("No user found with id {user_id}");
        std::process::exit(1);
    }

    let fields: Vec<&str> = line.splitn(5, '|').collect();
    if fields.len() < 5 {
        eprintln!("Unexpected psql output: {line}");
        std::process::exit(1);
    }

    let first_name = fields[1].trim();
    let last_name = fields[2].trim();
    let name = fields[3].trim();
    let is_company = fields[4].trim() == "t";

    let display_name = if !name.is_empty() {
        name.to_string()
    } else {
        format!("{first_name} {last_name}")
    };

    println!("User {user_id}: \"{display_name}\" (is_company={is_company})");

    // Register on AMP
    let client = ApiClient::new().await.unwrap();
    let req = RegisteredUserAdd {
        name: display_name.clone(),
        gaid: None,
        is_company,
    };

    match client.add_registered_user(&req).await {
        Ok(resp) => {
            println!("Registered on AMP -> registered_id: {}", resp.id);
            println!(
                "\nTo update the local DB:\n  UPDATE users SET registered_id = {} WHERE id = {};",
                resp.id, user_id
            );
        }
        Err(e) => {
            eprintln!("Failed to register: {e}");
            std::process::exit(1);
        }
    }
}
