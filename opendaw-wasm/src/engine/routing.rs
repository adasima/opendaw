//! オーディオルーティングモジュール
//!
//! パンやボリュームの設定に基づいて、オーディオ信号を
//! 複数チャンネル（ステレオ等）に分配するための計算を行います。

/// ボリュームとパンの値から、ステレオ（左右）のゲインを計算します。
///
/// # 引数
/// * `volume` - トラックのボリューム（0.0〜）
/// * `pan` - トラックのパン（-1.0〜1.0）
///
/// # 戻り値
/// `(left_gain, right_gain)` のタプル。
pub fn calculate_stereo_gains(volume: f32, pan: f32) -> (f32, f32) {
    // パンの適用 (Constant Power Panning ではなく、シンプルなリニアパンニングを仮実装)
    // -1.0(左) 〜 1.0(右) の範囲を 0.0 〜 1.0 に正規化
    let p = (pan + 1.0) / 2.0;
    let left_gain = (1.0 - p) * volume;
    let right_gain = p * volume;

    (left_gain, right_gain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_stereo_gains_center() {
        let (left, right) = calculate_stereo_gains(1.0, 0.0);
        assert_eq!(left, 0.5);
        assert_eq!(right, 0.5);
    }

    #[test]
    fn test_calculate_stereo_gains_hard_left() {
        let (left, right) = calculate_stereo_gains(1.0, -1.0);
        assert_eq!(left, 1.0);
        assert_eq!(right, 0.0);
    }

    #[test]
    fn test_calculate_stereo_gains_hard_right() {
        let (left, right) = calculate_stereo_gains(1.0, 1.0);
        assert_eq!(left, 0.0);
        assert_eq!(right, 1.0);
    }

    #[test]
    fn test_calculate_stereo_gains_with_volume() {
        let (left, right) = calculate_stereo_gains(0.5, 0.0);
        assert_eq!(left, 0.25);
        assert_eq!(right, 0.25);
    }
}
