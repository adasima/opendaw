//! シンセサイザーモジュール
//!
//! ソフトウェアインストゥルメント（シンセサイザー）の基盤機能を提供します。
//! まずは基本的なオシレータ（サイン波）を実装します。
//!
//! リアルタイムオーディオコールバックから呼ばれるため、
//! メモリアロケーションやブロッキング処理を含まないように設計されています。

use std::f32::consts::PI;

/// 基本的なオシレータ
///
/// 現在はサイン波のみを生成します。
#[derive(Clone, Debug)]
pub struct Oscillator {
    /// サンプリングレート
    sample_rate: f32,
    /// 周波数 (Hz)
    frequency: f32,
    /// 現在の位相 (0.0 〜 2.0 * PI)
    phase: f32,
    /// 発音状態 (true の場合は発音する)
    is_active: bool,
}

impl Oscillator {
    /// 新しいオシレータを作成します
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            // デフォルトはA4 (440Hz)
            frequency: 440.0,
            phase: 0.0,
            is_active: false,
        }
    }

    /// サンプリングレートを設定します
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    /// 周波数を設定します
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    /// 発音状態を設定します
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
        if !active {
            // 発音が停止したら位相をリセットする
            self.phase = 0.0;
        }
    }

    /// 発音状態を取得します
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// 次のサンプルを1つ生成し、位相を進めます。
    /// `is_active` が false の場合は `0.0` を返します。
    pub fn next_sample(&mut self) -> f32 {
        if !self.is_active {
            return 0.0;
        }

        let value = self.phase.sin();

        // 位相を進める
        let phase_increment = 2.0 * PI * self.frequency / self.sample_rate;
        self.phase += phase_increment;
        if self.phase >= 2.0 * PI {
            self.phase -= 2.0 * PI;
        }

        value
    }

    /// 指定されたバッファに対して、生成したサンプルを加算します。
    /// ミキサー等から直接呼ばれることを想定しています。
    ///
    /// `buffer` はインターリーブされたステレオまたはモノラルバッファです。
    /// `channels` に応じて出力先のチャンネルに同じ値（モノラル出力）を加算します。
    pub fn process_add(&mut self, buffer: &mut [f32], channels: u16) {
        if !self.is_active || channels == 0 {
            return;
        }

        let frames = buffer.len() / (channels as usize);
        for i in 0..frames {
            let sample = self.next_sample();

            // 各チャンネルに加算
            for c in 0..(channels as usize) {
                buffer[i * (channels as usize) + c] += sample;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oscillator_new() {
        let osc = Oscillator::new(44100.0);
        assert_eq!(osc.sample_rate, 44100.0);
        assert_eq!(osc.frequency, 440.0);
        assert_eq!(osc.phase, 0.0);
        assert!(!osc.is_active);
    }

    #[test]
    fn test_oscillator_inactive() {
        let mut osc = Oscillator::new(44100.0);
        // 初期状態(inactive)では0を返す
        assert_eq!(osc.next_sample(), 0.0);

        let mut buffer = vec![0.0; 10];
        osc.process_add(&mut buffer, 1);
        assert_eq!(buffer, vec![0.0; 10]); // 何も加算されない
    }

    #[test]
    fn test_oscillator_sine_wave() {
        // 分かりやすいようにサンプルレートと周波数を設定
        // サンプルレート4Hz, 周波数1Hz なら、1周期は4サンプル。
        // 位相の増分は 2π * 1 / 4 = π/2 になるはず。
        let mut osc = Oscillator::new(4.0);
        osc.set_frequency(1.0);
        osc.set_active(true);

        // sample 1: sin(0) = 0.0
        let s1 = osc.next_sample();
        assert!(s1.abs() < 1e-6);

        // sample 2: sin(π/2) = 1.0
        let s2 = osc.next_sample();
        assert!((s2 - 1.0).abs() < 1e-6);

        // sample 3: sin(π) = 0.0
        let s3 = osc.next_sample();
        assert!(s3.abs() < 1e-6);

        // sample 4: sin(3π/2) = -1.0
        let s4 = osc.next_sample();
        assert!((s4 - (-1.0)).abs() < 1e-6);

        // sample 5: sin(2π) -> 位相が0に戻るので sin(0) = 0.0
        let s5 = osc.next_sample();
        assert!(s5.abs() < 1e-6);
    }

    #[test]
    fn test_oscillator_process_add() {
        let mut osc = Oscillator::new(4.0);
        osc.set_frequency(1.0);
        osc.set_active(true);

        let mut buffer = vec![1.0, 2.0, 3.0, 4.0];
        // モノラルで追加
        osc.process_add(&mut buffer, 1);

        // 元のバッファに [0.0, 1.0, 0.0, -1.0] が足されるはず
        assert!((buffer[0] - 1.0).abs() < 1e-6);
        assert!((buffer[1] - 3.0).abs() < 1e-6);
        assert!((buffer[2] - 3.0).abs() < 1e-6);
        assert!((buffer[3] - 3.0).abs() < 1e-6);
    }
}
