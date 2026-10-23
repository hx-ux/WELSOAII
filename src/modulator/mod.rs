pub mod polarity;
pub mod wave_modulator;

use nannou_egui::egui;

pub trait Modulator {
    fn ui(&mut self, ui: &mut egui::Ui, current_beat: f32);
    fn modulated_value(&self, beat_pos: f32, amount: f32) -> f32;
}

