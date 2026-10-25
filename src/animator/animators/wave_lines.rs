use crate::{
    animator::{AnimatedObject, AnimatorSettings, ObjectShape, animation_type::AnimationType},
    color::ColorParam,
    parameters::{ConstantParam, ModulatedParam},
};

use crate::modulator::Modulator;
use crate::timecode::TimeCode;
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WaveLinesSettings {
    pub line_count: ConstantParam<u32>,
    pub amplitude: ModulatedParam,
    pub frequency: ModulatedParam,
    pub speed: ModulatedParam,
    pub thickness: ModulatedParam,
    pub phase_spread: ModulatedParam,
    pub h_amplitude: ModulatedParam,
    pub harmonic: ModulatedParam,
    pub decay: ModulatedParam,
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
            line_count: ConstantParam::new(14, 2, 60, "Lines", "lines"),
            amplitude: ModulatedParam::new(90.0, 5.0, 300.0, "Amplitude", "wave_amplitude"),
            frequency: ModulatedParam::new(0.018, 0.001, 0.12, "Frequency", "wave_frequency"),
            speed: ModulatedParam::new(1.5, 0.0, 10.0, "Speed", "wave_speed"),
            thickness: ModulatedParam::new(4.0, 1.0, 20.0, "Thickness", "wave_thickness"),
            phase_spread: ModulatedParam::new(0.0, -3.0, 3.0, "Phase Spread", "wave_spread"),
            h_amplitude: ModulatedParam::new(0.0, 0.0, 200.0, "H-Amplitude", "wave_h_amp"),
            harmonic: ModulatedParam::new(1.0, 1.0, 8.0, "Harmonic", "wave_harmonic"),
            decay: ModulatedParam::new(1.0, 0.0, 1.0, "Edge Decay", "wave_decay"),
            color: ColorParam::default(),
            width: win_rect.w(),
            height: win_rect.h(),
            animator: Vec::new(),
            visible: true,
        }
    }
}

impl AnimatorSettings for WaveLinesSettings {
    fn control_ui(&mut self, ui: &mut egui::Ui, modulators: &[Box<dyn Modulator>]) {
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
            for idx in current..target {
                self.animator.push(WaveLine::from_settings(self, idx));
            }
        } else if target < current {
            self.animator.truncate(target);
        }

        let amplitude = self.amplitude.value();
        let frequency = self.frequency.value();
        let speed = self.speed.value();
        let thickness = self.thickness.value();
        let phase_spread = self.phase_spread.value();
        let h_amplitude = self.h_amplitude.value();
        let harmonic = self.harmonic.value().round() as u32;
        let decay = self.decay.value();

        for line in self.animator.iter_mut() {
            line.total_lines = self.line_count.value;
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
            line.color = self.color.value_mapped(line.index);
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

    fn snapshot(&mut self, name: &str) -> anyhow::Result<()> {
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
    phase: f32,
    color: Rgba8,
    visible: bool,
}

impl WaveLine {
    fn from_settings(settings: &WaveLinesSettings, index: usize) -> Self {
        let amplitude = settings.amplitude.value();
        Self {
            index,
            total_lines: settings.line_count.value,
            width: settings.width,
            height: settings.height,
            amplitude_base: amplitude,
            amplitude_current: amplitude,
            frequency: settings.frequency.value(),
            speed: settings.speed.value(),
            thickness: settings.thickness.value(),
            phase_spread: settings.phase_spread.value(),
            harmonic: settings.harmonic.value().round() as u32,
            decay: settings.decay.value(),
            phase: random_range(0.0, TAU),
            color: settings.color.value_mapped(index),
            h_amplitude: settings.h_amplitude.value(),
            visible: settings.visible,
        }
    }

    fn base_x(&self) -> f32 {
        if self.total_lines == 0 {
            return 0.0;
        }
        let spacing = self.width / self.total_lines as f32;
        -self.width / 2.0 + spacing * (self.index as f32 + 0.5)
    }
}

impl AnimatedObject for WaveLine {
    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode) {
        self.width = win_rect.w();
        self.height = win_rect.h();

        self.phase += timecode.get_delta_time() * self.speed * TAU;
        self.amplitude_current = self.amplitude_base;
    }

    fn draw(&self, draw: &Draw) {
        let y_steps = 32.max((self.height / 20.0) as i32);
        let y_min = -self.height / 2.0;
        let y_max = self.height / 2.0;
        let base_x = self.base_x();
        let harm = self.harmonic.max(1) as f32;

        let points = (0..=y_steps).map(|i| {
            let t = i as f32 / y_steps as f32;
            let y = y_min + (y_max - y_min) * t;

            // Edge decay envelope — amplitude tapers off at top and bottom
            let decay_t = if self.decay < 1.0 {
                // smoothstep at edges
                let edge = 0.15;
                let lower = smoothstep(0.0, edge, t);
                let upper = smoothstep(1.0, 1.0 - edge, t);
                lower * upper
            } else {
                1.0
            };
            let decay_env = 1.0 - (1.0 - decay_t) * (1.0 - self.decay);
            let phase_offset = self.phase + self.phase_spread * self.index as f32;
            // Primary vertical wave (with harmonic multiplier)
            let x_v = (y * self.frequency * harm + phase_offset).sin()
                * self.amplitude_current
                * decay_env;
            // Secondary horizontal / Lissajous axis
            let x_h = (y * phase_offset * 0.5).cos() * self.h_amplitude;

            pt2(base_x + x_v + x_h, y)
        });

        if self.visible {
            draw.polyline()
                .weight(self.thickness)
                .color(self.color)
                .points(points);
        }
    }

    fn shape(&self) -> ObjectShape {
        let half_w = self.amplitude_current.abs() + self.thickness * 0.5;
        ObjectShape::Rect(Rect::from_x_y_w_h(
            self.base_x(),
            0.0,
            half_w * 2.0,
            self.height,
        ))
    }

    fn color(&self) -> Rgba8 {
        self.color
    }
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
