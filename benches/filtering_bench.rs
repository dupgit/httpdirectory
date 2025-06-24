extern crate httpdirectory;
use core::time::Duration;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use httpdirectory::entry::Entry;
use httpdirectory::httpdirectoryentry::HttpDirectoryEntry;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct HttpDirectory {
    entries: Vec<HttpDirectoryEntry>,
    url: Arc<String>,
    request: Arc<String>, // Simplified for the benchmark
}

impl HttpDirectory {
    // Naive version cloning everything and then retain elements
    pub fn filtering_naive<F>(&self, mut f: F) -> Self
    where
        F: FnMut(&HttpDirectoryEntry) -> bool,
    {
        let mut entries = self.entries.clone();
        entries.retain(|elem| f(elem));

        HttpDirectory {
            entries,
            url: self.url.clone(),
            request: self.request.clone(),
        }
    }

    // Filtering first and then clone only the entries we filtered
    pub fn filtering_v1<F>(&self, mut f: F) -> Self
    where
        F: FnMut(&HttpDirectoryEntry) -> bool,
    {
        let entries = self.entries.iter().filter(|entry| f(entry)).cloned().collect();

        Self {
            entries,
            url: Arc::clone(&self.url),
            request: Arc::clone(&self.request),
        }
    }

    // Using filter_map with and cloning only the wanted entries
    pub fn filtering_v3<F>(&self, mut f: F) -> Self
    where
        F: FnMut(&HttpDirectoryEntry) -> bool,
    {
        let entries = self
            .entries
            .iter()
            .filter_map(|entry| {
                if f(entry) {
                    Some(entry.clone())
                } else {
                    None
                }
            })
            .collect();

        Self {
            entries,
            url: Arc::clone(&self.url),
            request: Arc::clone(&self.request),
        }
    }

    // Using a pre allocated vector and manually pushing cloned entries
    // then shrinking the vector to it's size
    pub fn filtering_v4<F>(&self, mut f: F) -> Self
    where
        F: FnMut(&HttpDirectoryEntry) -> bool,
    {
        let mut entries = Vec::with_capacity(self.entries.len());
        for entry in &self.entries {
            if f(entry) {
                entries.push(entry.clone());
            }
        }
        entries.shrink_to_fit();

        Self {
            entries,
            url: Arc::clone(&self.url),
            request: Arc::clone(&self.request),
        }
    }
}

// Utility function to create entries
fn create_test_directory(size: usize) -> HttpDirectory {
    let mut entries = Vec::with_capacity(size);

    // One parent directory
    entries.push(HttpDirectoryEntry::ParentDirectory("../".to_string()));

    // Generating files and directories (2/3, 1/3)
    for i in 0..size - 1 {
        if i % 3 == 0 {
            let entry = Entry::new(&format!("item_{}", i), &format!("/item_{}", i), "2024-01-01 10:10", "-");
            entries.push(HttpDirectoryEntry::Directory(entry));
        } else {
            let entry = Entry::new(
                &format!("item_{}", i),
                &format!("/item_{}", i),
                "2024-01-01 10:10",
                &format!("{}", i * 1024),
            );
            entries.push(HttpDirectoryEntry::File(entry));
        }
    }

    HttpDirectory {
        entries,
        url: Arc::new("http://example.com/directory".to_string()),
        request: Arc::new("Reqwest client simulation".to_string()),
    }
}

// Utility function to filter files only
fn files_only(entry: &HttpDirectoryEntry) -> bool {
    matches!(entry, HttpDirectoryEntry::File(_))
}

// Utility function to filter files whose apparent size is bigger than 5 Kb
fn files_only_whose_size_is_greater_than_5120(entry: &HttpDirectoryEntry) -> bool {
    match entry {
        HttpDirectoryEntry::File(e) => e.apparent_size() > 5_120,
        _ => false,
    }
}

// Trying different filter methods to see which one is
// better adapted to our use case (we assume that most
// directories will contain less than a thousand files)
fn bench_filtering_methods(c: &mut Criterion) {
    let sizes = vec![10, 50, 100, 500, 1000, 5000, 10000];

    for size in sizes {
        let directory = create_test_directory(size);

        let mut group = c.benchmark_group(format!("filtering_size_{}", size));
        if size >= 5000 {
            group.measurement_time(Duration::new(10, 0));
        }

        group.bench_function("naive", |b| b.iter(|| black_box(directory.filtering_naive(files_only))));

        group.bench_function("filter_cloned", |b| b.iter(|| black_box(directory.filtering_v1(files_only))));

        group.bench_function("filter_map", |b| b.iter(|| black_box(directory.filtering_v3(files_only))));

        group.bench_function("with_capacity", |b| b.iter(|| black_box(directory.filtering_v4(files_only))));

        group.finish();
    }
}

// Trying to see the impact of the filter (with 1000 the number of
// filtered files are similar)
fn bench_filtering_with_different_filters(c: &mut Criterion) {
    let directory = create_test_directory(1000);

    let mut group = c.benchmark_group("filtering_with_different_filters");

    // Filtering files only
    group.bench_function("files_only_cloned", |b| b.iter(|| black_box(directory.filtering_v1(files_only))));
    group.bench_function("files_only_capacity", |b| b.iter(|| black_box(directory.filtering_v4(files_only))));

    // Filtering files whose size is greater than 5 Kb
    group.bench_function("files_size_greater_than_5120_cloned", |b| {
        b.iter(|| black_box(directory.filtering_v1(files_only_whose_size_is_greater_than_5120)))
    });
    group.bench_function("files_size_greater_than_5120_capacity", |b| {
        b.iter(|| black_box(directory.filtering_v4(files_only_whose_size_is_greater_than_5120)))
    });

    group.finish();
}

// Trying to see the impact of the number of filtered elements
fn bench_selectivity(c: &mut Criterion) {
    let directory = create_test_directory(100);

    let mut group = c.benchmark_group("selectivity");

    // High selectivity: keeping few entries (~10% with 100 entries)
    group.bench_function("high_selectivity_cloned", |b| {
        b.iter(|| {
            black_box(directory.filtering_v1(|entry| match entry {
                HttpDirectoryEntry::File(e) => e.apparent_size() > 900_000,
                _ => false,
            }))
        })
    });
    group.bench_function("high_selectivity_capacity", |b| {
        b.iter(|| {
            black_box(directory.filtering_v4(|entry| match entry {
                HttpDirectoryEntry::File(e) => e.apparent_size() > 900_000,
                _ => false,
            }))
        })
    });

    // Low selectivity: keeping a huge number of entries (~95% for 100 entries)
    group.bench_function("low_selectivity_cloned", |b| {
        b.iter(|| {
            black_box(directory.filtering_v1(|entry| match entry {
                HttpDirectoryEntry::File(e) => e.apparent_size() > 5_000,
                _ => false,
            }))
        })
    });
    group.bench_function("low_selectivity_capacity", |b| {
        b.iter(|| {
            black_box(directory.filtering_v4(|entry| match entry {
                HttpDirectoryEntry::File(e) => e.apparent_size() > 1000, // Beaucoup d'éléments
                _ => false,
            }))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_filtering_methods, bench_filtering_with_different_filters, bench_selectivity);
criterion_main!(benches);
