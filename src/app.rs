use eframe::egui;

pub struct AuraDawApp {
    pub is_playing: bool,
    pub is_looping: bool,
    pub playhead_pos: f32,
    pub master_volume: f32,
    pub is_muted: bool,
    pub bpm: f32,
}

impl Default for AuraDawApp {
    fn default() -> Self {
        Self {
            is_playing: false,
            is_looping: true,
            playhead_pos: 0.0,
            master_volume: 0.8,
            is_muted: false,
            bpm: 120.0,
        }
    }
}

impl AuraDawApp {
    pub fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted;
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // カスタムフォントやスタイルなどをここで設定
        crate::ui::setup_custom_style(&cc.egui_ctx);
        Self::default()
    }

    pub fn toggle_playback(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn stop_playback(&mut self) {
        self.is_playing = false;
        self.playhead_pos = 0.0;
    }

    pub fn toggle_loop(&mut self) {
        self.is_looping = !self.is_looping;
    }

    pub fn seek_to(&mut self, pos: f32) {
        self.playhead_pos = pos.clamp(0.0, 100.0);
    }

    pub fn tick_playback(&mut self) {
        if self.is_playing {
            // BPMに基づいて進行速度を調整 (120 BPM を基準 (1.0) とする)
            self.playhead_pos += 1.0 * (self.bpm / 120.0);
            // 画面端まで行ったらループさせるか停止する処理
            if self.playhead_pos > 100.0 {
                self.playhead_pos = 0.0;
                if !self.is_looping {
                    self.is_playing = false;
                }
            }
        }
    }
}

impl eframe::App for AuraDawApp {
    // Eframe 0.34
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 再生中の場合、プレイヘッドを進行させて再描画を要求
        if self.is_playing {
            self.tick_playback();
            ctx.request_repaint();
        }

        #[allow(deprecated)]
        egui::CentralPanel::default().show(ctx, |ui| {
             crate::ui::draw_main_ui(self, ui);
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        crate::ui::draw_main_ui(self, ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seek_to() {
        let mut app = AuraDawApp::default();

        app.seek_to(-10.0);
        assert_eq!(app.playhead_pos, 0.0);

        app.seek_to(150.0);
        assert_eq!(app.playhead_pos, 100.0);

        app.seek_to(50.0);
        assert_eq!(app.playhead_pos, 50.0);
    }

    #[test]
    fn test_toggle_playback() {
        let mut app = AuraDawApp::default();
        assert!(!app.is_playing);

        app.toggle_playback();
        assert!(app.is_playing);

        app.toggle_playback();
        assert!(!app.is_playing);
    }

    #[test]
    fn test_stop_playback() {
        let mut app = AuraDawApp::default();
        app.is_playing = true;
        app.playhead_pos = 50.0;

        app.stop_playback();
        assert!(!app.is_playing);
        assert_eq!(app.playhead_pos, 0.0);
    }

    #[test]
    fn test_toggle_loop() {
        let mut app = AuraDawApp::default();
        assert!(app.is_looping); // 💡 初期値は Default::default() により true

        app.toggle_loop();
        assert!(!app.is_looping);

        app.toggle_loop();
        assert!(app.is_looping);
    }

    #[test]
    fn test_playback_end_behavior_with_loop() {
        let mut app = AuraDawApp::default();
        app.is_playing = true;
        app.is_looping = true;
        app.playhead_pos = 100.0; // 次の tick で 100.0 を超える

        app.tick_playback();

        assert!(app.is_playing);
        assert_eq!(app.playhead_pos, 0.0);
    }

    #[test]
    fn test_playback_end_behavior_without_loop() {
        let mut app = AuraDawApp::default();
        app.is_playing = true;
        app.is_looping = false;
        app.playhead_pos = 100.0; // 次の tick で 100.0 を超える

        app.tick_playback();

        assert!(!app.is_playing);
        assert_eq!(app.playhead_pos, 0.0);
    }

    #[test]
    fn test_toggle_mute() {
        let mut app = AuraDawApp::default();
        assert!(!app.is_muted);

        app.toggle_mute();
        assert!(app.is_muted);

        app.toggle_mute();
        assert!(!app.is_muted);
    }

    #[test]
    fn test_bpm_affects_playback_speed() {
        let mut app120 = AuraDawApp::default();
        app120.is_playing = true;
        app120.bpm = 120.0;
        app120.tick_playback();

        let mut app240 = AuraDawApp::default();
        app240.is_playing = true;
        app240.bpm = 240.0;
        app240.tick_playback();

        // 240 BPMの場合は120 BPMの2倍進むはず
        assert_eq!(app120.playhead_pos * 2.0, app240.playhead_pos);
    }
}
