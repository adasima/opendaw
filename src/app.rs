use eframe::egui;
use crate::state::DawState;
use ringbuf::traits::Producer;

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
    /// エフェクトウィンドウを開いているトラックのID
    pub opened_effect_track_id: Option<usize>,
}

impl Default for AuraDawApp {
    fn default() -> Self {
        let (ui_channels, audio_channels) = crate::engine::channel::create_channels(CHANNEL_CAPACITY);
        Self {
            state: DawState::default(),
            audio_engine: crate::engine::AudioEngine::new(),
            ui_channels: Some(ui_channels),
            audio_channels_temp: Some(audio_channels),
            opened_effect_track_id: None,
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
        // 初期化時にオーディオチャンネルをエンジンに渡す
        if let Some(audio_channels) = self.audio_channels_temp.take() {
            self.audio_engine.set_channels(audio_channels);
        }

        // オーディオエンジンからのメッセージを受信
        if let Some(ui_channels) = &mut self.ui_channels {
            let pos_opt = self.audio_engine.poll_ui_messages(ui_channels);
            if let Some(pos) = pos_opt {
                self.state.playhead_pos = pos;
            }
        }

        // キーボードショートカット: スペースキーで再生/停止
        // テキスト入力等のUI要素がフォーカスされていない場合のみ反応させます。
        let focused = ctx.memory(|mem| mem.focused());
        if focused.is_none() && ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.state.toggle_playback();
            if let Some(ui_channels) = &mut self.ui_channels {
                let send_result = ui_channels.0.try_push(crate::engine::channel::UiToAudioMsg::SetPlaying(self.state.is_playing));
                if send_result.is_err() {
                    log::warn!("Failed to send SetPlaying message: channel full");
                }
            }
        }

        // 再生中の場合、プレイヘッドを進行させて再描画を要求
        if self.state.is_playing {
            self.state.tick_playback();
            ctx.request_repaint();
        }

        crate::ui::effects::draw_effects_window(ctx, self);

        #[allow(deprecated)]
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("OpenDAW");
                ui.separator();
                crate::ui::import::draw_import_ui(ui, self);
            });
        });

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

    #[test]
    fn test_app_update_channel_initialization() {
        // 初期化時に audio_channels_temp がエンジンに渡されることを確認
        let mut app = AuraDawApp::default();
        assert!(app.audio_channels_temp.is_some());

        if let Some(audio_channels) = app.audio_channels_temp.take() {
            app.audio_engine.set_channels(audio_channels);
        }

        assert!(app.audio_channels_temp.is_none());
        // channels 自体は private なので、間接的に送信して影響を確認するなどで代用するが、
        // 今回は audio_channels_temp が None になることのみを確認する。
    }
}
