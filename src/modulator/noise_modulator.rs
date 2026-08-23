use nannou_egui::egui;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{modulator::Modulator, parameters::ConstantParam};

#[derive(Debug, Clone, Copy, PartialEq, Default, Display, EnumIter)]
pub enum NoiseType {
    #[default]
    Perlin,
    #[strum(to_string = "White")]
    White,
    #[strum(to_string = "Brown")]
    Brown,
    #[strum(to_string = "Chaos")]
    Chaos,
}

#[derive(Clone)]
pub struct NoiseModulator {
    /// ±1.0 equals ±100% around the base value.
    pub amount: ConstantParam<f32>,
    pub cycles: ConstantParam<u32>,
    // pub amount_type: Polarity,
    pub noise_type: NoiseType,
    // pub freq_mul: f32,
    pub enabled: bool,
}

impl NoiseModulator {
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
            cycles: ConstantParam {
                value: 4,
                default: 4,
                lower: 1,
                upper: 8,
                display_text: "Cycles".to_string(),
                identifier: "cycles".to_string(),
            },

            noise_type: NoiseType::Perlin,
            // freq_mul: 1.0,
            enabled: true,
        }
    }

    /// Deterministic integer-lattice hash -> -1..=1.
    /// Stable across platforms, runs, and serialize/deserialize round-trips.
    fn hash(i: i64) -> f32 {
        let mut x = i.wrapping_mul(0x27D4EB2D);
        x ^= x >> 17;
        x = x.wrapping_mul(0x9E3779B1);
        x ^= x >> 15;
        ((x & 0xFFFF) as f32 / 32_768.0) - 1.0
    }

    /// 1D Perlin-style value noise. Continuous everywhere, output -1..=1.
    /// One unit of `x` = one cycle at the current rate/freq_mul.
    fn perlin(x: f32) -> f32 {
        let i = x.floor();
        let t = x - i;
        // Perlin's quintic fade: zero slope at both lattice points
        let u = t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
        let a = Self::hash(i as i64);
        let b = Self::hash(i as i64 + 1);
        a + (b - a) * u
    }

    /// Bipolar -1..=1 waveform value at a (continuous) cycle position.
    fn shaped(&self, cycles: f32) -> f32 {
        // let loop_cycles = 4.00;
        let loop_cycles = self.cycles.value as f32;
        match self.noise_type {
            NoiseType::Perlin => {
                let x = cycles.rem_euclid(loop_cycles);
                let w = x / loop_cycles;
                Self::perlin(x) * (1.0 - w) + Self::perlin(x - loop_cycles) * w
            }
            NoiseType::White => 1.00,
            NoiseType::Brown => 1.00,
            NoiseType::Chaos => 1.00,
        }
    }
}

impl Modulator for NoiseModulator {
    fn ui(&mut self, ui: &mut nannou_egui::egui::Ui, current_beat: f32) {
        self.cycles.to_slider(ui);

        egui::ComboBox::from_label("")
            .selected_text(format!("{}", self.noise_type))
            .show_ui(ui, |ui| {
                for w in NoiseType::iter() {
                    ui.selectable_value(&mut self.noise_type, w, format!("{}", w));
                }
            });
    }

    fn modulated_value(&self, beat_pos: f32, anmount: f32) -> f32 {
        self.shaped(beat_pos)
    }
}
