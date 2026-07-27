//! MCPサーバーの初期化およびルーティングを行うモジュール。
//! Tokioランタイム上で非同期に動作します。

pub mod channel;
pub mod handler;
pub mod tracks;
pub mod transport;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// AIエージェントと通信するためのMCP(Model Context Protocol)サーバー。
#[derive(Default)]
pub struct McpServer {
    pub transport_handler: transport::TransportHandler,
    pub tracks_handler: tracks::TracksHandler,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct RpcRequest {
    #[allow(dead_code)]
    jsonrpc: String,
    method: String,
    params: Option<Value>,
    id: Option<Value>,
}

#[derive(Serialize, Debug)]
#[allow(dead_code)]
struct RpcResponse {
    #[allow(dead_code)]
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<RpcError>,
    id: Option<Value>,
}

#[derive(Serialize, Debug)]
#[allow(dead_code)]
struct RpcError {
    code: i32,
    message: String,
}

impl McpServer {
    /// 新しい `McpServer` インスタンスを作成します。
    pub fn new() -> Self {
        Self::default()
    }

    /// MCPサーバーを起動し、リクエストの待機を開始します。
    /// このメソッドは非同期ランタイム(Tokio)上で実行されることを想定しています。
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn run(&self) {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

        println!("MCP Server is starting...");

        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let req: Result<RpcRequest, _> = serde_json::from_str(line.trim());
                    match req {
                        Ok(request) => {
                            let mut response = RpcResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: None,
                                id: request.id.clone(),
                            };

                            match request.method.as_str() {
                                "play" => {
                                    if let Err(e) = self.transport_handler.play().await {
                                        response.error = Some(RpcError {
                                            code: -32603,
                                            message: e,
                                        });
                                    } else {
                                        response.result = Some(Value::Null);
                                    }
                                }
                                "stop" => {
                                    if let Err(e) = self.transport_handler.stop().await {
                                        response.error = Some(RpcError {
                                            code: -32603,
                                            message: e,
                                        });
                                    } else {
                                        response.result = Some(Value::Null);
                                    }
                                }
                                "toggle_loop" => {
                                    if let Err(e) = self.transport_handler.toggle_loop().await {
                                        response.error = Some(RpcError {
                                            code: -32603,
                                            message: e,
                                        });
                                    } else {
                                        response.result = Some(Value::Null);
                                    }
                                }
                                "add_track" => {
                                    if let Err(e) = self.tracks_handler.add_track().await {
                                        response.error = Some(RpcError {
                                            code: -32603,
                                            message: e,
                                        });
                                    } else {
                                        response.result = Some(Value::Null);
                                    }
                                }
                                "remove_track" => {
                                    if let Some(params) = request.params {
                                        if let Some(track_id) =
                                            params.get("track_id").and_then(|v| v.as_u64())
                                        {
                                            if let Err(e) = self
                                                .tracks_handler
                                                .remove_track(track_id as usize)
                                                .await
                                            {
                                                response.error = Some(RpcError {
                                                    code: -32603,
                                                    message: e,
                                                });
                                            } else {
                                                response.result = Some(Value::Null);
                                            }
                                        } else {
                                            response.error = Some(RpcError {
                                                code: -32602,
                                                message:
                                                    "Invalid params: missing or invalid track_id"
                                                        .to_string(),
                                            });
                                        }
                                    } else {
                                        response.error = Some(RpcError {
                                            code: -32602,
                                            message: "Invalid params".to_string(),
                                        });
                                    }
                                }
                                _ => {
                                    response.error = Some(RpcError {
                                        code: -32601,
                                        message: "Method not found".to_string(),
                                    });
                                }
                            }

                            if let Ok(res_str) = serde_json::to_string(&response) {
                                let _ = stdout.write_all(res_str.as_bytes()).await;
                                let _ = stdout.write_all(b"\n").await;
                                let _ = stdout.flush().await;
                            }
                        }
                        Err(_) => {
                            let response = RpcResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(RpcError {
                                    code: -32700,
                                    message: "Parse error".to_string(),
                                }),
                                id: None,
                            };
                            if let Ok(res_str) = serde_json::to_string(&response) {
                                let _ = stdout.write_all(res_str.as_bytes()).await;
                                let _ = stdout.write_all(b"\n").await;
                                let _ = stdout.flush().await;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading stdin: {}", e);
                    break;
                }
            }
        }
    }

    /// MCPサーバーを起動し、リクエストの待機を開始します。
    #[cfg(target_arch = "wasm32")]
    pub async fn run(&self) {
        println!("MCP Server is starting (WASM)...");
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
