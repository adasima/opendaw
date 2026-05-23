use std::path::PathBuf;

/// トラックのフリーズ状態を管理する構造体
/// 重い処理（エフェクトやシンセなど）をバイパスするために、
/// レンダリング済みのオーディオデータへのパスやフリーズ状態を保持します。
#[derive(Debug, Clone, Default)]
pub struct FreezeState {
    /// トラックが現在フリーズされているかどうか
    pub is_frozen: bool,
    
    /// キャッシュされたWAVデータ（レンダリング結果）のファイルパス
    pub cached_wav_path: Option<PathBuf>,
    
    /// フリーズ作成時のトラックのバージョンやハッシュ。
    /// トラックの状態が変更された際にフリーズを無効化・再レンダリングするために使用します。
    pub source_version: u64,
}

impl FreezeState {
    pub fn new() -> Self {
        Self::default()
    }

    /// トラックをフリーズ状態にし、レンダリング済みファイルのパスを記録します。
    pub fn freeze(&mut self, cache_path: PathBuf, version: u64) {
        self.is_frozen = true;
        self.cached_wav_path = Some(cache_path);
        self.source_version = version;
    }

    /// フリーズを解除します。キャッシュされたファイルは再利用のために保持される場合があります。
    pub fn unfreeze(&mut self) {
        self.is_frozen = false;
    }
    
    /// キャッシュをクリアし、フリーズ状態も解除します。
    pub fn clear_cache(&mut self) {
        self.cached_wav_path = None;
        self.is_frozen = false;
    }

    /// トラックがフリーズされており、かつ有効なキャッシュファイルを持っているか確認します。
    pub fn is_active(&self) -> bool {
        self.is_frozen && self.cached_wav_path.is_some()
    }
}
