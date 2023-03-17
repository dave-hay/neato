use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

fn fib(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

fn fib2(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn fib3(n: u64) -> u64 {
    let mut seen = HashMap::new();
    seen.insert(0, 1);
    seen.insert(1, 1);
    seen.insert(2, 1);

    fn fib(n: u64, seen: &mut HashMap<u64, u64>) -> u64 {
        match seen.get(&n) {
            Some(p) => *p,
            None => {
                let res = fib(n - 1, seen) + fib(n - 2, seen);
                seen.insert(n, res);
                return res;
            }
        }
    }
    fib(n, &mut seen)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib", |b| b.iter(|| fib(black_box(20))));
    c.bench_function("fib", |b| b.iter(|| fib2(black_box(20))));
    c.bench_function("fib", |b| b.iter(|| fib3(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
