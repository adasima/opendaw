//! プロジェクト状態の同期を行うモジュール

use crate::state::DawState;

/// 外部からプロジェクトのJSONを受け取り、状態を同期する
#[allow(clippy::collapsible_if)]
pub fn sync_project_state_json(state: &mut DawState, is_dragging_clip: bool, json_str: &str) {
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(json_str) {
        // is_playing
        if let Some(is_playing) = parsed.get("is_playing").and_then(|v| v.as_bool()) {
            state.is_playing = is_playing;
        }
        // bpm
        if let Some(bpm) = parsed.get("bpm").and_then(|v| v.as_f64()) {
            state.bpm = bpm as f32;
        }
        // master_volume
        if let Some(vol) = parsed.get("master_volume").and_then(|v| v.as_f64()) {
            state.master_volume = vol as f32;
        }
        // grid_settings
        if let Some(grid_settings) = parsed.get("grid_settings") {
            if let Some(is_enabled) = grid_settings.get("is_enabled").and_then(|v| v.as_bool()) {
                state.is_grid_enabled = is_enabled;
            }
            if let Some(resolution) = grid_settings.get("resolution").and_then(|v| v.as_u64()) {
                state.grid_resolution = resolution as u32;
            }
        }
        // tracks
        if let Some(tracks_array) = parsed.get("tracks").and_then(|v| v.as_array()) {
            for track_val in tracks_array {
                if let Some(id) = track_val.get("id").and_then(|v| v.as_u64()) {
                    let id = id as usize;
                    let mut found = false;
                    for track in state.tracks.iter_mut() {
                        if track.id == id {
                            found = true;
                            if let Some(name) = track_val.get("name").and_then(|v| v.as_str()) {
                                track.name = name.to_string();
                            }
                            if let Some(vol) = track_val.get("volume").and_then(|v| v.as_f64()) {
                                track.volume = vol as f32;
                            }
                            if let Some(pan) = track_val.get("pan").and_then(|v| v.as_f64()) {
                                track.pan = pan as f32;
                            }
                            if let Some(muted) = track_val.get("is_muted").and_then(|v| v.as_bool())
                            {
                                track.is_muted = muted;
                            }
                            if let Some(solo) = track_val.get("is_solo").and_then(|v| v.as_bool()) {
                                track.is_solo = solo;
                            }
                            if let Some(armed) =
                                track_val.get("is_record_armed").and_then(|v| v.as_bool())
                            {
                                track.is_record_armed = armed;
                            }
                            // clips
                            if let Some(clips_array) =
                                track_val.get("clips").and_then(|v| v.as_array())
                            {
                                for clip_val in clips_array {
                                    if let Ok(parsed_clip) =
                                        serde_json::from_value::<crate::state::clip::AudioClip>(
                                            clip_val.clone(),
                                        )
                                    {
                                        if let Some(existing_clip) =
                                            track.clips.iter_mut().find(|c| c.id == parsed_clip.id)
                                        {
                                            existing_clip.name = parsed_clip.name;
                                            if !is_dragging_clip {
                                                existing_clip.start_pos = parsed_clip.start_pos;
                                            }
                                            existing_clip.length = parsed_clip.length;
                                        } else {
                                            track.clips.push(parsed_clip);
                                        }
                                    }
                                }
                                let backend_clip_ids: Vec<usize> = clips_array
                                    .iter()
                                    .filter_map(|c| {
                                        c.get("id").and_then(|v| v.as_u64()).map(|id| id as usize)
                                    })
                                    .collect();
                                track.clips.retain(|c| backend_clip_ids.contains(&c.id));
                            }
                            // automations
                            if let Some(automations_array) =
                                track_val.get("automations").and_then(|v| v.as_array())
                            {
                                let mut new_automations = Vec::new();
                                for auto_val in automations_array {
                                    if let Ok(parsed_auto) = serde_json::from_value::<
                                        crate::state::track::AutomationTrack,
                                    >(
                                        auto_val.clone()
                                    ) {
                                        new_automations.push(parsed_auto);
                                    }
                                }
                                track.automations = new_automations;
                            }
                            if let Some(visible) = track_val
                                .get("automation_visible")
                                .and_then(|v| v.as_bool())
                            {
                                track.automation_visible = visible;
                            }
                            if let Some(selected) = track_val.get("selected_automation") {
                                if selected.is_null() {
                                    track.selected_automation = None;
                                } else if let Some(s) = selected.as_str() {
                                    track.selected_automation = Some(s.to_string());
                                }
                            }

                            // midi_clips
                            if let Some(midi_clips_array) =
                                track_val.get("midi_clips").and_then(|v| v.as_array())
                            {
                                for clip_val in midi_clips_array {
                                    if let Ok(parsed_clip) =
                                        serde_json::from_value::<crate::state::clip::MidiClip>(
                                            clip_val.clone(),
                                        )
                                    {
                                        if let Some(existing_clip) = track
                                            .midi_clips
                                            .iter_mut()
                                            .find(|c| c.id == parsed_clip.id)
                                        {
                                            existing_clip.name = parsed_clip.name;
                                            if !is_dragging_clip {
                                                existing_clip.start_beat = parsed_clip.start_beat;
                                            }
                                            existing_clip.length_beats = parsed_clip.length_beats;
                                        } else {
                                            track.midi_clips.push(parsed_clip);
                                        }
                                    }
                                }
                                let backend_midi_ids: Vec<usize> = midi_clips_array
                                    .iter()
                                    .filter_map(|c| {
                                        c.get("id").and_then(|v| v.as_u64()).map(|id| id as usize)
                                    })
                                    .collect();
                                track
                                    .midi_clips
                                    .retain(|c| backend_midi_ids.contains(&c.id));
                            }
                            break;
                        }
                    }
                    if !found {
                        if let Ok(new_track) =
                            serde_json::from_value::<crate::state::track::Track>(track_val.clone())
                        {
                            state.tracks.push(new_track);
                        }
                    }
                }
            }

            // Remove tracks that are no longer in the backend
            let backend_ids: Vec<usize> = tracks_array
                .iter()
                .filter_map(|t| t.get("id").and_then(|v| v.as_u64()).map(|id| id as usize))
                .collect();
            state.tracks.retain(|t| backend_ids.contains(&t.id));
        }
    }
}
