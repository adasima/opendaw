#[derive(Debug, Clone, PartialEq)]
pub enum Clip {
    Audio {
        file_path: String,
        length_beats: f64,
    },
    Midi {
        notes_count: usize,
        length_beats: f64,
    },
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ClipSlotState {
    #[default]
    Empty,
    Stopped,
    Playing,
    Queued,
}

#[derive(Debug, Clone)]
pub struct ClipSlot {
    pub clip: Option<Clip>,
    pub state: ClipSlotState,
}

impl ClipSlot {
    pub fn new() -> Self {
        Self {
            clip: None,
            state: ClipSlotState::Empty,
        }
    }
}

impl Default for ClipSlot {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub name: String,
    pub slots: Vec<ClipSlot>,
}

impl Scene {
    pub fn new(name: String, track_count: usize) -> Self {
        Self {
            name,
            slots: vec![ClipSlot::new(); track_count],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Session {
    pub scenes: Vec<Scene>,
    pub track_count: usize,
}

impl Session {
    pub fn new(track_count: usize) -> Self {
        Self {
            scenes: Vec::new(),
            track_count,
        }
    }

    pub fn add_scene(&mut self, name: String) {
        self.scenes.push(Scene::new(name, self.track_count));
    }

    pub fn play_scene(&mut self, scene_index: usize) {
        if let Some(scene) = self.scenes.get_mut(scene_index) {
            for slot in &mut scene.slots {
                if slot.clip.is_some() {
                    slot.state = ClipSlotState::Queued;
                }
            }
        }
    }
}
