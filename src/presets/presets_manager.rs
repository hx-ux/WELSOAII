use crate::animator::animation_type::{AnimationType, UpdateBehaviour};

use nannou_egui::egui::{self};

#[derive(Clone)]
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
        ui.collapsing("Preset Management", |ui| {
            if ui.button("Save As New Preset").clicked() {
                changed = true;
            }

            ui.add_space(10.0);
            ui.separator();

            ui.horizontal(|ui| if ui.button("Apply ").clicked() {});
        });

        changed
    }

    pub fn save_to_file(&self, custom_file_name: Option<String>) -> Result<bool, anyhow::Error> {
        print!("saved");
        Ok(true)
    }
}
