pub mod polarity;
use nannou_egui::egui;

pub mod noise_modulator;
pub mod wave_modulator;

pub trait Modulator {
    fn ui(&mut self, ui: &mut egui::Ui, current_beat: f32);
    // return point for the modulated value
    fn modulated_value(&self, beat_pos: f32, anmount: f32) -> f32;
}
