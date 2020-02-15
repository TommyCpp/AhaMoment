use criterion::{criterion_group, criterion_main, Bencher, Benchmark, Criterion, ParameterizedBenchmark};
use tempfile::TempDir;
use kvs::{ThreadPool, KvStore, KvsEngine, KvServer, SharedQueueThreadPool};
use std::net::SocketAddr;
use std::str::FromStr;

fn bench_thread_pool_with_size(c: &mut Criterion) {
    let bench = ParameterizedBenchmark::new(
        "shared queue pool with heavy set job",
        |b: &mut Bencher, size: &u32| {
            let temp_dir = TempDir::new().unwrap();
            let mut store = KvStore::open(temp_dir.path()).unwrap();
            let thread_pool = SharedQueueThreadPool::new(*size).unwrap();
            let kv_server = KvServer::new(SocketAddr::from_str("127.0.0.1:6060").unwrap(), store, thread_pool).unwrap();
            //todo: finished the benchmark after refactor the kvs_server binary with options
        },
        vec![1, 4, 5, 6],
    );
    c.bench("shared queue with write-heavy job", bench);
}