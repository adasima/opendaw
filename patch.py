import sys

with open("opendaw-wasm/src/ui/timeline.rs", "r") as f:
    content = f.read()

# Change all_modified_clips
content = content.replace("for clip in &track.clips {", "for (clip_idx, clip) in track.clips.iter().enumerate() {")
content = content.replace("all_modified_clips.push((track.id, clip.id, new_pos));", "all_modified_clips.push((i, clip_idx, new_pos));")

# Change all_modified_midi_clips
content = content.replace("for clip in &track.midi_clips {", "for (clip_idx, clip) in track.midi_clips.iter().enumerate() {")
content = content.replace("all_modified_midi_clips.push((track.id, clip.id, new_pos));", "all_modified_midi_clips.push((i, clip_idx, new_pos));")

# Change all_modified_auto_points
content = content.replace("for point in &auto_track.points {", "for (point_idx, point) in auto_track.points.iter().enumerate() {")
content = content.replace("""                        all_modified_auto_points.push((
                            track.id,
                            auto_track_idx,
                            point.id,
                            new_time as f64,
                            new_val,
                        ));""", """                        all_modified_auto_points.push((
                            i,
                            auto_track_idx,
                            point_idx,
                            new_time as f64,
                            new_val,
                        ));""")

# Change the loops at the end
content = content.replace("""    for (t_id, auto_track_idx, point_id, new_time, new_val) in all_modified_auto_points {
        if let Some(track) = app.state.tracks.iter_mut().find(|t| t.id == t_id)
            && let Some(auto_track) = track.automations.get_mut(auto_track_idx)
        {
            if let Some(point) = auto_track.points.iter_mut().find(|p| p.id == point_id) {
                point.time = new_time;
                point.value = new_val;
            }
            auto_track
                .points
                .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        }
    }""", """    for (t_idx, auto_track_idx, point_idx, new_time, new_val) in all_modified_auto_points {
        if let Some(track) = app.state.tracks.get_mut(t_idx)
            && let Some(auto_track) = track.automations.get_mut(auto_track_idx)
        {
            if let Some(point) = auto_track.points.get_mut(point_idx) {
                point.time = new_time;
                point.value = new_val;
            }
            auto_track
                .points
                .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        }
    }""")

content = content.replace("""    for (t_id, clip_id, new_pos) in all_modified_clips {
        #[allow(clippy::collapsible_if)]
        if let Some(track) = app.state.tracks.iter_mut().find(|t| t.id == t_id) {
            if let Some(clip) = track.clips.iter_mut().find(|c| c.id == clip_id) {
                clip.start_pos = new_pos.max(0.0);
            }
        }
    }""", """    for (t_idx, clip_idx, new_pos) in all_modified_clips {
        #[allow(clippy::collapsible_if)]
        if let Some(track) = app.state.tracks.get_mut(t_idx) {
            if let Some(clip) = track.clips.get_mut(clip_idx) {
                clip.start_pos = new_pos.max(0.0);
            }
        }
    }""")

content = content.replace("""    for (t_id, clip_id, new_pos) in all_modified_midi_clips {
        #[allow(clippy::collapsible_if)]
        if let Some(track) = app.state.tracks.iter_mut().find(|t| t.id == t_id) {
            if let Some(clip) = track.midi_clips.iter_mut().find(|c| c.id == clip_id) {
                clip.start_beat = new_pos.max(0.0);
            }
        }
    }""", """    for (t_idx, clip_idx, new_pos) in all_modified_midi_clips {
        #[allow(clippy::collapsible_if)]
        if let Some(track) = app.state.tracks.get_mut(t_idx) {
            if let Some(clip) = track.midi_clips.get_mut(clip_idx) {
                clip.start_beat = new_pos.max(0.0);
            }
        }
    }""")

with open("opendaw-wasm/src/ui/timeline.rs", "w") as f:
    f.write(content)
