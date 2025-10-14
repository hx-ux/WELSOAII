pub enum AppMode {
    Presentation,
    Edit,
    Preview,
}

pub struct GlobalSettings {
    pub framerate: f64,
    pub view_window_size: (u32, u32),
    pub settings_window_size: (u32, u32),
}

impl GlobalSettings {
    pub fn default() -> Self {
        Self {
            framerate: 60.0,
            view_window_size: (1000, 1000),
            settings_window_size: (800, 600),
        }
    }
}
