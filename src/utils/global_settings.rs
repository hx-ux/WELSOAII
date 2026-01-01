use nannou_egui::egui;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum AppMode {
    Presentation,
    Edit,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct GlobalSettings {
    pub framerate: f64,
    pub view_window_size: (u32, u32),
    pub app_mode: AppMode,
}

impl GlobalSettings {
    pub fn load_or_default(path: &str) -> Self {
        let _result = std::fs::read_to_string(path);

        if !path.is_empty() {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(parsed) = serde_json::from_str::<GlobalSettings>(&content) {
                    return parsed;
                } else {
                    eprintln!(
                        "Warning: failed to parse settings JSON at '{}', using defaults",
                        path
                    );
                }
            } else {
                println!("No settings file found at '{}', using defaults", path);
            }
        }

        // defaults
        Self {
            framerate: 60.0,
            view_window_size: (1000, 1000),
            app_mode: AppMode::Edit,
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
        ui.separator();
        ui.horizontal(|ui| {
            changed |= ui
                .radio_value(&mut self.app_mode, AppMode::Presentation, "Presentation")
                .changed();
            changed |= ui
                .radio_value(&mut self.app_mode, AppMode::Edit, "Edit")
                .changed();
        });

        changed
    }

    pub fn app_name() -> String {
        String::from("WELOSA II")
    }
}