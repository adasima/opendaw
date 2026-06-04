//! MCPサーバーからのコマンドを受信して状態を更新します。

use crate::engine::channel::UiChannels;
use crate::engine::channel::UiToAudioMsg;
use crate::mcp::channel::{McpCommand, McpCommandReceiver};
use crate::state::DawState;
use ringbuf::traits::Producer;

/// MCPコマンドをポーリングして状態を更新します。
pub fn poll_mcp_commands(
    mcp_receiver: &Option<McpCommandReceiver>,
    state: &mut DawState,
    ui_channels: &mut Option<UiChannels>,
    selected_track_id: &mut Option<usize>,
    selected_clip_id: &mut Option<usize>,
) {
    if let Some(mcp_receiver) = mcp_receiver {
        while let Ok(cmd) = mcp_receiver.try_recv() {
            match cmd {
                McpCommand::Play => {
                    state.is_playing = true;
                    if let Some(ui_channels) = ui_channels {
                        let _ = ui_channels.0.try_push(UiToAudioMsg::SetPlaying(true));
                    }
                }
                McpCommand::Stop => {
                    state.stop_playback();
                    if let Some(ui_channels) = ui_channels {
                        let _ = ui_channels.0.try_push(UiToAudioMsg::SetPlaying(false));
                    }
                }
                McpCommand::ToggleLoop => {
                    state.toggle_loop();
                }
                McpCommand::AddTrack => {
                    state.add_track("New Track (MCP)");
                }
                McpCommand::RemoveTrack(id) => {
                    state.remove_track(id);
                }
                McpCommand::SelectTrack(id_opt) => {
                    *selected_track_id = id_opt;
                    *selected_clip_id = None;
                }
                McpCommand::ToggleMute(id) => {
                    if let Some(track) = state.tracks.iter_mut().find(|t| t.id == id) {
                        track.toggle_mute();
                    }
                }
                McpCommand::ToggleSolo(id) => {
                    if let Some(track) = state.tracks.iter_mut().find(|t| t.id == id) {
                        track.toggle_solo();
                    }
                }
                McpCommand::ToggleRecordArm(id) => {
                    if let Some(track) = state.tracks.iter_mut().find(|t| t.id == id) {
                        track.toggle_record_arm();
                    }
                }
                McpCommand::ToggleGlobalRecord => {}
                McpCommand::RequestTrackJson => {}
            }
        }
        #[cfg(target_arch = "wasm32")]
        crate::request_repaint();
    }
}
