use crate::state::DawState;
use eframe::egui;
use ringbuf::traits::Producer;

/// OpenDAWのメインアプリケーション状態を保持する構造体。
///
/// eframeのトップレベルとして機能し、オーディオエンジンの状態や
/// UI全体の共有データ(`DawState`)を管理します。
/// チャンネルの初期容量
const CHANNEL_CAPACITY: usize = 1024;

pub struct OpenDawApp {
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
    /// MCPサーバーからのコマンドを受信するチャンネル
    pub mcp_receiver: Option<crate::mcp::channel::McpCommandReceiver>,
    /// 現在のビューがセッションビューかどうか
    pub is_session_view: bool,
    /// プラグインブラウザを開いているかどうか
    pub is_plugin_browser_open: bool,
    pub recorder: Option<crate::engine::recording::Recorder>,
    pub was_recording: bool,
    pub piano_roll: crate::ui::piano_roll::PianoRoll,
    pub selected_track_id: Option<usize>,
    pub selected_clip_id: Option<usize>,
    pub is_dragging_clip: bool,
}

impl Default for OpenDawApp {
    fn default() -> Self {
        let (ui_channels, audio_channels) =
            crate::engine::channel::create_channels(CHANNEL_CAPACITY);
        Self {
            state: DawState::default(),
            audio_engine: crate::engine::AudioEngine::new(),
            ui_channels: Some(ui_channels),
            audio_channels_temp: Some(audio_channels),
            opened_effect_track_id: None,
            mcp_receiver: None,
            is_session_view: false,
            is_plugin_browser_open: false,
            recorder: Some(crate::engine::recording::Recorder::new()),
            was_recording: false,
            piano_roll: crate::ui::piano_roll::PianoRoll::default(),
            selected_track_id: None,
            selected_clip_id: None,
            is_dragging_clip: false,
        }
    }
}

impl OpenDawApp {
    /// UIで変更されたシンセサイザーのパラメータをポーリングしてオーディオエンジンに送信します。
    pub fn poll_effect_params(&mut self) {
        if let Some(ui_channels) = &mut self.ui_channels {
            for track in &mut self.state.tracks {
                for effect in &mut track.effects {
                    let changed = match &effect.last_sent_type {
                        Some(last_type) => last_type != &effect.effect_type,
                        None => true,
                    };

                    if changed {
                        match &effect.effect_type {
                            crate::state::track::EffectType::Delay {
                                time_ms,
                                feedback,
                                mix,
                            } => {
                                let params = crate::engine::channel::EffectParams::Delay {
                                    time_ms: *time_ms,
                                    feedback: *feedback,
                                    mix: *mix,
                                };
                                let _ = ui_channels.0.try_push(
                                    crate::engine::channel::UiToAudioMsg::UpdateEffectParams(
                                        track.id, effect.id, params,
                                    ),
                                );
                            }
                            crate::state::track::EffectType::Gain => {}
                            crate::state::track::EffectType::Filter => {}
                        }
                        effect.last_sent_type = Some(effect.effect_type.clone());
                    }
                }
            }
        }
    }

    pub fn poll_synth_params(&mut self) {
        if let Some(ui_channels) = &mut self.ui_channels {
            for track in &mut self.state.tracks {
                if track.synth.is_enabled {
                    let changed = match &track.synth.last_sent_params {
                        Some(params) => {
                            params.0 != track.synth.waveform || params.1 != track.synth.adsr
                        }
                        None => true,
                    };
                    if changed {
                        let current_params = (track.synth.waveform, track.synth.adsr);
                        let waveform = match track.synth.waveform {
                            crate::state::track::Waveform::Sine => {
                                crate::engine::synth::Waveform::Sine
                            }
                            crate::state::track::Waveform::Square => {
                                crate::engine::synth::Waveform::Square
                            }
                            crate::state::track::Waveform::Sawtooth => {
                                crate::engine::synth::Waveform::Sawtooth
                            }
                        };
                        let adsr = crate::engine::synth::AdsrParams {
                            attack: track.synth.adsr.attack,
                            decay: track.synth.adsr.decay,
                            sustain: track.synth.adsr.sustain,
                            release: track.synth.adsr.release,
                        };
                        let msg = crate::engine::channel::UiToAudioMsg::UpdateSynthParams(
                            track.id, waveform, adsr,
                        );
                        if ui_channels.0.try_push(msg).is_ok() {
                            track.synth.last_sent_params = Some(current_params);
                        }
                    }
                }
            }
        }
    }

    /// プレイヘッド位置から現在アクティブなノートを判定してオーディオエンジンに送信します。
    pub fn process_active_notes(&mut self) {
        let current_pos = self.state.playhead_pos as f64;

        if let Some(ui_channels) = &mut self.ui_channels {
            for track in &self.state.tracks {
                if !track.synth.is_enabled {
                    continue;
                }

                let mut active_freqs = [0.0; crate::engine::channel::MAX_ACTIVE_NOTES];
                let mut active_count = 0;

                if self.state.is_playing {
                    // 1. グローバルなスクラッチパッド(active_sequence)を再生
                    for note in &self.state.active_sequence.notes {
                        if current_pos >= note.start_beat
                            && current_pos < note.start_beat + note.duration_beats
                            && active_count < crate::engine::channel::MAX_ACTIVE_NOTES
                        {
                            let freq = 440.0 * 2.0_f32.powf((note.pitch as f32 - 69.0) / 12.0);
                            active_freqs[active_count] = freq;
                            active_count += 1;
                        }
                    }

                    // 2. トラック内のMidiClipを再生
                    for clip in &track.midi_clips {
                        let clip_end_beat = clip.start_beat + clip.length_beats;
                        if current_pos >= clip.start_beat && current_pos < clip_end_beat {
                            let local_pos = current_pos - clip.start_beat;
                            for note in &clip.sequence.notes {
                                if local_pos >= note.start_beat
                                    && local_pos < note.start_beat + note.duration_beats
                                    && active_count < crate::engine::channel::MAX_ACTIVE_NOTES
                                {
                                    let freq =
                                        440.0 * 2.0_f32.powf((note.pitch as f32 - 69.0) / 12.0);
                                    active_freqs[active_count] = freq;
                                    active_count += 1;
                                }
                            }
                        }
                    }
                }

                let _ = ui_channels
                    .0
                    .try_push(crate::engine::channel::UiToAudioMsg::ActiveNotes(
                        track.id,
                        active_freqs,
                        active_count,
                    ));
            }
        }
    }

    /// アプリケーションの新しいインスタンスを作成します。
    ///
    /// ここでカスタムフォントやUIテーマ（ダークテーマ・グラスモーフィズム風）の初期化を行います。
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        mcp_receiver: Option<crate::mcp::channel::McpCommandReceiver>,
    ) -> Self {
        // カスタムフォントやスタイルなどをここで設定
        crate::ui::setup_custom_style(&cc.egui_ctx);

        #[cfg(target_arch = "wasm32")]
        crate::EGUI_CTX.with(|ctx| {
            *ctx.borrow_mut() = Some(cc.egui_ctx.clone());
        });

        Self {
            mcp_receiver,
            ..Default::default()
        }
    }
}

impl eframe::App for OpenDawApp {
    // Eframe 0.34
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // MCPサーバーからのメッセージを受信して状態を更新
        crate::mcp::handler::poll_mcp_commands(
            &self.mcp_receiver,
            &mut self.state,
            &mut self.ui_channels,
            &mut self.selected_track_id,
            &mut self.selected_clip_id,
        );

        #[cfg(target_arch = "wasm32")]
        {
            let json_str = crate::get_tracks_json();
            crate::state::sync::sync_project_state_json(
                &mut self.state,
                self.is_dragging_clip,
                &json_str,
            );
        }

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

        // プレイヘッド位置をJS側に渡すためにグローバルに保存
        crate::set_playhead_pos(self.state.playhead_pos as f64);

        // トラック状態をJSON化してSvelteに渡す
        #[cfg(target_arch = "wasm32")]
        {
            #[derive(serde::Serialize)]
            struct TrackJson {
                id: usize,
                name: String,
                is_muted: bool,
                is_solo: bool,
                is_record_armed: bool,
            }
            let tracks_data: Vec<TrackJson> = self
                .state
                .tracks
                .iter()
                .map(|t| TrackJson {
                    id: t.id,
                    name: t.name.clone(),
                    is_muted: t.is_muted,
                    is_solo: t.is_solo,
                    is_record_armed: t.is_record_armed,
                })
                .collect();
            if let Ok(json_str) = serde_json::to_string(&tracks_data) {
                crate::set_tracks_json(json_str);
            }
        }

        if self.state.is_recording && !self.was_recording {
            if let Some(recorder) = &mut self.recorder {
                let _ = recorder.start_recording(None);
            }
            self.was_recording = true;
        } else if !self.state.is_recording && self.was_recording {
            if let Some(recorder) = &mut self.recorder {
                recorder.stop_recording();
                let data = recorder.collect_recorded_data();
                if !data.is_empty() {
                    let arc_data = std::sync::Arc::new(data.clone());
                    let clip_length = arc_data.len() as f32 / 44100.0;
                    if self.state.tracks.is_empty() {
                        self.state.add_track("Recorded Track");
                    }
                    let track_idx = 0;
                    let clip_id = self.state.tracks[track_idx].clips.len() + 1;
                    let start_pos = self.state.playhead_pos;
                    let mut new_clip = crate::state::clip::AudioClip::new(
                        clip_id,
                        "Recorded Clip",
                        start_pos,
                        clip_length,
                    );
                    let summary: Vec<f32> = data.iter().step_by(100).copied().collect();
                    new_clip.set_waveform_summary(summary);
                    self.state.tracks[track_idx].clips.push(new_clip);
                    if let Some(ui_channels) = &mut self.ui_channels {
                        let track_id = self.state.tracks[track_idx].id;
                        let start_sample = (start_pos * 44100.0 / 120.0) as usize; // 仮の変換（秒への変換が必要だが今回はBPM120基準で処理）
                        let _ = ui_channels.0.try_push(
                            crate::engine::channel::UiToAudioMsg::AddRecordedClip(
                                track_id,
                                start_sample,
                                arc_data,
                            ),
                        );
                    }
                }
            }
            self.was_recording = false;
        }

        // キーボードショートカット: スペースキーで再生/停止
        // テキスト入力等のUI要素がフォーカスされていない場合のみ反応させます。
        let focused = ctx.memory(|mem| mem.focused());
        if focused.is_none() && ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.state.toggle_playback();
            if let Some(ui_channels) = &mut self.ui_channels {
                let send_result =
                    ui_channels
                        .0
                        .try_push(crate::engine::channel::UiToAudioMsg::SetPlaying(
                            self.state.is_playing,
                        ));
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

        self.poll_synth_params();
        self.poll_effect_params();
        self.process_active_notes();

        crate::ui::effects::draw_effects_window(ctx, self);

        if self.is_plugin_browser_open {
            egui::Window::new("Plugin Browser")
                .open(&mut self.is_plugin_browser_open)
                .show(ctx, |ui| {
                    crate::ui::browser::draw_browser_panel(ui);
                });
        }

        #[allow(deprecated)]
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT))
            .show(ctx, |ui| {
                if let Some(_track_id) = self.selected_track_id {
                    // 脱出ボタン
                    if ui.button("⬅ 戻る (閉じる)").clicked() {
                        self.selected_track_id = None;
                        self.selected_clip_id = None;
                    }
                    ui.separator();
                    crate::ui::piano_roll::draw_piano_roll(ui, self);
                } else {
                    crate::ui::arranger::draw_arranger(ui, self);
                }
            });
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Some(_track_id) = self.selected_track_id {
            if ui.button("⬅ 戻る (閉じる)").clicked() {
                self.selected_track_id = None;
                self.selected_clip_id = None;
            }
            ui.separator();
            crate::ui::piano_roll::draw_piano_roll(ui, self);
        } else {
            crate::ui::arranger::draw_arranger(ui, self);
        }
    }

    // 背景を透明にする設定（eframe 0.34用）
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
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
        let app = OpenDawApp::default();
        assert!(!app.state.is_playing);
        assert_eq!(app.state.playhead_pos, 0.0);
        // チャンネルが初期化されていることを確認
        assert!(app.ui_channels.is_some());
        // デフォルトではセッションビューが無効であることを確認
        assert!(!app.is_session_view);
    }

    #[test]
    fn test_browser_switching() {
        let mut app = OpenDawApp::default();
        assert!(!app.is_plugin_browser_open);

        app.is_plugin_browser_open = true;
        assert!(app.is_plugin_browser_open);
    }

    #[test]
    fn test_view_switching() {
        // ビュー切り替えのテスト
        let mut app = OpenDawApp::default();
        assert!(!app.is_session_view);

        app.is_session_view = true;
        assert!(app.is_session_view);
    }

    #[test]
    fn test_app_update_channel_initialization() {
        // 初期化時に audio_channels_temp がエンジンに渡されることを確認
        let mut app = OpenDawApp::default();
        assert!(app.audio_channels_temp.is_some());

        if let Some(audio_channels) = app.audio_channels_temp.take() {
            app.audio_engine.set_channels(audio_channels);
        }

        assert!(app.audio_channels_temp.is_none());
        // channels 自体は private なので、間接的に送信して影響を確認するなどで代用するが、
        // 今回は audio_channels_temp が None になることのみを確認する。
    }

    #[test]
    fn test_process_active_notes() {
        use ringbuf::traits::Consumer;
        let mut app = OpenDawApp::default();
        app.state.add_track("Synth Track");
        app.state.tracks[0].synth.is_enabled = true;

        app.state.active_sequence.add_note(69, 100, 0.0, 1.0); // A4 = 440Hz

        // Not playing
        app.state.is_playing = false;
        app.state.playhead_pos = 0.5;
        app.process_active_notes();

        let mut received = false;
        if let Some(audio_channels) = &mut app.audio_channels_temp {
            while let Some(msg) = audio_channels.0.try_pop() {
                if let crate::engine::channel::UiToAudioMsg::ActiveNotes(id, _freqs, count) = msg {
                    assert_eq!(id, app.state.tracks[0].id);
                    assert_eq!(count, 0);
                    received = true;
                }
            }
        }
        assert!(received);

        // Playing
        app.state.is_playing = true;
        app.state.playhead_pos = 0.5;
        app.process_active_notes();

        let mut received_playing = false;
        if let Some(audio_channels) = &mut app.audio_channels_temp {
            while let Some(msg) = audio_channels.0.try_pop() {
                if let crate::engine::channel::UiToAudioMsg::ActiveNotes(id, freqs, count) = msg {
                    assert_eq!(id, app.state.tracks[0].id);
                    assert_eq!(count, 1);
                    assert_eq!(freqs[0], 440.0);
                    received_playing = true;
                }
            }
        }
        assert!(received_playing);
    }

    #[test]
    fn test_app_poll_mcp_commands() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = OpenDawApp::default();
        let (tx, rx) = crate::mcp::channel::create_mcp_channel(10);
        app.mcp_receiver = Some(rx);
        // Play command
        tx.send(crate::mcp::channel::McpCommand::Play)?;
        crate::mcp::handler::poll_mcp_commands(
            &app.mcp_receiver,
            &mut app.state,
            &mut app.ui_channels,
            &mut app.selected_track_id,
            &mut app.selected_clip_id,
        );
        assert!(app.state.is_playing);

        // Stop command
        tx.send(crate::mcp::channel::McpCommand::Stop)?;
        crate::mcp::handler::poll_mcp_commands(
            &app.mcp_receiver,
            &mut app.state,
            &mut app.ui_channels,
            &mut app.selected_track_id,
            &mut app.selected_clip_id,
        );
        assert!(!app.state.is_playing);

        // AddTrack command
        assert_eq!(app.state.tracks.len(), 0);
        tx.send(crate::mcp::channel::McpCommand::AddTrack)?;
        crate::mcp::handler::poll_mcp_commands(
            &app.mcp_receiver,
            &mut app.state,
            &mut app.ui_channels,
            &mut app.selected_track_id,
            &mut app.selected_clip_id,
        );
        assert_eq!(app.state.tracks.len(), 1);
        assert_eq!(app.state.tracks[0].name, "New Track (MCP)");

        // RemoveTrack command
        let track_id = app.state.tracks[0].id;
        tx.send(crate::mcp::channel::McpCommand::RemoveTrack(track_id))?;
        crate::mcp::handler::poll_mcp_commands(
            &app.mcp_receiver,
            &mut app.state,
            &mut app.ui_channels,
            &mut app.selected_track_id,
            &mut app.selected_clip_id,
        );
        assert_eq!(app.state.tracks.len(), 0);
        Ok(())
    }
}

#[cfg(test)]
mod tests_synth {
    use super::*;

    #[test]
    fn test_poll_synth_params() {
        use ringbuf::traits::Consumer;
        let mut app = OpenDawApp::default();
        app.state.add_track("Synth Track");
        app.state.tracks[0].synth.is_enabled = true;
        app.state.tracks[0].synth.waveform = crate::state::track::Waveform::Square;

        app.poll_synth_params();

        let mut received = false;
        if let Some(audio_channels) = &mut app.audio_channels_temp {
            while let Some(msg) = audio_channels.0.try_pop() {
                if let crate::engine::channel::UiToAudioMsg::UpdateSynthParams(
                    id,
                    waveform,
                    _adsr,
                ) = msg
                {
                    assert_eq!(id, app.state.tracks[0].id);
                    assert_eq!(waveform, crate::engine::synth::Waveform::Square);
                    received = true;
                }
            }
        }
        assert!(received);
    }
    #[test]
    fn test_poll_effect_params() {
        use ringbuf::traits::Consumer;
        let mut app = OpenDawApp::default();
        app.state.add_track("Effect Track");

        let delay_effect = crate::state::track::EffectType::Delay {
            time_ms: 300.0,
            feedback: 0.3,
            mix: 0.5,
        };
        app.state.tracks[0].add_effect(crate::state::track::EffectSetting::new(1, delay_effect));

        app.poll_effect_params();

        let mut received = false;
        if let Some(audio_channels) = &mut app.audio_channels_temp {
            while let Some(msg) = audio_channels.0.try_pop() {
                if let crate::engine::channel::UiToAudioMsg::UpdateEffectParams(
                    track_id,
                    effect_id,
                    params,
                ) = msg
                {
                    assert_eq!(track_id, app.state.tracks[0].id);
                    assert_eq!(effect_id, 1);
                    if let crate::engine::channel::EffectParams::Delay {
                        time_ms,
                        feedback,
                        mix,
                    } = params
                    {
                        assert_eq!(time_ms, 300.0);
                        assert_eq!(feedback, 0.3);
                        assert_eq!(mix, 0.5);
                        received = true;
                    }
                }
            }
        }
        assert!(received);

        // Test that second poll without changes doesn't send anything
        app.poll_effect_params();
        let mut received_second = false;
        if let Some(audio_channels) = &mut app.audio_channels_temp {
            while let Some(_) = audio_channels.0.try_pop() {
                received_second = true;
            }
        }
        assert!(!received_second);
    }
}
