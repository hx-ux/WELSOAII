use egui_plot::{Line, Plot, PlotPoints, Points};
use nannou_egui::egui::{self, Color32};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::modulator::Modulator;
use crate::modulator::polarity::Polarity;
use crate::parameters::ConstantParam;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default, Display, EnumIter)]
pub enum LfoWave {
    #[default]
    Sine,
    /// Triangle, skewable -> shows as "Pyramid" like in the reference
    #[strum(to_string = "Pyramid")]
    Pyramid,
    #[strum(to_string = "Square")]
    Square,
    #[strum(to_string = "Ramp Up")]
    RampUp,
    #[strum(to_string = "Ramp Down")]
    RampDown,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WaveModulator {
    /// ±1.0 equals ±100% around the base value.
    pub amount: ConstantParam<f32>,
    pub amount_type: Polarity,
    pub wave: LfoWave,
    pub freq_mul: f32,
    pub skew: ConstantParam<f32>,
    pub enabled: bool,
}

impl WaveModulator {
    pub fn new() -> Self {
        Self {
            amount: ConstantParam {
                value: 0.25,
                default: 0.25,
                lower: 0.0,
                upper: 1.0,
                display_text: "Depth".to_string(),
                identifier: "amount".to_string(),
            },
            amount_type: Polarity::Plus,
            wave: LfoWave::default(),
            freq_mul: 1.0,
            skew: ConstantParam::new(0.0, 0.0, 1.0, "Skew", "skew"),
            enabled: true,
        }
    }

    pub fn set_speed_hz(&mut self, hz: f32, bpm: f32) {
        self.freq_mul = hz * 60.0 / bpm;
    }

    pub fn speed_hz(&self, bpm: f32) -> f32 {
        self.freq_mul * bpm / 60.0
    }

    /// Bipolar -1..=1 waveform value at a (continuous) cycle position.
    fn shaped(&self, cycles: f32) -> f32 {
        let p = cycles.rem_euclid(1.0);

        // Skew = phase distortion: warps 0..1 progress before shaping.
        // skew > 0 stretches the rising side, < 0 the falling side.
        let e = (2.0f32).powf(self.skew.value.clamp(-2.0, 2.0));
        let ps = p.powf(e);

        match self.wave {
            LfoWave::Sine => (std::f32::consts::TAU * ps).sin(),
            LfoWave::Pyramid => 1.0 - (2.0 * ps - 1.0).abs(),
            // skew acts as PWM width for square
            LfoWave::Square => {
                if ps < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            LfoWave::RampUp => 2.0 * ps - 1.0,
            LfoWave::RampDown => 1.0 - 2.0 * ps,
        }
    }

    fn shaped_mapped(&self, cycles: f32) -> f32 {
        let v = self.shaped(cycles);
        match self.amount_type {
            Polarity::Plus => (v + 1.0) * 0.5,
            Polarity::Minus => (v - 1.0) * 0.5,
            Polarity::PlusMinus => v,
        }
    }

    /// Normalize value for preview
    fn preview(&self, cycles: f32) -> f32 {
        1.0 + self.amount.value * self.shaped_mapped(cycles)
    }
}

impl Default for WaveModulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Modulator for WaveModulator {
    fn ui(&mut self, ui: &mut egui::Ui, current_beat: f32) {
        let base_cycles = current_beat * self.freq_mul;

        const N: usize = 128;
        // blue accent color
        let accent = Color32::from_rgb(0x5A, 0xA9, 0xFF);

        let samples: Vec<[f64; 2]> = (0..=N)
            .map(|i| {
                let t = i as f32 / N as f32;
                [t as f64, self.preview(base_cycles + t) as f64]
            })
            .collect();

        // Floating line
        let line = Line::new(PlotPoints::from(samples.clone()))
            .width(1.5)
            .color(accent);

        // Point
        let handle = {
            let peak_t = 0.5f32.powf(1.0 / (2.0f32).powf(self.skew.value)); // where p^e == 0.5
            Points::new(PlotPoints::from(vec![[
                peak_t as f64,
                self.preview(base_cycles + peak_t) as f64,
            ]]))
            .radius(4.0)
            .color(accent)
        };

        Plot::new(ui.id().with("lfo_preview"))
            .view_aspect(6.0)
            .height(20.0)
            .width(200.0)
            .show_background(false)
            .show_grid([false; 2])
            .show_axes([false; 2])
            .allow_drag(false)
            .allow_zoom(false)
            .allow_boxed_zoom(false)
            .allow_scroll(false)
            .allow_double_click_reset(false)
            .include_x(0.0)
            .include_x(1.0)
            .include_y(0.0)
            .include_y(2.0)
            .show(ui, |plot_ui| {
                plot_ui.line(line);
                plot_ui.points(handle);
            });

        ui.horizontal_wrapped(|ui| {
            ui.add(
                egui::Slider::new(&mut self.freq_mul, 0.5..=2.0)
                    .text("Rate (x beat)")
                    .step_by(0.5),
            );

            egui::ComboBox::from_label("Wave")
                .selected_text(format!("{}", self.wave))
                .show_ui(ui, |ui| {
                    for w in LfoWave::iter() {
                        ui.selectable_value(&mut self.wave, w, format!("{}", w));
                    }
                });
        });

        self.amount.to_slider(ui);

        ui.horizontal(|ui| {
            for options in Polarity::iter() {
                ui.radio_value(
                    &mut self.amount_type,
                    options.clone(),
                    format!("{}", options),
                );
            }
        });

        self.skew.to_slider(ui);
    }

    fn modulated_value(&self, beat_pos: f32, anmount: f32) -> f32 {
        if !self.enabled {
            return 1.0;
        }

        let cycles = beat_pos * self.freq_mul;
        let result = self.shaped(cycles);

        let mapped_result = match self.amount_type {
            Polarity::Plus => (result + 1.0) / 2.0,
            Polarity::Minus => (result - 1.0) / 2.0,
            Polarity::PlusMinus => result,
        };

        let g = 1.0 + self.amount.value * mapped_result;

        1.0 + (g - 1.0) * anmount
    }
}
