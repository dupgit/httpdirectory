use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;

mod common {
    include!("../tests/common/mod.rs");
}

fn benchmark_debian_example(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create runtime");

    c.bench_function("debian_example", |b| {
        b.iter(|| {
            rt.block_on(async {
                match common::run_debian_example().await {
                    Ok(_) => {}
                    Err(e) => panic!("Benchmark failed: {}", e),
                }
            })
        });
    });
}

criterion_group!(benches, benchmark_debian_example);
criterion_main!(benches);
