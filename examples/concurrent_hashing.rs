use std::path::Path;
use std::time::Instant;
use threatflux_hashing::{calculate_all_hashes_with_config, HashAlgorithms, HashConfig};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure custom settings
    let config = HashConfig {
        algorithms: HashAlgorithms::all(),
        buffer_size: 16384, // 16KB buffer
        max_concurrent_operations: 20,
    };

    // Example 1: Hash multiple files concurrently
    println!("=== Concurrent File Hashing ===");
    let files = vec!["Cargo.toml", "src/lib.rs", "examples/basic_usage.rs"];

    let start = Instant::now();
    let mut tasks = JoinSet::new();

    for file in &files {
        let path = Path::new(file).to_path_buf();
        let config_clone = config.clone();

        tasks.spawn(async move {
            let result = calculate_all_hashes_with_config(&path, &config_clone).await;
            (path, result)
        });
    }

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok((path, Ok(hashes))) => {
                println!("\nFile: {}", path.display());
                println!("  MD5:    {:?}", hashes.md5);
                println!("  SHA256: {:?}", hashes.sha256);
            }
            Ok((path, Err(e))) => {
                eprintln!("Error hashing {}: {}", path.display(), e);
            }
            Err(e) => eprintln!("Task error: {}", e),
        }
    }

    println!("\nTotal time: {:?}", start.elapsed());

    // Example 2: Selective algorithm hashing
    println!("\n=== Selective Algorithm Hashing ===");
    let fast_config = HashConfig {
        algorithms: HashAlgorithms {
            md5: true,
            sha256: false, // Skip SHA256
            sha512: false, // Skip SHA512
            blake3: true,  // BLAKE3 is fast!
        },
        buffer_size: 32768, // 32KB buffer for speed
        max_concurrent_operations: config.max_concurrent_operations,
    };

    let path = Path::new("Cargo.toml");
    let start = Instant::now();
    let hashes = calculate_all_hashes_with_config(path, &fast_config).await?;

    println!("Fast hashing completed in: {:?}", start.elapsed());
    println!("MD5:    {:?}", hashes.md5);
    println!("BLAKE3: {:?}", hashes.blake3);
    println!("SHA256: {:?} (skipped)", hashes.sha256);
    println!("SHA512: {:?} (skipped)", hashes.sha512);

    Ok(())
}
