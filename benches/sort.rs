use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use neato::sort::*;
use rand::{distributions::Uniform, prelude::*};

// https://stackoverflow.com/questions/73813699/rust-criterion-how-to-pass-mutable-reference-within-bench-with-input
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting");

    for size in [10, 100, 1000].iter() {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 1000);

        group.bench_function(BenchmarkId::new("bubble sort {}", size), |b| {
            b.iter_batched_ref(
                || -> Vec<usize> { (0..*size).map(|_| rng.sample(&range)).collect() },
                |v| bubble(v),
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("insertion sort {}", size), |b| {
            b.iter_batched_ref(
                || -> Vec<usize> { (0..*size).map(|_| rng.sample(&range)).collect() },
                |v| insertion(v),
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("selection sort {}", size), |b| {
            b.iter_batched_ref(
                || -> Vec<usize> { (0..*size).map(|_| rng.sample(&range)).collect() },
                |v| selection(v),
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("quick sort {}", size), |b| {
            b.iter_batched_ref(
                || -> Vec<usize> { (0..*size).map(|_| rng.sample(&range)).collect() },
                |v| quick(v),
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("merge sort {}", size), |b| {
            b.iter_batched_ref(
                || -> Vec<usize> { (0..*size).map(|_| rng.sample(&range)).collect() },
                |v| mergesort(v),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
