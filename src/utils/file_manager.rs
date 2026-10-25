use std::{fs, path::PathBuf};

use anyhow::{Context, Result};

use crate::{animator::animation_type::AnimationType, utils::AppSettings};

pub struct FileManager;

impl FileManager {
    fn app_root_path() -> Result<PathBuf> {
        let mut doc =
            dirs::document_dir().context("Could not determine user document directory")?;
        doc.push(AppSettings::APP_NAME);
        fs::create_dir_all(&doc)
            .with_context(|| format!("Failed to create app directory at {:?}", doc))?;
        Ok(doc)
    }

    pub fn save_app_settings(app_settings: &AppSettings) -> Result<()> {
        let path = Self::app_settings_path()?;
        nannou::io::save_to_json(&path, app_settings)
            .with_context(|| format!("Failed to save settings to {:?}", path))?;
        Ok(())
    }

    pub fn app_settings_path() -> Result<PathBuf> {
        let mut path = Self::app_root_path()?;
        path.push("settings.json");
        Ok(path)
    }
    pub fn preset_path(ani_type: AnimationType) -> Result<PathBuf> {
        let mut path = Self::app_root_path()?;
        path.push("presets");
        path.push(ani_type.to_string());
        fs::create_dir_all(path.clone())?;
        Ok(path)
    }
}
