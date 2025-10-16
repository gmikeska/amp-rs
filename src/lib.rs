//! # AMP Client Library
//! 
//! A comprehensive Rust client library for the Blockstream AMP (Asset Management Platform) API
//! with integrated transaction signing capabilities for Elements/Liquid networks.
//! 
//! ## Modules
//! 
//! - [`client`] - HTTP API client for AMP operations
//! - [`model`] - Data structures for API requests and responses  
//! - [`mocks`] - Mock implementations for testing
//! - [`signer`] - Transaction signing implementations ⚠️ **TESTNET ONLY**
//! 
//! ## Signer Security Warning
//! 
//! The [`signer`] module provides software-based transaction signing using mnemonic phrases.
//! 
//! **⚠️ CRITICAL SECURITY NOTICE ⚠️**
//! 
//! The signer implementations in this library are designed **EXCLUSIVELY** for testnet
//! and regtest environments. They store mnemonic phrases in plain text and should
//! **NEVER** be used in production or with real funds.
//! 
//! For production use cases, integrate with:
//! - Hardware wallets (Ledger, Trezor)
//! - Encrypted key storage solutions
//! - Remote signing services with proper security
//! - Hardware Security Modules (HSMs)

pub mod client;
pub mod mocks;
pub mod model;
pub mod signer;

pub use client::{ApiClient, Error};
pub use signer::{Signer, LwkSoftwareSigner, SignerError};
