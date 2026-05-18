use eframe::egui;
use crate::state::DawState;

/// OpenDAWのメインアプリケーション状態を保持する構造体。
///
/// eframeのトップレベルとして機能し、オーディオエンジンの状態や
/// UI全体の共有データ(`DawState`)を管理します。
/// チャンネルの初期容量
const CHANNEL_CAPACITY: usize = 1024;

pub struct AuraDawApp {
    /// DAWのコア状態（再生状態、ボリューム、プレイヘッド位置など）
    pub state: DawState,
    /// オーディオエンジンのインスタンス
    pub audio_engine: crate::engine::AudioEngine,
    /// UI ↔ オーディオ間の通信チャンネル
    pub ui_channels: Option<crate::engine::channel::UiChannels>,
    /// オーディオエンジンに渡すまでの通信チャンネルの一時保持
    pub audio_channels_temp: Option<crate::engine::channel::AudioChannels>,
}

impl Default for AuraDawApp {
    fn default() -> Self {
        let (ui_channels, audio_channels) = crate::engine::channel::create_channels(CHANNEL_CAPACITY);
        Self {
            state: DawState::default(),
            audio_engine: crate::engine::AudioEngine::new(),
            ui_channels: Some(ui_channels),
            audio_channels_temp: Some(audio_channels),
        }
    }
}

impl AuraDawApp {
    /// アプリケーションの新しいインスタンスを作成します。
    ///
    /// ここでカスタムフォントやUIテーマ（ダークテーマ・グラスモーフィズム風）の初期化を行います。
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // カスタムフォントやスタイルなどをここで設定
        crate::ui::setup_custom_style(&cc.egui_ctx);
        Self::default()
    }
}

impl eframe::App for AuraDawApp {
    // Eframe 0.34
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // キーボードショートカット: スペースキーで再生/停止
        // テキスト入力等のUI要素がフォーカスされていない場合のみ反応させます。
        let focused = ctx.memory(|mem| mem.focused());
        if focused.is_none() && ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.state.toggle_playback();
        }

        // 再生中の場合、プレイヘッドを進行させて再描画を要求
        if self.state.is_playing {
            self.state.tick_playback();
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
    fn test_app_new() {
        // App構造体の初期化が正常にできるか確認
        // eframe::CreationContextをモックするのは難しいため、
        // Default::default() で状態が初期化されることのみを確認します。
        let app = AuraDawApp::default();
        assert!(!app.state.is_playing);
        assert_eq!(app.state.playhead_pos, 0.0);
        // チャンネルが初期化されていることを確認
        assert!(app.ui_channels.is_some());
    }
}
