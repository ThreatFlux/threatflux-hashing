# Migration Guide: Using threatflux-hashing in file-scanner

This guide explains how to update the file-scanner project to use the new `threatflux-hashing` library.

## Step 1: Update Cargo.toml

Replace the individual hash dependencies with the new library:

```toml
# Remove these dependencies:
# sha2 = "0.10"
# md-5 = "0.10"
# blake3 = "1.5"

# Add this instead:
threatflux-hashing = { version = "0.1.0", features = ["serde"] }
```

## Step 2: Create Compatibility Wrapper

To minimize changes in the existing codebase, create a new `src/hash.rs` that wraps the library:

```rust
// src/hash.rs - Compatibility wrapper
pub use threatflux_hashing::{
    calculate_all_hashes,
    calculate_md5,
    Hashes,
};

// Re-export for backward compatibility
pub use threatflux_hashing::Result;
```

## Step 3: Update Imports

No changes needed in other files! The wrapper maintains the same API.

## Step 4: Optional - Use New Features

Once migrated, you can take advantage of new features:

### Custom Configuration

```rust
use threatflux_hashing::{calculate_all_hashes_with_config, HashConfig, HashAlgorithms};

let config = HashConfig {
    algorithms: HashAlgorithms {
        md5: true,
        sha256: true,
        sha512: false,  // Skip for performance
        blake3: true,
    },
    buffer_size: 16384,
    max_concurrent_operations: 20,
};

let hashes = calculate_all_hashes_with_config(path, &config).await?;
```

### Selective Hashing for Performance

For the `llm_analyze_file` tool that only needs MD5:

```rust
use threatflux_hashing::{calculate_all_hashes_with_config, HashConfig, HashAlgorithms};

let config = HashConfig {
    algorithms: HashAlgorithms::only_md5(),
    ..Default::default()
};

let hashes = calculate_all_hashes_with_config(path, &config).await?;
let md5 = hashes.md5.unwrap_or_default();
```

## Step 5: Testing

Run the existing test suite to ensure compatibility:

```bash
cargo test
cargo test --package file-scanner hash
```

## Benefits of Migration

1. **Maintainability**: Hash functionality is now a separate, reusable library
2. **Performance**: Can skip unnecessary hash calculations
3. **Flexibility**: Configure buffer sizes and concurrency
4. **Future Features**: Easy to add new hash algorithms or features

## Publishing the Library

If you want to publish to crates.io:

```bash
cd threatflux-hashing
cargo publish --dry-run  # Test first
cargo publish           # Actually publish
```

## Alternative: Git Dependency

For private use, you can use a git dependency:

```toml
threatflux-hashing = { git = "https://github.com/ThreatFlux/threatflux-hashing" }
```

Or a local path during development:

```toml
threatflux-hashing = { path = "./threatflux-hashing" }
```