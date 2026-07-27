use timestretch::engine::{Engine, EngineConfig, EngineProfile, EngineProcessor, EngineController, source::SourceProducer};

fn main() {
    let config = EngineConfig {
        channels: 1,
        sample_rate: 44100,
        profile: EngineProfile::Keylock,
        ..EngineConfig::default()
    };
    let handles = Engine::build(config).unwrap();
    let controller = handles.controller;
    let mut processor = handles.processor;
    let mut source = handles.source;

    let input = vec![1.0f32; 1000];
    source.push(&input);
    controller.set_tempo_rate(1.5);

    // Check underrun
    let mut out = vec![0.0f32; 100];
    processor.process(&mut out);
    println!("Processed 100 frames, out[0]: {}", out[0]);
    println!("Underrun: {}", controller.underrun_frames());
}
