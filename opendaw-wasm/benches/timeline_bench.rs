use criterion::{black_box, criterion_group, criterion_main, Criterion};

// We will benchmark a mock of the O(N^2) update logic
fn mock_timeline_update_baseline(points_count: usize, modified_count: usize) {
    struct Point { id: usize, time: f64, value: f32 }

    let mut points: Vec<Point> = (0..points_count).map(|i| Point { id: i, time: i as f64, value: 0.0 }).collect();

    let modified: Vec<(usize, f64, f32)> = (0..modified_count)
        .map(|i| (i, (i as f64) + 0.1, 0.5))
        .collect();

    for (point_id, new_time, new_val) in modified {
        if let Some(point) = points.iter_mut().find(|p| p.id == point_id) {
            point.time = new_time;
            point.value = new_val;
        }
        points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    }
}

fn mock_timeline_update_optimized(points_count: usize, modified_count: usize) {
    struct Point { id: usize, time: f64, value: f32 }

    let mut points: Vec<Point> = (0..points_count).map(|i| Point { id: i, time: i as f64, value: 0.0 }).collect();

    let modified: Vec<(usize, f64, f32)> = (0..modified_count)
        .map(|i| (i, (i as f64) + 0.1, 0.5)) // using indices now
        .collect();

    let mut modified_points = Vec::new();
    for (point_idx, new_time, new_val) in modified {
        if let Some(point) = points.get_mut(point_idx) {
            point.time = new_time;
            point.value = new_val;
        }
        modified_points.push(point_idx);
    }
    points.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("timeline_update_baseline", |b| b.iter(|| mock_timeline_update_baseline(black_box(1000), black_box(100))));
    c.bench_function("timeline_update_optimized", |b| b.iter(|| mock_timeline_update_optimized(black_box(1000), black_box(100))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
