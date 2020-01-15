use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tempfile::TempDir;
use kvs::KvStore;

fn insert_benchmark(c: &mut Criterion) {
    c.bench_function("insert 1000 times", |b| b.iter(|| {
        let temp_dir = TempDir::new().expect("unable to create temporary working directory");
        let mut store = KvStore::open(temp_dir.path()).ok().unwrap();
        for i in 1..1000 {
            store.set(format!("key{}", i), String::from("String"));
        }
    }));
}

fn get_benchmark(c: &mut Criterion){
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut store = KvStore::open(temp_dir.path()).ok().unwrap();
    for i in 1..1000 {
        store.set(format!("key{}", i), String::from("String"));
    }

    c.bench_function("get 1000 times", |b| b.iter(||{
        for i in 1..1000{
            store.get(format!("key{}", i));
        }
    }));
}

criterion_group!(benches, insert_benchmark, get_benchmark);
criterion_main!(benches);