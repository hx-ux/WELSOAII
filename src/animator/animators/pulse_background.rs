use crate::{
    animator::{
        AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour,
        animation_type::{AnimationType, PulseModes},
    },
    color::ColorParam,
};
use crate::{modulator::Modulator, parameters::ConstantParam};
use crate::{parameters::ModulatedParam, timecode::TimeCode};
use anyhow::Ok;
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize)]
pub struct PulseBackgroundSettings {
    pub ring_count: ConstantParam<u32>,
    pub mode: PulseModes,
    pub speed: ModulatedParam,
    pub color: ColorParam,
    pub limit: ModulatedParam,
    pub rotation_speed: ModulatedParam,
}

impl PulseBackgroundSettings {
    pub fn new(_win_rect: &Rect) -> Self {
        Self {
            mode: PulseModes::default(),
            speed: ModulatedParam::new(100.0, 1.0, 200.0, "Speed", "pulse_speed"),
            color: ColorParam::default(),
            limit: ModulatedParam::new(0.8, 0.1, 1.0, "Limit", "pulse_limit"),
            ring_count: ConstantParam::new(3, 1, 10, "Ring Count", "ring_count"),
            rotation_speed: ModulatedParam::new(0.5, 0.0, 3.0, "Rotation", "pulse_rotation"),
        }
    }
}

impl AnimatorSettings for PulseBackgroundSettings {
    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
        vec![&mut self.speed, &mut self.limit, &mut self.rotation_speed]
    }

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

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        vec![Box::new(PulseBackground::new(
            self.mode,
            self.color.clone().value_mapped(0),
            *self.speed.value(),
            *self.limit.value(),
            self.ring_count.value,
            *self.rotation_speed.value(),
            0,
        ))]
    }

    fn set_dimension(&mut self, _window_rect: &Rect) {}

    fn hot_update(&self, objects: &mut Vec<Box<dyn AnimatedObject>>) {
        for obj in objects.iter_mut() {
            if let Some(pulse_bg) = obj.as_any_mut().downcast_mut::<PulseBackground>() {
                pulse_bg.color = self.color.clone().value_mapped(pulse_bg.index);
                pulse_bg.speed = *self.speed.value();
                pulse_bg.mode = self.mode;
                pulse_bg.limit = *self.limit.value();
                pulse_bg.ring_count = self.ring_count.value as usize;
                pulse_bg.rotation_speed = *self.rotation_speed.value();
            }
        }
    }

    fn reset(&mut self) {
        self.ring_count.reset();
        self.speed.reset();
        self.limit.reset();
        self.rotation_speed.reset();
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn color_ui(&mut self, ui: &mut egui::Ui) -> UpdateBehaviour {
        let mut update = UpdateBehaviour::None;
        if self.color.ui(ui) {
            update = UpdateBehaviour::HotUpdate;
        }

        return update;
    }
}

pub struct PulseBackground {
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

impl PulseBackground {
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
            current_size_w: 20.0,
            current_size_h: 20.0,
            limit,
            index,
            ring_count: ring_count as usize,
            rotation_speed,
            rotation: 0.0,
        }
    }
}

impl AnimatedObject for PulseBackground {
    fn update(&mut self, win_rect: &Rect, delta_time: f32, clock: &TimeCode) {
        let min_w = 20.0;
        let min_h = 20.0;

        let max_w = win_rect.w();
        let max_h = win_rect.h();

        let max_w_allowed = max_w * self.limit;
        let max_h_allowed = max_h * self.limit;

        // Update rotation based on beat
        let beat_progress = clock.get_beat_progress();
        self.rotation += delta_time * self.rotation_speed;

        match self.mode {
            PulseModes::Smooth => {
                // Direct beat synchronization - pulse exactly on beat
                let beats = clock.get_beats();
                let beat_cycle = beats.fract();

                self.current_size_w = (beat_cycle * max_w_allowed).max(min_w);
                self.current_size_h = (beat_cycle * max_h_allowed).max(min_h);
            }
            PulseModes::Elastic => {
                let beat_phase = beat_progress * std::f32::consts::PI * 2.0;
                let normalized = (beat_phase.sin() + 1.0) * 0.5;

                // Apply easing curve for smooth, powerful pulsing
                let eased = normalized * normalized * (3.0 - 2.0 * normalized); // Smoothstep

                self.current_size_w = min_w + eased * (max_w_allowed - min_w);
                self.current_size_h = min_h + eased * (max_h_allowed - min_h);
            }
        }
    }

    fn draw(&self, draw: &Draw) {
        // Draw multiple concentric rings for awesome pulse effect
        for i in 0..self.ring_count {
            let ring_progress = (i as f32 + 1.0) / self.ring_count as f32;
            let ring_size_w = self.current_size_w * ring_progress;
            let ring_size_h = self.current_size_h * ring_progress;

            // Vary opacity for depth effect
            let alpha = (self.color.alpha as f32 * (1.0 - ring_progress * 0.5)) as u8;
            let mut ring_color = self.color;
            ring_color.alpha = alpha;

            // Add rotation to rings for extra visual interest
            let ring_rotation = self.rotation * (i as f32 + 1.0) * 0.1;

            draw.rect()
                .x(0.0)
                .y(0.0)
                .width(ring_size_w)
                .height(ring_size_h)
                .rotate(ring_rotation)
                .color(ring_color);
        }

        // Draw main pulse
        draw.rect()
            .x(0.0)
            .y(0.0)
            .width(self.current_size_w)
            .height(self.current_size_h)
            .rotate(self.rotation)
            .color(self.color);
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Rect(Rect::from_x_y_w_h(
            0.0,
            0.0,
            self.current_size_w,
            self.current_size_h,
        ))
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn color(&self) -> Rgba8 {
        self.color
    }
}
