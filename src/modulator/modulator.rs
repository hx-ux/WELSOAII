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
pub enum ModWave {
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
    #[strum(to_string = "Random")]
    Random,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default, Display, EnumIter)]
pub enum ModTarget {
    #[default]
    #[strum(to_string = "None")]
    None,
    // Bouncing Balls
    #[strum(to_string = "Bounce Speed")]
    BouncingSpeed,
    #[strum(to_string = "Bounce Radius")]
    BouncingRadius,
    // Scan Line
    #[strum(to_string = "Scan Speed")]
    ScanSpeed,
    #[strum(to_string = "Scan Width")]
    ScanWidth,
    #[strum(to_string = "Pulse Speed")]
    PulseSpeed,
    #[strum(to_string = "Pulse Limit")]
    PulseLimit,
    #[strum(to_string = "Pulse Beat Multiplier")]
    PulseBeatMult,
    #[strum(to_string = "Pulse Ring Count")]
    PulseRingCount,
    #[strum(to_string = "Pulse Rotation")]
    PulseRotation,
    // Wave Lines
    #[strum(to_string = "Wave Amplitude")]
    WaveAmplitude,
    #[strum(to_string = "Wave Frequency")]
    WaveFrequency,
    #[strum(to_string = "Wave Speed")]
    WaveSpeed,
    #[strum(to_string = "Wave Thickness")]
    WaveThickness,
    #[strum(to_string = "Wave Spread")]
    WavePhaseSpread,
}

// TODO sloppy impl
impl ModTarget {
    pub fn animation_type(self) -> Option<AnimationType> {
        match self {
            ModTarget::None => None,
            ModTarget::BouncingSpeed | ModTarget::BouncingRadius => {
                Some(AnimationType::BouncingBalls)
            }
            ModTarget::ScanSpeed | ModTarget::ScanWidth => Some(AnimationType::ScanLine),
            ModTarget::PulseSpeed
            | ModTarget::PulseLimit
            | ModTarget::PulseBeatMult
            | ModTarget::PulseRingCount
            | ModTarget::PulseRotation => Some(AnimationType::PulseBackground),
            ModTarget::WaveAmplitude
            | ModTarget::WaveFrequency
            | ModTarget::WaveSpeed
            | ModTarget::WaveThickness
            | ModTarget::WavePhaseSpread => Some(AnimationType::WaveLines),
        }
    }

    pub fn for_animation(self, animation_type: AnimationType) -> bool {
        match self.animation_type() {
            Some(target_animation) => target_animation == animation_type,
            None => true,
        }
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

#[derive(Clone, Serialize, Deserialize)]
pub struct ModMatrix {
    pub routes: Vec<ModRoute>,

    /// ±1.0 equals ±100% around the base value.
    pub amount: f32,
    pub amount_type: AmountType,
    pub wave: ModWave,
    /// Multiplier for the global beat clock (1.0 = 1 cycle per beat).
    pub freq_mul: f32,
    /// Phase offset in beats.
    pub phase: f32,
    pub enabled: bool,
    pub mod_route_placeholder: ModRoute,
}

impl Default for ModMatrix {
    fn default() -> Self {
        Self {
            routes: Default::default(),
            amount: 0.25,
            amount_type: AmountType::Plus,
            wave: ModWave::default(),
            freq_mul: 1.0,
            phase: 1.0,
            enabled: true,
            mod_route_placeholder: ModRoute::default(),
        }
    }
}

impl ModMatrix {
    pub fn has_target(&self, target: ModTarget) -> bool {
        self.routes.iter().any(|r| r.enabled && r.target == target)
    }

    pub fn set_enables(&mut self, state: bool, target: ModTarget) {
        if let Some(dev) = self.routes.iter_mut().find(|d| d.target == target) {
            dev.enabled = state;
        } else {
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, animation_type: AnimationType) {
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
        // todo higher steps
        // ui.add(egui::Slider::new(&mut self.phase, -4.0..=4.0).text("Phase (beats)"));

        egui::ComboBox::from_label("Wave")
            .selected_text(format!("{:?}", self.wave))
            .show_ui(ui, |ui| {
                for ss in ModWave::iter() {
                    ui.selectable_value(&mut self.wave, ss, format!("{}", ss));
                }
            });

        let mut remove_idx: Option<usize> = None;

        for (i, route) in self
            .routes
            .iter_mut()
            .enumerate()
            .filter(|(_, route)| route.target.for_animation(animation_type))
        {
            ui.separator();
            ui.horizontal(|ui| {
                ui.checkbox(&mut route.enabled, format!("{}", route.target));
                ui.add(egui::Slider::new(&mut route.amount, 0.0..=1.0).text("Depth"));
            });
        }
        if let Some(idx) = remove_idx {
            self.routes.remove(idx);
        }
    }

    pub fn calc_modulation(&self, beat_pos: f32, target: ModTarget) -> f32 {
        
        if !self.enabled {
            return 1.0;
        }
        // let lfo = sample_wave(self.wave, beat_pos * self.freq_mul + self.phase);
        let result = create_modulation(self.wave, beat_pos * self.freq_mul);
        let mut factor = 1.0;
        let mut amount = self.amount;

        if self.amount_type == AmountType::Minus {
            amount *= -1.0;
        }

        for route in self
            .routes
            .iter()
            .filter(|route| route.enabled && route.target == target)
        {
            factor *= 1.0 + (amount * route.amount) * result;
        }

        factor
    }
}

/// Sample a waveform at the given beat position. Output in [-1, 1].
pub fn create_modulation(wave: ModWave, beat_pos: f32) -> f32 {
    let phase = beat_pos * std::f32::consts::TAU;
    match wave {
        ModWave::Sine => phase.sin(),
        ModWave::Triangle => {
            // saw to triangle transform
            let saw = (phase / std::f32::consts::TAU).fract() * 2.0 - 1.0;
            2.0 * saw.abs() - 1.0
        }
        ModWave::Square => {
            if phase.sin() >= 0.0 {
                1.0
            } else {
                -1.0
            }
        }
        ModWave::RampUp => ((phase / std::f32::consts::TAU).fract()) * 2.0 - 1.0,
        ModWave::RampDown => 1.0 - ((phase / std::f32::consts::TAU).fract()) * 2.0,
        // TODO
        ModWave::Random => 0.0,
    }
}
