//! MIDIファイルインポート機能
//!
//! `.mid` ファイルをパースし、DAWの内部構造（`Track`, `Sequence`, `MidiClip` 等）に変換する。
//! 外部クレート `midly` を使用してSMF (Standard MIDI File) を読み込む。

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};

use crate::midi::sequence::Sequence;
use crate::state::clip::MidiClip;
use crate::state::track::Track;

/// MIDIファイルから抽出された単一トラックのデータ
#[derive(Debug, Clone)]
pub struct ImportedMidiData {
    pub name: String,
    pub sequence: Sequence,
}

/// 指定されたパスのMIDIファイルをパースし、DAW内部構造の元となるデータを抽出する
pub fn read_midi_file<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<ImportedMidiData>, Box<dyn std::error::Error>> {
    let data = fs::read(path)?;
    parse_midi_data(&data)
}

/// メモリ上のMIDIデータをパースし、シーケンスのリストを返す
pub fn parse_midi_data(data: &[u8]) -> Result<Vec<ImportedMidiData>, Box<dyn std::error::Error>> {
    let smf = Smf::parse(data)?;

    let ticks_per_beat = match smf.header.timing {
        Timing::Metrical(ticks) => f64::from(ticks.as_int()),
        Timing::Timecode(_, _) => {
            // SMPTEタイムコードの場合はフォールバックとして一般的な480を使用
            480.0
        }
    };

    let mut imported_tracks = Vec::new();

    for (track_idx, track_events) in smf.tracks.iter().enumerate() {
        let mut sequence = Sequence::new();
        let mut track_name = format!("Track {}", track_idx + 1);
        let mut current_ticks = 0u64;

        // ノートオン状態を記録するためのバッファ
        // (pitch, channel) -> (start_beat, velocity)
        let mut active_notes: HashMap<(u8, u8), (f64, u8)> = HashMap::new();

        for event in track_events {
            current_ticks += event.delta.as_int() as u64;
            let current_beat = current_ticks as f64 / ticks_per_beat;

            match &event.kind {
                #[allow(clippy::collapsible_match)]
                #[allow(clippy::collapsible_if)]
                TrackEventKind::Meta(meta) => {
                    if let MetaMessage::TrackName(name_bytes) = meta {
                        if let Ok(name) = String::from_utf8(name_bytes.to_vec()) {
                            track_name = name;
                        }
                    }
                }
                TrackEventKind::Midi { channel, message } => {
                    match message {
                        MidiMessage::NoteOn { key, vel } => {
                            let pitch = key.as_int();
                            let velocity = vel.as_int();
                            let ch = channel.as_int();

                            if velocity > 0 {
                                // ノートオン
                                active_notes.insert((pitch, ch), (current_beat, velocity));
                            } else {
                                // ベロシティ0のノートオンはノートオフとして扱う
                                if let Some((start_beat, note_vel)) =
                                    active_notes.remove(&(pitch, ch))
                                {
                                    let duration = current_beat - start_beat;
                                    if duration > 0.0 {
                                        sequence.add_note(pitch, note_vel, start_beat, duration);
                                    }
                                }
                            }
                        }
                        MidiMessage::NoteOff { key, .. } => {
                            let pitch = key.as_int();
                            let ch = channel.as_int();

                            if let Some((start_beat, velocity)) = active_notes.remove(&(pitch, ch))
                            {
                                let duration = current_beat - start_beat;
                                if duration > 0.0 {
                                    sequence.add_note(pitch, velocity, start_beat, duration);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // 終了していないノートがあった場合、デフォルト長さを与えて追加する
        for ((pitch, _ch), (start_beat, velocity)) in active_notes {
            sequence.add_note(pitch, velocity, start_beat, 1.0);
        }

        // ノートが存在する、またはトラック名が明示的に設定されている場合のみ追加
        if !sequence.notes.is_empty() || track_name != format!("Track {}", track_idx + 1) {
            imported_tracks.push(ImportedMidiData {
                name: track_name,
                sequence,
            });
        }
    }

    Ok(imported_tracks)
}

/// MIDIファイルを読み込み、DAWの内部構造である `Track` と `MidiClip` のペアに変換する
///
/// `start_id`: 生成されるTrackとClipに割り当てる一意のIDの開始番号
pub fn import_midi_as_tracks<P: AsRef<Path>>(
    path: P,
    start_id: usize,
) -> Result<Vec<(Track, MidiClip)>, Box<dyn std::error::Error>> {
    let imported_data = read_midi_file(path)?;
    let mut results = Vec::new();

    for (i, data) in imported_data.into_iter().enumerate() {
        let id = start_id + i;

        // トラックの生成
        let mut track = Track::new(id, data.name.clone());
        // MIDIインポートされたトラックは、プレビュー用にシンセを有効にしておく
        track.toggle_synth();

        // クリップの長さを計算 (最後のノートの終端)
        let length_beats = data
            .sequence
            .notes
            .iter()
            .map(|n| n.start_beat + n.duration_beats)
            .fold(0.0_f64, f64::max);
        // クリップの最低長さを1拍とする
        let length_beats = if length_beats > 0.0 { length_beats } else { 1.0 };

        // MidiClipの生成
        let mut clip = MidiClip::new(id, format!("{} Clip", data.name), 0.0, length_beats);
        clip.sequence = data.sequence;

        results.push((track, clip));
    }

    Ok(results)
}
