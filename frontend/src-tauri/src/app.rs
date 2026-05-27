use std::sync::Arc;
use crate::engine::EngineHandle;

pub struct AppState {
    pub engine: Arc<EngineHandle>,
}
