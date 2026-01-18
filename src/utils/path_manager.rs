use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;

use crate::{
    animator::animation_type::{AnimationType, ModeHelper},
    utils::GlobalSettings,
};
pub struct PathManager {}

impl PathManager {
    pub fn root_path() -> PathBuf {
        let mut doc_dir = dirs::document_dir().unwrap();
        doc_dir.push(GlobalSettings::APP_NAME);
        fs::create_dir_all(&doc_dir).unwrap();
        doc_dir
    }

    pub fn settings_path() -> PathBuf {
        let mut path = Self::root_path();
        path.push("settings.json");
        path
    }

    pub fn get_preset_path() -> PathBuf {
        let doc_dir = dirs::document_dir()
            .unwrap()
            .join(GlobalSettings::APP_NAME)
            .join(GlobalSettings::EFFECTS_FOLDER);
        fs::create_dir_all(&doc_dir).unwrap();
        doc_dir
    }
    pub fn get_preset_folder(animation_type: &AnimationType) -> PathBuf {
        let path = Self::get_preset_path().join(animation_type.as_str());
        path
    }

    pub fn get_devices_folder() -> PathBuf {
        let doc_dir = dirs::document_dir()
            .unwrap()
            .join(GlobalSettings::APP_NAME)
            .join(GlobalSettings::DEVICES_FOLDER);
        fs::create_dir_all(&doc_dir).unwrap();
        doc_dir
    }

    pub fn createFileStructure() {}
}
