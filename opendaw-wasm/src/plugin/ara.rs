// src/plugin/ara.rs

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// ARA2におけるテンポ情報の構造体
#[derive(Debug, Clone, PartialEq)]
pub struct TempoInfo {
    pub bpm: f64,
    pub time_signature_numerator: u32,
    pub time_signature_denominator: u32,
}

/// ARA2における再生位置・トランスポート情報の構造体
#[derive(Debug, Clone, PartialEq)]
pub struct TransportInfo {
    pub is_playing: bool,
    pub position_seconds: f64,
    pub position_beats: f64,
}

/// ARA2でやり取りされるノート情報の構造体 (SV2等の音声合成向け)
#[derive(Debug, Clone, PartialEq)]
pub struct AraNote {
    pub start_seconds: f64,
    pub duration_seconds: f64,
    pub pitch: u8, // MIDIノートナンバー
    pub velocity: u8,
    pub lyric: Option<String>,
}

/// ARA2ホストが提供する情報へのアクセスを抽象化するトレイト
pub trait AraHostAccess: Send + Sync {
    /// 現在のテンポ情報を取得する
    fn get_tempo_info(&self) -> TempoInfo;
    
    /// 現在のトランスポート状態を取得する
    fn get_transport_info(&self) -> TransportInfo;
    
    /// 指定された時間範囲のノートイベントを取得する
    fn get_notes_in_range(&self, start_sec: f64, end_sec: f64) -> Vec<AraNote>;
}

/// ARA2プラグイン側のインターフェース
pub trait AraPluginExtension: Send + Sync {
    /// ホストからテンポが変更された際のコールバック
    fn on_tempo_changed(&mut self, info: &TempoInfo);
    
    /// ホストからトランスポート状態が変更された際のコールバック
    fn on_transport_changed(&mut self, info: &TransportInfo);
    
    /// ホスト側でノートデータが編集・追加された際のコールバック
    fn on_notes_updated(&mut self, notes: &[AraNote]);
}

/// 音声合成用(SV2等)のARAプラグイン実装の骨格
pub struct VocalSynthAraExtension {
    host: Arc<dyn AraHostAccess>,
    current_tempo: TempoInfo,
    current_transport: TransportInfo,
    cached_notes: HashMap<usize, AraNote>,
    next_note_id: usize,
}

impl VocalSynthAraExtension {
    /// 初期化
    pub fn new(host: Arc<dyn AraHostAccess>) -> Self {
        let initial_tempo = host.get_tempo_info();
        let initial_transport = host.get_transport_info();
        
        Self {
            host,
            current_tempo: initial_tempo,
            current_transport: initial_transport,
            cached_notes: HashMap::new(),
            next_note_id: 0,
        }
    }
    
    /// ホストとの同期状態を確認し、内部状態を更新する
    pub fn sync_with_host(&mut self) {
        let new_tempo = self.host.get_tempo_info();
        if self.current_tempo != new_tempo {
            self.on_tempo_changed(&new_tempo);
        }
        
        let new_transport = self.host.get_transport_info();
        if self.current_transport != new_transport {
            self.on_transport_changed(&new_transport);
        }
        
        // キャッシュ更新例: 現在位置周辺のノートを取得
        let pos = self.current_transport.position_seconds;
        let notes = self.host.get_notes_in_range(pos, pos + 10.0);
        self.on_notes_updated(&notes);
    }
}

impl AraPluginExtension for VocalSynthAraExtension {
    fn on_tempo_changed(&mut self, info: &TempoInfo) {
        // テンポ変更時の再計算など（音声合成エンジンのタイムライン再構築等）
        self.current_tempo = info.clone();
        println!("ARA: Tempo updated - BPM: {}", info.bpm);
    }

    fn on_transport_changed(&mut self, info: &TransportInfo) {
        // 再生位置や状態の変更
        self.current_transport = info.clone();
        println!("ARA: Transport updated - Playing: {}, Pos: {:.2}s", info.is_playing, info.position_seconds);
    }

    fn on_notes_updated(&mut self, notes: &[AraNote]) {
        // 渡されたノートリストを内部キャッシュに反映する
        self.cached_notes.clear();
        for note in notes {
            self.cached_notes.insert(self.next_note_id, note.clone());
            self.next_note_id += 1;
            
            if let Some(lyric) = &note.lyric {
                println!("ARA: Note updated - Pitch: {}, Lyric: {}", note.pitch, lyric);
            } else {
                println!("ARA: Note updated - Pitch: {}", note.pitch);
            }
        }
        // ここで音声合成エンジン(SV2等)にレンダリング指示を出す
    }
}

/// DAWホスト側のARAモック実装。
/// テンポやノート・歌詞データをプラグインと同期するためのインターフェースとして機能します。
pub struct MockDawHost {
    tempo: RwLock<TempoInfo>,
    transport: RwLock<TransportInfo>,
    notes: RwLock<Vec<AraNote>>,
}

impl Default for MockDawHost {
    fn default() -> Self {
        Self::new()
    }
}

impl MockDawHost {
    pub fn new() -> Self {
        Self {
            tempo: RwLock::new(TempoInfo {
                bpm: 120.0,
                time_signature_numerator: 4,
                time_signature_denominator: 4,
            }),
            transport: RwLock::new(TransportInfo {
                is_playing: false,
                position_seconds: 0.0,
                position_beats: 0.0,
            }),
            notes: RwLock::new(Vec::new()),
        }
    }

    /// テンポを更新します。
    pub fn set_tempo(&self, bpm: f64) {
        if let Ok(mut tempo) = self.tempo.write() {
            tempo.bpm = bpm;
        }
    }

    /// トランスポート状態を更新します。
    pub fn set_transport(&self, is_playing: bool, position_seconds: f64) {
        if let Ok(mut t) = self.transport.write() {
            t.is_playing = is_playing;
            t.position_seconds = position_seconds;
            if let Ok(tempo) = self.tempo.read() {
                t.position_beats = position_seconds * (tempo.bpm / 60.0);
            }
        }
    }

    /// ノート（歌詞データ含む）を追加します。
    pub fn add_note(&self, note: AraNote) {
        if let Ok(mut notes) = self.notes.write() {
            notes.push(note);
        }
    }

    /// ノートを全てクリアします。
    pub fn clear_notes(&self) {
        if let Ok(mut notes) = self.notes.write() {
            notes.clear();
        }
    }
}

impl AraHostAccess for MockDawHost {
    fn get_tempo_info(&self) -> TempoInfo {
        self.tempo.read().map(|g| g.clone()).unwrap_or(TempoInfo { bpm: 120.0, time_signature_numerator: 4, time_signature_denominator: 4 })
    }

    fn get_transport_info(&self) -> TransportInfo {
        self.transport.read().map(|g| g.clone()).unwrap_or(TransportInfo { is_playing: false, position_seconds: 0.0, position_beats: 0.0 })
    }

    fn get_notes_in_range(&self, start_sec: f64, end_sec: f64) -> Vec<AraNote> {
        self.notes.read().map(|g| g.iter().filter(|n| n.start_seconds >= start_sec && n.start_seconds < end_sec).cloned().collect()).unwrap_or_default()

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ara_sync() {
        let host = Arc::new(MockDawHost::new());
        let mut plugin = VocalSynthAraExtension::new(host.clone());

        // 初期同期
        plugin.sync_with_host();
        assert_eq!(plugin.current_tempo.bpm, 120.0);

        // ホスト側でテンポ変更
        host.set_tempo(140.0);
        
        // ホスト側でノート追加
        host.add_note(AraNote {
            start_seconds: 1.0,
            duration_seconds: 0.5,
            pitch: 60,
            velocity: 100,
            lyric: Some("あ".to_string()),
        });

        // プラグイン側で再度同期
        plugin.sync_with_host();
        
        assert_eq!(plugin.current_tempo.bpm, 140.0);
        assert_eq!(plugin.cached_notes.len(), 1);
        assert_eq!(plugin.cached_notes.get(&0).and_then(|n| n.lyric.as_deref()), Some("あ"));
    }
}
