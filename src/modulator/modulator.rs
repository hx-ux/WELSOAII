use std::ops::RangeInclusive;

use crate::animator::animation_type::AnimationType;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ModTarget(pub String);

impl ModTarget {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn none() -> Self {
        Self(String::new())
    }

    pub fn is_none(&self) -> bool {
        self.0.is_empty() || self.0 == "None"
    }

    pub fn name(&self) -> &str {
        if self.is_none() {
            "None"
        } else {
            &self.0
        }
    }
}

impl std::fmt::Display for ModTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModRoute {
    pub target: ModTarget,
    pub enabled: bool,
    pub amount: f32,
}

impl ModRoute {
    pub fn new(target: ModTarget) -> Self {
        Self {
            target,
            enabled: false,
            amount: 0.5,
        }
    }
}

impl Default for ModRoute {
    fn default() -> Self {
        Self {
            target: Default::default(),
            enabled: true,
            amount: 0.5,
        }
    }
}

// Actuaal Modulator, which creates the values
#[derive(Clone, Serialize, Deserialize)]
pub struct Modulator {
    pub routes: Vec<ModRoute>,

    /// ±1.0 equals ±100% around the base value.
    pub amount: f32,
    pub amount_type: AmountType,
    pub wave: ModSourceEngine,
    /// Multiplier for the global beat clock (1.0 = 1 cycle per beat).
    pub freq_mul: f32,
    /// Phase offset in beats.
    pub phase: f32,
    pub enabled: bool,
    pub mod_route_placeholder: ModRoute,
}

impl Default for Modulator {
    fn default() -> Self {
        Self {
            routes: Default::default(),
            amount: 0.25,
            amount_type: AmountType::Plus,
            wave: ModSourceEngine::default(),
            freq_mul: 1.0,
            phase: 1.0,
            enabled: true,
            mod_route_placeholder: ModRoute::default(),
        }
    }
}

impl Modulator {
    pub fn set_enables(&mut self, state: bool, target: &ModTarget) {
        if let Some(dev) = self.routes.iter_mut().find(|d| &d.target == target) {
            dev.enabled = state;
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, _animation_type: AnimationType) {
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
    }

    pub fn calc_modulation(&self, beat_pos: f32, target: &ModTarget) -> f32 {
        if !self.enabled {
            return 1.0;
        }
        let result = create_modulation(self.wave, beat_pos * self.freq_mul);
        let mut factor = 1.0;
        let mut amount = self.amount;

        if self.amount_type == AmountType::Minus {
            amount *= -1.0;
        }

        for route in self
            .routes
            .iter()
            .filter(|route| route.enabled && &route.target == target)
        {
            factor *= 1.0 + (amount * route.amount) * result;
        }

        factor
    }
}

/// Sample a waveform at the given beat position. Output in [-1, 1].
pub fn create_modulation(wave: ModSourceEngine, beat_pos: f32) -> f32 {
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
