use std::ops::RangeInclusive;

use egui_plot::{Line, Plot, PlotPoints};
use nannou_egui::egui::{self};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;

#[derive(Clone, Display, EnumIter, Serialize, Deserialize, PartialEq)]
pub enum AmountType {
    #[strum(to_string = "+")]
    Plus,
    #[strum(to_string = "-")]
    Minus,
    #[strum(to_string = "+-")]
    PlusMinus,
}

impl AmountType {
    pub fn range(&self) -> RangeInclusive<f32> {
        match self {
            AmountType::Plus => RangeInclusive::new(0.0, 1.0),
            AmountType::Minus => RangeInclusive::new(0.0, 1.0),
            AmountType::PlusMinus => RangeInclusive::new(-1.0, 1.0),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default, Display, EnumIter)]
pub enum ModSourceEngine {
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

// Actuaal Modulator, which creates the values
#[derive(Clone, Serialize, Deserialize)]
pub struct Modulator {
    /// ±1.0 equals ±100% around the base value.
    pub amount: f32,
    pub amount_type: AmountType,
    pub wave: ModSourceEngine,
    /// Multiplier for the global beat clock (1.0 = 1 cycle per beat).
    pub freq_mul: f32,
    /// Phase offset in beats.
    pub phase: f32,
    pub enabled: bool,
}

impl Default for Modulator {
    fn default() -> Self {
        Self {
            amount: 0.25,
            amount_type: AmountType::Plus,
            wave: ModSourceEngine::default(),
            freq_mul: 1.0,
            phase: 1.0,
            enabled: true,
        }
    }
}

impl Modulator {
    pub fn ui(&mut self, ui: &mut egui::Ui, current_beat: f32) {
        ui.add(egui::Slider::new(&mut self.amount, self.amount_type.range()).text("Depth"));

        ui.horizontal(|ui| {
            for options in AmountType::iter() {
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
                for ss in ModSourceEngine::iter() {
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

    pub fn modulated_value(&self, beat_pos: f32, anmount: f32) -> f32 {
        if !self.enabled {
            return 1.0;
        }
        let result = modulation_creators(self.wave, beat_pos * self.freq_mul);

        let mapped_result = match self.amount_type {
            AmountType::Plus => (result + 1.0) / 2.0,
            AmountType::Minus => (result - 1.0) / 2.0,
            AmountType::PlusMinus => result,
        };

        let g = 1.0 + self.amount * mapped_result;

        1.0 + (g - 1.0) * anmount
    }
}

pub fn modulation_creators(wave: ModSourceEngine, beat_pos: f32) -> f32 {
    let phase = beat_pos * std::f32::consts::TAU;
    match wave {
        ModSourceEngine::Sine => phase.sin(),
        ModSourceEngine::Triangle => {
            // saw to triangle transform
            let saw = (phase / std::f32::consts::TAU).fract() * 2.0 - 1.0;
            2.0 * saw.abs() - 1.0
        }
        ModSourceEngine::Square => {
            if phase.sin() >= 0.0 {
                1.0
            } else {
                -1.0
            }
        }
        ModSourceEngine::RampUp => ((phase / std::f32::consts::TAU).fract()) * 2.0 - 1.0,
        ModSourceEngine::RampDown => 1.0 - ((phase / std::f32::consts::TAU).fract()) * 2.0,
    }
}
