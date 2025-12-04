use criterion::{Criterion, criterion_group, criterion_main};
use std::time::Duration;
use tokio::runtime::Runtime;

mod common {
    include!("../tests/common/mod.rs");
}

fn benchmark_debian_table_examples(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create runtime");
    let mut group = c.benchmark_group("table");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("debian_example", |b| {
        b.iter(|| {
            rt.block_on(async {
                match common::table::run_debian_example().await {
                    Ok(_) => {}
                    Err(e) => panic!("Benchmark failed: {}", e),
                }
            })
        });
    });

    group.bench_function("old_bsd_example", |b| {
        b.iter(|| {
            rt.block_on(async {
                match common::table::run_old_bsd_example().await {
                    Ok(_) => {}
                    Err(e) => panic!("Benchmark failed: {}", e),
                }
            })
        });
    });

    group.bench_function("debian_archive_thafficmanager_net", |b| {
        b.iter(|| {
            rt.block_on(async {
                match common::traffic_manager::run_debian_archive_trafficmanager_net().await {
                    Ok(_) => {}
                    Err(e) => panic!("Benchmark failed: {}", e),
                }
            })
        });
    });
}

fn benchmark_debian_pre_examples(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create runtime");
    let mut group = c.benchmark_group("pre");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("bsd_example", |b| {
        b.iter(|| {
            rt.block_on(async {
                match common::pre::run_bsd_example().await {
                    Ok(_) => {}
                    Err(e) => panic!("Benchmark failed: {}", e),
                }
            })
        });
    });

    group.bench_function("pre_img_example", |b| {
        b.iter(|| {
            rt.block_on(async {
                match common::pre::run_pre_img_example().await {
                    Ok(_) => {}
                    Err(e) => panic!("Benchmark failed: {}", e),
                }
            })
        });
    });
}

fn benchmark_debian_h5ai_example(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create runtime");
    let mut group = c.benchmark_group("h5ai");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("debian_h5ai", |b| {
        b.iter(|| {
            rt.block_on(async {
                match common::h5ai::run_debian_h5ai().await {
                    Ok(_) => {}
                    Err(e) => panic!("Benchmark failed: {}", e),
                }
            })
        });
    });
}

fn benchmark_self_miniserve(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create runtime");
    let mut group = c.benchmark_group("miniserve");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("self_miniserve", |b| {
        b.iter(|| {
            rt.block_on(async {
                match common::miniserve::run_self_miniserve().await {
                    Ok(_) => {}
                    Err(e) => panic!("Benchmark failed: {}", e),
                }
            })
        });
    });
}

fn benchmark_debian_snt(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create runtime");
    let mut group = c.benchmark_group("snt");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("debian_snt", |b| {
        b.iter(|| {
            rt.block_on(async {
                match common::snt::run_debian_snt().await {
                    Ok(_) => {}
                    Err(e) => panic!("Benchmark failed: {}", e),
                }
            })
        });
    });
}

fn benchmark_debian_ul(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create runtime");
    let mut group = c.benchmark_group("ul");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("debian_ul", |b| {
        b.iter(|| {
            rt.block_on(async {
                match common::ul::run_debian_ul().await {
                    Ok(_) => {}
                    Err(e) => panic!("Benchmark failed: {}", e),
                }
            })
        });
    });
}

criterion_group!(
    benches,
    benchmark_debian_table_examples,
    benchmark_debian_pre_examples,
    benchmark_debian_h5ai_example,
    benchmark_self_miniserve,
    benchmark_debian_snt,
    benchmark_debian_ul,
);
criterion_main!(benches);
