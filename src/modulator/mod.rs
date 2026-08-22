pub mod polarity;
pub mod wave_modulator;
use nannou_egui::egui;
use wave_modulator::WaveModulator;

pub trait Modulator {
    fn ui(&mut self, ui: &mut egui::Ui, current_beat: f32);
    fn modulated_value(&self, beat_pos: f32, anmount: f32) -> f32;
}
