use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn piano_roll_hit_test_benchmark(c: &mut Criterion) {
    c.bench_function("piano_roll_hit_test", |b| {
        b.iter(|| {
            // ベンチマークロジックをここに追加
            black_box(1 + 1)
        });
    });
}

criterion_group!(benches, piano_roll_hit_test_benchmark);
criterion_main!(benches);
