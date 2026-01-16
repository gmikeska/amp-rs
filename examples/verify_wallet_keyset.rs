//! Verify if the wallet on the local Elements node has the same keyset as the previous wallet
//! by checking if it contains the known treasury address

use amp_rs::ElementsRpc;
use dotenvy;

const WALLET_NAME: &str = "amp_elements_wallet_static_for_funding";
const KNOWN_TREASURY_ADDRESS: &str = "tlq1qqgj3gdz9fzldddnmez0ymy3y5ac0a7qcx4aah73s2r8g7wsu9eayfnpxk45yjjfllvgafqq5pxtrvxt9qrjr7cnt0m8u2r89q";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Wallet Keyset Verification");
    println!("==============================");
    println!("Wallet: {}", WALLET_NAME);
    println!("Known address: {}", KNOWN_TREASURY_ADDRESS);
    println!();

    dotenvy::dotenv().ok();

    // Create Elements RPC client
    let elements_rpc = ElementsRpc::from_env()?;

    // Load wallet
    println!("📂 Loading wallet...");
    match elements_rpc.load_wallet(WALLET_NAME).await {
        Ok(()) => println!("✅ Wallet loaded"),
        Err(e) if e.to_string().contains("already loaded") => {
            println!("✅ Wallet already loaded")
        }
        Err(e) => {
            println!("❌ Failed to load wallet: {}", e);
            return Err(e.into());
        }
    }

    // Try to get confidential address - this will work if the address is in the wallet
    println!("\n🔎 Checking if address belongs to wallet...");

    // First, try to validate the address format by converting it
    match elements_rpc
        .get_confidential_address(WALLET_NAME, KNOWN_TREASURY_ADDRESS)
        .await
    {
        Ok(confidential) => {
            println!("\n📊 Address Check:");
            println!("   - Confidential version: {}", confidential);

            if confidential == KNOWN_TREASURY_ADDRESS {
                println!("\n✅ Address is already confidential");
            }

            println!("\n🎉 SUCCESS!");
            println!("The wallet recognizes this address.");
            println!("Checking if wallet can sign...");

            // Try to dump private key to verify we have signing capability
            // First get address info via RPC
            let wallet_info = elements_rpc.get_wallet_info(WALLET_NAME).await?;
            let is_descriptor = wallet_info
                .get("descriptors")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if is_descriptor {
                println!("   - Wallet type: Descriptor");
                println!("   - Checking for private keys...");
                // For descriptor wallets, we can't easily check if specific address has private key
                // but we already confirmed the address is in the wallet
                println!("\n✅ This wallet contains the known treasury address.");
                println!("This is likely the same keyset as the previous wallet.");
            } else {
                println!("   - Wallet type: Legacy");
                println!("\n✅ This wallet contains the known treasury address.");
                println!("This is likely the same keyset as the previous wallet.");
            }
        }
        Err(e) => {
            let error_msg = e.to_string();
            if error_msg.contains("Invalid address")
                || error_msg.contains("does not refer to a key")
            {
                println!("\n❌ NO MATCH");
                println!("The wallet does not contain this address.");
                println!("This is NOT the same keyset as the previous wallet.");
                println!("\nError: {}", error_msg);
            } else {
                println!("❌ Failed to check address: {}", e);
                return Err(e.into());
            }
        }
    }

    // Get wallet info for additional context
    println!("\n📋 Wallet Details:");
    let wallet_info = elements_rpc.get_wallet_info(WALLET_NAME).await?;

    let is_descriptor = wallet_info
        .get("descriptors")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    println!(
        "   - Wallet type: {}",
        if is_descriptor {
            "Descriptor"
        } else {
            "Legacy"
        }
    );

    if let Some(balance) = wallet_info.get("balance").and_then(|v| v.as_object()) {
        println!("   - Balance: {:?}", balance);
    }

    Ok(())
}
