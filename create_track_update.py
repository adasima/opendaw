import re

with open("src/state/track.rs", "r") as f:
    content = f.read()

search_block = """/// シンセサイザーの設定
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SynthSetting {
    /// シンセサイザーが有効かどうか
    pub is_enabled: bool,
    /// オシレーターの基本周波数 (Hz)
    pub frequency: f32,
}

impl Default for SynthSetting {
    fn default() -> Self {
        Self {
            is_enabled: false,
            frequency: 440.0,
        }
    }
}"""

replace_block = """/// 波形の列挙型 (state用)
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Waveform {
    /// サイン波
    Sine,
    /// 矩形波
    Square,
    /// ノコギリ波
    Sawtooth,
}

impl Default for Waveform {
    fn default() -> Self {
        Self::Sine
    }
}

/// ADSRパラメータ (state用)
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdsrParams {
    /// アタックタイム (秒)
    pub attack: f32,
    /// ディケイタイム (秒)
    pub decay: f32,
    /// サステインレベル (0.0 〜 1.0)
    pub sustain: f32,
    /// リリースタイム (秒)
    pub release: f32,
}

impl Default for AdsrParams {
    fn default() -> Self {
        Self {
            attack: 0.01,
            decay: 0.1,
            sustain: 0.5,
            release: 0.1,
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
}

impl Default for SynthSetting {
    fn default() -> Self {
        Self {
            is_enabled: false,
            frequency: 440.0,
            waveform: Waveform::default(),
            adsr: AdsrParams::default(),
        }
    }
}"""

if search_block in content:
    content = content.replace(search_block, replace_block)
    with open("src/state/track.rs", "w") as f:
        f.write(content)
    print("Replaced Track block")
else:
    print("Track search block not found")
