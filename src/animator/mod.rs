use crate::{
    animator::animation_type::AnimationType, modulator::Modulator, parameters::ModulatedParam,
    preset::PresetState, timecode::TimeCode, utils::FileManager,
};
use anyhow::Result;
use nannou::prelude::*;
use nannou_egui::egui;

pub mod animation_type;
#[allow(clippy::module_inception)]
pub mod animator;
mod animators;

pub use animator::Animator;

pub enum ObjectShape {
    Circle(Vec2, f32),
    Rect(Rect),
}

pub trait AnimatedObject {
    fn update(&mut self, win_rect: &Rect, clock: &TimeCode);
    fn draw(&self, draw: &Draw);
    fn shape(&self) -> ObjectShape;
    fn color(&self) -> Rgba8;
}

pub trait AnimatorSettings {
    fn control_ui(&mut self, ui: &mut egui::Ui, mods: &[Box<dyn Modulator>]);
    fn animation_type(&self) -> AnimationType;
    fn init(&mut self);
    fn set_dimension(&mut self, _window_rect: &Rect) {}
    fn hot_update(&mut self);
    fn draw(&self, draw: &Draw);
    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode);
    fn get_objects(&self) -> Vec<&dyn AnimatedObject>;
    #[allow(dead_code)]
    fn get_objects_mut(&mut self) -> Vec<&mut dyn AnimatedObject>;
    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam>;

    fn update_modulations(&mut self, beat_pos: f32, modulators: &[Box<dyn Modulator>]) {
        let mut any_modulated = false;
        for param in self.modulated_params_mut() {
            param.modulate(beat_pos, modulators);
            if param.modulation_active {
                any_modulated = true;
            }
        }
        if any_modulated {
            self.hot_update();
        }
    }

    fn reset_modulations(&mut self) {
        for param in self.modulated_params_mut() {
            param.ghost_value = None;
        }
    }

    fn set_visibility(&mut self, state: bool);
    fn get_visibility(&self) -> bool;

    fn preset_state(&self) -> &PresetState;
    fn preset_state_mut(&mut self) -> &mut PresetState;
    fn to_json(&self) -> anyhow::Result<String>; // serde_json::to_string_pretty(self)
    fn apply_json(&mut self, json: &str) -> anyhow::Result<()>; // parse + copy fields

    fn save_preset(&mut self, name: &str) -> anyhow::Result<()> {
        let mut path = FileManager::preset_path(self.animation_type())?;
        path.push(format!("{name}.json"));
        std::fs::write(path, self.to_json()?)?;
        Ok(())
    }
    fn load_preset_list(&mut self) -> anyhow::Result<()> {
        let dir = FileManager::preset_path(self.animation_type())?;
        let mut names = Vec::new();
        if dir.is_dir() {
            for e in std::fs::read_dir(dir)? {
                names.push(e?.file_name().to_string_lossy().into_owned());
            }
        }
        let st = self.preset_state_mut();
        if let Some(first) = names.first() {
            st.selected = first.clone();
        }
        st.available = names;
        Ok(())
    }
    fn load_preset(&mut self) -> anyhow::Result<()> {
        let mut path = FileManager::preset_path(self.animation_type())?;
        path.push(&self.preset_state().selected);
        let json = std::fs::read_to_string(path)?;
        self.apply_json(&json)
    }

    fn preset_ui(&mut self, ui: &mut egui::Ui) {
        let mut save = false;
        let mut load = false;
        let st = self.preset_state_mut();
        ui.text_edit_singleline(&mut st.prest_save_name);
        save = ui.button("save").clicked();
        egui::ComboBox::from_label("")
            .selected_text(&st.selected)
            .show_ui(ui, |ui| {
                for o in st.available.clone() {
                    ui.selectable_value(&mut st.selected, o.clone(), o);
                }
            });
        load = ui.button("load").clicked();
        if save {
            let n = self.preset_state().prest_save_name.clone();
            let _ = self.save_preset(&n);
            let _ = self.load_preset_list();
        }
        if load {
            let _ = self.load_preset();
        }
    }
}
