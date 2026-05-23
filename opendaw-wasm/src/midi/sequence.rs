//! MIDIシーケンスデータ構造モジュール
//!
//! ピアノロールなどで編集されるMIDIノートの集合を管理する。

use serde::{Deserialize, Serialize};

/// MIDIノートイベント
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteEvent {
    /// 一意の識別子
    pub id: usize,
    /// MIDIノート番号 (0-127)
    pub pitch: u8,
    /// ベロシティ (0-127)
    pub velocity: u8,
    /// 開始位置（拍単位）
    pub start_beat: f64,
    /// 長さ（拍単位）
    pub duration_beats: f64,
}

impl NoteEvent {
    /// 新しいNoteEventを作成する
    pub fn new(id: usize, pitch: u8, velocity: u8, start_beat: f64, duration_beats: f64) -> Self {
        Self {
            id,
            pitch,
            velocity,
            start_beat,
            duration_beats,
        }
    }
}

/// MIDIシーケンス
///
/// 複数のMIDIノートを管理するデータ構造。
/// IDの衝突を防ぐため、追加時に自動でインクリメントされるIDを付与する。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Sequence {
    /// ノートのリスト
    pub notes: Vec<NoteEvent>,
    /// 次に割り当てるID
    next_note_id: usize,
}

impl Sequence {
    /// 新しい空のSequenceを作成する
    pub fn new() -> Self {
        Self::default()
    }

    /// ノートを追加し、そのIDを返す
    pub fn add_note(
        &mut self,
        pitch: u8,
        velocity: u8,
        start_beat: f64,
        duration_beats: f64,
    ) -> usize {
        let id = self.next_note_id;
        self.next_note_id += 1;

        let note = NoteEvent::new(id, pitch, velocity, start_beat, duration_beats);
        self.notes.push(note);

        id
    }

    /// 指定されたIDのノートを削除する
    ///
    /// 削除に成功した場合は `true` を返す。
    pub fn remove_note(&mut self, id: usize) -> bool {
        let initial_len = self.notes.len();
        self.notes.retain(|n| n.id != id);
        self.notes.len() < initial_len
    }

    /// すべてのノートを削除する
    pub fn clear(&mut self) {
        self.notes.clear();
    }

    /// ノートを追加する (ID付き)
    /// 既存のテスト等を壊さないためのエイリアス
    pub fn add_note_event(&mut self, note: NoteEvent) {
        if note.id >= self.next_note_id {
            self.next_note_id = note.id + 1;
        }
        self.notes.push(note);
    }

    /// 指定されたIDのノートへのミュータブル参照を取得する
    pub fn get_note_mut(&mut self, id: usize) -> Option<&mut NoteEvent> {
        self.notes.iter_mut().find(|n| n.id == id)
    }

    /// 指定されたIDのノートへの参照を取得する
    pub fn get_note(&self, id: usize) -> Option<&NoteEvent> {
        self.notes.iter().find(|n| n.id == id)
    }

    /// 指定されたIDのノートの位置（ピッチと開始位置）を変更する
    ///
    /// 変更に成功した場合は `true` を返す。
    pub fn move_note(&mut self, id: usize, pitch: u8, start_beat: f64) -> bool {
        if let Some(note) = self.get_note_mut(id) {
            note.pitch = pitch;
            note.start_beat = start_beat;
            true
        } else {
            false
        }
    }

    /// 指定されたIDのノートの長さ（duration_beats）を変更する
    ///
    /// 変更に成功した場合は `true` を返す。
    pub fn resize_note(&mut self, id: usize, duration_beats: f64) -> bool {
        if let Some(note) = self.get_note_mut(id) {
            note.duration_beats = duration_beats;
            true
        } else {
            false
        }
    }

    /// 指定されたIDのノートのベロシティを変更する
    ///
    /// 変更に成功した場合は `true` を返す。
    pub fn update_velocity(&mut self, id: usize, velocity: u8) -> bool {
        if let Some(note) = self.get_note_mut(id) {
            note.velocity = velocity;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_event_creation() {
        let note = NoteEvent::new(1, 60, 100, 1.0, 0.5);
        assert_eq!(note.id, 1);
        assert_eq!(note.pitch, 60);
        assert_eq!(note.velocity, 100);
        assert_eq!(note.start_beat, 1.0);
        assert_eq!(note.duration_beats, 0.5);
    }

    #[test]
    fn test_sequence_add_note() {
        let mut seq = Sequence::new();

        let id1 = seq.add_note(60, 100, 0.0, 1.0);
        assert_eq!(id1, 0);
        assert_eq!(seq.notes.len(), 1);

        let id2 = seq.add_note(62, 100, 1.0, 1.0);
        assert_eq!(id2, 1);
        assert_eq!(seq.notes.len(), 2);
    }

    #[test]
    fn test_sequence_remove_note() {
        let mut seq = Sequence::new();
        let id1 = seq.add_note(60, 100, 0.0, 1.0);
        let id2 = seq.add_note(62, 100, 1.0, 1.0);

        assert!(seq.remove_note(id1));
        assert_eq!(seq.notes.len(), 1);
        assert_eq!(seq.notes[0].id, id2);

        // 存在しないIDの削除
        assert!(!seq.remove_note(999));
        assert_eq!(seq.notes.len(), 1);
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = Sequence::new();
        seq.add_note(60, 100, 0.0, 1.0);
        seq.add_note(62, 100, 1.0, 1.0);

        seq.clear();
        assert_eq!(seq.notes.len(), 0);

        // clear後にaddした場合、IDはリセットされずインクリメントされ続けるのが望ましい
        let id3 = seq.add_note(64, 100, 2.0, 1.0);
        assert_eq!(id3, 2);
    }

    #[test]
    fn test_sequence_get_note() -> Result<(), Box<dyn std::error::Error>> {
        let mut seq = Sequence::new();
        let id1 = seq.add_note(60, 100, 0.0, 1.0);

        {
            let note = seq.get_note_mut(id1).ok_or("Note not found")?;
            note.pitch = 61;
        }

        let note = seq.get_note(id1).ok_or("Note not found")?;
        assert_eq!(note.pitch, 61);

        assert!(seq.get_note(999).is_none());
        Ok(())
    }

    #[test]
    fn test_sequence_move_note() -> Result<(), Box<dyn std::error::Error>> {
        let mut seq = Sequence::new();
        let id = seq.add_note(60, 100, 0.0, 1.0);

        assert!(seq.move_note(id, 62, 2.0));
        let note = seq.get_note(id).ok_or("Note not found")?;
        assert_eq!(note.pitch, 62);
        assert_eq!(note.start_beat, 2.0);

        assert!(!seq.move_note(999, 62, 2.0));
        Ok(())
    }

    #[test]
    fn test_sequence_resize_note() -> Result<(), Box<dyn std::error::Error>> {
        let mut seq = Sequence::new();
        let id = seq.add_note(60, 100, 0.0, 1.0);

        assert!(seq.resize_note(id, 2.5));
        let note = seq.get_note(id).ok_or("Note not found")?;
        assert_eq!(note.duration_beats, 2.5);

        assert!(!seq.resize_note(999, 2.5));
        Ok(())
    }

    #[test]
    fn test_sequence_update_velocity() -> Result<(), Box<dyn std::error::Error>> {
        let mut seq = Sequence::new();
        let id = seq.add_note(60, 100, 0.0, 1.0);

        assert!(seq.update_velocity(id, 127));
        let note = seq.get_note(id).ok_or("Note not found")?;
        assert_eq!(note.velocity, 127);

        assert!(!seq.update_velocity(999, 127));
        Ok(())
    }
}
