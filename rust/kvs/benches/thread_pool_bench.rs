#[macro_use]
extern crate criterion;

use criterion::{criterion_group, criterion_main, Bencher, Benchmark, Criterion, ParameterizedBenchmark};
use crossbeam::sync::WaitGroup;
use tempfile::TempDir;
use kvs::{ThreadPool, KvStore, KvsEngine, KvServer, SharedQueueThreadPool};
use std::net::SocketAddr;
use std::str::FromStr;
use std::process::Command;
use assert_cmd::cargo::CommandCargoExt;
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use rand::random;
use std::thread::sleep;

fn bench_thread_pool_with_size_heavy_set(c: &mut Criterion) {
    let bench = ParameterizedBenchmark::new(
        "shared queue pool with heavy set job",
        |b: &mut Bencher, size: &u32| {
            let (sender, receiver) = mpsc::sync_channel::<()>(0);
            let temp_dir = TempDir::new().unwrap();
            let num_thread = (*size).to_string();
            let stderr_path = temp_dir.path().join("stderr");
            let mut cmd = Command::cargo_bin("kvs-server").unwrap();
            let mut child = cmd
                .args(&["--engine", "kvs", "--addr", "127.0.0.1:4001", "--thread_pool", "sharedqueue", "--num_thread", num_thread.as_str()])
                .current_dir(&temp_dir)
                .stderr(File::create(&stderr_path).unwrap())
                .spawn()
                .unwrap();
            thread::sleep(Duration::from_secs(1));
            thread::spawn(move || {
                let _ = receiver.recv();
                child.kill().expect("Error when close server")
            });

            b.iter(|| {
                let wg = WaitGroup::new();
                for _ in 0..20 {
                    let wg = wg.clone();
                    thread::spawn(move || {
                        let temp_dir = TempDir::new().unwrap();
                        let random_key: String = random::<u32>().to_string();
                        let random_val: String = random::<u32>().to_string();
                        Command::cargo_bin("kvs-client")
                            .unwrap()
                            .args(&["set", random_key.as_str(), random_val.as_str(), "--addr", "127.0.0.1:4001"])
                            .current_dir(&temp_dir);
                        drop(wg);
                    });
                }
                wg.wait();
            });
            sender.send(()); //close
        },
        vec![1, 2, 4, 8],
    )
        .sample_size(50);
    c.bench("shared queue with write-heavy job", bench);
}

fn bench_thread_pool_with_size_heavy_get(c: &mut Criterion) {
    let bench = ParameterizedBenchmark::new(
        "shared queue pool with heavy get job",
        |b: &mut Bencher, size: &u32| {
            let (sender, receiver) = mpsc::sync_channel::<()>(0);
            let temp_dir = TempDir::new().unwrap();
            let num_thread = (*size).to_string();
            let stderr_path = temp_dir.path().join("stderr");
            let mut cmd = Command::cargo_bin("kvs-server").unwrap();
            let mut child = cmd
                .args(&["--engine", "kvs", "--addr", "127.0.0.1:4001", "--thread_pool", "sharedqueue", "--num_thread", num_thread.as_str()])
                .current_dir(&temp_dir)
                .stderr(File::create(&stderr_path).unwrap())
                .spawn()
                .unwrap();
            thread::sleep(Duration::from_secs(1));
            Command::cargo_bin("kvs-client")
                .unwrap()
                .args(&["set", "key1", "value1", "--addr", "127.0.0.1:4001"])
                .current_dir(&temp_dir);
            thread::spawn(move || {
                let _ = receiver.recv();
                child.kill().expect("Error when close server")
            });

            b.iter(|| {
                let wg = WaitGroup::new();
                for _ in 0..20 {
                    let wg = wg.clone();
                    thread::spawn(|| {
                        let temp_dir = TempDir::new().unwrap();
                        Command::cargo_bin("kvs-client")
                            .unwrap()
                            .args(&["get", "key1", "--addr", "127.0.0.1:4001"])
                            .current_dir(&temp_dir);
                    });
                    drop(wg);
                }
                wg.wait();
            });
            sender.send(()); //close
        },
        vec![1, 2, 4, 8],
    )
        .sample_size(30);
    c.bench("shared queue with read-heavy job", bench);
}



criterion_group!(benches, bench_thread_pool_with_size_heavy_set, bench_thread_pool_with_size_heavy_get);
criterion_main!(benches);