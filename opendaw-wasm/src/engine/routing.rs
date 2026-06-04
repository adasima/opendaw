//! オーディオルーティングモジュール
//!
//! パンやボリューム等によるゲイン計算、
//! およびオーディオ信号のルーティングロジックを提供します。

/// パンとボリュームから左右チャンネルのゲインを計算します。
///
/// `pan` は -1.0 (左) 〜 1.0 (右) の範囲、
/// `volume` は 0.0 以上の値を受け取ります。
pub fn calculate_stereo_gains(pan: f32, volume: f32) -> (f32, f32) {
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
    fn test_calculate_stereo_gains() {
        // センター
        let (l, r) = calculate_stereo_gains(0.0, 1.0);
        assert_eq!(l, 0.5);
        assert_eq!(r, 0.5);

        // 左振り切り
        let (l, r) = calculate_stereo_gains(-1.0, 1.0);
        assert_eq!(l, 1.0);
        assert_eq!(r, 0.0);

        // 右振り切り
        let (l, r) = calculate_stereo_gains(1.0, 1.0);
        assert_eq!(l, 0.0);
        assert_eq!(r, 1.0);

        // ボリューム半分
        let (l, r) = calculate_stereo_gains(0.0, 0.5);
        assert_eq!(l, 0.25);
        assert_eq!(r, 0.25);
    }
}
