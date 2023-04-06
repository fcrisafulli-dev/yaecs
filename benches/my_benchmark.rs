use criterion::{black_box, criterion_group, criterion_main, Criterion};
use yaecs::benchmark_4wide_query;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Query and modify 4 components, 50 entities", |b| b.iter(|| benchmark_4wide_query()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);