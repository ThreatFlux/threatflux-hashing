# ThreatFlux Hashing

A high-performance async file hashing library for Rust, supporting MD5, SHA256, SHA512, and BLAKE3 algorithms with concurrent processing capabilities.

## Features

- 🚀 **Async/Await Support** - Built on tokio for efficient async operations
- ⚡ **Concurrent Processing** - Calculate multiple hashes in parallel
- 🔧 **Configurable** - Customize buffer sizes, concurrency limits, and algorithms
- 📦 **Selective Hashing** - Choose which algorithms to calculate
- 🔄 **Zero-Copy Operations** - Efficient memory usage
- 📝 **Optional Serde Support** - Serialize/deserialize hash results

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
threatflux-hashing = "0.1.0"
```

## Quick Start

```rust
use threatflux_hashing::calculate_all_hashes;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hashes = calculate_all_hashes(Path::new("file.bin")).await?;
    
    println!("MD5:    {:?}", hashes.md5);
    println!("SHA256: {:?}", hashes.sha256);
    println!("SHA512: {:?}", hashes.sha512);
    println!("BLAKE3: {:?}", hashes.blake3);
    
    Ok(())
}
```

## Advanced Usage

### Custom Configuration

```rust
use threatflux_hashing::{calculate_all_hashes_with_config, HashConfig, HashAlgorithms};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = HashConfig {
        algorithms: HashAlgorithms {
            md5: true,
            sha256: true,
            sha512: false,  // Skip SHA512 for speed
            blake3: true,
        },
        buffer_size: 16384,  // 16KB buffer
        max_concurrent_operations: 20,
    };
    
    let hashes = calculate_all_hashes_with_config(Path::new("file.bin"), &config).await?;
    Ok(())
}
```

### Single Hash Calculation

```rust
use threatflux_hashing::calculate_md5;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let md5 = calculate_md5(Path::new("file.bin")).await?;
    println!("MD5: {}", md5);
    Ok(())
}
```

### Concurrent File Processing

```rust
use threatflux_hashing::calculate_all_hashes;
use tokio::task::JoinSet;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = vec!["file1.bin", "file2.bin", "file3.bin"];
    let mut tasks = JoinSet::new();
    
    for file in files {
        tasks.spawn(async move {
            calculate_all_hashes(Path::new(file)).await
        });
    }
    
    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok(hashes)) => println!("Hashes: {:?}", hashes),
            Ok(Err(e)) => eprintln!("Hash error: {}", e),
            Err(e) => eprintln!("Task error: {}", e),
        }
    }
    
    Ok(())
}
```

## Performance Tips

1. **Buffer Size**: Larger buffers (16KB-64KB) generally improve performance for large files
2. **Algorithm Selection**: Skip unnecessary algorithms to reduce computation time
3. **Concurrency**: Adjust `max_concurrent_operations` based on your system's capabilities
4. **BLAKE3**: Consider using BLAKE3 for the best performance-to-security ratio

## Supported Algorithms

| Algorithm | Hash Length | Performance | Use Case |
|-----------|-------------|-------------|----------|
| MD5       | 128 bits    | Fast        | Legacy compatibility, checksums |
| SHA256    | 256 bits    | Moderate    | General purpose, secure |
| SHA512    | 512 bits    | Slower      | High security requirements |
| BLAKE3    | 256 bits    | Very Fast   | Modern applications |

## Error Handling

The library uses custom error types for better error handling:

```rust
use threatflux_hashing::{calculate_all_hashes, HashError};
use std::path::Path;

#[tokio::main]
async fn main() {
    match calculate_all_hashes(Path::new("nonexistent.bin")).await {
        Ok(hashes) => println!("Success: {:?}", hashes),
        Err(HashError::Io(e)) => eprintln!("IO Error: {}", e),
        Err(e) => eprintln!("Other error: {}", e),
    }
}
```

## Features

- `serde` (default): Enable serialization/deserialization support

```toml
[dependencies]
threatflux-hashing = { version = "0.1.0", default-features = false }
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Benchmarks

Run benchmarks with:

```bash
cargo bench
```

Typical performance on modern hardware:
- MD5: ~500 MB/s
- SHA256: ~300 MB/s
- SHA512: ~200 MB/s
- BLAKE3: ~1 GB/s

Performance varies based on file size, system capabilities, and configuration.