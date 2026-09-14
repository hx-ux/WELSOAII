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
            amount: ConstantParam::new(
                0.25,
                0.0,
                1.0,
                &"display_text.".to_string(),
                &"amount".to_string(),
            ),

            amount_type: Polarity::Plus,
            wave: LfoWave::default(),
            freq_mul: 1.0,
            skew: ConstantParam::new(0.0, -5.0, 5.0, "Skew", "skew"),
            enabled: true,
        }
    }

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

        let (rect, _response) =
            ui.allocate_exact_size(egui::vec2(200.0, 40.0), egui::Sense::hover());
        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);

            let bg_color = Color32::from_rgb(30, 30, 30);
            painter.rect_filled(rect, 2.0, bg_color);

            let mut mesh = egui::Mesh::default();
            let mut line_points = Vec::with_capacity(N + 1);

            for i in 0..=N {
                let t = i as f32 / N as f32;
                let v = self.preview(t);
                let x = rect.left() + t * rect.width();
                let y = rect.bottom() - (v / 2.0) * rect.height();
                let pos = egui::pos2(x, y);
                line_points.push(pos);

                let bottom_pos = egui::pos2(x, rect.bottom());

                let top_idx = mesh.vertices.len() as u32;
                mesh.vertices.push(egui::epaint::Vertex {
                    pos,
                    uv: egui::pos2(0.0, 0.0),
                    color: accent.linear_multiply(0.3),
                });

                let bottom_idx = mesh.vertices.len() as u32;
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: bottom_pos,
                    uv: egui::pos2(0.0, 0.0),
                    color: accent.linear_multiply(0.0),
                });

                if i > 0 {
                    let prev_top = top_idx - 2;
                    let prev_bottom = bottom_idx - 2;
                    mesh.indices.extend_from_slice(&[
                        prev_top,
                        prev_bottom,
                        bottom_idx,
                        prev_top,
                        bottom_idx,
                        top_idx,
                    ]);
                }
            }

            painter.add(egui::Shape::mesh(mesh));
            painter.add(egui::Shape::line(
                line_points,
                egui::Stroke::new(1.0, accent),
            ));

            let current_t = base_cycles.rem_euclid(1.0);
            let current_v = self.preview(current_t);
            let cx = rect.left() + current_t * rect.width();
            let cy = rect.bottom() - (current_v / 2.0) * rect.height();
            painter.circle_filled(egui::pos2(cx, cy), 4.0, accent);
        }

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

    fn modulated_value(&self, beat_pos: f32, mod_amount: f32) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        let cycles = beat_pos * self.freq_mul;
        let result = self.shaped(cycles);

        let mapped_result = match self.amount_type {
            Polarity::Plus => (result + 1.0) / 2.0,
            Polarity::Minus => (result - 1.0) / 2.0,
            Polarity::PlusMinus => result,
        };

        mapped_result * self.amount.value * mod_amount
    }
}
