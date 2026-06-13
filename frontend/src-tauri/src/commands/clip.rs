use log::info;

use tauri::State;

use crate::app::AppState;

/// オーディオクリップを追加する
#[tauri::command]
pub fn add_audio_clip(
    track_id: usize,
    name: String,
    start_pos: f32,
    length: f32,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    info!("Project: Add audio clip '{}' to track {}", name, track_id);
    let mut project_state = state
        .engine
        .project_state
        .write()
        .unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();
    if let Some(track_arc) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        let track = std::sync::Arc::make_mut(track_arc);
        let new_id = track.clips.iter().map(|c| c.id).max().unwrap_or(0) + 1;
        let clip = crate::state::clip::AudioClip {
            id: new_id,
            name,
            start_pos,
            length,
            waveform_summary: Vec::new(),
        };
        track.clips.push(clip);
        state
            .engine
            .history
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .save_snapshot(project_state_snapshot);
        Ok(new_id)
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// オーディオクリップを削除する
#[tauri::command]
pub fn remove_audio_clip(
    track_id: usize,
    clip_id: usize,
    state: State<'_, AppState>,
) -> Result<(), String> {
    info!(
        "Project: Remove audio clip {} from track {}",
        clip_id, track_id
    );
    let mut project_state = state
        .engine
        .project_state
        .write()
        .unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();
    if let Some(track_arc) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        let track = std::sync::Arc::make_mut(track_arc);
        track.clips.retain(|c| c.id != clip_id);
        state
            .engine
            .history
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .save_snapshot(project_state_snapshot);
        Ok(())
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// オーディオクリップを移動する
#[tauri::command]
pub fn move_audio_clip(
    track_id: usize,
    clip_id: usize,
    new_start_pos: f32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    info!(
        "Project: Move audio clip {} in track {} to {}",
        clip_id, track_id, new_start_pos
    );
    let mut project_state = state
        .engine
        .project_state
        .write()
        .unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();
    if let Some(track_arc) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        let track = std::sync::Arc::make_mut(track_arc);
        if let Some(clip) = track.clips.iter_mut().find(|c| c.id == clip_id) {
            clip.start_pos = new_start_pos;
            state
                .engine
                .history
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .save_snapshot(project_state_snapshot);
            Ok(())
        } else {
            Err(format!("Clip {} not found in track {}", clip_id, track_id))
        }
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// MIDIクリップを追加する
#[tauri::command]
pub fn add_midi_clip(
    track_id: usize,
    name: String,
    start_beat: f64,
    length_beats: f64,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    info!("Project: Add midi clip '{}' to track {}", name, track_id);
    let mut project_state = state
        .engine
        .project_state
        .write()
        .unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();
    if let Some(track_arc) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        let track = std::sync::Arc::make_mut(track_arc);
        let new_id = track.midi_clips.iter().map(|c| c.id).max().unwrap_or(0) + 1;
        let clip = crate::state::clip::MidiClip {
            id: new_id,
            name,
            start_beat,
            length_beats,
            sequence: crate::midi::sequence::Sequence::new(),
        };
        track.midi_clips.push(clip);
        state
            .engine
            .history
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .save_snapshot(project_state_snapshot);
        Ok(new_id)
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// MIDIクリップを削除する
#[tauri::command]
pub fn remove_midi_clip(
    track_id: usize,
    clip_id: usize,
    state: State<'_, AppState>,
) -> Result<(), String> {
    info!(
        "Project: Remove midi clip {} from track {}",
        clip_id, track_id
    );
    let mut project_state = state
        .engine
        .project_state
        .write()
        .unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();
    if let Some(track_arc) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        let track = std::sync::Arc::make_mut(track_arc);
        track.midi_clips.retain(|c| c.id != clip_id);
        state
            .engine
            .history
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .save_snapshot(project_state_snapshot);
        Ok(())
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// MIDIクリップを移動する
#[tauri::command]
pub fn move_midi_clip(
    track_id: usize,
    clip_id: usize,
    new_start_beat: f64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    info!(
        "Project: Move midi clip {} in track {} to {}",
        clip_id, track_id, new_start_beat
    );
    let mut project_state = state
        .engine
        .project_state
        .write()
        .unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();
    if let Some(track_arc) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        let track = std::sync::Arc::make_mut(track_arc);
        if let Some(clip) = track.midi_clips.iter_mut().find(|c| c.id == clip_id) {
            clip.start_beat = new_start_beat;
            state
                .engine
                .history
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .save_snapshot(project_state_snapshot);
            Ok(())
        } else {
            Err(format!("Clip {} not found in track {}", clip_id, track_id))
        }
    } else {
        Err(format!("Track {} not found", track_id))
    }
}

/// MIDIクリップのノート情報を更新する
#[tauri::command]
pub fn update_midi_clip_notes(
    track_id: usize,
    clip_id: usize,
    notes: Vec<crate::midi::sequence::NoteEvent>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    info!(
        "Project: Update midi clip notes for clip {} in track {}",
        clip_id, track_id
    );
    let mut project_state = state
        .engine
        .project_state
        .write()
        .unwrap_or_else(|e| e.into_inner());
    let project_state_snapshot = project_state.clone();
    if let Some(track_arc) = project_state.tracks.iter_mut().find(|t| t.id == track_id) {
        let track = std::sync::Arc::make_mut(track_arc);
        if let Some(clip) = track.midi_clips.iter_mut().find(|c| c.id == clip_id) {
            clip.sequence.notes = notes;

            // Sequence の next_note_id は private フィールドなので直接変更できない。
            // しかし Sequence 構造体に `clear()` して `add_note_event()` で入れ直すか、
            // もしくは `notes` フィールドは pub なので、Sequence自体を新しいものに置き換える。

            // 新しい Sequence を作り、notes を設定し、
            // 次のIDが適切になるようにするため、add_note_eventを使う
            let mut new_seq = crate::midi::sequence::Sequence::new();
            for note in &clip.sequence.notes {
                new_seq.add_note_event(note.clone());
            }
            clip.sequence = new_seq;

            state
                .engine
                .history
                .write()
                .unwrap_or_else(|e| e.into_inner())
                .save_snapshot(project_state_snapshot);
            Ok(())
        } else {
            Err(format!("Clip {} not found in track {}", clip_id, track_id))
        }
    } else {
        Err(format!("Track {} not found", track_id))
    }
}
