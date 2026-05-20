//! MCPとUI間の通信チャンネル。
use crossbeam_channel::{Receiver, Sender, bounded};

/// MCPサーバーからUIへ送信されるコマンド。
#[derive(Debug, Clone, PartialEq)]
pub enum McpCommand {
    Play,
    Stop,
    ToggleLoop,
    AddTrack,
    RemoveTrack(usize),
}

/// MCPコマンドの送信側
pub type McpCommandSender = Sender<McpCommand>;
/// MCPコマンドの受信側
pub type McpCommandReceiver = Receiver<McpCommand>;

/// MCPコマンドチャンネルを生成します。
pub fn create_mcp_channel(capacity: usize) -> (McpCommandSender, McpCommandReceiver) {
    bounded(capacity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_command_channel() -> Result<(), Box<dyn std::error::Error>> {
        let (tx, rx) = create_mcp_channel(10);
        tx.send(McpCommand::Play)?;
        assert_eq!(rx.recv()?, McpCommand::Play);
        Ok(())
    }
}
