use std::{fs, path::PathBuf, result};
        use chrono::prelude::*;
use crate::{
    animator::{AnimatorSettings, animation_type::AnimationType},
    utils::GlobalSettings,
};
use anyhow::Result;
use nannou_egui::egui::{self};
#[derive(Default)]
pub struct Preset {
    pub name: String,
    pub path: PathBuf,
}

impl Preset {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }
}

#[derive(Default)]
pub struct PresetManager {
    pub animation_type: AnimationType,
    pub available_preset_paths: Vec<Preset>,
    selected_preset_idx: usize,
}
impl PresetManager {
    pub fn new(animationType: AnimationType) -> Self {
        let available_preset_paths = Self::list_preset_files(&animationType).unwrap_or_default();

        Self {
            animation_type: animationType,
            available_preset_paths,
            selected_preset_idx: 0,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.collapsing("Preset Management", |ui| {
            if self.available_preset_paths.is_empty() {}

            // Refresh button
            // Disabled for now
            // ui.horizontal(|ui| {
            //     if ui.button("Refresh List").clicked() {}
            //     ui.label(format!(
            //         "{} presets available",
            //         self.available_presets.len()
            //     ));
            // });

            if ui.button("Save As New Preset").clicked() {}
            ui.add_space(10.0);
            ui.separator();

            if !self.available_preset_paths.is_empty() {
                ui.label("Manage Presets:");

                // egui::ComboBox::from_label("Select Preset")
                //     .selected_text(
                //         self.available_preset_paths.name
                //             .get(self.selected_preset_idx)
                //             .unwrap_or(&"None".to_string()),
                //     )
                //     .show_ui(ui, |ui| {
                //         for (idx, preset) in self.available_preset_paths.iter().enumerate() {
                //             ui.selectable_value(&mut self.selected_preset_idx, idx, preset);
                //         }
                //     });
            }
            // egui::ComboBox::from_label("Select Preset")
            //     .selected_text(
            //         self.available_preset_paths
            //             .get(self.selected_preset_idx)
            //             .unwrap_or(&"None".to_string()),
            //     )
            //     .show_ui(ui, |ui| {
            //         for (idx, preset) in self.available_preset_paths.iter().enumerate() {
            //             ui.selectable_value(&mut self.selected_preset_idx, idx, preset);
            //         }
            //     });

            ui.horizontal(|ui| {
                if ui.button("Apply ").clicked() {
                    if let Some(filename) =
                        self.available_preset_paths.get(self.selected_preset_idx)
                    {
                        // let path =
                        //     crate::utils::GlobalSettings::get_preset_folder(&self.animationType)
                        //         .join(filename);
                        // if let Ok(content) = std::fs::read_to_string(&path) {

                        //      if let Ok(mut loaded) = serde_json::from_str::<Self>(&content) {
                        //          // Preserve UI state and recalculate dimensions
                        //          loaded.available_presets = self.available_presets.clone();
                        //          loaded.selected_preset_idx = self.selected_preset_idx;
                        //          loaded.dimension = self.dimension;
                        //          *self = loaded;
                        //   changed = true;
                    }
                }
                // }
            });

            // DISABLED FOR NOW

            //  if ui.button("Delete Selected").clicked() {
            //      if let Some(filename) = self.available_presets.get(self.selected_preset_idx)
            //      {
            //          let _ =
            //              crate::animator::delete_preset(&self.animationType, filename);
            //          if let Ok(presets) =
            //              crate::animator::list_preset_files(&self.animationType)
            //          {
            //              self.available_presets = presets;
            //              self.selected_preset_idx = 0;
            //          }
            //      }
            //  }
        });

        true
    }

    fn generate_filename_timestamp() -> String {
        let now: DateTime<Local> = Local::now();
        format!("preset_{}.json", now.format("%Y%m%d_%H%M%S"))
    }

   pub fn save_to_file(
        filename: &str,
        animation_type: &AnimationType,
        content: String,
    ) -> Result<bool> {
        // let json = serde_json::to_string_pretty(self)?;

        let path = GlobalSettings::get_preset_folder(&animation_type).join(filename);

        std::fs::write(path, content)?;
        Ok(true)
    }

    fn list_preset_files(animation_type: &AnimationType) -> Result<Vec<Preset>> {
        let path = GlobalSettings::get_preset_folder(&animation_type);
        let mut entries: Vec<Preset> = Vec::new();
        if path.exists() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Some(name) = entry.file_name().to_str() {
                        entries.push(Preset::new(name.to_string(), entry.path()));
                    }
                }
            }
        }

        entries.reverse(); // Most recent first
        Ok(entries)
    }

    fn delete_preset(animation_type: &AnimationType, filename: &str) -> Result<bool> {
        let path = GlobalSettings::get_preset_folder(animation_type).join(filename);
        fs::remove_file(path).unwrap_or_default();
        Ok(true)
    }
}
