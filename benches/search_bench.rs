use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn search_bench(c: &mut Criterion) {
    c.bench_function("search_smoke", |b| {
        b.iter(|| {
            // TODO: replace with real search benchmark once the search API is defined
            black_box(42usize);
        })
    });
}

criterion_group!(benches, search_bench);
criterion_main!(benches);