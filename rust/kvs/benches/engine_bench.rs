#[macro_use]
extern crate criterion;

use criterion::{BatchSize, Criterion, ParameterizedBenchmark, Bencher};
use kvs::{KvStore, KvsEngine, SledStore};
use rand::prelude::*;
use sled::Db;
use std::iter;
use tempfile::TempDir;

fn set_bench(c: &mut Criterion) {
    let bench = ParameterizedBenchmark::new(
        "kvs",
        |b: &mut Bencher, _| {
            b.iter_batched(
                || {
                    let temp_dir = TempDir::new().unwrap();
                    (KvStore::open(temp_dir.path()).unwrap(), temp_dir)
                },
                |(mut store, _temp_dir)| {
                    for i in 1..(1 << 12) {
                        store.set(format!("key{}", i), "value".to_string()).unwrap();
                    }
                },
                BatchSize::NumBatches(10),
            )
        },
        iter::once(()),
    )
        .with_function("sled", |b: &mut Bencher, _| {
            b.iter_batched(
                || {
                    let temp_dir = TempDir::new().unwrap();
                    (SledStore::new(Db::start_default(&temp_dir).unwrap()), temp_dir)
                },
                |(mut db, _temp_dir)| {
                    for i in 1..(1 << 12) {
                        db.set(format!("key{}", i), "value".to_string()).unwrap();
                    }
                },
                BatchSize::NumBatches(10),
            )
        });
    c.bench("set_bench", bench);
}

fn get_bench(c: &mut Criterion) {
    let bench = ParameterizedBenchmark::new(
        "kvs",
        |b: &mut Bencher, i: &i32| {
            let temp_dir = TempDir::new().unwrap();
            let mut store = KvStore::open(temp_dir.path()).unwrap();
            for key_i in 1..(1 << i) {
                store
                    .set(format!("key{}", key_i), "value".to_string())
                    .unwrap();
            }
            let mut rng = SmallRng::from_seed([0; 16]);
            b.iter(|| {
                store
                    .get(format!("key{}", rng.gen_range(1, 1 << i)))
                    .unwrap();
            })
        },
        vec![8, 12, 16, 20],
    )
        .with_function("sled", |b: &mut Bencher, i: &i32| {
            let temp_dir = TempDir::new().unwrap();
            let mut db = SledStore::new(Db::start_default(&temp_dir).unwrap());
            for key_i in 1..(1 << i) {
                db.set(format!("key{}", key_i), "value".to_string())
                    .unwrap();
            }
            let mut rng = SmallRng::from_seed([0; 16]);
            b.iter(|| {
                db.get(format!("key{}", rng.gen_range(1, 1 << i))).unwrap();
            })
        });
    c.bench("get_bench", bench);
}

criterion_group!(benches, set_bench, get_bench);
criterion_main!(benches);