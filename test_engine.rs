use timestretch::engine::{Engine, EngineConfig, EngineProfile, EngineProcessor, EngineController, source::SourceProducer};
use timestretch::core::types::StretchParams;

fn main() {
    let mut config = EngineConfig::default();
    config.channels = 2;
    config.sample_rate = 44100;
    config.profile = EngineProfile::Keylock;

    let handles = Engine::build(config).unwrap();
    let controller = handles.controller;
    let mut processor = handles.processor;
    let mut source = handles.source;
}
