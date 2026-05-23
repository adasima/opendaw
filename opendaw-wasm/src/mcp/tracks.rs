//! MCPトラック操作ハンドラ。
//!
//! AIエージェントなどからMCPプロトコル経由で
//! トラックの追加や削除などの操作を行うためのモジュールです。

/// トラック操作のハンドラ
#[derive(Default)]
pub struct TracksHandler {
    pub sender: Option<crate::mcp::channel::McpCommandSender>,
    // 将来的にUiChannelsやSharedStateなどを持つ
}

impl TracksHandler {
    /// 新しい `TracksHandler` インスタンスを作成します。
    pub fn new() -> Self {
        Self { sender: None }
    }

    /// 新しいトラックを追加します。
    pub async fn add_track(&self) -> Result<(), String> {
        // Phase 7タスク仕様に基づき、今回はMCP側のスケルトン実装に留めます。
        // 将来的にtokioチャンネルを使用してUIスレッド（app.rs）と連携します。
        log::info!("MCP: Track added");
        if let Some(sender) = &self.sender {
            let _ = sender.try_send(crate::mcp::channel::McpCommand::AddTrack);
        }
        Ok(())
    }

    /// 指定したトラックを削除します。
    pub async fn remove_track(&self, track_id: usize) -> Result<(), String> {
        // 将来的にUIスレッドにトラック削除メッセージを送信します
        log::info!("MCP: Track removed");
        if let Some(sender) = &self.sender {
            let _ = sender.try_send(crate::mcp::channel::McpCommand::RemoveTrack(track_id));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tracks_handler_add_track() {
        let handler = TracksHandler::new();
        let result = handler.add_track().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_tracks_handler_remove_track() {
        let handler = TracksHandler::new();
        let result = handler.remove_track(0).await;
        assert!(result.is_ok());
    }
}
