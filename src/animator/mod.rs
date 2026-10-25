use crate::{
    animator::animation_type::AnimationType, modulator::Modulator, parameters::ModulatedParam,
    timecode::TimeCode,
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

    #[allow(dead_code)]
    fn snapshot(&mut self, name: &str) -> Result<()> {
        Ok(())
    }

    fn set_visibility(&mut self, state: bool);
    fn get_visibility(&self) -> bool;
}
