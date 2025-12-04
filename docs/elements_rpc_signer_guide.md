# ElementsRpcSigner Guide

## Overview

The `ElementsRpcSigner` is a signer implementation that wraps an Elements node's wallet signing capability. It provides a uniform `Signer` trait interface for signing transactions using the node's wallet, making it easy to switch between different signing backends without changing application code.

## ⚠️ SECURITY WARNING ⚠️

**TESTNET/REGTEST ONLY**: This signer delegates signing to an Elements node's wallet. Ensure the node is properly secured and only used in testnet or regtest environments for development and testing purposes.

## Architecture

The `ElementsRpcSigner` acts as a bridge between the `Signer` trait and the Elements node's wallet RPC interface. This design provides several benefits:

1. **Uniform Interface**: All signers implement the same `Signer` trait
2. **Easy Backend Switching**: Swap between software signers, node wallets, and HSMs without code changes
3. **Future-Proof**: Prepares the codebase for external signer integration
4. **Consistent API**: Maintains consistency across all signer types

## When to Use ElementsRpcSigner

### Use Cases

- **Integration Testing**: When you need to test against a running node with an existing wallet
- **Manual Testing**: For debugging and development with familiar node wallet tools
- **Node-Based Workflows**: When your infrastructure already uses Elements node wallets
- **Gradual Migration**: When transitioning from node-based to software-based signing

### Comparison with LwkSoftwareSigner

| Feature | ElementsRpcSigner | LwkSoftwareSigner |
|---------|-------------------|-------------------|
| Key Storage | Node wallet | In-memory from mnemonic |
| Setup Required | Node + wallet | Just mnemonic |
| Network Dependency | Requires node connection | Self-contained |
| Best For | Integration testing | Automated testing |
| Portability | Tied to specific node | Portable across environments |
| CI/CD Friendly | Requires node setup | Minimal dependencies |

## Basic Usage

### Creating an ElementsRpcSigner

```rust
use amp_rs::{ElementsRpc, signer::ElementsRpcSigner};

// Create an Elements RPC client
let rpc = ElementsRpc::new(
    "http://localhost:18884".to_string(),
    "user".to_string(),
    "pass".to_string()
);

// Create node-based signer
let signer = ElementsRpcSigner::new(rpc);
```

### Using with Environment Variables

```rust
use amp_rs::{ElementsRpc, signer::ElementsRpcSigner};

// Load from environment variables:
// - ELEMENTS_RPC_URL
// - ELEMENTS_RPC_USER
// - ELEMENTS_RPC_PASSWORD
let rpc = ElementsRpc::from_env()?;
let signer = ElementsRpcSigner::new(rpc);
```

### Signing a Transaction

```rust
use amp_rs::signer::Signer;

// Sign an unsigned transaction
let unsigned_tx = "020000000001..."; // Unsigned transaction hex
let signed_tx = signer.sign_transaction(unsigned_tx).await?;

println!("Transaction signed: {}", signed_tx);
```

## Integration with Asset Operations

The `ElementsRpcSigner` works seamlessly with all asset operations that accept a `&dyn Signer`:

### Asset Reissuance

```rust
use amp_rs::{ApiClient, ElementsRpc, signer::ElementsRpcSigner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_client = ApiClient::new().await?;
    let rpc = ElementsRpc::from_env()?;
    let signer = ElementsRpcSigner::new(rpc.clone());
    
    // Use node signer for reissuance
    api_client.reissue_asset(
        "asset-uuid-123",
        1000000,
        &rpc,
        &signer  // Node wallet will sign
    ).await?;
    
    Ok(())
}
```

### Asset Distribution

```rust
use amp_rs::model::AssetDistributionAssignment;

let assignments = vec![
    AssetDistributionAssignment {
        address: "tex1q...".to_string(),
        amount: 500000,
    },
    AssetDistributionAssignment {
        address: "tex1q...".to_string(),
        amount: 300000,
    },
];

api_client.distribute_asset(
    "asset-uuid-123",
    assignments,
    &rpc,
    "wallet_name",
    &signer
).await?;
```

### Asset Burning

```rust
api_client.burn_asset(
    "asset-uuid-123",
    100000,
    &rpc,
    "wallet_name",
    &signer
).await?;
```

## Polymorphic Usage

One of the key benefits of the signer architecture is polymorphic usage. You can write functions that accept any signer implementation:

```rust
use amp_rs::signer::{Signer, SignerError};

async fn sign_and_process(
    signer: &dyn Signer,
    unsigned_tx: &str
) -> Result<String, SignerError> {
    // Works with ElementsRpcSigner, LwkSoftwareSigner, or any future signer
    let signed_tx = signer.sign_transaction(unsigned_tx).await?;
    
    // Process the signed transaction
    // ...
    
    Ok(signed_tx)
}

// Use with ElementsRpcSigner
let node_signer = ElementsRpcSigner::new(rpc);
sign_and_process(&node_signer, tx_hex).await?;

// Or use with LwkSoftwareSigner
let (_, lwk_signer) = LwkSoftwareSigner::generate_new()?;
sign_and_process(&lwk_signer, tx_hex).await?;
```

## Node Wallet Requirements

For `ElementsRpcSigner` to work properly, the Elements node must meet these requirements:

### 1. Node Configuration

```conf
# elements.conf
server=1
rpcuser=user
rpcpassword=pass
rpcport=18884

# For testnet
chain=liquidtestnet
```

### 2. Wallet Setup

The node must have a wallet loaded and unlocked:

```bash
# Create wallet (if needed)
elements-cli createwallet "mywallet"

# Load wallet
elements-cli loadwallet "mywallet"

# Unlock wallet (if encrypted)
elements-cli walletpassphrase "password" 600
```

### 3. Verify Wallet Status

```rust
// Verify node connectivity
let network_info = rpc.get_network_info().await?;
println!("Connected to node: {}", network_info.subversion);

// Verify wallet is loaded by attempting a wallet RPC call
let addr = rpc.get_new_address("", Some("bech32")).await?;
println!("Wallet is accessible: {}", addr);
```

## Error Handling

The `ElementsRpcSigner` can return several types of errors:

```rust
use amp_rs::signer::{Signer, SignerError};

match signer.sign_transaction(unsigned_tx).await {
    Ok(signed_tx) => {
        println!("Transaction signed: {}", signed_tx);
    },
    Err(SignerError::Lwk(msg)) => {
        // Node wallet error (wallet locked, missing keys, etc.)
        eprintln!("Node wallet error: {}", msg);
        eprintln!("Ensure wallet is loaded and unlocked");
    },
    Err(SignerError::InvalidTransaction(msg)) => {
        // Transaction validation error or signing incomplete
        eprintln!("Transaction error: {}", msg);
    },
    Err(e) => {
        // Other errors
        eprintln!("Signing failed: {}", e);
    }
}
```

## Common Issues and Solutions

### Issue: "Node wallet signing failed"

**Cause**: Node is offline, unreachable, or credentials are incorrect

**Solution**:
```bash
# Verify node is running
elements-cli getnetworkinfo

# Check RPC credentials in environment
echo $ELEMENTS_RPC_URL
echo $ELEMENTS_RPC_USER
```

### Issue: "Transaction signing incomplete"

**Cause**: Node wallet doesn't contain the private keys needed to sign all inputs

**Solution**:
```bash
# Import the required private keys
elements-cli importprivkey "<private_key>"

# Or use the correct wallet that owns the inputs
elements-cli loadwallet "correct_wallet"
```

### Issue: "Wallet is locked"

**Cause**: Wallet is encrypted and needs to be unlocked

**Solution**:
```bash
# Unlock wallet for 10 minutes (600 seconds)
elements-cli walletpassphrase "password" 600
```

## Testing Strategies

### Unit Testing

For unit tests, use mock implementations or `LwkSoftwareSigner` to avoid node dependencies:

```rust
#[cfg(test)]
mod tests {
    use amp_rs::signer::LwkSoftwareSigner;
    
    #[tokio::test]
    async fn test_asset_operation() {
        // Use software signer for fast, isolated tests
        let (_, signer) = LwkSoftwareSigner::generate_new()?;
        
        // Test your code...
    }
}
```

### Integration Testing

For integration tests with a real node, use `ElementsRpcSigner`:

```rust
#[tokio::test]
#[ignore] // Requires live node
async fn test_with_node_wallet() {
    let rpc = ElementsRpc::from_env().unwrap();
    let signer = ElementsRpcSigner::new(rpc);
    
    // Test with real node wallet...
}
```

## Migration Path

### From Direct Node RPC to Signer Trait

**Before** (direct node wallet usage):
```rust
// Old code: node does signing internally
let reissue_result = node_rpc.reissueasset(asset_id, amount).await?;
```

**After** (using signer trait):
```rust
// New code: explicit signer interface
let signer = ElementsRpcSigner::new(node_rpc.clone());
api_client.reissue_asset(asset_uuid, amount, &node_rpc, &signer).await?;
```

### From Node Wallet to Software Signer

When ready to move away from node wallet dependency:

```rust
// Before: Using node wallet
let signer = ElementsRpcSigner::new(rpc);

// After: Using software signer (no node wallet required)
let (_, signer) = LwkSoftwareSigner::generate_new()?;

// Rest of code stays the same!
api_client.reissue_asset(asset_uuid, amount, &rpc, &signer).await?;
```

## Future: External Signer Support

The signer architecture is designed to support future external signers with no code changes:

```rust
// Future: HSM signer (not yet implemented)
// let signer = HsmSigner::new(hsm_config)?;

// Future: Hardware wallet signer (not yet implemented)
// let signer = LedgerSigner::new(device)?;

// All work with the same interface!
// api_client.reissue_asset(asset_uuid, amount, &rpc, &signer).await?;
```

## Best Practices

1. **Environment Configuration**: Use environment variables for node credentials
2. **Error Handling**: Always check wallet status before attempting operations
3. **Testing Strategy**: Use software signers for unit tests, node signers for integration tests
4. **Wallet Management**: Keep wallets unlocked only as long as necessary
5. **Documentation**: Document which wallet/keys are required for each operation
6. **Logging**: Enable debug logging to diagnose signing issues

## See Also

- [Signer Integration Guide](signer_integration_guide.md) - General signer usage patterns
- [LwkSoftwareSigner Documentation](../src/signer/lwk.rs) - Software-based signer
- [Signer Comparison Example](../examples/signer_comparison.rs) - Compare different signers
