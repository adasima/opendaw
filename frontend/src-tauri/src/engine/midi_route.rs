/// MIDI入力信号を各トラックにルーティングするロジック
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use crossbeam_channel::{bounded, Receiver, Sender};
use log::info;

/// MIDIイベントを表す構造体
#[derive(Debug, Clone)]
pub struct MidiEvent {
    pub device: String,
    pub channel: u8,
    pub note: u8,
    pub velocity: u8,
}

/// トラックのMIDIルーティング設定
#[derive(Clone)]
pub struct TrackMidiRoute {
    pub track_id: u32,
    pub device: String,
    pub channel: Arc<AtomicU8>,
}

impl TrackMidiRoute {
    /// 新しいルーティング設定を作成する
    pub fn new(track_id: u32, device: String, channel: u8) -> Self {
        Self {
            track_id,
            device,
            channel: Arc::new(AtomicU8::new(channel)),
        }
    }

    /// デバイスとチャンネルが一致するか確認する
    pub fn matches(&self, event: &MidiEvent) -> bool {
        let ch = self.channel.load(Ordering::Acquire);
        self.device == event.device && (ch == 0 || ch == event.channel)
    }
}

/// MIDIルーター
pub struct MidiRouter {
    routes: Vec<TrackMidiRoute>,
    _event_tx: Sender<MidiEvent>,
}

impl Default for MidiRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl MidiRouter {
    /// 新しいMidiRouterとイベントレシーバーを作成する
    pub fn create_channel() -> (Self, Receiver<MidiEvent>) {
        let (tx, rx) = bounded(1024);
        let router = Self {
            routes: Vec::new(),
            _event_tx: tx,
        };
        (router, rx)
    }

    /// 新しいMidiRouterを作成する (テスト用)
    pub fn new() -> Self {
        Self::create_channel().0
    }

    /// トラックにルーティングを追加・更新する
    pub fn set_route(&mut self, track_id: u32, device: String, channel: u8) {
        if let Some(route) = self.routes.iter_mut().find(|r| r.track_id == track_id) {
            route.device = device;
            route.channel.store(channel, Ordering::Release);
        } else {
            self.routes.push(TrackMidiRoute::new(track_id, device, channel));
        }
    }

    /// MIDIイベントを受信し、該当するトラックにルーティングする
    pub fn process_event(&self, event: MidiEvent) {
        for route in &self.routes {
            if route.matches(&event) {
                info!("Routing MIDI event {:?} to track {}", event, route.track_id);
                // 実際にはここでトラックごとのキューなどに送る
            }
        }
    }
}
