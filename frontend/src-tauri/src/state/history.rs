use crate::state::ProjectState;
use serde::{Deserialize, Serialize};

/// 履歴管理（Undo/Redo）を担う構造体

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryManager {
    undo_stack: std::collections::VecDeque<ProjectState>,
    redo_stack: std::collections::VecDeque<ProjectState>,
    max_history: usize,
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self {
            undo_stack: std::collections::VecDeque::new(),
            redo_stack: std::collections::VecDeque::new(),
            max_history: 50, // 最大履歴数
        }
    }
}

impl HistoryManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// 現在の状態をスナップショットとして保存する
    pub fn save_snapshot(&mut self, state: &ProjectState) {

        if let Some(last) = self.undo_stack.back() {
            // Compare the JSON representations to check for equality since ProjectState does not derive PartialEq
            if let (Ok(last_json), Ok(new_json)) = (serde_json::to_string(last), serde_json::to_string(state)) {
                if last_json == new_json {
                    return;
                }
            }
        }


        // 履歴が上限に達している場合は古いものから削除
        if self.undo_stack.len() >= self.max_history {
            self.undo_stack.pop_front();
        }

        // ProjectStateをCloneして保存
        self.undo_stack.push_back(state.clone());

        // 新しい操作が行われたらRedoスタックはクリアする
        self.redo_stack.clear();
    }

    /// Undo (取り消し) を実行する
    pub fn undo(&mut self, current_state: &ProjectState) -> Option<ProjectState> {
        if let Some(previous_state) = self.undo_stack.pop_back() {
            // 現在の状態をRedoスタックに退避
            self.redo_stack.push_back(current_state.clone());
            Some(previous_state)
        } else {
            None
        }
    }

    /// Redo (やり直し) を実行する
    pub fn redo(&mut self, current_state: &ProjectState) -> Option<ProjectState> {
        if let Some(next_state) = self.redo_stack.pop_back() {
            // 現在の状態をUndoスタックに退避
            self.undo_stack.push_back(current_state.clone());
            Some(next_state)
        } else {
            None
        }
    }

    /// スタックをクリアする
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}
