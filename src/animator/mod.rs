use crate::{
    animator::{
        animation_type::AnimationType,
        animators::{
            bouncing_ball, pulse_background, quantum_tunnel::QuantumTunnelSettings, scan_line,
            wave_lines::WaveLinesSettings,
        },
    },
    modulator::wave_modulator::WaveModulator,
    parameters::ModulatedParam,
    receiver::ReceiverGrid,
    timecode::TimeCode,
    ui::icons::{ICON_ADD, ICON_EYE_ACTIVE, ICON_EYE_DISABLED, ICON_REMOVE},
};
use anyhow::Result;
use nannou::prelude::*;
use nannou_egui::egui::{self, Color32};
use strum::IntoEnumIterator;
pub mod animation_type;
mod animators;
use crate::modulator::Modulator;
pub mod animator;

use bouncing_ball::BouncingBallSettings;
use pulse_background::PulseBackgroundSettings;
use scan_line::ScanLineSettings;

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
    fn control_ui(&mut self, ui: &mut egui::Ui, mods: &mut Vec<Box<dyn Modulator>>);
    fn animation_type(&self) -> AnimationType;
    fn init(&mut self);
    fn set_dimension(&mut self, _window_rect: &Rect) {}
    fn hot_update(&mut self);
    fn draw(&self, draw: &Draw);
    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode);
    fn get_objects(&self) -> Vec<&dyn AnimatedObject>;
    fn get_objects_mut(&mut self) -> Vec<&mut dyn AnimatedObject>;
    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam>;

    fn update_modulations(&mut self, beat_pos: f32, modulators: &mut Vec<Box<dyn Modulator>>) {
        for param in self.modulated_params_mut() {
            param.modulate(beat_pos, modulators);
        }
    }

    fn reset_modulations(&mut self) {
        for param in self.modulated_params_mut() {
            param.ghost_value = None;
        }
    }

    fn snapshot(&mut self) -> Result<()> {
        Ok(())
    }

    fn set_visiblity(&mut self, state: bool);
    fn get_visiblity(&self) -> bool;
}
