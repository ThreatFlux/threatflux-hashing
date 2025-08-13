//! # ThreatFlux Hashing
//!
//! A high-performance async file hashing library supporting MD5, SHA256, SHA512, and BLAKE3.
//!
//! ## Features
//!
//! - Async/await support with tokio
//! - Concurrent hash calculation
//! - Configurable buffer sizes and concurrency limits
//! - Optional serde support
//! - Zero-copy operations where possible
//!
//! ## Quick Start
//!
//! ```no_run
//! use threatflux_hashing::calculate_all_hashes;
//! use std::path::Path;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let hashes = calculate_all_hashes(Path::new("file.bin")).await?;
//!     println!("MD5: {:?}", hashes.md5);
//!     println!("SHA256: {:?}", hashes.sha256);
//!     println!("SHA512: {:?}", hashes.sha512);
//!     println!("BLAKE3: {:?}", hashes.blake3);
//!     Ok(())
//! }
//! ```
//!
//! ## Custom Configuration
//!
//! ```no_run
//! use threatflux_hashing::{calculate_all_hashes_with_config, HashConfig, HashAlgorithms};
//! use std::path::Path;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = HashConfig {
//!         algorithms: HashAlgorithms {
//!             md5: true,
//!             sha256: true,
//!             sha512: false,  // Skip SHA512
//!             blake3: true,
//!         },
//!         buffer_size: 16384,  // 16KB buffer
//!         max_concurrent_operations: 20,
//!     };
//!     
//!     let hashes = calculate_all_hashes_with_config(Path::new("file.bin"), &config).await?;
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod hasher;

pub use error::{HashError, Result};
pub use hasher::{
    calculate_all_hashes, calculate_all_hashes_with_config, calculate_md5, HashAlgorithms,
    HashConfig, Hashes,
};

// Re-export futures for convenience
pub use tokio;
