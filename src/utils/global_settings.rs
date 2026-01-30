use anyhow::Result;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use std::fs::{self};
use std::path::PathBuf;

use crate::animator::animation_type::{AnimationType};
use crate::animator::animator_structs::AnimationParam;
use crate::utils::PathManager;

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
    pub window_opacity: AnimationParam<u8>, 
}

impl GlobalSettings {
    pub const APP_NAME: &str = "Welosa2";
    pub const EFFECTS_FOLDER: &str = "Effects";
    pub const DEVICES_FOLDER: &str = "Devices";

    pub fn new() -> Self {
        Self {
            framerate: 60.0,
            view_window_size: (1000, 1000),
            app_mode: AppMode::Edit,
            window_opacity: AnimationParam::new(200, 1, 255, "opacity"),
        }
    }

    pub fn create_settings_folder() -> Result<bool> {
        // todo use result
        let z: PathBuf = PathManager::get_preset_path();
        fs::create_dir_all(&z).unwrap();

        for animation in AnimationType::iter() {
            let e_path = z.join(format!("{}", animation) );
            fs::create_dir_all(&e_path).unwrap();
        }

        Ok(true)
    }

    pub fn load_or_default() -> Self {
        match Self::load(PathManager::settings_path()) {
            Ok(c) => c,
            Err(_) => Self::new(),
        }
    }

    // todo refactor
    pub fn load(path: PathBuf) -> Result<Self> {
        let file = fs::File::open(path)?;
        let t = serde_json::from_reader(file)?;
        Ok(t)
    }

    // todo refactor
    pub fn save(&self) -> Result<bool> {
        nannou::io::save_to_json(PathManager::settings_path(), self)?;
        Ok(true)
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
        
        self.window_opacity.to_slider(ui);

        if ui.button("Save Settings").clicked() {
            if let Err(e) = self.save() {
                eprintln!("Failed to save settings: {}", e);
            } else {
                println!("Settings saved successfully!");
            }
        }

        changed
    }
}
