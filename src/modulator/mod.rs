pub mod polarity;
use nannou_egui::egui;

pub mod wave_modulator;

pub trait Modulator {
    fn ui(&mut self, ui: &mut egui::Ui, current_beat: f32);
    fn modulated_value(&self, beat_pos: f32, anmount: f32) -> f32;
}
