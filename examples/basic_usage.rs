use std::path::Path;
use threatflux_hashing::{calculate_all_hashes, calculate_md5};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Calculate all hashes for a file
    println!("=== Calculate All Hashes ===");
    let path = Path::new("Cargo.toml");

    match calculate_all_hashes(path).await {
        Ok(hashes) => {
            println!("File: {}", path.display());
            println!("MD5:    {:?}", hashes.md5);
            println!("SHA256: {:?}", hashes.sha256);
            println!("SHA512: {:?}", hashes.sha512);
            println!("BLAKE3: {:?}", hashes.blake3);
        }
        Err(e) => eprintln!("Error calculating hashes: {}", e),
    }

    println!("\n=== Calculate Single Hash ===");
    // Example 2: Calculate only MD5
    match calculate_md5(path).await {
        Ok(md5) => println!("MD5 only: {}", md5),
        Err(e) => eprintln!("Error calculating MD5: {}", e),
    }

    Ok(())
}
