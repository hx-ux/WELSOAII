use anyhow::Result;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::animator::animation_type::{AnimationType, ModeHelper};

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
    pub const APP_NAME: &str = "Welosa2";
    const EFFECTS_FOLDER: &str = "Effects";

    pub fn get_root_path() -> PathBuf {
        let mut doc_dir = dirs::document_dir().unwrap();
        doc_dir.push(Self::APP_NAME);
        fs::create_dir_all(&doc_dir).unwrap();
        doc_dir
    }

    pub fn get_preset_path() -> PathBuf {
        let mut doc_dir = dirs::document_dir().unwrap();
        doc_dir.push(Self::APP_NAME);
        doc_dir.push(Self::EFFECTS_FOLDER);

        fs::create_dir_all(&doc_dir).unwrap();
        doc_dir
    }

    pub fn new() -> Self {
        Self {
            framerate: 60.0,
            view_window_size: (1000, 1000),
            app_mode: AppMode::Edit,
        }
    }

    pub fn create_settings_folder() -> Result<bool> {
        // todo use result
        let z: PathBuf = Self::get_preset_path();
        fs::create_dir_all(&z).unwrap();

        for animation in AnimationType::iterator() {
            let e_path = z.join(animation.as_str());
            fs::create_dir_all(&e_path).unwrap();
        }

        Ok(true)
    }

    pub fn get_preset_folder(animation_type: &AnimationType) -> PathBuf {
        let path = Self::get_preset_path().join(animation_type.as_str());
        path
    }

    pub fn settings_path() -> PathBuf {
        let mut path = Self::get_root_path();
        path.push("settings.json");
        path
    }

    pub fn load_or_default() -> Self {
        match Self::create_settings_folder() {
            Ok(_) => GlobalSettings::new(),
            Err(_) => GlobalSettings::new(),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
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

        if changed {
            let _ = self.save();
        }

        changed
    }
}
