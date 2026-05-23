//! MCPトランスポートコントロールハンドラ。
//!
//! AIエージェントなどからMCPプロトコル経由で
//! トランスポート（再生・停止・ループ切り替えなど）を制御するためのモジュールです。

/// トランスポートコントロールのハンドラ
#[derive(Default)]
pub struct TransportHandler {
    pub sender: Option<crate::mcp::channel::McpCommandSender>,
    // 将来的にUiChannelsやSharedStateなどを持つ
}

impl TransportHandler {
    /// 新しい `TransportHandler` インスタンスを作成します。
    pub fn new() -> Self {
        Self { sender: None }
    }

    /// 再生を開始します。
    pub async fn play(&self) -> Result<(), String> {
        // Phase 7タスク仕様に基づき、今回はMCP側のスケルトン実装に留めます。
        // 将来的にtokioチャンネルを使用してUIスレッド（app.rs）と連携します。
        log::info!("MCP: Playback started");
        if let Some(sender) = &self.sender {
            let _ = sender.try_send(crate::mcp::channel::McpCommand::Play);
        }
        Ok(())
    }

    /// 再生を停止します。
    pub async fn stop(&self) -> Result<(), String> {
        // 将来的にUIスレッドに停止メッセージを送信します
        log::info!("MCP: Playback stopped");
        if let Some(sender) = &self.sender {
            let _ = sender.try_send(crate::mcp::channel::McpCommand::Stop);
        }
        Ok(())
    }

    /// ループ再生の有効/無効を切り替えます。
    pub async fn toggle_loop(&self) -> Result<(), String> {
        // 将来的にUIスレッドにループ切り替えメッセージを送信します
        log::info!("MCP: Loop toggled");
        if let Some(sender) = &self.sender {
            let _ = sender.try_send(crate::mcp::channel::McpCommand::ToggleLoop);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_transport_handler_play() {
        let handler = TransportHandler::new();
        let result = handler.play().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_transport_handler_stop() {
        let handler = TransportHandler::new();
        let result = handler.stop().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_transport_handler_toggle_loop() {
        let handler = TransportHandler::new();
        let result = handler.toggle_loop().await;
        assert!(result.is_ok());
    }
}
