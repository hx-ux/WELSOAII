use crate::{
    animator::animation_type::{AnimationType, UpdateBehaviour},
    utils::PathManager,
};

use chrono::prelude::*;
use nannou_egui::egui::{self};
use serde::Serialize;
use std::{fs, marker::PhantomData, path::PathBuf};

#[derive(Clone)]
pub enum PresetMode {
    Grid,
    Settings,
    Animator,
}

#[derive(Clone)]
pub struct Preset<T> {
    pub name: String,
    pub path: PathBuf,
    _phantom: PhantomData<T>,
}

impl<T> Preset<T> {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            _phantom: PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct PresetManager<T> {
    pub animation_type: Option<AnimationType>,
    pub presets: Vec<Preset<T>>,
    selected_preset_idx: usize,
    pub set_filename: Option<String>,
    preset_mode: PresetMode,
    desc: String,
}
impl<T> PresetManager<T> {
    pub fn new_animator(animation_type: AnimationType) -> Self {
        Self {
            animation_type: Some(animation_type),
            presets: Vec::new(),
            // presets: Self::load_all_presets(&animation_type.clone()).unwrap_or_default(),
            selected_preset_idx: 0,
            set_filename: None,
            preset_mode: PresetMode::Animator,
            desc: String::default(),
        }
    }

    pub fn new_grid(preset_mode: PresetMode, desc: String) -> Self {
        Self {
            animation_type: None,
            presets: Vec::new(),
            selected_preset_idx: 0,
            set_filename: None,
            preset_mode,
            desc,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.separator();
        ui.add_space(5.0);

        let _update_behaviour = (false, UpdateBehaviour::None);
        ui.collapsing("Preset Management", |ui| {
            if ui.button("Save As New Preset").clicked() {
                changed = true;
            }

            ui.add_space(10.0);
            ui.separator();

            if !self.presets.is_empty() {
                ui.label("Manage Presets:");

                egui::ComboBox::from_label("Select Preset")
                    .selected_text(
                        self.presets
                            .get(self.selected_preset_idx)
                            .map(|p| &p.name)
                            .unwrap_or(&"None".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        for (idx, preset) in self.presets.iter().enumerate() {
                            ui.selectable_value(&mut self.selected_preset_idx, idx, &preset.name);
                        }
                    });
            } else {
                ui.label("No presets:");
            }

            ui.horizontal(|ui| if ui.button("Apply ").clicked() {});
        });

        changed
    }

    fn generate_filename(&self, custom_file_name: Option<String>) -> String {
        if let Some(name) = custom_file_name {
            return format!("{}.json", name);
        }
        match self.preset_mode {
            PresetMode::Grid => format!("grid_{}.json", self.desc),
            PresetMode::Settings => "settings.json".to_string(),
            PresetMode::Animator => {
                let now: DateTime<Local> = Local::now();
                format!("preset_{}.json", now.format("%Y%m%d_%H%M%S"))
            }
        }
    }

    pub fn save_to_file(
        &self,
        data: &T,
        custom_file_name: Option<String>,
    ) -> Result<bool, anyhow::Error>
    where
        T: Serialize,
    {
        match self.preset_mode {
            PresetMode::Grid => {
                let path = PathManager::get_devices_folder()
                    .join(&self.generate_filename(custom_file_name));

                nannou::io::save_to_json(path, data)?;
            }
            PresetMode::Settings => return Err(anyhow::anyhow!("Missing attribute:")),
            PresetMode::Animator => match self.animation_type {
                Some(atype) => {
                    let path = PathManager::get_preset_folder(&atype)
                        .join(&self.generate_filename(custom_file_name));
                    nannou::io::save_to_json(path, data)?;
                }
                None => return Err(anyhow::anyhow!("Missing attribute:")),
            },
        }

        print!("saved");
        Ok(true)
    }

    pub fn get_preset_path(&self) -> Option<PathBuf> {
        let z = self.set_presets();
        z.ok()
    }

    pub fn set_presets(&self) -> Result<PathBuf, anyhow::Error> {
        if self.presets.is_empty() {
            return Err(anyhow::anyhow!("No presets available"));
        }

        if let Some(preset) = self.presets.get(self.selected_preset_idx) {
            Ok(preset.path.clone())
        } else {
            Err(anyhow::anyhow!("Selected preset index is out of bounds"))
        }
    }

    fn update_presets(&mut self) {
        if let Ok(p) = self.load_all_presets() {
            self.presets = p
        }
    }

    fn load_all_presets(&self) -> Result<Vec<Preset<T>>, anyhow::Error> {
        match self.preset_mode {
            PresetMode::Animator => match self.animation_type {
                Some(atype) => {
                    let path = PathManager::get_preset_folder(&atype);
                    let mut entries: Vec<Preset<T>> = Vec::new();

                    if path.exists() {
                        for entry in fs::read_dir(path)? {
                            let entry = entry?;
                            if entry.path().extension().and_then(|s| s.to_str()) == Some("json")
                                && let Some(name) = entry.file_name().to_str()
                            {
                                entries.push(Preset::new(name.to_string(), entry.path()));
                            }
                        }
                    }
                    entries.reverse(); // Most recent first
                    Ok(entries)
                }
                None => Err(anyhow::anyhow!("Cant find type")),
            },
            PresetMode::Settings | PresetMode::Grid => Err(anyhow::anyhow!("Nothing")),
        }
    }
    pub fn ui_simple(&mut self, ui: &mut egui::Ui) -> bool {
        if ui.button("Save").clicked() {
            return true;
        }
        false
    }
}

impl<T> Default for Preset<T> {
    fn default() -> Self {
        Self {
            name: String::new(),
            path: PathBuf::new(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for PresetManager<T> {
    fn default() -> Self {
        Self {
            animation_type: Some(AnimationType::default()),
            presets: Vec::new(),
            selected_preset_idx: 0,
            set_filename: None,
            preset_mode: PresetMode::Animator,
            desc: String::default(),
        }
    }
}
