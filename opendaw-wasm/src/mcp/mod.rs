//! MCPサーバーの初期化およびルーティングを行うモジュール。
//! Tokioランタイム上で非同期に動作します。

pub mod channel;
pub mod tracks;
pub mod transport;

/// AIエージェントと通信するためのMCP(Model Context Protocol)サーバー。
#[derive(Default)]
pub struct McpServer {
    pub transport_handler: transport::TransportHandler,
    pub tracks_handler: tracks::TracksHandler,
}

impl McpServer {
    /// 新しい `McpServer` インスタンスを作成します。
    pub fn new() -> Self {
        Self::default()
    }

    /// MCPサーバーを起動し、リクエストの待機を開始します。
    /// このメソッドは非同期ランタイム(Tokio)上で実行されることを想定しています。
    pub async fn run(&self) {
        // TODO: MCPプロトコルのハンドリングやトランスポート層の実装を追加する
        println!("MCP Server is starting...");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_server_creation() {
        let _server = McpServer::new();
        // 構造体が正しく初期化できることを確認
        assert!(true);
    }
}


pub mod handler;
