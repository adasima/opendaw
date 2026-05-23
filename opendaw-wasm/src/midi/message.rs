//! MIDIメッセージのパースを管理するモジュール

/// タイムスタンプとメッセージ内容を含むMIDIイベント
#[derive(Debug, Clone, PartialEq)]
pub struct MidiEvent {
    /// タイムスタンプ
    pub stamp: u64,
    /// 受信した生データ
    pub message: Vec<u8>,
    /// パース済みのメッセージ
    pub parsed: MidiMessage,
}

impl MidiEvent {
    /// 新しいMidiEventを作成する
    pub fn new(stamp: u64, message: &[u8]) -> Self {
        Self {
            stamp,
            message: message.to_vec(),
            parsed: MidiMessage::parse(message),
        }
    }
}

/// 解析済みのMIDIメッセージ
#[derive(Debug, Clone, PartialEq)]
pub enum MidiMessage {
    /// ノートオフ (チャンネル, ノート番号, ベロシティ)
    NoteOff(u8, u8, u8),
    /// ノートオン (チャンネル, ノート番号, ベロシティ)
    NoteOn(u8, u8, u8),
    /// コントロールチェンジ (チャンネル, コントロール番号, 値)
    ControlChange(u8, u8, u8),
    /// ピッチベンド (チャンネル, 値)
    PitchBend(u8, u16),
    /// 未対応のメッセージ
    Unknown(Vec<u8>),
}

impl MidiMessage {
    /// 生のバイト列からMidiMessageをパースする
    pub fn parse(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return MidiMessage::Unknown(bytes.to_vec());
        }

        let status = bytes[0];
        let message_type = status & 0xF0;
        let channel = status & 0x0F;

        match message_type {
            // Note Off (0x80)
            0x80 if bytes.len() >= 3 => MidiMessage::NoteOff(channel, bytes[1], bytes[2]),
            // Note On (0x90)
            0x90 if bytes.len() >= 3 => {
                let velocity = bytes[2];
                if velocity == 0 {
                    // Velocity 0 is typically interpreted as Note Off
                    MidiMessage::NoteOff(channel, bytes[1], 0)
                } else {
                    MidiMessage::NoteOn(channel, bytes[1], velocity)
                }
            }
            // Control Change (0xB0)
            0xB0 if bytes.len() >= 3 => MidiMessage::ControlChange(channel, bytes[1], bytes[2]),
            // Pitch Bend (0xE0)
            0xE0 if bytes.len() >= 3 => {
                let lsb = bytes[1] as u16;
                let msb = bytes[2] as u16;
                // Pitch bend uses 14 bits
                let value = (msb << 7) | lsb;
                MidiMessage::PitchBend(channel, value)
            }
            _ => MidiMessage::Unknown(bytes.to_vec()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_note_on() {
        // Note On, Channel 0, Note 60 (C4), Velocity 100
        let msg = MidiMessage::parse(&[0x90, 60, 100]);
        assert_eq!(msg, MidiMessage::NoteOn(0, 60, 100));

        // Note On, Channel 2, Note 64 (E4), Velocity 127
        let msg = MidiMessage::parse(&[0x92, 64, 127]);
        assert_eq!(msg, MidiMessage::NoteOn(2, 64, 127));
    }

    #[test]
    fn test_parse_note_on_velocity_zero_as_note_off() {
        // Note On, Channel 0, Note 60 (C4), Velocity 0
        let msg = MidiMessage::parse(&[0x90, 60, 0]);
        assert_eq!(msg, MidiMessage::NoteOff(0, 60, 0));
    }

    #[test]
    fn test_parse_note_off() {
        // Note Off, Channel 0, Note 60 (C4), Velocity 64
        let msg = MidiMessage::parse(&[0x80, 60, 64]);
        assert_eq!(msg, MidiMessage::NoteOff(0, 60, 64));
    }

    #[test]
    fn test_parse_control_change() {
        // CC, Channel 0, Controller 7 (Volume), Value 100
        let msg = MidiMessage::parse(&[0xB0, 7, 100]);
        assert_eq!(msg, MidiMessage::ControlChange(0, 7, 100));
    }

    #[test]
    fn test_parse_pitch_bend() {
        // Pitch Bend, Channel 0, Center value (MSB 64, LSB 0)
        let msg = MidiMessage::parse(&[0xE0, 0, 64]);
        // 64 << 7 | 0 = 8192
        assert_eq!(msg, MidiMessage::PitchBend(0, 8192));
    }

    #[test]
    fn test_parse_unknown() {
        // System Exclusive
        let msg = MidiMessage::parse(&[0xF0, 0x7E, 0x7F, 0x09, 0x01, 0xF7]);
        assert_eq!(
            msg,
            MidiMessage::Unknown(vec![0xF0, 0x7E, 0x7F, 0x09, 0x01, 0xF7])
        );

        // Incomplete message
        let msg = MidiMessage::parse(&[0x90, 60]);
        assert_eq!(msg, MidiMessage::Unknown(vec![0x90, 60]));
    }

    #[test]
    fn test_midi_event_new() {
        let event = MidiEvent::new(1000, &[0x90, 60, 100]);
        assert_eq!(event.stamp, 1000);
        assert_eq!(event.message, vec![0x90, 60, 100]);
        assert_eq!(event.parsed, MidiMessage::NoteOn(0, 60, 100));
    }
}
