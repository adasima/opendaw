use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/core/math.rs"]
mod math;

fn bench_pitch_to_freq(c: &mut Criterion) {
    c.bench_function("pitch_to_freq (cached)", |b| {
        b.iter(|| {
            for pitch in 0..128u8 {
                black_box(math::pitch_to_freq(black_box(pitch)));
            }
        })
    });

    c.bench_function("pitch_to_freq (powf fallback)", |b| {
        b.iter(|| {
            for pitch in 0..128u8 {
                black_box(440.0 * 2.0_f32.powf((black_box(pitch) as f32 - 69.0) / 12.0));
            }
        })
    });
}

criterion_group!(benches, bench_pitch_to_freq);
criterion_main!(benches);
