use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// トラックごとのMIDIルーティング設定
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MidiRoute {
    /// 入力デバイス名（空の場合は全デバイス）
    pub input_device: String,
    /// 入力チャンネル（0はオムニ・全チャンネル、1-16は指定チャンネル）
    pub channel: u8,
}



/// MIDIルーティングマネージャー
/// 入力されたMIDIイベントを、設定されたルーティングに基づいて
/// 適切なトラックへ分配する機能を提供する
#[derive(Debug, Default)]
pub struct MidiRouter {
    /// 各トラックIDに対応するルーティング設定
    routes: HashMap<u32, MidiRoute>,
}

impl MidiRouter {
    /// 新しいMidiRouterを作成する
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// 特定のトラックにMIDIルーティングを設定する
    pub fn set_route(&mut self, track_id: u32, device: String, channel: u8) {
        info!("MIDI Router: Set track {} route - Device: '{}', Channel: {}", track_id, device, channel);
        self.routes.insert(track_id, MidiRoute {
            input_device: device,
            channel,
        });
    }

    /// 特定のトラックのルーティング設定を取得する
    pub fn get_route(&self, track_id: u32) -> Option<&MidiRoute> {
        self.routes.get(&track_id)
    }

    /// 特定のMIDI入力（デバイス名、チャンネル）が、どのトラックIDにルーティングされるべきかを判定する
    pub fn resolve_targets(&self, device_name: &str, channel: u8) -> Vec<u32> {
        let mut targets = Vec::new();

        for (track_id, route) in &self.routes {
            let device_match = route.input_device.is_empty() || route.input_device == device_name;
            let channel_match = route.channel == 0 || route.channel == channel;

            if device_match && channel_match {
                targets.push(*track_id);
            }
        }

        targets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_router() {
        let mut router = MidiRouter::new();

        // トラック1: Launchkey, CH 1
        router.set_route(1, "Launchkey".to_string(), 1);
        // トラック2: Launchkey, CH 2
        router.set_route(2, "Launchkey".to_string(), 2);
        // トラック3: 全デバイス, CH 0 (オムニ)
        router.set_route(3, "".to_string(), 0);

        // Launchkey CH 1 からの入力
        let targets = router.resolve_targets("Launchkey", 1);
        assert!(targets.contains(&1));
        assert!(!targets.contains(&2));
        assert!(targets.contains(&3)); // トラック3はオムニ

        // 別のデバイスからの入力
        let targets_other = router.resolve_targets("OtherDevice", 1);
        assert!(!targets_other.contains(&1));
        assert!(targets_other.contains(&3));
    }
}
