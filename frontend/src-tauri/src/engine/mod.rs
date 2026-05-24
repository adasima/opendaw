use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

/// エンジンの状態を管理・制御するためのハンドル
#[derive(Clone)]
pub struct EngineHandle {
    is_playing: Arc<AtomicBool>,
    bpm: Arc<AtomicU32>,
    master_volume: Arc<AtomicU32>,
}

impl Default for EngineHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineHandle {
    /// 新しいEngineHandleを作成する
    pub fn new() -> Self {
        Self {
            is_playing: Arc::new(AtomicBool::new(false)),
            bpm: Arc::new(AtomicU32::new(12000)), // 120.0 BPM = 12000
            master_volume: Arc::new(AtomicU32::new(800)), // 0.8 = 800
        }
    }

    /// 再生を開始する
    pub fn play(&self) {
        self.is_playing.store(true, Ordering::Release);
    }

    /// 再生を一時停止する
    pub fn pause(&self) {
        self.is_playing.store(false, Ordering::Release);
    }

    /// 再生を停止する
    pub fn stop(&self) {
        self.is_playing.store(false, Ordering::Release);
    }

    /// BPMを設定する
    pub fn set_bpm(&self, bpm: f64) {
        let bpm_fixed = (bpm * 100.0) as u32;
        self.bpm.store(bpm_fixed, Ordering::Release);
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
    }

    /// 現在のマスターボリュームを取得する
    pub fn get_master_volume(&self) -> f64 {
        self.master_volume.load(Ordering::Acquire) as f64 / 1000.0
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
