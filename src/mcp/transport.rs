//! MCPトランスポートコントロールハンドラ。
//!
//! AIエージェントなどからMCPプロトコル経由で
//! トランスポート（再生・停止・ループ切り替えなど）を制御するためのモジュールです。

/// トランスポートコントロールのハンドラ
#[derive(Default)]
pub struct TransportHandler {
    // 将来的にUiChannelsやSharedStateなどを持つ
}

impl TransportHandler {
    /// 新しい `TransportHandler` インスタンスを作成します。
    pub fn new() -> Self {
        Self::default()
    }

    /// 再生を開始します。
    pub async fn play(&self) -> Result<(), String> {
        // Phase 7タスク仕様に基づき、今回はMCP側のスケルトン実装に留めます。
        // 将来的にtokioチャンネルを使用してUIスレッド（app.rs）と連携します。
        log::info!("MCP: Playback started");
        Ok(())
    }

    /// 再生を停止します。
    pub async fn stop(&self) -> Result<(), String> {
        // 将来的にUIスレッドに停止メッセージを送信します
        log::info!("MCP: Playback stopped");
        Ok(())
    }

    /// ループ再生の有効/無効を切り替えます。
    pub async fn toggle_loop(&self) -> Result<(), String> {
        // 将来的にUIスレッドにループ切り替えメッセージを送信します
        log::info!("MCP: Loop toggled");
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
