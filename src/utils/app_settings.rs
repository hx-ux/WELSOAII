use crate::utils::{FileManager, Severity};
use crate::{parameters::ConstantParam, utils::AppLogger};
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

    pub fn load_or_default(logger: &AppLogger) -> Self {
        let path = match FileManager::app_settings_path() {
            Ok(p) => p,
            Err(_) => {
                logger.log(
                    "Warning: Could not determine app settings path. Using defaults.",
                    Severity::Error,
                );
                return Self::default();
            }
        };

        let file = match fs::File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                logger.log(
                    &format!(
                        "No existing settings found at {:?} ({}). Using defaults.",
                        path, e
                    ),
                    Severity::Error,
                );
                return Self::default();
            }
        };

        match serde_json::from_reader(file) {
            Ok(settings) => {
                logger.log(
                    "Global settings successfully loaded from file.",
                    Severity::Info,
                );
                settings
            }
            Err(e) => {
                logger.log(
                    &format!(
                        "Error parsing settings file: {}. Falling back to defaults.",
                        e
                    ),
                    Severity::Error,
                );
                Self::default()
            }
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, logger: &AppLogger) -> bool {
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
            match FileManager::save_app_settings(self) {
                Ok(_) => {
                    logger.log("Saved Settings file", Severity::Info);
                }
                Err(err) => {
                    logger.log(
                        &format!("Error saving settings file {}", err),
                        Severity::Error,
                    );
                }
            }
        }
        changed
    }
}
