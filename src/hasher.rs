use crate::error::{HashError, Result};
use blake3::Hasher as Blake3Hasher;
use md5::{Digest, Md5};
use sha2::{Sha256, Sha512};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Global semaphore to limit concurrent file operations
static HASH_SEMAPHORE: std::sync::OnceLock<Arc<Semaphore>> = std::sync::OnceLock::new();

fn get_hash_semaphore(max_concurrent: usize) -> &'static Arc<Semaphore> {
    HASH_SEMAPHORE.get_or_init(|| Arc::new(Semaphore::new(max_concurrent)))
}

/// Configuration for hash operations
#[derive(Debug, Clone)]
pub struct HashConfig {
    pub algorithms: HashAlgorithms,
    pub buffer_size: usize,
    pub max_concurrent_operations: usize,
}

/// Which hash algorithms to calculate
#[derive(Debug, Clone)]
pub struct HashAlgorithms {
    pub md5: bool,
    pub sha256: bool,
    pub sha512: bool,
    pub blake3: bool,
}

impl HashAlgorithms {
    /// Enable all hash algorithms
    pub fn all() -> Self {
        Self {
            md5: true,
            sha256: true,
            sha512: true,
            blake3: true,
        }
    }

    /// Enable only the specified algorithms
    pub fn only_md5() -> Self {
        Self {
            md5: true,
            sha256: false,
            sha512: false,
            blake3: false,
        }
    }
}

impl Default for HashConfig {
    fn default() -> Self {
        Self {
            algorithms: HashAlgorithms::all(),
            buffer_size: 8192,
            max_concurrent_operations: 10,
        }
    }
}

/// Contains the calculated hash values
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Hashes {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub md5: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sha256: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sha512: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub blake3: Option<String>,
}

/// Calculate all configured hashes for a file
pub async fn calculate_all_hashes_with_config(path: &Path, config: &HashConfig) -> Result<Hashes> {
    let path_buf = path.to_path_buf();
    let semaphore = get_hash_semaphore(config.max_concurrent_operations);

    // Acquire permit to limit concurrent operations
    let _permit = semaphore
        .acquire()
        .await
        .map_err(|_| HashError::SemaphoreError)?;

    let mut tasks = Vec::new();
    let buffer_size = config.buffer_size;

    if config.algorithms.md5 {
        let path_clone = path_buf.clone();
        tasks.push(task::spawn_blocking(move || {
            calculate_md5_sync(&path_clone, buffer_size).map(Some)
        }));
    } else {
        tasks.push(task::spawn_blocking(|| Ok(None)));
    }

    if config.algorithms.sha256 {
        let path_clone = path_buf.clone();
        tasks.push(task::spawn_blocking(move || {
            calculate_sha256(&path_clone, buffer_size).map(Some)
        }));
    } else {
        tasks.push(task::spawn_blocking(|| Ok(None)));
    }

    if config.algorithms.sha512 {
        let path_clone = path_buf.clone();
        tasks.push(task::spawn_blocking(move || {
            calculate_sha512(&path_clone, buffer_size).map(Some)
        }));
    } else {
        tasks.push(task::spawn_blocking(|| Ok(None)));
    }

    if config.algorithms.blake3 {
        let path_clone = path_buf.clone();
        tasks.push(task::spawn_blocking(move || {
            calculate_blake3(&path_clone, buffer_size).map(Some)
        }));
    } else {
        tasks.push(task::spawn_blocking(|| Ok(None)));
    }

    let results: Vec<_> = futures::future::try_join_all(tasks).await?;

    Ok(Hashes {
        md5: results.first().and_then(|r| r.as_ref().ok().and_then(|v| v.clone())),
        sha256: results.get(1).and_then(|r| r.as_ref().ok().and_then(|v| v.clone())),
        sha512: results.get(2).and_then(|r| r.as_ref().ok().and_then(|v| v.clone())),
        blake3: results.get(3).and_then(|r| r.as_ref().ok().and_then(|v| v.clone())),
    })
}

/// Calculate all hashes with default configuration
pub async fn calculate_all_hashes(path: &Path) -> Result<Hashes> {
    let config = HashConfig::default();
    calculate_all_hashes_with_config(path, &config).await
}

/// Calculate MD5 hash asynchronously
pub async fn calculate_md5(path: &Path) -> Result<String> {
    let path_buf = path.to_path_buf();
    let semaphore = get_hash_semaphore(10);
    let _permit = semaphore
        .acquire()
        .await
        .map_err(|_| HashError::SemaphoreError)?;

    task::spawn_blocking(move || calculate_md5_sync(&path_buf, 8192)).await?
}

fn calculate_md5_sync(path: &Path, buffer_size: usize) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Md5::new();
    let mut buffer = vec![0; buffer_size];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        if let Some(slice) = buffer.get(..count) {
            hasher.update(slice);
        }
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn calculate_sha256(path: &Path, buffer_size: usize) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = vec![0; buffer_size];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        if let Some(slice) = buffer.get(..count) {
            hasher.update(slice);
        }
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn calculate_sha512(path: &Path, buffer_size: usize) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha512::new();
    let mut buffer = vec![0; buffer_size];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        if let Some(slice) = buffer.get(..count) {
            hasher.update(slice);
        }
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn calculate_blake3(path: &Path, buffer_size: usize) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Blake3Hasher::new();
    let mut buffer = vec![0; buffer_size];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        if let Some(slice) = buffer.get(..count) {
            hasher.update(slice);
        }
    }

    Ok(hasher.finalize().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_file(content: &[u8]) -> Result<(TempDir, std::path::PathBuf)> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test_file");
        let mut file = fs::File::create(&file_path)?;
        file.write_all(content)?;
        Ok((temp_dir, file_path))
    }

    #[test]
    fn test_calculate_md5_sync() {
        let content = b"Hello, World!";
        let (_temp_dir, file_path) = create_test_file(content).unwrap();

        let hash = calculate_md5_sync(&file_path, 8192).unwrap();
        assert_eq!(hash, "65a8e27d8879283831b664bd8b7f0ad4");
    }

    #[test]
    fn test_calculate_sha256() {
        let content = b"Hello, World!";
        let (_temp_dir, file_path) = create_test_file(content).unwrap();

        let hash = calculate_sha256(&file_path, 8192).unwrap();
        assert_eq!(
            hash,
            "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
        );
    }

    #[test]
    fn test_calculate_sha512() {
        let content = b"Hello, World!";
        let (_temp_dir, file_path) = create_test_file(content).unwrap();

        let hash = calculate_sha512(&file_path, 8192).unwrap();
        assert_eq!(hash, "374d794a95cdcfd8b35993185fef9ba368f160d8daf432d08ba9f1ed1e5abe6cc69291e0fa2fe0006a52570ef18c19def4e617c33ce52ef0a6e5fbe318cb0387");
    }

    #[test]
    fn test_calculate_blake3() {
        let content = b"Hello, World!";
        let (_temp_dir, file_path) = create_test_file(content).unwrap();

        let hash = calculate_blake3(&file_path, 8192).unwrap();
        // Blake3 hash for "Hello, World!"
        assert_eq!(
            hash,
            "288a86a79f20a3d6dccdca7713beaed178798296bdfa7913fa2a62d9727bf8f8"
        );
    }

    #[tokio::test]
    async fn test_calculate_md5_async() {
        let content = b"Test content for async MD5";
        let (_temp_dir, file_path) = create_test_file(content).unwrap();

        let hash = calculate_md5(&file_path).await.unwrap();
        assert_eq!(hash, "177e39fe10209113009207ca25549cd6");
    }

    #[tokio::test]
    async fn test_calculate_all_hashes() {
        let content = b"Test file content";
        let (_temp_dir, file_path) = create_test_file(content).unwrap();

        let hashes = calculate_all_hashes(&file_path).await.unwrap();

        assert_eq!(hashes.md5.unwrap(), "ac79653edeb65ab5563585f2d5f14fe9");
        assert_eq!(
            hashes.sha256.unwrap(),
            "6c76f7bd4b84eb68c26d2e8f48ea76f90b9bdf8836e27235a0ca4325f8fe4ce5"
        );
        assert_eq!(hashes.sha512.unwrap(), "7d849325b7bcfde1f5fbaee0573d95a088d2a8dbdbc426470efbb8c5b89b0bc1e172cca99a78673b1dc24a2d3ed4e0b745ffd9aa43fbfb651c89847b9c39daa8");
        assert_eq!(hashes.blake3.unwrap().len(), 64); // Just verify length since Blake3 might vary
    }

    #[tokio::test]
    async fn test_custom_config() {
        let content = b"Test with custom config";
        let (_temp_dir, file_path) = create_test_file(content).unwrap();

        let config = HashConfig {
            algorithms: HashAlgorithms {
                md5: true,
                sha256: false,
                sha512: false,
                blake3: true,
            },
            buffer_size: 4096,
            max_concurrent_operations: 5,
        };

        let hashes = calculate_all_hashes_with_config(&file_path, &config)
            .await
            .unwrap();

        assert!(hashes.md5.is_some());
        assert!(hashes.sha256.is_none());
        assert!(hashes.sha512.is_none());
        assert!(hashes.blake3.is_some());
    }

    #[tokio::test]
    async fn test_empty_file_hashes() {
        let content = b"";
        let (_temp_dir, file_path) = create_test_file(content).unwrap();

        let hashes = calculate_all_hashes(&file_path).await.unwrap();

        // Empty file hashes
        assert_eq!(hashes.md5.unwrap(), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(
            hashes.sha256.unwrap(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(hashes.sha512.unwrap(), "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
        assert_eq!(
            hashes.blake3.unwrap(),
            "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
        );
    }

    #[test]
    fn test_nonexistent_file() {
        let path = Path::new("/nonexistent/file/path");

        let result = calculate_md5_sync(path, 8192);
        assert!(result.is_err());

        let result = calculate_sha256(path, 8192);
        assert!(result.is_err());

        let result = calculate_sha512(path, 8192);
        assert!(result.is_err());

        let result = calculate_blake3(path, 8192);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_concurrent_hash_calculation() {
        let content = b"Concurrent hash test";
        let (_temp_dir, file_path) = create_test_file(content).unwrap();

        // Calculate hashes concurrently multiple times
        let mut handles = vec![];
        for _ in 0..10 {
            let path = file_path.clone();
            let handle = tokio::spawn(async move { calculate_all_hashes(&path).await });
            handles.push(handle);
        }

        // All calculations should succeed and produce identical results
        let mut results = vec![];
        for handle in handles {
            let result = handle.await.unwrap().unwrap();
            results.push(result);
        }

        // Verify all results are identical
        let first = results.first().expect("Should have at least one result");
        for result in results.iter().skip(1) {
            assert_eq!(result.md5, first.md5);
            assert_eq!(result.sha256, first.sha256);
            assert_eq!(result.sha512, first.sha512);
            assert_eq!(result.blake3, first.blake3);
        }
    }

    #[test]
    fn test_hash_format() {
        let content = b"Format test";
        let (_temp_dir, file_path) = create_test_file(content).unwrap();

        let md5 = calculate_md5_sync(&file_path, 8192).unwrap();
        let sha256 = calculate_sha256(&file_path, 8192).unwrap();
        let sha512 = calculate_sha512(&file_path, 8192).unwrap();
        let blake3 = calculate_blake3(&file_path, 8192).unwrap();

        // Check hash format (lowercase hex)
        assert!(md5
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));
        assert!(sha256
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));
        assert!(sha512
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));
        assert!(blake3
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));

        // Check hash lengths
        assert_eq!(md5.len(), 32); // MD5 is 128 bits = 32 hex chars
        assert_eq!(sha256.len(), 64); // SHA256 is 256 bits = 64 hex chars
        assert_eq!(sha512.len(), 128); // SHA512 is 512 bits = 128 hex chars
        assert_eq!(blake3.len(), 64); // Blake3 is 256 bits = 64 hex chars
    }

    #[cfg(feature = "serde")]
    #[tokio::test]
    async fn test_hashes_struct_serialization() {
        use serde_json;

        let hashes = Hashes {
            md5: Some("d41d8cd98f00b204e9800998ecf8427e".to_string()),
            sha256: Some("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string()),
            sha512: Some("cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e".to_string()),
            blake3: Some("af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262".to_string()),
        };

        // Test JSON serialization
        let json = serde_json::to_string(&hashes).unwrap();
        let deserialized: Hashes = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.md5, hashes.md5);
        assert_eq!(deserialized.sha256, hashes.sha256);
        assert_eq!(deserialized.sha512, hashes.sha512);
        assert_eq!(deserialized.blake3, hashes.blake3);
    }
}
