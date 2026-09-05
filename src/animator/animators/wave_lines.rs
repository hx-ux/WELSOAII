use crate::{
    animator::{
        AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour,
        animation_type::AnimationType,
    },
    color::ColorParam,
    modulator::Modulator,
    parameters::{ConstantParam, ModulatedParam},
};

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
        }
    }
}

impl AnimatorSettings for WaveLinesSettings {
    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
        vec![
            &mut self.amplitude,
            &mut self.frequency,
            &mut self.speed,
            &mut self.thickness,
            &mut self.phase_spread,
            &mut self.h_amplitude,
            &mut self.harmonic,
            &mut self.decay,
        ]
    }

    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut Vec<Box<dyn Modulator>>) -> UpdateBehaviour {
        let mut change = UpdateBehaviour::None;
        ui.heading(format!("{}", self.animation_type()));

        if self.line_count.to_slider(ui) {
            change = UpdateBehaviour::NeedsReset;
        }
        if self.amplitude.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.frequency.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.speed.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.thickness.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.phase_spread.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }

        ui.separator();
        ui.label("Lissajous / Harmonic");

        if self.h_amplitude.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.harmonic.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.decay.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }

        if self.color.ui(ui) {
            change = UpdateBehaviour::HotUpdate;
        }

        change
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::WaveLines
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        (0..self.line_count.value)
            .map(|idx| {
                Box::new(WaveLine::new(
                    idx as usize,
                    self.line_count.value,
                    self.width,
                    self.height,
                    *self.amplitude.value(),
                    *self.frequency.value(),
                    *self.speed.value(),
                    *self.thickness.value(),
                    *self.phase_spread.value(),
                    *self.h_amplitude.value(),
                    *self.harmonic.value(),
                    *self.decay.value(),
                    self.color.clone().value_mapped(idx as usize),
                )) as Box<dyn AnimatedObject>
            })
            .collect()
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.width = window_rect.w();
        self.height = window_rect.h();
    }

    fn hot_update(&self, objects: &mut Vec<Box<dyn AnimatedObject>>) {
        let current = objects.len();
        let target = self.line_count.value as usize;

        if target > current {
            for idx in current..target {
                objects.push(Box::new(WaveLine::new(
                    idx,
                    self.line_count.value,
                    self.width,
                    self.height,
                    *self.amplitude.value(),
                    *self.frequency.value(),
                    *self.speed.value(),
                    *self.thickness.value(),
                    *self.phase_spread.value(),
                    *self.h_amplitude.value(),
                    *self.harmonic.value(),
                    *self.decay.value(),
                    self.color.clone().value_mapped(idx),
                )));
            }
        } else if target < current {
            objects.truncate(target);
        }

        for obj in objects.iter_mut() {
            if let Some(line) = obj.as_any_mut().downcast_mut::<WaveLine>() {
                line.total_lines = self.line_count.value;
                line.width = self.width;
                line.height = self.height;
                line.amplitude_base = *self.amplitude.value();
                line.amplitude_current = *self.amplitude.value();
                line.frequency = *self.frequency.value();
                line.speed = *self.speed.value();
                line.thickness = *self.thickness.value();
                line.phase_spread = *self.phase_spread.value();
                line.h_amplitude = *self.h_amplitude.value();
                line.harmonic = (*self.harmonic.value()).round() as u32;
                line.decay = *self.decay.value();
                line.color = self.color.clone().value_mapped(line.index);
            }
        }
    }

    fn reset(&mut self) {
        self.line_count.reset();
        self.amplitude.reset();
        self.frequency.reset();
        self.speed.reset();
        self.thickness.reset();
        self.phase_spread.reset();
        self.h_amplitude.reset();
        self.harmonic.reset();
        self.decay.reset();
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

impl WaveLinesSettings {}

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
}

impl WaveLine {
    fn new(
        index: usize,
        total_lines: u32,
        width: f32,
        height: f32,
        amplitude: f32,
        frequency: f32,
        speed: f32,
        thickness: f32,
        phase_spread: f32,
        h_amplitude: f32,
        harmonic: f32,
        decay: f32,
        color: Rgba8,
    ) -> Self {
        Self {
            index,
            total_lines,
            width,
            height,
            amplitude_base: amplitude,
            amplitude_current: amplitude,
            frequency,
            speed,
            thickness,
            phase_spread,
            h_amplitude,
            harmonic: harmonic.round() as u32,
            decay,
            phase: random_range(0.0, TAU),
            color,
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
    fn update(&mut self, win_rect: &Rect, clock: &TimeCode) {
        self.width = win_rect.w();
        self.height = win_rect.h();

        self.phase += clock.get_delta_time() * self.speed * TAU;
        self.amplitude_current = self.amplitude_base;
    }

    fn draw(&self, draw: &Draw) {
        let y_steps = 48.max((self.height / 15.0) as i32);
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

        draw.polyline()
            .weight(self.thickness)
            .color(self.color)
            .points(points);
    }

    fn shape(&self) -> ObjectShape {
        let half_w = self.amplitude_current.abs() + self.h_amplitude.abs() + self.thickness * 0.5;
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

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Smoothstep helper
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
