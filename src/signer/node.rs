//! # Node-based Signer Implementation
//!
//! This module provides a `Signer` implementation that wraps an Elements node's
//! wallet signing capability. This allows the node's wallet to be used as a signer
//! through the standard `Signer` trait interface, enabling consistent signing patterns
//! across different signer backends (software signers, hardware wallets, HSMs, etc.).
//!
//! ## ⚠️ SECURITY WARNING ⚠️
//!
//! **TESTNET/REGTEST ONLY**: This implementation delegates signing to an Elements
//! node's wallet. Ensure the node is properly secured and only used in testnet or
//! regtest environments for development and testing purposes.
//!
//! ## Architecture
//!
//! The `ElementsRpcSigner` acts as a bridge between the `Signer` trait and the
//! Elements node's wallet RPC interface. When `sign_transaction()` is called:
//!
//! 1. The unsigned transaction hex is sent to the node via `signrawtransactionwithwallet` RPC
//! 2. The node's wallet signs the transaction using its managed private keys
//! 3. The signed transaction is returned through the trait interface
//!
//! This design allows seamless switching between different signing backends without
//! changing the calling code.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use amp_rs::signer::{Signer, ElementsRpcSigner};
//! use amp_rs::ElementsRpc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create Elements RPC client
//!     let rpc = ElementsRpc::new(
//!         "http://localhost:18884".to_string(),
//!         "user".to_string(),
//!         "pass".to_string()
//!     );
//!     
//!     // Create node-based signer
//!     let signer = ElementsRpcSigner::new(rpc);
//!     
//!     // Use signer through trait interface
//!     let unsigned_tx = "020000000001..."; // Unsigned transaction hex
//!     let signed_tx = signer.sign_transaction(unsigned_tx).await?;
//!     
//!     println!("Transaction signed: {}", signed_tx);
//!     Ok(())
//! }
//! ```
//!
//! ## Integration with Asset Operations
//!
//! The `ElementsRpcSigner` can be used with any method that accepts a `&dyn Signer`:
//!
//! ```rust,no_run
//! use amp_rs::{ApiClient, ElementsRpc};
//! use amp_rs::signer::ElementsRpcSigner;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let api_client = ApiClient::new().await?;
//!     let rpc = ElementsRpc::from_env()?;
//!     let signer = ElementsRpcSigner::new(rpc.clone());
//!     
//!     // Use node signer for reissuance
//!     api_client.reissue_asset(
//!         "asset-uuid-123",
//!         1000000,
//!         &rpc,
//!         &signer  // Node wallet will sign
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```

use super::error::SignerError;
use super::Signer;
use crate::ElementsRpc;
use async_trait::async_trait;

/// Signer implementation that uses an Elements node's wallet for transaction signing
///
/// This signer wraps an `ElementsRpc` client and delegates all signing operations
/// to the connected Elements node's wallet via the `signrawtransactionwithwallet` RPC call.
///
/// # Architecture Benefits
///
/// By wrapping the node's signing capability in the `Signer` trait:
/// - Provides a uniform interface for all signing operations
/// - Enables easy switching between signing backends (node wallet, software signer, HSM)
/// - Prepares the codebase for external signer integration
/// - Maintains consistency with the trait-based design pattern
///
/// # Security Considerations
///
/// - The underlying Elements node must be properly secured
/// - Wallet must be unlocked for signing operations
/// - Use only in testnet/regtest environments for testing
/// - For production, consider HSM-backed signers instead
///
/// # Example
///
/// ```rust,no_run
/// use amp_rs::{ElementsRpc, signer::{Signer, ElementsRpcSigner}};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rpc = ElementsRpc::from_env()?;
///     let signer = ElementsRpcSigner::new(rpc);
///     
///     let unsigned_tx = "020000000001...";
///     let signed_tx = signer.sign_transaction(unsigned_tx).await?;
///     
///     println!("Signed: {}", signed_tx);
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct ElementsRpcSigner {
    /// The Elements RPC client used to communicate with the node
    rpc: ElementsRpc,
}

impl ElementsRpcSigner {
    /// Creates a new `ElementsRpcSigner` that wraps the given Elements RPC client
    ///
    /// # Arguments
    ///
    /// * `rpc` - An `ElementsRpc` client connected to an Elements node with an unlocked wallet
    ///
    /// # Returns
    ///
    /// Returns a new `ElementsRpcSigner` instance
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use amp_rs::{ElementsRpc, signer::ElementsRpcSigner};
    ///
    /// let rpc = ElementsRpc::new(
    ///     "http://localhost:18884".to_string(),
    ///     "user".to_string(),
    ///     "pass".to_string()
    /// );
    /// let signer = ElementsRpcSigner::new(rpc);
    /// ```
    pub fn new(rpc: ElementsRpc) -> Self {
        Self { rpc }
    }

    /// Gets a reference to the underlying Elements RPC client
    ///
    /// This can be useful for performing additional node operations alongside signing.
    ///
    /// # Returns
    ///
    /// Returns a reference to the wrapped `ElementsRpc` client
    pub fn rpc(&self) -> &ElementsRpc {
        &self.rpc
    }
}

#[async_trait]
impl Signer for ElementsRpcSigner {
    /// Signs an unsigned transaction using the Elements node's wallet
    ///
    /// This method sends the unsigned transaction to the Elements node via the
    /// `signrawtransactionwithwallet` RPC call. The node's wallet must be unlocked
    /// and contain the necessary private keys to sign all inputs.
    ///
    /// # Arguments
    ///
    /// * `unsigned_tx` - Hex-encoded unsigned Elements/Liquid transaction
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// - `Ok(String)` - Hex-encoded signed transaction on success
    /// - `Err(SignerError)` - Error if signing fails
    ///
    /// # Errors
    ///
    /// This method can return various `SignerError` variants:
    /// - `SignerError::HexParse` - Invalid hex encoding in input
    /// - `SignerError::InvalidTransaction` - Malformed transaction or signing failed
    /// - `SignerError::Lwk` - Node wallet error (wallet locked, missing keys, etc.)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use amp_rs::{ElementsRpc, signer::{Signer, ElementsRpcSigner}};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let rpc = ElementsRpc::from_env()?;
    ///     let signer = ElementsRpcSigner::new(rpc);
    ///     
    ///     let unsigned_tx = "020000000001...";
    ///     match signer.sign_transaction(unsigned_tx).await {
    ///         Ok(signed_tx) => println!("Transaction signed: {}", signed_tx),
    ///         Err(e) => eprintln!("Signing failed: {}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    async fn sign_transaction(&self, unsigned_tx: &str) -> Result<String, SignerError> {
        tracing::debug!(
            "ElementsRpcSigner: Signing transaction via node wallet: {}...",
            &unsigned_tx[..std::cmp::min(unsigned_tx.len(), 64)]
        );

        // Validate hex format before sending to node
        if unsigned_tx.is_empty() {
            return Err(SignerError::InvalidTransaction(
                "Unsigned transaction hex cannot be empty".to_string(),
            ));
        }

        if unsigned_tx.len() % 2 != 0 {
            return Err(SignerError::InvalidTransaction(
                "Unsigned transaction hex must have even length".to_string(),
            ));
        }

        // Call the Elements node's signrawtransactionwithwallet RPC method
        let sign_result = self
            .rpc
            .rpc_call::<serde_json::Value>(
                "signrawtransactionwithwallet",
                serde_json::json!([unsigned_tx]),
            )
            .await
            .map_err(|e| {
                SignerError::Lwk(format!(
                    "Node wallet signing failed: {}. Ensure wallet is unlocked and contains required keys.",
                    e
                ))
            })?;

        // Extract the signed transaction hex from the result
        let signed_tx = sign_result
            .get("hex")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                SignerError::InvalidTransaction(
                    "Node signing result missing 'hex' field".to_string(),
                )
            })?
            .to_string();

        // Check if signing was complete
        let complete = sign_result
            .get("complete")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if !complete {
            // If signing wasn't complete, include error details if available
            let errors = sign_result
                .get("errors")
                .and_then(|v| serde_json::to_string(v).ok())
                .unwrap_or_else(|| "Unknown errors".to_string());

            return Err(SignerError::InvalidTransaction(format!(
                "Transaction signing incomplete. The node's wallet may be missing required keys. Errors: {}",
                errors
            )));
        }

        tracing::debug!(
            "ElementsRpcSigner: Transaction signed successfully by node wallet: {}...",
            &signed_tx[..std::cmp::min(signed_tx.len(), 64)]
        );

        Ok(signed_tx)
    }

    /// Returns self as Any for downcasting to concrete types
    ///
    /// This method enables downcasting from the trait object to the concrete
    /// `ElementsRpcSigner` implementation when needed.
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elements_rpc_signer_creation() {
        let rpc = ElementsRpc::new(
            "http://localhost:18884".to_string(),
            "testuser".to_string(),
            "testpass".to_string(),
        );

        let signer = ElementsRpcSigner::new(rpc.clone());

        // Verify signer can access the RPC client
        assert_eq!(signer.rpc().base_url(), "http://localhost:18884");
    }

    #[test]
    fn test_elements_rpc_signer_clone() {
        let rpc = ElementsRpc::new(
            "http://localhost:18884".to_string(),
            "testuser".to_string(),
            "testpass".to_string(),
        );

        let signer1 = ElementsRpcSigner::new(rpc);
        let signer2 = signer1.clone();

        // Verify both signers work independently
        assert_eq!(signer1.rpc().base_url(), signer2.rpc().base_url());
    }

    #[test]
    fn test_signer_trait_object() {
        let rpc = ElementsRpc::new(
            "http://localhost:18884".to_string(),
            "testuser".to_string(),
            "testpass".to_string(),
        );

        let signer = ElementsRpcSigner::new(rpc);

        // Verify signer can be used as trait object
        let _trait_obj: &dyn Signer = &signer;
    }

    #[test]
    fn test_as_any_downcast() {
        let rpc = ElementsRpc::new(
            "http://localhost:18884".to_string(),
            "testuser".to_string(),
            "testpass".to_string(),
        );

        let signer = ElementsRpcSigner::new(rpc);
        let trait_obj: &dyn Signer = &signer;

        // Verify downcasting works
        let downcasted = trait_obj.as_any().downcast_ref::<ElementsRpcSigner>();
        assert!(downcasted.is_some());
    }

    #[tokio::test]
    async fn test_sign_transaction_validates_input() {
        let rpc = ElementsRpc::new(
            "http://localhost:18884".to_string(),
            "testuser".to_string(),
            "testpass".to_string(),
        );

        let signer = ElementsRpcSigner::new(rpc);

        // Test empty transaction
        let result = signer.sign_transaction("").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            SignerError::InvalidTransaction(msg) => {
                assert!(msg.contains("cannot be empty"));
            }
            other => panic!("Expected InvalidTransaction error, got: {:?}", other),
        }

        // Test odd-length hex
        let result = signer.sign_transaction("abc").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            SignerError::InvalidTransaction(msg) => {
                assert!(msg.contains("even length"));
            }
            other => panic!("Expected InvalidTransaction error, got: {:?}", other),
        }
    }
}
