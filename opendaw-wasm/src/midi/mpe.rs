// MPE (MIDI Polyphonic Expression) サポート
// ノートごとに独立したピッチベンド、プレッシャー、ティンバーを保持・解析するデータ構造の骨格です。

/// 個別のMPEノートデータ
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MpeNote {
    /// MIDIノート番号 (0-127)
    pub note_number: u8,
    /// ノートオン・ベロシティ (0-127)
    pub velocity: u8,
    /// 個別ピッチベンド (-8192 ~ 8191)
    pub pitch_bend: i16,
    /// 個別プレッシャー / チャンネルアフタータッチ (0-127)
    pub pressure: u8,
    /// 個別ティンバー (通常CC74) (0-127)
    pub timbre: u8,
    /// リリース・ベロシティ (0-127)
    pub release_velocity: u8,
}

impl MpeNote {
    /// 新しいMPEノートインスタンスを作成します
    pub fn new(note_number: u8, velocity: u8) -> Self {
        Self {
            note_number,
            velocity,
            pitch_bend: 0,
            pressure: 0,
            timbre: 64, // ティンバー（CC74）の一般的な初期値（中央値）
            release_velocity: 0,
        }
    }
}

/// MPE ゾーンの管理構造体
/// マスターチャンネルの全体情報と、メンバーチャンネル（各ノート）の状態を管理します。
#[derive(Debug, Clone)]
pub struct MpeZone {
    /// マスターチャンネル（0-15、通常は0(Ch.1)または15(Ch.16)）
    pub master_channel: u8,
    /// 割り当てられているメンバーチャンネル数
    pub member_channel_count: u8,
    /// 各メンバーチャンネル（0-15）のアクティブなノート
    pub member_notes: [Option<MpeNote>; 16],
    /// マスターピッチベンド (-8192 ~ 8191)
    pub master_pitch_bend: i16,
}

impl MpeZone {
    /// 新しいMPEゾーンを作成します
    pub fn new(master_channel: u8, member_channel_count: u8) -> Self {
        Self {
            master_channel,
            member_channel_count,
            member_notes: [None; 16],
            master_pitch_bend: 0,
        }
    }

    /// ノートオンメッセージの処理
    pub fn handle_note_on(&mut self, channel: u8, note_number: u8, velocity: u8) {
        if velocity == 0 {
            self.handle_note_off(channel, note_number, 0);
            return;
        }
        if channel < 16 && channel != self.master_channel {
            self.member_notes[channel as usize] = Some(MpeNote::new(note_number, velocity));
        }
    }

    /// ノートオフメッセージの処理
    pub fn handle_note_off(&mut self, channel: u8, _note_number: u8, release_velocity: u8) {
        if channel < 16 && channel != self.master_channel {
            // 対象チャンネルのノートを無効化する
            if let Some(note) = &mut self.member_notes[channel as usize] {
                note.release_velocity = release_velocity;
                // 必要に応じてノートのリリース処理をここで呼び出します
            }
            self.member_notes[channel as usize] = None;
        }
    }

    /// ピッチベンドメッセージの処理
    #[allow(clippy::collapsible_if)]
    pub fn handle_pitch_bend(&mut self, channel: u8, value: i16) {
        if channel == self.master_channel {
            self.master_pitch_bend = value;
        } else if channel < 16 {
            if let Some(note) = &mut self.member_notes[channel as usize] {
                note.pitch_bend = value;
            }
        }
    }

    /// プレッシャー（チャンネルアフタータッチ）メッセージの処理
    #[allow(clippy::collapsible_if)]
    pub fn handle_pressure(&mut self, channel: u8, value: u8) {
        if channel != self.master_channel && channel < 16 {
            if let Some(note) = &mut self.member_notes[channel as usize] {
                note.pressure = value;
            }
        }
    }

    /// コントロールチェンジメッセージの処理（CC74等ティンバー）
    #[allow(clippy::collapsible_if)]
    pub fn handle_control_change(&mut self, channel: u8, controller: u8, value: u8) {
        // MPEにおけるティンバーは主にCC74を使用
        if controller == 74 && channel != self.master_channel && channel < 16 {
            if let Some(note) = &mut self.member_notes[channel as usize] {
                note.timbre = value;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpe_note_new() {
        let note = MpeNote::new(60, 100);
        assert_eq!(note.note_number, 60);
        assert_eq!(note.velocity, 100);
        assert_eq!(note.pitch_bend, 0);
        assert_eq!(note.pressure, 0);
        assert_eq!(note.timbre, 64);
        assert_eq!(note.release_velocity, 0);
    }

    #[test]
    fn test_mpe_zone_new() {
        let zone = MpeZone::new(0, 15);
        assert_eq!(zone.master_channel, 0);
        assert_eq!(zone.member_channel_count, 15);
        assert_eq!(zone.master_pitch_bend, 0);
        assert!(zone.member_notes.iter().all(|n| n.is_none()));
    }

    #[test]
    fn test_handle_note_on() {
        let mut zone = MpeZone::new(0, 15);

        // Test normal note on a member channel (channel 1)
        zone.handle_note_on(1, 60, 100);
        assert_eq!(zone.member_notes[1], Some(MpeNote::new(60, 100)));

        // Test note on master channel is ignored
        zone.handle_note_on(0, 62, 100);
        assert_eq!(zone.member_notes[0], None);

        // Test velocity 0 acts as note off
        zone.handle_note_on(1, 60, 0);
        assert_eq!(zone.member_notes[1], None);
    }

    #[test]
    fn test_handle_note_off() {
        let mut zone = MpeZone::new(0, 15);
        zone.handle_note_on(1, 60, 100);
        assert!(zone.member_notes[1].is_some());

        // Test note off
        zone.handle_note_off(1, 60, 64);
        assert_eq!(zone.member_notes[1], None);

        // Test note off on master channel is ignored
        zone.handle_note_on(2, 62, 100);
        zone.handle_note_off(0, 62, 64);
        assert!(zone.member_notes[2].is_some());
    }

    #[test]
    fn test_handle_pitch_bend() {
        let mut zone = MpeZone::new(0, 15);
        zone.handle_note_on(1, 60, 100);

        // Pitch bend on master channel
        zone.handle_pitch_bend(0, 4096);
        assert_eq!(zone.master_pitch_bend, 4096);

        // Pitch bend on member channel
        zone.handle_pitch_bend(1, 2048);
        assert_eq!(zone.member_notes[1].unwrap().pitch_bend, 2048);
    }

    #[test]
    fn test_handle_pressure() {
        let mut zone = MpeZone::new(0, 15);
        zone.handle_note_on(1, 60, 100);

        // Pressure on member channel
        zone.handle_pressure(1, 100);
        assert_eq!(zone.member_notes[1].unwrap().pressure, 100);

        // Pressure on master channel (ignored for note)
        zone.handle_pressure(0, 127);
        assert_eq!(zone.member_notes[1].unwrap().pressure, 100);
    }

    #[test]
    fn test_handle_control_change() {
        let mut zone = MpeZone::new(0, 15);
        zone.handle_note_on(1, 60, 100);

        // CC74 on member channel changes timbre
        zone.handle_control_change(1, 74, 100);
        assert_eq!(zone.member_notes[1].unwrap().timbre, 100);

        // Other CCs are ignored
        zone.handle_control_change(1, 1, 127); // Mod wheel
        assert_eq!(zone.member_notes[1].unwrap().timbre, 100);

        // Master channel CC74 is ignored for notes
        zone.handle_control_change(0, 74, 127);
        assert_eq!(zone.member_notes[1].unwrap().timbre, 100);
    }
}
