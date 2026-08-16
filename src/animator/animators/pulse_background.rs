use crate::{
    animator::{
        AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour,
        animation_type::{AnimationType, PulseModes},
    },
    color::ColorParam,
    modulator::Modulator,
    parameters::{ConstantParam, ModulatedParam},
    timecode::TimeCode,
};
use anyhow::Ok;
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

const LIMIT: f32 = 0.8;
const SPEED: f32 = 100.0;
const RINGCOUNT: u32 = 3;
const ROTATION: f32 = 0.5;

#[derive(Serialize, Deserialize)]
pub struct PulseBackgroundSettings {
    pub ring_count: ConstantParam<u32>,
    pub mode: PulseModes,
    pub speed: ModulatedParam,
    pub color: ColorParam,
    pub limit: ModulatedParam,
    pub rotation_speed: ModulatedParam,
    #[serde(skip)]
    pub animator: Vec<PulseBackgroundAnimator>, // Refactored to concrete type
}

impl PulseBackgroundSettings {
    pub fn new(_win_rect: &Rect) -> Self {
        Self {
            mode: PulseModes::default(),
            speed: ModulatedParam::new(SPEED, 1.0, 200.0, "Speed", "pulse_speed"),
            color: ColorParam::default(),
            limit: ModulatedParam::new(LIMIT, 0.1, 1.0, "Limit", "pulse_limit"),
            ring_count: ConstantParam::new(RINGCOUNT, 1, 10, "Ring Count", "ring_count"),
            rotation_speed: ModulatedParam::new(ROTATION, 0.0, 3.0, "Rotation", "pulse_rotation"),
            animator: Vec::new(),
        }
    }
}

impl AnimatorSettings for PulseBackgroundSettings {
    fn control_ui(&mut self, ui: &mut egui::Ui, mods: &mut Modulator) -> UpdateBehaviour {
        let mut update = UpdateBehaviour::None;

        ui.add_space(5.0);
        ui.label("Mode:");

        ui.horizontal(|ui| {
            for options in PulseModes::iter() {
                if ui
                    .radio_value(&mut self.mode, options, format!("{}", options))
                    .changed()
                {
                    update = UpdateBehaviour::NeedsReset;
                };
            }
        });

        if self.speed.to_slider_modulate(ui, mods) {
            update = UpdateBehaviour::HotUpdate;
        }
        if self.limit.to_slider_modulate(ui, mods) {
            update = UpdateBehaviour::HotUpdate;
        }
        if self.ring_count.to_slider(ui) {
            update = UpdateBehaviour::HotUpdate;
        }
        if self.rotation_speed.to_slider_modulate(ui, mods) {
            update = UpdateBehaviour::HotUpdate;
        }

        update
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::PulseBackground
    }

    fn init(&mut self) {
        self.animator.push(PulseBackgroundAnimator::new(
            self.mode,
            self.color.clone().value_mapped(0),
            *self.speed.value(),
            *self.limit.value(),
            self.ring_count.value,
            *self.rotation_speed.value(),
            0,
        ));
    }

    fn set_dimension(&mut self, _window_rect: &Rect) {}

    fn hot_update(&mut self) {
        // self.animator.color = self.color.clone().value_mapped(self.animator.index);
        // self.animator.speed = *self.speed.value();
        // self.animator.mode = self.mode;
        // self.animator.limit = *self.limit.value();
        // self.animator.ring_count = self.ring_count.value as usize;
        // self.animator.rotation_speed = *self.rotation_speed.value();
    }

    fn reset(&mut self) {
        self.ring_count.reset();
        self.speed.reset();
        self.limit.reset();
        self.rotation_speed.reset();
    }

    fn draw(&self, draw: &Draw) {
        for g in self.animator.iter() {
            g.draw(draw);
        }
    }

    fn update(&mut self, win_rect: &Rect, delta_time: f32, timecode: &TimeCode) {
        for g in self.animator.iter_mut() {
            g.update(win_rect, delta_time, timecode);
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
        vec![&mut self.speed, &mut self.limit, &mut self.rotation_speed]
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn connect_modulations(&mut self, mod_matrix: &mut Modulator) {
        for param in self.modulated_params_mut() {
            param.connect_modulation(mod_matrix);
        }
    }

    fn update_modulations(&mut self, beat_pos: f32, mod_matrix: &Modulator) {
        for param in self.modulated_params_mut() {
            param.modulate(beat_pos, mod_matrix);
        }
    }

    fn reset_modulations(&mut self) {
        for param in self.modulated_params_mut() {
            param.ghost_value = None;
        }
    }

    fn color_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| self.color.ui(ui));
    }
}

pub struct PulseBackgroundAnimator {
    mode: PulseModes,
    pub color: Rgba8,
    pub speed: f32,
    current_size_w: f32,
    current_size_h: f32,
    pub limit: f32,
    index: usize,
    pub ring_count: usize,
    pub rotation_speed: f32,
    rotation: f32,
}

impl Default for PulseBackgroundAnimator {
    fn default() -> Self {
        Self::new(
            PulseModes::default(),
            Rgba8::new(0, 0, 0, 0),
            0.0,
            0.0,
            0,
            0.0,
            0,
        )
    }
}

impl PulseBackgroundAnimator {
    fn new(
        mode: PulseModes,
        color: Rgba8,
        speed: f32,
        limit: f32,
        ring_count: u32,
        rotation_speed: f32,
        index: usize,
    ) -> Self {
        Self {
            mode,
            speed,
            color,
            limit,
            index,
            rotation_speed,
            current_size_w: 20.0,
            current_size_h: 20.0,
            ring_count: ring_count as usize,
            rotation: 0.0,
        }
    }
}

impl AnimatedObject for PulseBackgroundAnimator {
    fn update(&mut self, win_rect: &Rect, delta_time: f32, timecode: &TimeCode) {
        let min_w = 20.0;
        let min_h = 20.0;
        let max_w_allowed = win_rect.w() * self.limit;
        let max_h_allowed = win_rect.h() * self.limit;

        let beat_progress = timecode.get_beat_fract();
        self.rotation += delta_time * self.rotation_speed;

        match self.mode {
            PulseModes::Smooth => {
                let beat_cycle = timecode.get_beats().fract();
                self.current_size_w = (beat_cycle * max_w_allowed).max(min_w);
                self.current_size_h = (beat_cycle * max_h_allowed).max(min_h);
            }
            PulseModes::Elastic => {
                let beat_phase = beat_progress * std::f32::consts::PI * 2.0;
                let normalized = (beat_phase.sin() + 1.0) * 0.5;
                let eased = normalized * normalized * (3.0 - 2.0 * normalized);
                self.current_size_w = min_w + eased * (max_w_allowed - min_w);
                self.current_size_h = min_h + eased * (max_h_allowed - min_h);
            }
        }
    }

    fn draw(&self, draw: &Draw) {
        for i in 0..self.ring_count {
            let ring_progress = (i as f32 + 1.0) / self.ring_count as f32;
            let mut ring_color = self.color;
            ring_color.alpha = (self.color.alpha as f32 * (1.0 - ring_progress * 0.5)) as u8;
            let ring_rotation = self.rotation * (i as f32 + 1.0) * 0.1;

            draw.rect()
                .w_h(
                    self.current_size_w * ring_progress,
                    self.current_size_h * ring_progress,
                )
                .rotate(ring_rotation)
                .color(ring_color);
        }

        draw.rect()
            .w_h(self.current_size_w, self.current_size_h)
            .rotate(self.rotation)
            .color(self.color);
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Rect(Rect::from_w_h(self.current_size_w, self.current_size_h))
    }

    fn color(&self) -> Rgba8 {
        self.color
    }
}
