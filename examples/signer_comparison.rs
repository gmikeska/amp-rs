//! # Signer Comparison Example
//!
//! This example demonstrates the different signer implementations available in amp-rs
//! and when to use each one. It showcases:
//!
//! - `LwkSoftwareSigner`: Software-based signing using mnemonic phrases
//! - `ElementsRpcSigner`: Node wallet-based signing via RPC
//!
//! Both signers implement the same `Signer` trait, making them interchangeable
//! in the codebase and preparing for future external signer support (HSMs, hardware wallets).
//!
//! ## ⚠️ TESTNET/REGTEST ONLY ⚠️
//!
//! This example is for development and testing purposes only. Never use these
//! patterns in production or with real funds.
//!
//! ## When to Use Each Signer
//!
//! ### LwkSoftwareSigner
//! - **Use for**: Independent key management, testing without node wallet setup
//! - **Pros**: Full control over keys, no node wallet required, portable
//! - **Cons**: Keys stored in memory, requires mnemonic management
//! - **Best for**: Automated testing, CI/CD, development
//!
//! ### ElementsRpcSigner
//! - **Use for**: Integration with existing node wallet, manual testing
//! - **Pros**: Leverages node's wallet infrastructure, familiar workflow
//! - **Cons**: Requires node wallet setup and unlocking, tied to specific node
//! - **Best for**: Integration testing, debugging, node-based workflows
//!
//! ## Running This Example
//!
//! ```bash
//! # Set up environment variables
//! export ELEMENTS_RPC_URL="http://localhost:18884"
//! export ELEMENTS_RPC_USER="user"
//! export ELEMENTS_RPC_PASSWORD="pass"
//!
//! # Run the example
//! cargo run --example signer_comparison
//! ```

use amp_rs::signer::{ElementsRpcSigner, LwkSoftwareSigner, Signer, SignerError};
use amp_rs::ElementsRpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("\n=== AMP-RS Signer Comparison Example ===\n");
    println!("⚠️  TESTNET/REGTEST ONLY - Never use in production!\n");

    // =============================================================================
    // Part 1: LwkSoftwareSigner - Software-based signing with mnemonic
    // =============================================================================

    println!("📝 Part 1: LwkSoftwareSigner (Software-based signing)\n");

    // Create a software signer from a generated mnemonic
    let (mnemonic, lwk_signer) = LwkSoftwareSigner::generate_new_indexed(9000)?;
    println!("✓ Created LwkSoftwareSigner");
    println!("  Mnemonic (first 50 chars): {}...", &mnemonic[..50]);
    println!("  Network: Testnet");
    println!(
        "  Type: {} ({})\n",
        std::any::type_name_of_val(&lwk_signer),
        "Software key management"
    );

    // Demonstrate trait usage
    println!("✓ LwkSoftwareSigner implements Signer trait");
    demonstrate_signer_trait(&lwk_signer).await?;

    // =============================================================================
    // Part 2: ElementsRpcSigner - Node wallet-based signing
    // =============================================================================

    println!("\n📝 Part 2: ElementsRpcSigner (Node wallet-based signing)\n");

    // Create node RPC client from environment variables
    match ElementsRpc::from_env() {
        Ok(rpc) => {
            println!("✓ Connected to Elements node");
            println!("  Endpoint: {}", rpc.base_url());

            // Verify node connectivity
            match rpc.get_network_info().await {
                Ok(network_info) => {
                    println!("  Node version: {}", network_info.subversion);
                    println!("  Connections: {}", network_info.connections);

                    // Create node-based signer
                    let node_signer = ElementsRpcSigner::new(rpc.clone());
                    println!("\n✓ Created ElementsRpcSigner");
                    println!(
                        "  Type: {} ({})",
                        std::any::type_name_of_val(&node_signer),
                        "Node wallet signing"
                    );
                    println!("  Delegates to: Elements node wallet via RPC\n");

                    // Demonstrate trait usage
                    println!("✓ ElementsRpcSigner implements Signer trait");
                    demonstrate_signer_trait(&node_signer).await?;

                    // =============================================================================
                    // Part 3: Polymorphic Usage - Both signers use the same interface
                    // =============================================================================

                    println!("\n📝 Part 3: Polymorphic Usage (Trait-based design)\n");

                    println!("✓ Both signers can be used through the same trait interface:");
                    println!("  - Enables easy switching between signing backends");
                    println!("  - Prepares for future external signer support (HSM, hardware wallet)");
                    println!("  - Maintains consistent API across all signer types\n");

                    // Demonstrate polymorphic usage
                    let signers: Vec<&dyn Signer> = vec![&lwk_signer, &node_signer];

                    for (i, _signer) in signers.iter().enumerate() {
                        let signer_type = if i == 0 {
                            "LwkSoftwareSigner"
                        } else {
                            "ElementsRpcSigner"
                        };
                        println!("  Testing {} via trait interface...", signer_type);

                        // Both signers can be used through the same interface
                        // In real usage, they would sign actual transactions
                        // Here we just demonstrate the interface is the same
                        println!("    ✓ Compatible with &dyn Signer");
                    }

                    println!("\n✓ Polymorphic usage demonstration complete");
                }
                Err(e) => {
                    println!("⚠️  Could not get network info: {}", e);
                    println!("   (Node may be offline or credentials incorrect)");
                }
            }
        }
        Err(e) => {
            println!("⚠️  Could not connect to Elements node: {}", e);
            println!("   Set ELEMENTS_RPC_URL, ELEMENTS_RPC_USER, ELEMENTS_RPC_PASSWORD");
            println!("   Skipping ElementsRpcSigner demonstration");
        }
    }

    // =============================================================================
    // Part 4: Summary and Recommendations
    // =============================================================================

    println!("\n📝 Part 4: Summary and Recommendations\n");

    println!("✓ Two signer implementations are available:");
    println!("  1. LwkSoftwareSigner - Software key management with mnemonics");
    println!("  2. ElementsRpcSigner - Node wallet delegation via RPC\n");

    println!("✓ Both implement the Signer trait, enabling:");
    println!("  - Uniform API across all signing operations");
    println!("  - Easy switching between signing backends");
    println!("  - Preparation for external signers (HSM, hardware wallet)\n");

    println!("✓ Use LwkSoftwareSigner when:");
    println!("  - Running automated tests without node setup");
    println!("  - Need independent key management");
    println!("  - Testing in CI/CD environments\n");

    println!("✓ Use ElementsRpcSigner when:");
    println!("  - Integrating with existing node wallet");
    println!("  - Manual testing and debugging");
    println!("  - Node-based workflows are already established\n");

    println!("✓ Future: Add HsmSigner or HardwareWalletSigner");
    println!("  - Same Signer trait interface");
    println!("  - Drop-in replacement in existing code");
    println!("  - No changes needed to calling code\n");

    println!("=== Example Complete ===\n");

    Ok(())
}

/// Demonstrates that a signer implements the Signer trait correctly
///
/// This function accepts any type that implements the Signer trait,
/// demonstrating the polymorphic nature of the design.
async fn demonstrate_signer_trait(signer: &dyn Signer) -> Result<(), Box<dyn std::error::Error>> {
    // Note: We don't actually sign a transaction here because we don't have
    // a valid unsigned transaction. In real usage, you would pass actual
    // unsigned transaction hex to sign_transaction().

    println!("  - Implements async sign_transaction() method");
    println!("  - Can be used as &dyn Signer trait object");
    println!("  - Compatible with all methods accepting Signer trait");

    // Demonstrate downcasting capability
    if signer.as_any().downcast_ref::<LwkSoftwareSigner>().is_some() {
        println!("  - Concrete type: LwkSoftwareSigner");
    } else if signer
        .as_any()
        .downcast_ref::<ElementsRpcSigner>()
        .is_some()
    {
        println!("  - Concrete type: ElementsRpcSigner");
    }

    Ok(())
}

/// Example of a function that accepts any signer implementation
///
/// This demonstrates how asset operations can work with any signer type.
#[allow(dead_code)]
async fn example_asset_operation(
    _signer: &dyn Signer,
    _unsigned_tx_hex: &str,
) -> Result<String, SignerError> {
    // In real usage:
    // let signed_tx = signer.sign_transaction(unsigned_tx_hex).await?;
    // // Broadcast signed_tx to network
    // Ok(signed_tx)

    Ok("example_txid".to_string())
}
