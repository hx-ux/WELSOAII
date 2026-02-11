use super::modulation::ModTarget;
use super::{AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour};
use crate::animator::animation_type::{AnimationType, PulseModes};
use crate::animator::animator_structs::AnimationParam;
use crate::animator::modulation::ModMatrix;
use crate::animator::presets_manager::PresetManager;
use crate::color::ColorParam;
use nannou::prelude::*;
use nannou_egui::egui::{self};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize)]
pub struct PulseBackgroundSettings {
    pub mode: PulseModes,
    pub speed: AnimationParam<f32>,
    pub color: ColorParam,
    pub limit: AnimationParam<f32>,
    // pub beat_multiplier: AnimationParam<f32>,
    pub ring_count: AnimationParam<u32>,
    pub rotation_speed: AnimationParam<f32>,
    #[serde(skip)]
    pub presets: PresetManager<PulseBackgroundSettings>,
}

impl AnimatorSettings for PulseBackgroundSettings {
    fn new(win_rect: &Rect) -> Self {
        Self {
            mode: PulseModes::default(),
            speed: AnimationParam::new_modulate(100.0, 1.0, 200.0, "speed", ModTarget::PulseSpeed),
            color: ColorParam::default(),
            limit: AnimationParam::new_modulate(0.8, 0.1, 1.0, "limit", ModTarget::PulseLimit),
            // beat_multiplier: AnimationParam::new(1.0, 0.0, 4.0, "beat_mult"),
            ring_count: AnimationParam::new_modulate(
                3,
                1,
                10,
                "ring_count",
                ModTarget::PulseRingCount,
            ),
            rotation_speed: AnimationParam::new_modulate(
                0.5,
                0.0,
                3.0,
                "rotation",
                ModTarget::PulseRotation,
            ),
            presets: PresetManager::new_animator(AnimationType::PulseBackground),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut ModMatrix) -> UpdateBehaviour {
        let mut change_type = UpdateBehaviour::None;

        ui.heading(format!("{}", self.animation_type()));

        ui.add_space(5.0);
        ui.label("Mode:");

        ui.horizontal(|ui| {
            for options in PulseModes::iter() {
                if ui
                    .radio_value(&mut self.mode, options, format!("{}", options))
                    .changed()
                {
                    change_type = UpdateBehaviour::NeedsReset;
                };
            }
        });

        if self.speed.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.limit.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.ring_count.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.rotation_speed.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.presets.ui(ui) {
            change_type = UpdateBehaviour::LoadPreset
        }

        if self.color.ui(ui) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        change_type
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::PulseBackground
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        let mut animated_objects: Vec<Box<dyn AnimatedObject>> = Vec::new();

        animated_objects.push(Box::new(PulseBackground::new(
            self.mode,
            self.color.clone().value_mapped(0),
            self.speed.value,
            self.limit.value,
            self.ring_count.value,
            self.rotation_speed.value,
            0,
        )));

        animated_objects
    }

    fn set_dimension(&mut self, window_rect: &Rect) {}

    fn hot_update(&self, objects: &mut Vec<Box<dyn AnimatedObject>>) {
        for obj in objects.iter_mut() {
            if let Some(pulse_bg) = obj.as_any_mut().downcast_mut::<PulseBackground>() {
                pulse_bg.color = self.color.clone().value_mapped(pulse_bg.index);
                pulse_bg.speed = self.speed.value;
                pulse_bg.mode = self.mode;
                pulse_bg.limit = self.limit.value;
                pulse_bg.ring_count = self.ring_count.value as usize;
                pulse_bg.rotation_speed = self.rotation_speed.value;
            }
        }
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        self.presets.save_to_file(self, None)?;
        Ok(())
    }

    fn reset(&mut self) {
        todo!()
    }
}

pub struct PulseBackground {
    mode: PulseModes,
    pub color: Rgba8,
    pub speed: f32,
    current_size_w: f32,
    current_size_h: f32,
    time: f32,
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
            time: 0.0,
            limit,
            index,
            ring_count: ring_count as usize,
            rotation_speed,
            rotation: 0.0,
        }
    }
}

impl AnimatedObject for PulseBackground {
    fn update(
        &mut self,
        win_rect: &Rect,
        delta_time: f32,
        clock: &crate::animator::timecode::TimeCode,
    ) {
        let min_w = 20.0;
        let min_h = 20.0;

        let max_w = win_rect.w();
        let max_h = win_rect.h();

        let max_w_allowed = max_w * self.limit;
        let max_h_allowed = max_h * self.limit;

        self.time += delta_time;

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

        // Add corner highlights for extra visual pop
        if self.current_size_w > 100.0 {
            let highlight_color = rgba8(255, 255, 255, 80);
            let offset = self.current_size_w * 0.4;

            // Four corner highlights
            let corners = [
                vec2(offset, offset),
                vec2(-offset, offset),
                vec2(offset, -offset),
                vec2(-offset, -offset),
            ];

            for corner in corners.iter() {
                draw.ellipse()
                    .xy(*corner)
                    .radius(20.0)
                    .color(highlight_color);
            }
        }
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
