use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn parser_bench(c: &mut Criterion) {
    c.bench_function("parser_smoke", |b| {
        b.iter(|| {
            // TODO: replace with real parser benchmark once the parser API is defined
            black_box("recur");
        })
    });
}

criterion_group!(benches, parser_bench);
criterion_main!(benches);
