use std::{
    fs::{self},
    path::PathBuf,
};

use crate::utils::AppSettings;
pub struct FileManager {}

impl FileManager {
    fn app_root_path() -> Result<PathBuf, ()> {
        let mut doc = dirs::document_dir().expect("");
        doc.push(AppSettings::APP_NAME);
        let _ = fs::create_dir_all(&doc).map_err(|e| e.to_string());
        Ok(doc)
    }

    pub fn save_app_settings(app_settings: &AppSettings) -> Result<(), String> {
        let path = FileManager::app_settings_path().expect("");

        nannou::io::save_to_json(&path, &app_settings)
            .map_err(|e| format!("Failed to save settings: {}", e))?;

        Ok(())
    }

    pub fn app_settings_path() -> Result<PathBuf, ()> {
        match FileManager::app_root_path() {
            Ok(mut path) => {
                path.push("settings.json");
                Ok(path)
            }
            Err(_) => Err(()),
        }
    }
}
