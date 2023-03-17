use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use neato::sort::sort::*;
use rand::{distributions::Uniform, prelude::*};

// https://stackoverflow.com/questions/73813699/rust-criterion-how-to-pass-mutable-reference-within-bench-with-input
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting");

    for size in [10, 100, 1000].iter() {
        // let mut v = rand_vec(*size);

        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 100);

        group.bench_function(BenchmarkId::new("bubble sort {}", size), |b| {
            b.iter_batched_ref(
                || -> Vec<usize> { (0..*size).map(|_| rng.sample(&range)).collect() },
                |v| bubble(v),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
