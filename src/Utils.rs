use nannou_egui::egui;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum AppMode {
    Presentation,
    Edit,
    Preview,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct GlobalSettings {
    pub framerate: f64,
    pub view_window_size: (u32, u32),
    pub settings_window_size: (u32, u32),
    pub app_mode: AppMode,
}

impl GlobalSettings {
    pub fn load_or_default(path: &str) -> Self {
        let result = std::fs::read_to_string(path);

        Self {
            framerate: 60.0,
            view_window_size: (1000, 1000),
            settings_window_size: (800, 400),
            app_mode: AppMode::Preview,
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json_string)?;
        println!("Successfully saved settings to '{}'", path);
        Ok(())
    }
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.label("Framerate");
        let framerate_response = ui.add(egui::Slider::new(&mut self.framerate, 1.0..=60.0));
        ui.separator();
        ui.horizontal(|ui| {
  
            changed |= ui
                .radio_value(&mut self.app_mode, AppMode::Presentation, "Presentation")
                .changed();
            changed |= ui
                .radio_value(&mut self.app_mode, AppMode::Edit, "Edit")
                .changed();
            changed |= ui
                .radio_value(&mut self.app_mode, AppMode::Preview, "Preview")
                .changed();
        });

        if framerate_response.changed() {
            changed = true;
        }

        changed
    }
}
