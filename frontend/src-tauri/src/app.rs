use crate::engine::EngineHandle;
use std::sync::Arc;

pub struct AppState {
    pub engine: Arc<EngineHandle>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_save_load_project_serialize() {
        let mut state = crate::state::ProjectState::default();
        state.bpm = 130.0;
        state.master_volume = 0.5;

        let track = crate::state::Track::new(1, "Test Track");
        state.tracks.push(std::sync::Arc::new(track));

        let json = serde_json::to_string(&state).expect("Failed to serialize state");
        let loaded: crate::state::ProjectState =
            serde_json::from_str(&json).expect("Failed to deserialize state");

        assert_eq!(loaded.bpm, 130.0);
        assert_eq!(loaded.master_volume, 0.5);
        assert_eq!(loaded.tracks.len(), 1);
        assert_eq!(loaded.tracks[0].name, "Test Track");
    }
}
