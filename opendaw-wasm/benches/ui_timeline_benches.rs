use criterion::{Criterion, criterion_group, criterion_main};
use opendaw::state::ProjectState;
use opendaw::state::track::{AutomationPoint, AutomationTrack, Track};
use std::hint::black_box;

fn create_mock_project_state(num_tracks: usize, num_auto_points: usize) -> ProjectState {
    let mut state = ProjectState::default();
    for i in 0..num_tracks {
        let mut track = Track::new(i, format!("Track {}", i));

        // Add an automation track
        let mut auto_track = AutomationTrack {
            parameter_name: "Volume".to_string(),
            points: Vec::new(),
        };

        for j in 0..num_auto_points {
            auto_track.points.push(AutomationPoint {
                id: j as usize,
                time: j as f64 * 0.1,
                value: 0.5,
            });
        }

        track.automations.push(auto_track);
        state.daw_state.tracks.push(track);
    }
    state
}

fn bench_timeline_update_auto_points(c: &mut Criterion) {
    let num_tracks = 50;
    let num_auto_points = 100;

    let mut group = c.benchmark_group("timeline");

    group.bench_function("update_auto_points_original", |b| {
        b.iter_with_setup(
            || {
                let state = create_mock_project_state(num_tracks, num_auto_points);
                let mut all_modified_auto_points = Vec::new();

                // Simulate modifying 20 points across various tracks
                for i in 0..20 {
                    let track_idx = i % num_tracks;
                    let track = &state.daw_state.tracks[track_idx];
                    let auto_track_idx = 0;
                    let point_id = (i * 2 % num_auto_points) as usize;
                    all_modified_auto_points.push((track.id, auto_track_idx, point_id, 0.5, 0.5));
                }

                (state, all_modified_auto_points)
            },
            |(mut state, all_modified_auto_points)| {
                for (t_id, auto_track_idx, point_id, new_time, new_val) in all_modified_auto_points
                {
                    if let Some(track) = state.daw_state.tracks.iter_mut().find(|t| t.id == t_id) {
                        if let Some(auto_track) = track.automations.get_mut(auto_track_idx) {
                            if let Some(point) =
                                auto_track.points.iter_mut().find(|p| p.id == point_id)
                            {
                                point.time = new_time;
                                point.value = new_val;
                            }
                            auto_track
                                .points
                                .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
                        }
                    }
                }
                black_box(state)
            },
        )
    });

    group.bench_function("update_auto_points_optimized", |b| {
        b.iter_with_setup(
            || {
                let state = create_mock_project_state(num_tracks, num_auto_points);
                let mut all_modified_auto_points = Vec::new();

                // Simulate modifying 20 points across various tracks
                // We'll store the track index instead of the track ID
                for i in 0..20 {
                    let track_idx = i % num_tracks;
                    let auto_track_idx = 0;
                    let point_id = (i * 2 % num_auto_points) as usize;
                    all_modified_auto_points.push((track_idx, auto_track_idx, point_id, 0.5, 0.5));
                }

                (state, all_modified_auto_points)
            },
            |(mut state, all_modified_auto_points)| {
                for (track_idx, auto_track_idx, point_id, new_time, new_val) in
                    all_modified_auto_points
                {
                    if let Some(track) = state.daw_state.tracks.get_mut(track_idx) {
                        if let Some(auto_track) = track.automations.get_mut(auto_track_idx) {
                            if let Some(point) =
                                auto_track.points.iter_mut().find(|p| p.id == point_id)
                            {
                                point.time = new_time;
                                point.value = new_val;
                            }
                            auto_track
                                .points
                                .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
                        }
                    }
                }
                black_box(state)
            },
        )
    });

    group.finish();
}

criterion_group!(benches, bench_timeline_update_auto_points);
criterion_main!(benches);
