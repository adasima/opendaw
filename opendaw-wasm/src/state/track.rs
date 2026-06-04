//! トラック状態管理モジュール
//!
//! 各トラックの名前、ボリューム、パン、ミュート、ソロ状態などを管理する構造体。

pub use crate::state::track_plugin::*;
pub use crate::state::track_clip::*;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AutomationPoint {
    pub id: usize,
    pub time: f64,
    pub value: f32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AutomationTrack {
    pub parameter_name: String,
    pub points: Vec<AutomationPoint>,
}

/// DAW内の単一トラックの状態を保持する構造体
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Track {
    /// トラックの一意なID
    pub id: usize,
    /// トラック名
    pub name: String,
    /// トラックのボリューム (0.0 = 無音, 1.0 = デフォルト, 1.0以上 = ブースト)
    pub volume: f32,
    /// パン (-1.0 = 左, 0.0 = センター, 1.0 = 右)
    pub pan: f32,
    /// ミュート状態（trueなら発音しない）
    pub is_muted: bool,
    /// ソロ状態（trueなら他のソロでないトラックはミュートされる）
    pub is_solo: bool,
    /// 録音待機状態
    pub is_record_armed: bool,
    /// トラックに適用されるエフェクトチェーン
    pub effects: Vec<EffectSetting>,
    /// シンセサイザーの設定
    #[serde(default)]
    pub synth: SynthSetting,
    /// トラックの種類
    #[serde(default)]
    pub track_type: TrackType,
    /// ボーカルシンセ(ARA/SV2)の設定
    #[serde(default)]
    pub vocal_synth: VocalSynthSetting,
    /// トラック内のオーディオクリップ
    #[serde(default)]
    pub clips: Vec<crate::state::clip::AudioClip>,
    /// トラック内のMIDIクリップ
    #[serde(default)]
    pub midi_clips: Vec<crate::state::clip::MidiClip>,
    #[serde(default)]
    pub automations: Vec<AutomationTrack>,
    #[serde(default)]
    pub automation_visible: bool,
    #[serde(default)]
    pub selected_automation: Option<String>,
}

impl Track {
    /// 新しいトラックを初期値で作成します。
    pub fn new(id: usize, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            volume: 1.0,
            pan: 0.0,
            is_muted: false,
            is_solo: false,
            is_record_armed: false,
            effects: Vec::new(),
            synth: SynthSetting::default(),
            track_type: TrackType::default(),
            vocal_synth: VocalSynthSetting::default(),
            clips: Vec::new(),
            midi_clips: Vec::new(),
            automations: Vec::new(),
            automation_visible: false,
            selected_automation: None,
        }
    }

    /// トラックの名前を設定します。
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// トラックのボリュームを設定します。
    /// 値は 0.0 以上にクランプされます。
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.max(0.0);
    }

    /// トラックのパンを設定します。
    /// 値は -1.0 から 1.0 の間にクランプされます。
    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan.clamp(-1.0, 1.0);
    }

    /// ミュート状態を切り替えます。
    pub fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted;
    }

    /// ソロ状態を切り替えます。
    pub fn toggle_solo(&mut self) {
        self.is_solo = !self.is_solo;
    }

    /// 録音待機状態を切り替えます。
    pub fn toggle_record_arm(&mut self) {
        self.is_record_armed = !self.is_record_armed;
    }

    /// エフェクトを追加します。
    pub fn add_effect(&mut self, effect: EffectSetting) {
        self.effects.push(effect);
    }

    /// 指定したIDのエフェクトを削除します。
    pub fn remove_effect(&mut self, id: usize) {
        self.effects.retain(|e| e.id != id);
    }

    /// エフェクトの順序を移動します。
    pub fn move_effect(&mut self, from_index: usize, to_index: usize) {
        if from_index < self.effects.len() && to_index < self.effects.len() {
            let effect = self.effects.remove(from_index);
            self.effects.insert(to_index, effect);
        }
    }

    /// シンセサイザーの有効/無効を切り替えます。
    pub fn toggle_synth(&mut self) {
        self.synth.is_enabled = !self.synth.is_enabled;
    }

    /// シンセサイザーの周波数を設定します。
    pub fn set_synth_frequency(&mut self, freq: f32) {
        // 一般的な可聴域と少しの余裕を持たせる (20.0Hz ~ 20000.0Hz)
        self.synth.frequency = freq.clamp(20.0, 20000.0);
    }

    /// トラックの種類を設定します。
    pub fn set_track_type(&mut self, track_type: TrackType) {
        self.track_type = track_type;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_new() {
        let track = Track::new(1, "Vocals");
        assert_eq!(track.id, 1);
        assert_eq!(track.name, "Vocals");
        assert_eq!(track.volume, 1.0);
        assert_eq!(track.pan, 0.0);
        assert!(!track.is_muted);
        assert!(!track.is_solo);
        assert!(track.effects.is_empty());
        assert!(!track.synth.is_enabled);
        assert_eq!(track.synth.frequency, 440.0);
        assert_eq!(track.track_type, TrackType::Normal);
        assert!(!track.vocal_synth.is_enabled);
        assert!(track.clips.is_empty());
        assert!(track.midi_clips.is_empty());
    }

    #[test]
    fn test_track_set_name() {
        let mut track = Track::new(1, "Vocals");
        track.set_name("Main Vocals");
        assert_eq!(track.name, "Main Vocals");
    }

    #[test]
    fn test_track_set_volume() {
        let mut track = Track::new(1, "Vocals");

        track.set_volume(0.5);
        assert_eq!(track.volume, 0.5);

        // 負の値は0.0にクランプされる
        track.set_volume(-0.5);
        assert_eq!(track.volume, 0.0);

        // 1.0以上の値は許容される(ブースト)
        track.set_volume(2.0);
        assert_eq!(track.volume, 2.0);
    }

    #[test]
    fn test_track_set_pan() {
        let mut track = Track::new(1, "Vocals");

        track.set_pan(-0.5);
        assert_eq!(track.pan, -0.5);

        track.set_pan(0.8);
        assert_eq!(track.pan, 0.8);

        // 範囲外の値はクランプされる
        track.set_pan(-2.0);
        assert_eq!(track.pan, -1.0);

        track.set_pan(1.5);
        assert_eq!(track.pan, 1.0);
    }

    #[test]
    fn test_track_toggle_mute() {
        let mut track = Track::new(1, "Vocals");
        assert!(!track.is_muted);

        track.toggle_mute();
        assert!(track.is_muted);

        track.toggle_mute();
        assert!(!track.is_muted);
    }

    #[test]
    fn test_track_toggle_solo() {
        let mut track = Track::new(1, "Vocals");
        assert!(!track.is_solo);

        track.toggle_solo();
        assert!(track.is_solo);

        track.toggle_solo();
        assert!(!track.is_solo);
    }

    #[test]
    fn test_track_add_effect() {
        let mut track = Track::new(1, "Vocals");
        let effect = EffectSetting::new(1, EffectType::Gain);
        track.add_effect(effect.clone());

        assert_eq!(track.effects.len(), 1);
        assert_eq!(track.effects[0], effect);
    }

    #[test]
    fn test_track_remove_effect() {
        let mut track = Track::new(1, "Vocals");
        track.add_effect(EffectSetting::new(1, EffectType::Gain));
        track.add_effect(EffectSetting::new(2, EffectType::Filter));

        assert_eq!(track.effects.len(), 2);

        track.remove_effect(1);
        assert_eq!(track.effects.len(), 1);
        assert_eq!(track.effects[0].id, 2);
    }

    #[test]
    fn test_track_move_effect() {
        let mut track = Track::new(1, "Vocals");
        track.add_effect(EffectSetting::new(1, EffectType::Gain));
        track.add_effect(EffectSetting::new(2, EffectType::Filter));
        track.add_effect(EffectSetting::new(3, EffectType::Gain));

        track.move_effect(0, 2);

        assert_eq!(track.effects.len(), 3);
        assert_eq!(track.effects[0].id, 2);
        assert_eq!(track.effects[1].id, 3);
        assert_eq!(track.effects[2].id, 1);
    }

    #[test]
    fn test_track_toggle_synth() {
        let mut track = Track::new(1, "Synth Track");
        assert!(!track.synth.is_enabled);
        track.toggle_synth();
        assert!(track.synth.is_enabled);
    }

    #[test]
    fn test_track_set_synth_frequency() {
        let mut track = Track::new(1, "Synth Track");
        track.set_synth_frequency(880.0);
        assert_eq!(track.synth.frequency, 880.0);

        track.set_synth_frequency(10.0); // clamped to 20.0
        assert_eq!(track.synth.frequency, 20.0);
    }
}
