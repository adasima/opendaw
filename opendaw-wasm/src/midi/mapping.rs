use std::collections::HashMap;
use std::sync::Arc;

/// DAW内のパラメータを一意に識別するID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParameterId(pub u32);

/// MIDIコントローラーのCC (Control Change) を一意に識別する情報
/// チャンネルとコントロール番号のペア
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MidiCcKey {
    pub channel: u8,
    pub cc_number: u8,
}

/// MIDI CCからDAWパラメータへのマッピング情報を管理するレジストリ
pub struct MidiMappingRegistry {
    /// MIDI CCのキーから対象のパラメータIDへのマッピング
    cc_to_parameter: HashMap<MidiCcKey, ParameterId>,
    /// パラメータの現在値を保持するマップ (0.0 ~ 1.0 に正規化された値などを想定)
    parameter_values: HashMap<ParameterId, f32>,
}

impl MidiMappingRegistry {
    pub fn new() -> Self {
        Self {
            cc_to_parameter: HashMap::new(),
            parameter_values: HashMap::new(),
        }
    }

    /// MIDI CCとパラメータIDを紐付ける (MIDI Learn機能)
    pub fn learn_mapping(&mut self, key: MidiCcKey, param_id: ParameterId) {
        self.cc_to_parameter.insert(key, param_id);
    }

    /// マッピングを解除する
    pub fn unlearn_mapping(&mut self, key: &MidiCcKey) {
        self.cc_to_parameter.remove(key);
    }

    /// MIDI CCの入力値を受け取り、マッピングされていればパラメータの値を更新する
    /// valueは0~127のMIDI標準値
    pub fn handle_cc_input(&mut self, key: MidiCcKey, value: u8) -> Option<(ParameterId, f32)> {
        if let Some(&param_id) = self.cc_to_parameter.get(&key) {
            // MIDI CCの 0~127 を 0.0~1.0 に正規化
            let normalized_value = (value as f32) / 127.0;
            self.parameter_values.insert(param_id, normalized_value);
            return Some((param_id, normalized_value));
        }
        None
    }

    /// 指定したパラメータの現在値を取得する
    pub fn get_parameter_value(&self, param_id: ParameterId) -> Option<f32> {
        self.parameter_values.get(&param_id).copied()
    }
}

impl Default for MidiMappingRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// スレッドセーフなMidiMappingRegistryのラッパー
pub type SharedMidiMappingRegistry = Arc<std::sync::RwLock<MidiMappingRegistry>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_default() {
        let registry1 = MidiMappingRegistry::new();
        assert!(registry1.cc_to_parameter.is_empty());
        assert!(registry1.parameter_values.is_empty());

        let registry2 = MidiMappingRegistry::default();
        assert!(registry2.cc_to_parameter.is_empty());
        assert!(registry2.parameter_values.is_empty());
    }

    #[test]
    fn test_learn_and_unlearn_mapping() {
        let mut registry = MidiMappingRegistry::new();
        let key = MidiCcKey {
            channel: 1,
            cc_number: 10,
        };
        let param_id = ParameterId(42);

        // Map
        registry.learn_mapping(key, param_id);
        assert_eq!(registry.cc_to_parameter.get(&key), Some(&param_id));

        // Unmap
        registry.unlearn_mapping(&key);
        assert_eq!(registry.cc_to_parameter.get(&key), None);
    }

    #[test]
    fn test_handle_cc_input_mapped() {
        let mut registry = MidiMappingRegistry::new();
        let key = MidiCcKey {
            channel: 1,
            cc_number: 10,
        };
        let param_id = ParameterId(42);

        registry.learn_mapping(key, param_id);

        // Send max value 127
        let result = registry.handle_cc_input(key, 127);
        assert_eq!(result, Some((param_id, 1.0)));
        assert_eq!(registry.get_parameter_value(param_id), Some(1.0));

        // Send half value 64
        let result = registry.handle_cc_input(key, 64);
        let expected_val = 64.0 / 127.0;
        assert_eq!(result, Some((param_id, expected_val)));
        assert_eq!(registry.get_parameter_value(param_id), Some(expected_val));

        // Send min value 0
        let result = registry.handle_cc_input(key, 0);
        assert_eq!(result, Some((param_id, 0.0)));
        assert_eq!(registry.get_parameter_value(param_id), Some(0.0));
    }

    #[test]
    fn test_handle_cc_input_unmapped() {
        let mut registry = MidiMappingRegistry::new();
        let key = MidiCcKey {
            channel: 1,
            cc_number: 10,
        };

        let result = registry.handle_cc_input(key, 127);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_parameter_value_not_found() {
        let registry = MidiMappingRegistry::new();
        let param_id = ParameterId(42);

        assert_eq!(registry.get_parameter_value(param_id), None);
    }
}
