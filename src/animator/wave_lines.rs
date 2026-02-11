use super::{AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour};
use crate::{
    animator::{
        animation_type::AnimationType,
        animator_structs::AnimationParam,
        modulation::{ModMatrix, ModTarget},
        presets_manager::PresetManager,
    },
    color::ColorParam,
};

use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WaveLinesSettings {
    pub line_count: AnimationParam<u32>,
    pub amplitude: AnimationParam<f32>,
    pub frequency: AnimationParam<f32>,
    pub speed: AnimationParam<f32>,
    pub thickness: AnimationParam<f32>,
    pub phase_spread: AnimationParam<f32>,
    color: ColorParam,
    #[serde(skip)]
    width: f32,
    #[serde(skip)]
    height: f32,
    #[serde(skip)]
    presets: PresetManager<WaveLinesSettings>,
}

impl AnimatorSettings for WaveLinesSettings {
    fn new(win_rect: &Rect) -> Self {
        Self {
            line_count: AnimationParam::new(14, 2, 60, "lines"),
            amplitude: AnimationParam::new_modulate(
                90.0,
                5.0,
                260.0,
                "amplitude",
                ModTarget::WaveAmplitude,
            ),
            frequency: AnimationParam::new_modulate(
                0.018,
                0.003,
                0.08,
                "frequency",
                ModTarget::WaveFrequency,
            ),
            speed: AnimationParam::new_modulate(1.5, 0.1, 6.0, "speed", ModTarget::WaveSpeed),
            thickness: AnimationParam::new_modulate(
                4.0,
                1.0,
                14.0,
                "thickness",
                ModTarget::WaveThickness,
            ),
            phase_spread: AnimationParam::new_modulate(
                0.0,
                -2.0,
                2.0,
                "phase spread",
                ModTarget::WavePhaseSpread,
            ),
            color: ColorParam::default(),
            width: win_rect.w(),
            height: win_rect.h(),
            presets: PresetManager::new_animator(AnimationType::WaveLines),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut ModMatrix) -> UpdateBehaviour {
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

        if self.color.ui(ui) {
            change = UpdateBehaviour::HotUpdate;
        }

        if self.presets.ui(ui) {
            change = UpdateBehaviour::LoadPreset;
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
                    self.amplitude.value,
                    self.frequency.value,
                    self.speed.value,
                    self.thickness.value,
                    self.phase_spread.value,
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
                    self.amplitude.value,
                    self.frequency.value,
                    self.speed.value,
                    self.thickness.value,
                    self.phase_spread.value,
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
                line.amplitude_base = self.amplitude.value;
                line.amplitude_current = self.amplitude.value;
                line.frequency = self.frequency.value;
                line.speed = self.speed.value;
                line.thickness = self.thickness.value;
                line.phase_spread = self.phase_spread.value;
                line.color = self.color.clone().value_mapped(line.index);
            }
        }
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        self.presets.save_to_file(self, None)?;
        Ok(())
    }

    fn reset(&mut self) {}
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
    fn update(
        &mut self,
        win_rect: &Rect,
        delta_time: f32,
        clock: &crate::animator::timecode::TimeCode,
    ) {
        self.width = win_rect.w();
        self.height = win_rect.h();
        let beat = clock.get_beat_progress();
        let beat_amp = 1.0 + (beat * TAU).sin() * 0.12;

        self.phase += delta_time * self.speed * TAU;
        self.amplitude_current = self.amplitude_base * beat_amp;
    }

    fn draw(&self, draw: &Draw) {
        let y_steps = 32.max((self.height / 20.0) as i32);
        let y_min = -self.height / 2.0;
        let y_max = self.height / 2.0;
        let base_x = self.base_x();

        let points = (0..=y_steps).map(|i| {
            let t = i as f32 / y_steps as f32;
            let y = y_min + (y_max - y_min) * t;
            let phase_offset = self.phase + self.phase_spread * self.index as f32;
            let x = base_x + (y * self.frequency + phase_offset).sin() * self.amplitude_current;
            pt2(x, y)
        });

        draw.polyline()
            .weight(self.thickness)
            .color(self.color)
            .points(points);
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

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
