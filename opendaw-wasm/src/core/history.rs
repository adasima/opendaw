use std::fmt::Debug;

/// Represents an operation that can be executed and undone.
pub trait Command: Debug + Send + Sync {
    /// Executes the command.
    fn execute(&mut self) -> Result<(), String>;

    /// Undoes the command.
    fn undo(&mut self) -> Result<(), String>;

    /// Returns the name of the command for display in the UI (e.g., "Move Clip").
    fn name(&self) -> &str;
}

/// Manages a history of executed commands to provide Undo/Redo functionality.
pub struct HistoryManager {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
    /// Optional limit on the number of history items to keep.
    max_history: Option<usize>,
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoryManager {
    /// Creates a new, empty HistoryManager.
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history: None,
        }
    }

    /// Creates a new HistoryManager with a maximum history size limit.
    pub fn with_limit(limit: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history: Some(limit),
        }
    }

    /// Executes a new command, adding it to the undo stack and clearing the redo stack.
    pub fn execute_command(&mut self, mut command: Box<dyn Command>) -> Result<(), String> {
        command.execute()?;
        self.undo_stack.push(command);
        self.redo_stack.clear(); // Executing a new command invalidates redo history

        if let Some(limit) = self.max_history {
            while self.undo_stack.len() > limit {
                self.undo_stack.remove(0);
            }
        }

        Ok(())
    }

    /// Undoes the last executed command, moving it to the redo stack.
    pub fn undo(&mut self) -> Result<(), String> {
        if let Some(mut command) = self.undo_stack.pop() {
            command.undo()?;
            self.redo_stack.push(command);
            Ok(())
        } else {
            Err("Nothing to undo".to_string())
        }
    }

    /// Redoes the last undone command, moving it back to the undo stack.
    pub fn redo(&mut self) -> Result<(), String> {
        if let Some(mut command) = self.redo_stack.pop() {
            command.execute()?;
            self.undo_stack.push(command);
            Ok(())
        } else {
            Err("Nothing to redo".to_string())
        }
    }

    /// Returns true if there are commands available to undo.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Returns true if there are commands available to redo.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clears the history.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

// --- Dummy implementation for demonstration purposes ---

#[derive(Debug)]
pub struct MoveClipCommand {
    clip_id: u32,
    old_position: f64,
    new_position: f64,
}

impl MoveClipCommand {
    pub fn new(clip_id: u32, old_position: f64, new_position: f64) -> Self {
        Self {
            clip_id,
            old_position,
            new_position,
        }
    }
}

impl Command for MoveClipCommand {
    fn execute(&mut self) -> Result<(), String> {
        // In a real implementation, you would call methods on your domain models.
        println!(
            "Executing MoveClipCommand: moving clip {} from {} to {}",
            self.clip_id, self.old_position, self.new_position
        );
        Ok(())
    }

    fn undo(&mut self) -> Result<(), String> {
        println!(
            "Undoing MoveClipCommand: moving clip {} back to {}",
            self.clip_id, self.old_position
        );
        Ok(())
    }

    fn name(&self) -> &str {
        "Move Clip"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_manager() {
        let mut history = HistoryManager::new();
        let cmd1 = Box::new(MoveClipCommand::new(1, 0.0, 1.0));
        let cmd2 = Box::new(MoveClipCommand::new(1, 1.0, 2.0));

        assert!(!history.can_undo());
        assert!(!history.can_redo());

        history
            .execute_command(cmd1)
            .expect("Failed to execute cmd1");
        assert!(history.can_undo());

        history
            .execute_command(cmd2)
            .expect("Failed to execute cmd2");

        history.undo().expect("Failed to undo cmd2"); // undo cmd2
        assert!(history.can_redo());

        history.redo().expect("Failed to redo cmd2"); // redo cmd2
        assert!(history.can_undo());
    }
}
