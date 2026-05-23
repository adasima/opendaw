#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginFormat {
    Vst3,
    Clap,
}

#[derive(Debug, Clone)]
pub struct PluginDescriptor {
    pub name: String,
    pub vendor: String,
    pub version: String,
    pub format: PluginFormat,
}

pub trait PluginInstance: Send + Sync {
    fn process_audio(&mut self, inputs: &[&[f32]], outputs: &mut [&mut [f32]]);
    fn get_parameter(&self, index: u32) -> f32;
    fn set_parameter(&mut self, index: u32, value: f32);
}

pub struct Host {
    plugins: Vec<Box<dyn PluginInstance>>,
}

impl Host {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn scan_plugins(&self) -> Vec<PluginDescriptor> {
        // スタブ
        vec![]
    }

    pub fn load_plugin(&mut self, _descriptor: &PluginDescriptor) -> Result<(), String> {
        // スタブ
        Err("Not implemented".to_string())
    }
}

impl Default for Host {
    fn default() -> Self {
        Self::new()
    }
}
