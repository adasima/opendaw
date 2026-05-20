//! MIDIデバイスの列挙と接続を管理するモジュール

use crate::midi::message::MidiEvent;
use midir::{Ignore, MidiInput, MidiInputConnection};
use std::sync::mpsc::{Receiver, channel};

/// 利用可能なMIDI入力デバイスのリストを取得する
pub fn available_input_devices() -> Vec<String> {
    let mut devices = Vec::new();
    if let Ok(mut midi_in) = MidiInput::new("OpenDAW MIDI Input") {
        midi_in.ignore(Ignore::None);
        for port in midi_in.ports() {
            if let Ok(name) = midi_in.port_name(&port) {
                devices.push(name);
            }
        }
    }
    devices
}

/// MIDI入力デバイスに接続し、メッセージを受信するチャンネルを返す
pub fn connect_to_input(
    device_name: &str,
) -> Result<(MidiInputConnection<()>, Receiver<MidiEvent>), String> {
    let mut midi_in = MidiInput::new("OpenDAW MIDI Input")
        .map_err(|e| format!("Failed to create MIDI input: {}", e))?;
    midi_in.ignore(Ignore::None);

    let ports = midi_in.ports();
    let port = ports
        .into_iter()
        .find(|p| midi_in.port_name(p).unwrap_or_default() == device_name)
        .ok_or_else(|| format!("Device not found: {}", device_name))?;

    let (tx, rx) = channel();

    // コールバックは別スレッドで実行される
    let connection = midi_in
        .connect(
            &port,
            "OpenDAW Input Connection",
            move |stamp, message, _| {
                let event = MidiEvent::new(stamp, message);
                // エラーを無視する (受信側がドロップされた場合)
                let _ = tx.send(event);
            },
            (),
        )
        .map_err(|e| format!("Failed to connect to port: {}", e))?;

    Ok((connection, rx))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::absurd_extreme_comparisons)]
    #[allow(unused_comparisons)]
    fn test_available_input_devices_does_not_panic() {
        let devices = available_input_devices();
        // CI環境などではデバイスがない可能性があるため、
        // パニックせずに空のベクターが返ってくることを確認
        // len() >= 0 は常に真ですが、実行できたことが重要です
        assert!(devices.len() >= 0);
    }
}
