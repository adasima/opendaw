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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_manager_undo_redo() {
        let mut history = HistoryManager::new();
        let initial_state = ProjectState::default();

        let mut state1 = ProjectState::default();
        state1.bpm = 130.0;

        let mut state2 = ProjectState::default();
        state2.bpm = 140.0;

        history.save_snapshot(&initial_state);
        history.save_snapshot(&state1);

        // Undo 1
        if let Some(prev_state) = history.undo(&state2) {
            assert_eq!(prev_state.bpm, 130.0);

            // Undo 2
            if let Some(initial) = history.undo(&prev_state) {
                assert_eq!(initial.bpm, 120.0);

                // Redo 1
                if let Some(redone1) = history.redo(&initial) {
                    assert_eq!(redone1.bpm, 130.0);

                    // Redo 2
                    if let Some(redone2) = history.redo(&redone1) {
                        assert_eq!(redone2.bpm, 140.0);
                    } else {
                        panic!("Expected second redo state");
                    }
                } else {
                    panic!("Expected first redo state");
                }
            } else {
                panic!("Expected initial state on second undo");
            }
        } else {
            panic!("Expected previous state on first undo");
        }
    }
}
