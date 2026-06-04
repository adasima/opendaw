use crate::engine::synth::{AdsrParams, Waveform};

/// エフェクトの種類
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EffectType {
    /// ゲインエフェクト
    Gain,
    /// フィルターエフェクト
    Filter,
    /// ディレイエフェクト
    Delay { time_ms: f32, feedback: f32, mix: f32 },
}

/// トラックに適用されるエフェクトの設定
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EffectSetting {
    /// エフェクトの一意なID
    pub id: usize,
    /// エフェクトの種類
    pub effect_type: EffectType,
    /// エフェクトが有効かどうか
    pub is_enabled: bool,
    /// 前回オーディオエンジンに送信した種類（変更検知用）
    #[serde(skip)]
    pub last_sent_type: Option<EffectType>,
}

impl EffectSetting {
    /// 新しいエフェクト設定を作成します
    pub fn new(id: usize, effect_type: EffectType) -> Self {
        Self {
            id,
            effect_type,
            is_enabled: true,
            last_sent_type: None,
        }
    }
}

/// シンセサイザーの設定
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SynthSetting {
    /// シンセサイザーが有効かどうか
    pub is_enabled: bool,
    /// オシレーターの基本周波数 (Hz)
    pub frequency: f32,
    /// 波形
    #[serde(default)]
    pub waveform: Waveform,
    /// ADSRパラメータ
    #[serde(default)]
    pub adsr: AdsrParams,
    /// 前回オーディオエンジンに送信した設定（変更検知用）
    #[serde(skip)]
    pub last_sent_params: Option<(Waveform, AdsrParams)>,
}

impl Default for SynthSetting {
    fn default() -> Self {
        Self {
            is_enabled: false,
            frequency: 440.0,
            waveform: Waveform::default(),
            adsr: AdsrParams::default(),
            last_sent_params: None,
        }
    }
}

/// ARA/SV2等のボーカルシンセ専用の設定
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct VocalSynthSetting {
    /// 音声合成エンジンが有効かどうか
    pub is_enabled: bool,
    /// 選択されているシンガーのIDや名前など
    pub singer_id: Option<String>,
}
