use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fs::File;
use std::hint::black_box;
use std::io::Write;
use tempfile::TempDir;
use threatflux_hashing::{
    calculate_all_hashes, calculate_all_hashes_with_config, HashAlgorithms, HashConfig,
};
use tokio::runtime::Runtime;

fn create_test_file(size: usize) -> (TempDir, std::path::PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temporary directory for benchmark");
    let file_path = temp_dir.path().join("bench_file");
    let mut file = File::create(&file_path).expect("Failed to create benchmark file");

    // Create predictable content
    let chunk = b"0123456789ABCDEF";
    let mut written = 0;
    while written < size {
        let to_write = std::cmp::min(chunk.len(), size - written);
        file.write_all(&chunk[..to_write]).expect("Failed to write benchmark data");
        written += to_write;
    }

    (temp_dir, file_path)
}

fn bench_all_hashes(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create tokio runtime");
    let mut group = c.benchmark_group("all_hashes");

    for size in [1024, 1024 * 1024, 10 * 1024 * 1024].iter() {
        let (_temp_dir, file_path) = create_test_file(*size);

        group.throughput(criterion::Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                rt.block_on(async { calculate_all_hashes(black_box(&file_path)).await.expect("Hash calculation failed") })
            });
        });
    }

    group.finish();
}

fn bench_individual_algorithms(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create tokio runtime");
    let size = 1024 * 1024; // 1MB file
    let (_temp_dir, file_path) = create_test_file(size);

    let mut group = c.benchmark_group("individual_algorithms");
    group.throughput(criterion::Throughput::Bytes(size as u64));

    // MD5 only
    group.bench_function("md5_only", |b| {
        let config = HashConfig {
            algorithms: HashAlgorithms {
                md5: true,
                sha256: false,
                sha512: false,
                blake3: false,
            },
            ..Default::default()
        };

        b.iter(|| {
            rt.block_on(async {
                calculate_all_hashes_with_config(black_box(&file_path), &config)
                    .await
                    .expect("Hash calculation failed")
            })
        });
    });

    // SHA256 only
    group.bench_function("sha256_only", |b| {
        let config = HashConfig {
            algorithms: HashAlgorithms {
                md5: false,
                sha256: true,
                sha512: false,
                blake3: false,
            },
            ..Default::default()
        };

        b.iter(|| {
            rt.block_on(async {
                calculate_all_hashes_with_config(black_box(&file_path), &config)
                    .await
                    .expect("Hash calculation failed")
            })
        });
    });

    // SHA512 only
    group.bench_function("sha512_only", |b| {
        let config = HashConfig {
            algorithms: HashAlgorithms {
                md5: false,
                sha256: false,
                sha512: true,
                blake3: false,
            },
            ..Default::default()
        };

        b.iter(|| {
            rt.block_on(async {
                calculate_all_hashes_with_config(black_box(&file_path), &config)
                    .await
                    .expect("Hash calculation failed")
            })
        });
    });

    // BLAKE3 only
    group.bench_function("blake3_only", |b| {
        let config = HashConfig {
            algorithms: HashAlgorithms {
                md5: false,
                sha256: false,
                sha512: false,
                blake3: true,
            },
            ..Default::default()
        };

        b.iter(|| {
            rt.block_on(async {
                calculate_all_hashes_with_config(black_box(&file_path), &config)
                    .await
                    .expect("Hash calculation failed")
            })
        });
    });

    group.finish();
}

fn bench_buffer_sizes(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create tokio runtime");
    let size = 10 * 1024 * 1024; // 10MB file
    let (_temp_dir, file_path) = create_test_file(size);

    let mut group = c.benchmark_group("buffer_sizes");
    group.throughput(criterion::Throughput::Bytes(size as u64));

    for buffer_size in [4096, 8192, 16384, 32768, 65536].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(buffer_size),
            buffer_size,
            |b, &buffer_size| {
                let config = HashConfig {
                    buffer_size,
                    ..Default::default()
                };

                b.iter(|| {
                    rt.block_on(async {
                        calculate_all_hashes_with_config(black_box(&file_path), &config)
                            .await
                            .expect("Hash calculation failed")
                    })
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_all_hashes,
    bench_individual_algorithms,
    bench_buffer_sizes
);
criterion_main!(benches);
