//! オーディオバス処理モジュール
//!
//! 録音されたクリップ等の複数ソースをミックスし、
//! 指定されたチャンネルへルーティングするマスターバス等の処理を管理します。

use std::sync::Arc;

/// バス上で管理するクリップの最大数
pub const MAX_CLIPS: usize = 16;

/// マスターバス
///
/// 録音されたオーディオクリップなどを管理し、オーディオコールバックごとに
/// ミキシングバッファへの合算を行います。
pub struct MasterBus {
    pub recorded_clips: [Option<(usize, Arc<Vec<f32>>)>; MAX_CLIPS],
    pub next_clip_idx: usize,
}

impl Default for MasterBus {
    fn default() -> Self {
        Self::new()
    }
}

impl MasterBus {
    /// 新しい MasterBus を作成します。
    pub fn new() -> Self {
        Self {
            recorded_clips: Default::default(),
            next_clip_idx: 0,
        }
    }

    /// 新しいクリップをバスに追加します。
    pub fn add_clip(&mut self, start_pos: usize, data: Arc<Vec<f32>>) {
        if self.next_clip_idx < MAX_CLIPS {
            self.recorded_clips[self.next_clip_idx] = Some((start_pos, data));
            self.next_clip_idx += 1;
        }
    }

    /// 現在の再生位置とバッファサイズに応じて、管理しているクリップ群を
    /// `mix_slice` バッファに加算します。
    ///
    /// * `mix_slice` - ミキシング対象のバッファスライス
    /// * `current_pos` - 現在の再生位置（サンプル単位）
    /// * `samples_to_read` - 今回処理するサンプルフレーム数
    /// * `channels` - チャンネル数（例: 2 for ステレオ）
    pub fn mix_clips(
        &self,
        mix_slice: &mut [f32],
        current_pos: usize,
        samples_to_read: usize,
        channels: u16,
    ) {
        for (start_pos, clip) in self.recorded_clips.iter().flatten() {
            let clip_len = clip.len() / channels as usize;
            if current_pos + samples_to_read > *start_pos && current_pos < *start_pos + clip_len {
                let play_offset = current_pos.saturating_sub(*start_pos);
                let buf_offset = (*start_pos).saturating_sub(current_pos);
                let to_read = (samples_to_read - buf_offset).min(clip_len - play_offset);
                for i in 0..to_read {
                    for ch in 0..channels as usize {
                        let src_idx = (play_offset + i) * channels as usize + ch;
                        let dst_idx = (buf_offset + i) * channels as usize + ch;
                        if src_idx < clip.len() && dst_idx < mix_slice.len() {
                            mix_slice[dst_idx] += clip[src_idx];
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_master_bus_add_clip() {
        let mut bus = MasterBus::new();
        let clip_data = Arc::new(vec![0.5, -0.5]);

        bus.add_clip(10, clip_data.clone());

        assert_eq!(bus.next_clip_idx, 1);
        if let Some((pos, data)) = &bus.recorded_clips[0] {
            assert_eq!(*pos, 10);
            assert_eq!(data.len(), 2);
        } else {
            panic!("Clip not added properly");
        }
    }

    #[test]
    fn test_master_bus_mix_clips() {
        let mut bus = MasterBus::new();
        // 4 frames of stereo (8 samples)
        let clip_data = Arc::new(vec![0.1, 0.1, 0.2, 0.2, 0.3, 0.3, 0.4, 0.4]);
        bus.add_clip(2, clip_data); // starts at frame 2

        let mut mix_slice = vec![0.0; 8]; // 4 frames stereo

        // request from frame 1, length 4 frames
        bus.mix_clips(&mut mix_slice, 1, 4, 2);

        // frame 1: mix_slice[0, 1] = 0.0, 0.0 (clip hasn't started)
        // frame 2: mix_slice[2, 3] += clip[0, 1] (0.1, 0.1)
        // frame 3: mix_slice[4, 5] += clip[2, 3] (0.2, 0.2)
        // frame 4: mix_slice[6, 7] += clip[4, 5] (0.3, 0.3)
        assert_eq!(mix_slice, vec![0.0, 0.0, 0.1, 0.1, 0.2, 0.2, 0.3, 0.3]);
    }
}
