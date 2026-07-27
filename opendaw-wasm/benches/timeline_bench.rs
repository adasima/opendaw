use criterion::{Criterion, criterion_group, criterion_main};

// We will mock the necessary structs to benchmark the loop

#[derive(Clone)]
struct Clip {
    id: u32,
    start_pos: f32,
}

#[derive(Clone)]
struct Track {
    id: u32,
    clips: Vec<Clip>,
}

fn bench_timeline_update(c: &mut Criterion) {
    let num_tracks = 100;
    let clips_per_track = 100;

    let mut tracks = vec![];
    for t in 0..num_tracks {
        let mut clips = vec![];
        for c in 0..clips_per_track {
            clips.push(Clip {
                id: c,
                start_pos: 0.0,
            });
        }
        tracks.push(Track { id: t, clips });
    }

    let mut modified_clips_id = vec![];
    let mut modified_clips_idx = vec![];

    // Select every other clip to be modified
    for t_idx in 0..num_tracks {
        for c_idx in 0..clips_per_track {
            if c_idx % 2 == 0 {
                let t_id = tracks[t_idx as usize].id;
                let c_id = tracks[t_idx as usize].clips[c_idx as usize].id;
                modified_clips_id.push((t_id, c_id, 1.0f32));
                modified_clips_idx.push((t_idx as usize, c_idx as usize, 1.0f32));
            }
        }
    }

    c.bench_function("timeline_update_by_id", |b| {
        b.iter(|| {
            let mut test_tracks = tracks.clone();
            for (t_id, clip_id, new_pos) in &modified_clips_id {
                if let Some(track) = test_tracks.iter_mut().find(|t| t.id == *t_id) {
                    if let Some(clip) = track.clips.iter_mut().find(|c| c.id == *clip_id) {
                        clip.start_pos = new_pos.max(0.0);
                    }
                }
            }
            std::hint::black_box(test_tracks);
        })
    });

    c.bench_function("timeline_update_by_idx", |b| {
        b.iter(|| {
            let mut test_tracks = tracks.clone();
            for (t_idx, clip_idx, new_pos) in &modified_clips_idx {
                if let Some(track) = test_tracks.get_mut(*t_idx) {
                    if let Some(clip) = track.clips.get_mut(*clip_idx) {
                        clip.start_pos = new_pos.max(0.0);
                    }
                }
            }
            std::hint::black_box(test_tracks);
        })
    });
}

criterion_group!(benches, bench_timeline_update);
criterion_main!(benches);
