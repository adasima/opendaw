//! オーディオエフェクトモジュール
//!
//! 各トラックに適用可能なオーディオエフェクトの抽象化と実装を提供します。

pub mod filter;
pub mod gain;

/// 全てのオーディオエフェクトが実装すべきトレイト
pub trait AudioEffect: Send + Sync {
    /// エフェクトの処理を行います。
    ///
    /// `buffer` はインターリーブされたオーディオサンプル（f32）です。
    /// `channels` はチャンネル数（1=モノラル, 2=ステレオ 等）です。
    fn process(&mut self, buffer: &mut [f32], channels: u16);

    /// エフェクトの名前を返します。
    fn name(&self) -> &str;

    /// エフェクトが有効かどうかを返します。
    fn is_enabled(&self) -> bool;

    /// エフェクトの有効/無効を切り替えます。
    fn set_enabled(&mut self, enabled: bool);
}
pub mod delay;
