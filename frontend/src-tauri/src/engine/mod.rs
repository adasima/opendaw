use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use crossbeam_channel::{bounded, Sender, Receiver};
use ringbuf::{HeapRb, SharedRb, traits::{Split, Consumer}};
use std::sync::Mutex;
use ringbuf::storage::Heap;

pub type StateProd = ringbuf::wrap::caching::Caching<Arc<SharedRb<Heap<EngineStateUpdate>>>, true, false>;
pub type StateCons = ringbuf::wrap::caching::Caching<Arc<SharedRb<Heap<EngineStateUpdate>>>, false, true>;

/// イベントの種類 (メインスレッド -> オーディオスレッド)
pub enum EngineEvent {
    Play,
    Pause,
    Stop,
    SetBpm(f64),
    SetMasterVolume(f64),
}

/// オーディオスレッドからメインスレッドへの状態同期メッセージ
pub enum EngineStateUpdate {
    PlaybackPos(f64),
    CpuLoad(f32),
}

/// エンジンの状態を管理・制御するためのハンドル
#[derive(Clone)]
pub struct EngineHandle {
    is_playing: Arc<AtomicBool>,
    bpm: Arc<AtomicU32>,
    master_volume: Arc<AtomicU32>,
    event_tx: Sender<EngineEvent>,
    state_rx: Arc<Mutex<StateCons>>,
}

impl Default for EngineHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineHandle {
    /// 新しいEngineHandleとイベントレシーバー、状態プロデューサーを作成する
    pub fn create_channel() -> (Self, Receiver<EngineEvent>, StateProd) {
        let (tx, rx) = bounded(1024);
        let rb = HeapRb::<EngineStateUpdate>::new(1024);
        let (prod, cons) = rb.split();
        let handle = Self {
            is_playing: Arc::new(AtomicBool::new(false)),
            bpm: Arc::new(AtomicU32::new(12000)), // 120.0 BPM = 12000
            master_volume: Arc::new(AtomicU32::new(800)), // 0.8 = 800
            event_tx: tx,
            state_rx: Arc::new(Mutex::new(cons)),
        };
        (handle, rx, prod)
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

    /// オーディオスレッドからの状態更新を取得する (ノンブロッキング)
    pub fn poll_state_updates(&self) -> Vec<EngineStateUpdate> {
        let mut updates = Vec::new();
        if let Ok(mut cons) = self.state_rx.try_lock() {
            while let Some(update) = cons.try_pop() {
                updates.push(update);
            }
        }
        updates
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ringbuf::traits::Producer;

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

    #[test]
    fn test_state_updates() {
        let (handle, _rx, mut prod) = EngineHandle::create_channel();
        let _ = prod.try_push(EngineStateUpdate::PlaybackPos(1.5));
        let updates = handle.poll_state_updates();
        assert_eq!(updates.len(), 1);
        if let EngineStateUpdate::PlaybackPos(pos) = updates[0] {
            assert_eq!(pos, 1.5);
        } else {
            panic!("Unexpected update type");
        }
    }
}
