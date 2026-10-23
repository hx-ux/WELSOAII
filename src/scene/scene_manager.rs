#![allow(dead_code)]

use crate::animator::animation_type::UpdateBehaviour;
use anyhow::Result;
use nannou_egui::egui;

#[derive(Clone, Default)]
pub struct SceneManager {
    pub filename: Option<String>,
    desc: String,
}

impl SceneManager {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.separator();
        ui.add_space(5.0);

        let _update_behaviour = (false, UpdateBehaviour::None);
        ui.collapsing("Scene Manager", |ui| {
            if ui.button("Save As New Preset").clicked() {
                changed = true;
            }

            ui.add_space(10.0);
            ui.separator();

            ui.horizontal(|ui| {
                let _ = ui.button("Apply");
            });
        });

        changed
    }

    pub fn save_to_file(&self, _custom_file_name: Option<String>) -> Result<bool> {
        Ok(true)
    }
}
