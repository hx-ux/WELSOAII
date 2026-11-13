use crate::{
    animator::{
        AnimatedObject, AnimatorSettings, ObjectShape,
        animation_type::{AnimationType, WaveMode},
    },
    color::{ColorPalette, ColorParam},
    parameters::{ConstantParam, ModulatedParam},
};

use crate::modulator::Modulator;
use crate::timecode::TimeCode;
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize)]
pub struct WaveLinesSettings {
    pub mode: WaveMode,
    pub line_count: ConstantParam<u32>,
    pub amplitude: ModulatedParam,
    pub frequency: ModulatedParam,
    pub speed: ModulatedParam,
    pub thickness: ModulatedParam,
    pub phase_spread: ModulatedParam,
    pub h_amplitude: ModulatedParam,
    pub harmonic: ModulatedParam,
    pub decay: ModulatedParam,
    pub ribbon_veil: ConstantParam<bool>,
    color: ColorParam,
    #[serde(skip)]
    width: f32,
    #[serde(skip)]
    height: f32,
    #[serde(skip)]
    pub animator: Vec<WaveLine>,
    visible: bool,
}

impl WaveLinesSettings {
    pub fn new(win_rect: &Rect) -> Self {
        Self {
            mode: WaveMode::default(),
            line_count: ConstantParam::new(24, 2, 60, "Ribbons", "lines"),
            amplitude: ModulatedParam::new(110.0, 5.0, 300.0, "Amplitude", "wave_amplitude"),
            frequency: ModulatedParam::new(0.015, 0.002, 0.10, "Frequency", "wave_frequency"),
            speed: ModulatedParam::new(1.15, 0.0, 8.0, "Flow Speed", "wave_speed"),
            thickness: ModulatedParam::new(3.5, 1.0, 20.0, "Thickness", "wave_thickness"),
            phase_spread: ModulatedParam::new(0.38, -3.0, 3.0, "Phase Spread", "wave_spread"),
            h_amplitude: ModulatedParam::new(36.0, 0.0, 200.0, "Overtone Swell", "wave_h_amp"),
            harmonic: ModulatedParam::new(2.0, 1.0, 8.0, "Harmonic", "wave_harmonic"),
            decay: ModulatedParam::new(0.22, 0.0, 1.0, "Edge Pinch", "wave_decay"),
            ribbon_veil: ConstantParam::new(
                true,
                false,
                true,
                "Aurora Veil & Pearls",
                "wave_veil",
            ),
            color: ColorParam::with_palette(ColorPalette::PastelHorizon),
            width: win_rect.w(),
            height: win_rect.h(),
            animator: Vec::new(),
            visible: true,
        }
    }
}

impl AnimatorSettings for WaveLinesSettings {
    fn control_ui(&mut self, ui: &mut egui::Ui, modulators: &[Box<dyn Modulator>]) {
        ui.label("Wave Weave:");
        ui.horizontal_wrapped(|ui| {
            for option in WaveMode::iter() {
                if ui
                    .radio_value(&mut self.mode, option, option.to_string())
                    .changed()
                {
                    self.hot_update();
                }
            }
        });

        if self.line_count.slider_ui(ui) {
            self.hot_update();
        }
        if self.amplitude.slider_ui(ui, modulators) {
            self.hot_update();
        }
        if self.frequency.slider_ui(ui, modulators) {
            self.hot_update();
        }
        if self.speed.slider_ui(ui, modulators) {
            self.hot_update();
        }
        if self.thickness.slider_ui(ui, modulators) {
            self.hot_update();
        }
        if self.phase_spread.slider_ui(ui, modulators) {
            self.hot_update();
        }
        if self.h_amplitude.slider_ui(ui, modulators) {
            self.hot_update();
        }
        if self.harmonic.slider_ui(ui, modulators) {
            self.hot_update();
        }
        if self.decay.slider_ui(ui, modulators) {
            self.hot_update();
        }
        if self.ribbon_veil.checkbox_ui(ui) {
            self.hot_update();
        }
        if self.color.ui(ui, modulators) {
            self.hot_update();
        }
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::WaveLines
    }

    fn init(&mut self) {
        self.animator.clear();
        for idx in 0..self.line_count.value as usize {
            self.animator.push(WaveLine::from_settings(self, idx));
        }
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.width = window_rect.w();
        self.height = window_rect.h();
    }

    fn hot_update(&mut self) {
        let current = self.animator.len();
        let target = self.line_count.value as usize;

        if target > current {
            let base_phase = self.animator.first().map(|l| l.phase).unwrap_or(0.0);
            for idx in current..target {
                let mut line = WaveLine::from_settings(self, idx);
                line.phase = base_phase;
                self.animator.push(line);
            }
        } else if target < current {
            self.animator.truncate(target);
        }

        let mode = self.mode;
        let amplitude = self.amplitude.value();
        let frequency = self.frequency.value();
        let speed = self.speed.value();
        let thickness = self.thickness.value();
        let phase_spread = self.phase_spread.value();
        let h_amplitude = self.h_amplitude.value();
        let harmonic = self.harmonic.value().round().max(1.0) as u32;
        let decay = self.decay.value();
        let ribbon_veil = self.ribbon_veil.value;

        for (idx, line) in self.animator.iter_mut().enumerate() {
            line.index = idx;
            line.mode = mode;
            line.total_lines = self.line_count.value.max(1);
            line.width = self.width;
            line.height = self.height;
            line.amplitude_base = amplitude;
            line.amplitude_current = amplitude;
            line.frequency = frequency;
            line.speed = speed;
            line.thickness = thickness;
            line.phase_spread = phase_spread;
            line.h_amplitude = h_amplitude;
            line.harmonic = harmonic;
            line.decay = decay;
            line.ribbon_veil = ribbon_veil;
            line.color = self.color.value_mapped(idx);
            line.twin_color = self.color.value_mapped(idx + 4);
            line.visible = self.visible;
        }
    }

    fn draw(&self, draw: &Draw) {
        for g in self.animator.iter() {
            g.draw(draw);
        }
    }

    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode) {
        for g in self.animator.iter_mut() {
            g.update(win_rect, timecode);
        }
    }

    fn get_objects(&self) -> Vec<&dyn AnimatedObject> {
        self.animator
            .iter()
            .map(|b| b as &dyn AnimatedObject)
            .collect()
    }

    fn get_objects_mut(&mut self) -> Vec<&mut dyn AnimatedObject> {
        self.animator
            .iter_mut()
            .map(|b| b as &mut dyn AnimatedObject)
            .collect()
    }

    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
        let mut params = vec![
            &mut self.amplitude,
            &mut self.frequency,
            &mut self.speed,
            &mut self.thickness,
            &mut self.phase_spread,
            &mut self.h_amplitude,
            &mut self.harmonic,
            &mut self.decay,
        ];
        params.extend(self.color.modulated_params_mut());
        params
    }

    fn snapshot(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn set_visibility(&mut self, state: bool) {
        self.visible = state;
        self.hot_update();
    }

    fn get_visibility(&self) -> bool {
        self.visible
    }
}

pub struct WaveLine {
    index: usize,
    mode: WaveMode,
    total_lines: u32,
    width: f32,
    height: f32,
    pub amplitude_base: f32,
    pub amplitude_current: f32,
    pub frequency: f32,
    pub speed: f32,
    pub thickness: f32,
    pub phase_spread: f32,
    pub h_amplitude: f32,
    pub harmonic: u32,
    pub decay: f32,
    pub ribbon_veil: bool,
    phase: f32,
    beat_kick: f32,
    color: Rgba8,
    twin_color: Rgba8,
    visible: bool,
}

impl WaveLine {
    fn from_settings(settings: &WaveLinesSettings, index: usize) -> Self {
        let amplitude = settings.amplitude.value();
        Self {
            index,
            mode: settings.mode,
            total_lines: settings.line_count.value.max(1),
            width: settings.width,
            height: settings.height,
            amplitude_base: amplitude,
            amplitude_current: amplitude,
            frequency: settings.frequency.value(),
            speed: settings.speed.value(),
            thickness: settings.thickness.value(),
            phase_spread: settings.phase_spread.value(),
            harmonic: settings.harmonic.value().round().max(1.0) as u32,
            decay: settings.decay.value(),
            ribbon_veil: settings.ribbon_veil.value,
            // Coherent starting phase across all ribbons so phase_spread sculpts a flowing 3D wave surface
            phase: 0.0,
            beat_kick: 0.0,
            color: settings.color.value_mapped(index),
            twin_color: settings.color.value_mapped(index + 4),
            h_amplitude: settings.h_amplitude.value(),
            visible: settings.visible,
        }
    }

    fn base_x(&self) -> f32 {
        if self.total_lines == 0 {
            return 0.0;
        }
        match self.mode {
            WaveMode::DnaBraid => {
                // Group ribbons into braided double-helix columns
                let pairs = (self.total_lines / 2).max(1);
                let pair_idx = (self.index as u32 / 2).min(pairs - 1);
                let spacing = (self.width * 0.84) / pairs as f32;
                -self.width * 0.42 + spacing * (pair_idx as f32 + 0.5)
            }
            _ => {
                let spacing = (self.width * 0.88) / self.total_lines as f32;
                -self.width * 0.44 + spacing * (self.index as f32 + 0.5)
            }
        }
    }

    fn base_y(&self) -> f32 {
        if self.total_lines == 0 {
            return 0.0;
        }
        let spacing = (self.height * 0.84) / self.total_lines as f32;
        -self.height * 0.42 + spacing * (self.index as f32 + 0.5)
    }

    fn sample_ribbon(&self, phase_shift: f32, amp_scale: f32) -> Vec<Point2> {
        let steps = 64usize;
        let harm = self.harmonic.max(1) as f32;
        let idx_norm = self.index as f32 / self.total_lines.max(1) as f32;
        let swell = (1.0 + self.beat_kick * 0.22) * amp_scale;
        let amp = self.amplitude_current * swell;
        let h_amp = self.h_amplitude * swell;

        let envelope = |t: f32| -> f32 {
            if self.decay < 0.999 {
                let edge = 0.22;
                let lower = smoothstep(0.0, edge, t);
                let upper = smoothstep(1.0, 1.0 - edge, t);
                let taper = lower * upper;
                1.0 - (1.0 - taper) * (1.0 - self.decay)
            } else {
                1.0
            }
        };

        match self.mode {
            WaveMode::AuroraSilk => {
                let y_min = -self.height * 0.5;
                let y_max = self.height * 0.5;
                let bx = self.base_x();
                let phase_offset =
                    self.phase + phase_shift + self.phase_spread * self.index as f32;

                (0..=steps)
                    .map(|i| {
                        let t = i as f32 / steps as f32;
                        let y = y_min + (y_max - y_min) * t;
                        let env = envelope(t);

                        let primary = (y * self.frequency + phase_offset).sin() * amp * env;
                        let overtone = (y * self.frequency * harm - phase_offset * 0.72
                            + idx_norm * TAU)
                            .cos()
                            * h_amp
                            * env;
                        let y_curl =
                            (y * self.frequency * 0.5 + phase_offset).cos() * h_amp * 0.14 * env;

                        pt2(bx + primary + overtone, y + y_curl)
                    })
                    .collect()
            }
            WaveMode::DnaBraid => {
                let y_min = -self.height * 0.5;
                let y_max = self.height * 0.5;
                let bx = self.base_x();
                let pair_idx = (self.index / 2) as f32;
                let strand_phase = if self.index.is_multiple_of(2) {
                    0.0
                } else {
                    PI
                };
                let phase_offset =
                    self.phase + phase_shift + pair_idx * self.phase_spread + strand_phase;

                (0..=steps)
                    .map(|i| {
                        let t = i as f32 / steps as f32;
                        let y = y_min + (y_max - y_min) * t;
                        let env = envelope(t);

                        let helix_x = (y * self.frequency * 1.15 + phase_offset).sin()
                            * (amp * 0.75)
                            * env;
                        let spine_sway = (y * self.frequency * 0.45 * harm - self.phase * 0.6
                            + pair_idx)
                            .cos()
                            * h_amp
                            * 0.85
                            * env;

                        pt2(bx + helix_x + spine_sway, y)
                    })
                    .collect()
            }
            WaveMode::CyberOcean => {
                let x_min = -self.width * 0.5;
                let x_max = self.width * 0.5;
                let by = self.base_y();
                let phase_offset =
                    self.phase + phase_shift + self.phase_spread * self.index as f32;

                (0..=steps)
                    .map(|i| {
                        let t = i as f32 / steps as f32;
                        let x = x_min + (x_max - x_min) * t;
                        let env = envelope(t);

                        let wave_y = (x * self.frequency + phase_offset).sin() * (amp * 0.75) * env;
                        let crest_y = (x * self.frequency * harm - phase_offset * 0.8
                            + idx_norm * TAU)
                            .cos()
                            * (h_amp * 0.75)
                            * env;
                        let surge_x =
                            (x * self.frequency + phase_offset).cos() * (h_amp * 0.25) * env;

                        pt2(x + surge_x, by + wave_y + crest_y)
                    })
                    .collect()
            }
        }
    }
}

impl AnimatedObject for WaveLine {
    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode) {
        self.width = win_rect.w();
        self.height = win_rect.h();

        self.phase += timecode.get_delta_time() * self.speed * TAU;
        self.beat_kick = (1.0 - timecode.get_beat_progress()).powi(3);
        self.amplitude_current = self.amplitude_base;
    }

    fn draw(&self, draw: &Draw) {
        if !self.visible {
            return;
        }

        let base_a = self.color.alpha as f32;
        let active_weight = self.thickness * (1.0 + self.beat_kick * 0.25);

        // 1. 3D Phase-Echo Aurora Veil
        if self.ribbon_veil {
            let veil_pts = self.sample_ribbon(-0.42, 0.88);
            draw.polyline()
                .weight(active_weight * 2.2)
                .color(Rgba8::new(
                    self.twin_color.red,
                    self.twin_color.green,
                    self.twin_color.blue,
                    (base_a * 0.24) as u8,
                ))
                .points(veil_pts);
        }

        let pts = self.sample_ribbon(0.0, 1.0);

        // 2. Soft neon bloom underlay
        draw.polyline()
            .weight(active_weight * 2.4)
            .color(Rgba8::new(
                self.color.red,
                self.color.green,
                self.color.blue,
                (base_a * 0.28) as u8,
            ))
            .points(pts.iter().copied());

        // 3. Crisp silk ribbon core
        draw.polyline()
            .weight(active_weight)
            .color(self.color)
            .points(pts.iter().copied());

        // 4. Shimmering harmonic pearls riding along the ribbon crests
        if self.ribbon_veil && pts.len() > 8 {
            let pearl_count = 3usize;
            let travel = (self.phase * 0.15).rem_euclid(1.0);
            let pearl_r = (active_weight * 0.9 + self.beat_kick * 2.2).clamp(2.0, 7.0);
            let pearl_a = (base_a * (0.65 + self.beat_kick * 0.3).min(1.0)) as u8;
            for p in 0..pearl_count {
                let frac = (travel + p as f32 / pearl_count as f32).rem_euclid(1.0);
                let idx = ((frac * (pts.len() - 1) as f32) as usize).min(pts.len() - 1);
                draw.ellipse()
                    .xy(pts[idx])
                    .radius(pearl_r)
                    .color(Rgba8::new(255, 250, 245, pearl_a));
            }
        }
    }

    fn shape(&self) -> ObjectShape {
        let span_amp = (self.amplitude_current.abs() + self.h_amplitude.abs() * 0.5
            + self.thickness)
            .max(12.0);
        match self.mode {
            WaveMode::CyberOcean => ObjectShape::Rect(Rect::from_x_y_w_h(
                0.0,
                self.base_y(),
                self.width,
                span_amp * 1.4,
            )),
            _ => ObjectShape::Rect(Rect::from_x_y_w_h(
                self.base_x(),
                0.0,
                span_amp * 1.4,
                self.height,
            )),
        }
    }

    fn color(&self) -> Rgba8 {
        self.color
    }
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
