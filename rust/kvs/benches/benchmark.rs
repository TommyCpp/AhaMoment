use criterion::{criterion_group, criterion_main, Benchmark, Criterion};
use kvs::KvStore;
use tempfile::TempDir;
use kvs::KvsEngine;

const RUN_TIMES: i32 = 2000;
const KEY_NUM: i32 = 1000;
const SAMPLE_SIZE: usize = 20;

fn insert_benchmark(c: &mut Criterion) {
    c.bench(
        "edit",
        Benchmark::new(
            format!("insert {} times, and then delete", RUN_TIMES).as_str(),
            |b| {
                b.iter(|| {
                    let temp_dir =
                        TempDir::new().expect("unable to create temporary working directory");
                    let mut store = KvStore::open(temp_dir.path()).ok().unwrap();
                    for i in 1..RUN_TIMES {
                        store.set(
                            format!("key{}", i % KEY_NUM),
                            String::from(String::from_utf8_lossy(&[0x08 as u8; 512])),
                        );
                    }

                    for i in 1..KEY_NUM {
                        store.remove(format!("key{}", i));
                    }
                })
            },
        )
        .sample_size(SAMPLE_SIZE),
    );
}

fn get_benchmark(c: &mut Criterion) {
    //setup
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut store = KvStore::open(temp_dir.path()).ok().unwrap();
    for i in 1..RUN_TIMES {
        store.set(format!("key{}", i % KEY_NUM), String::from("String"));
    }

    c.bench(
        "get",
        Benchmark::new(format!("get {} times", RUN_TIMES).as_str(), move |b| {
            b.iter(|| {
                for i in 1..RUN_TIMES {
                    store.get(format!("key{}", i % KEY_NUM));
                }
            })
        })
        .sample_size(SAMPLE_SIZE),
    );
}

criterion_group!(benches, insert_benchmark, get_benchmark);
criterion_main!(benches);
