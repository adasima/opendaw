use crossbeam_channel::{bounded, Receiver, Sender};
use ringbuf::storage::Heap;
use ringbuf::wrap::caching::Caching;
use ringbuf::SharedRb;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

pub mod midi_route;
use crate::state::ProjectState;

pub type MidiEventProducer = Caching<Arc<SharedRb<Heap<midi_route::MidiEvent>>>, true, false>;
pub type MidiEventConsumer = Caching<Arc<SharedRb<Heap<midi_route::MidiEvent>>>, false, true>;

pub type MidiRouteProducer = crossbeam_channel::Sender<midi_route::TrackMidiRoute>;
pub type MidiRouteConsumer = crossbeam_channel::Receiver<midi_route::TrackMidiRoute>;

/// イベントの種類 (メインスレッド -> オーディオスレッド)
pub enum EngineEvent {
    Play,
    Pause,
    Stop,
    SetBpm(f64),
    SetMasterVolume(f64),
}

/// エンジンの状態を管理・制御するためのハンドル
#[derive(Clone)]
pub struct EngineHandle {
    is_playing: Arc<AtomicBool>,
    bpm: Arc<AtomicU32>,
    master_volume: Arc<AtomicU32>,
    // The main thread needs to be able to send routing updates to the audio thread
    midi_route_tx: MidiRouteProducer,
    event_tx: Sender<EngineEvent>,
    pub project_state: Arc<std::sync::RwLock<ProjectState>>,
    pub history: Arc<std::sync::RwLock<crate::state::history::HistoryManager>>,
}

impl Default for EngineHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineHandle {
    /// 新しいEngineHandleとイベントレシーバーを作成する
    pub fn create_channel() -> (Self, Receiver<EngineEvent>, MidiRouteConsumer) {
        let (tx, rx) = bounded(1024);
        let (route_tx, route_rx) = bounded(1024);

        let handle = Self {
            is_playing: Arc::new(AtomicBool::new(false)),
            bpm: Arc::new(AtomicU32::new(12000)), // 120.0 BPM = 12000
            master_volume: Arc::new(AtomicU32::new(800)), // 0.8 = 800
            midi_route_tx: route_tx,
            event_tx: tx,
            project_state: Arc::new(std::sync::RwLock::new(ProjectState::default())),
            history: Arc::new(std::sync::RwLock::new(
                crate::state::history::HistoryManager::new(),
            )),
        };
        (handle, rx, route_rx)
    }

    /// 新しいEngineHandleを作成する (テスト用)
    pub fn new() -> Self {
        Self::create_channel().0
    }

    /// 再生を開始する
    pub fn play(&self) {
        self.is_playing.store(true, Ordering::Release);
        let _ = self.event_tx.try_send(EngineEvent::Play);
    }

    /// 再生を一時停止する
    pub fn pause(&self) {
        self.is_playing.store(false, Ordering::Release);
        let _ = self.event_tx.try_send(EngineEvent::Pause);
    }

    /// 再生を停止する
    pub fn stop(&self) {
        self.is_playing.store(false, Ordering::Release);
        let _ = self.event_tx.try_send(EngineEvent::Stop);
    }

    /// BPMを設定する
    pub fn set_bpm(&self, bpm: f64) {
        let bpm_fixed = (bpm * 100.0) as u32;
        self.bpm.store(bpm_fixed, Ordering::Release);
        let _ = self.event_tx.try_send(EngineEvent::SetBpm(bpm));
    }

    /// 現在再生中かどうかを取得する
    pub fn is_playing(&self) -> bool {
        self.is_playing.load(Ordering::Acquire)
    }

    /// 現在のBPMを取得する
    pub fn get_bpm(&self) -> f64 {
        self.bpm.load(Ordering::Acquire) as f64 / 100.0
    }

    /// マスターボリュームを設定する (0.0 - 1.0)
    pub fn set_master_volume(&self, volume: f64) {
        let vol_fixed = (volume * 1000.0) as u32;
        self.master_volume.store(vol_fixed, Ordering::Release);
        let _ = self.event_tx.try_send(EngineEvent::SetMasterVolume(volume));
    }

    /// 現在のマスターボリュームを取得する
    pub fn get_master_volume(&self) -> f64 {
        self.master_volume.load(Ordering::Acquire) as f64 / 1000.0
    }

    /// トラックに対するMIDIデバイスとチャンネルのルーティングを設定する
    pub fn set_track_midi_route(&self, track_id: u32, device: String, channel: u8) {
        let _ = self
            .midi_route_tx
            .try_send(midi_route::TrackMidiRoute::new(track_id, device, channel));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_handle() {
        let handle = EngineHandle::new();
        assert!(!handle.is_playing());
        assert_eq!(handle.get_bpm(), 120.0);

        handle.play();
        assert!(handle.is_playing());

        handle.pause();
        assert!(!handle.is_playing());

        handle.set_bpm(140.5);
        assert_eq!(handle.get_bpm(), 140.5);
    }
}
