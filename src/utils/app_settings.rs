use crate::parameters::ConstantParam;
use crate::utils::FileManager;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};
use std::fs::{self};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum AppMode {
    Presentation,
    Edit,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct AppSettings {
    pub framerate: f64,
    pub view_window_size: (u32, u32),
    pub app_mode: AppMode,
    pub control_windows_opacity: ConstantParam<u8>,
    pub fully_transparent: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            framerate: 60.0,
            view_window_size: (1000, 1000),
            app_mode: AppMode::Edit,
            control_windows_opacity: ConstantParam::new(200, 1, 255, "Opacity", "opactity"),
            fully_transparent: false,
        }
    }
}

impl AppSettings {
    pub const APP_NAME: &str = "Welosa2";

    pub fn load_or_default() -> Self {
        FileManager::app_settings_path()
            .and_then(|p| fs::File::open(p).map_err(|_| ()))
            .and_then(|f| serde_json::from_reader(f).map_err(|_| ()))
            .unwrap_or_default()
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

        self.control_windows_opacity.to_slider(ui);

        ui.checkbox(&mut self.fully_transparent, "transparent");

        if ui.button("Save Settings").clicked() {
            let _ = FileManager::save_app_settings(self);
        }
        changed
    }
}
