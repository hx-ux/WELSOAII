use egui_plot::{Line, Plot, PlotPoints};
use nannou_egui::egui::{self};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;

use crate::modulator::Modulator;
use crate::modulator::polarity::Polarity;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default, Display, EnumIter)]
pub enum WaveType {
    #[default]
    Sine,
    #[strum(to_string = "Triangle")]
    Triangle,
    #[strum(to_string = "Square")]
    Square,
    #[strum(to_string = "RampUp")]
    RampUp,
    #[strum(to_string = "RampDown")]
    RampDown,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WaveModulator {
    /// ±1.0 equals ±100% around the base value.
    pub amount: f32,
    pub amount_type: Polarity,
    pub wave: WaveType,
    /// Multiplier for the global beat clock (1.0 = 1 cycle per beat).
    pub freq_mul: f32,
    /// Phase offset in beats.
    pub phase: f32,
    pub enabled: bool,
}

impl WaveModulator {
    pub fn new() -> Self {
        Self {
            amount: 0.25,
            amount_type: Polarity::Plus,
            wave: WaveType::default(),
            freq_mul: 1.0,
            phase: 1.0,
            enabled: true,
        }
    }
}

impl Modulator for WaveModulator {
    fn ui(&mut self, ui: &mut egui::Ui, current_beat: f32) {
        ui.add(egui::Slider::new(&mut self.amount, self.amount_type.range()).text("Depth"));

        ui.horizontal(|ui| {
            for options in Polarity::iter() {
                ui.radio_value(
                    &mut self.amount_type,
                    options.clone(),
                    format!("{}", options),
                );
            }
        });

        ui.add(
            egui::Slider::new(&mut self.freq_mul, 0.5..=2.0)
                .text("Rate (x beat)")
                .step_by(0.5),
        );

        egui::ComboBox::from_label("Wave")
            .selected_text(format!("{:?}", self.wave))
            .show_ui(ui, |ui| {
                for ss in WaveType::iter() {
                    ui.selectable_value(&mut self.wave, ss, format!("{}", ss));
                }
            });

        let points = 100;
        let line = Line::new(PlotPoints::from_parametric_callback(
            |t| {
                let beat = current_beat + t as f32;
                let val = self.modulated_value(beat, 1.00);
                (t, val as f64)
            },
            0.0..1.0, // show 2 beats ahead
            points,
        ))
        .width(1.0);

        Plot::new(ui.id().with("lfo_plot"))
            .view_aspect(6.0)
            .allow_boxed_zoom(false)
            .sharp_grid_lines(true)
            .show_background(false)
            // grid behind the line
            .show_grid([false; 2])
            .show_axes([false; 2])
            .height(20.0)
            .width(200.0)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .allow_drag(false)
            .include_y(0.0)
            .include_y(2.0)
            .show(ui, |plot_ui| plot_ui.line(line));
    }

    fn modulated_value(&self, beat_pos: f32, anmount: f32) -> f32 {
        if !self.enabled {
            return 1.0;
        }
        let result = modulation_creators(self.wave, beat_pos * self.freq_mul);

        let mapped_result = match self.amount_type {
            Polarity::Plus => (result + 1.0) / 2.0,
            Polarity::Minus => (result - 1.0) / 2.0,
            Polarity::PlusMinus => result,
        };

        let g = 1.0 + self.amount * mapped_result;

        1.0 + (g - 1.0) * anmount
    }
}

pub fn modulation_creators(wave: WaveType, beat_pos: f32) -> f32 {
    let phase = beat_pos * std::f32::consts::TAU;
    match wave {
        WaveType::Sine => phase.sin(),
        WaveType::Triangle => {
            // saw to triangle transform
            let saw = (phase / std::f32::consts::TAU).fract() * 2.0 - 1.0;
            2.0 * saw.abs() - 1.0
        }
        WaveType::Square => {
            if phase.sin() >= 0.0 {
                1.0
            } else {
                -1.0
            }
        }
        WaveType::RampUp => ((phase / std::f32::consts::TAU).fract()) * 2.0 - 1.0,
        WaveType::RampDown => 1.0 - ((phase / std::f32::consts::TAU).fract()) * 2.0,
    }
}
