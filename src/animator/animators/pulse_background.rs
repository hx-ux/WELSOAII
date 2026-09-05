use crate::parameters::ConstantParam;
use crate::{
    animator::{
        AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour,
        animation_type::{AnimationType, PulseModes, PulseShape},
    },
    color::ColorParam,
    modulator::Modulator,
    parameters::ModulatedParam,
    timecode::TimeCode,
};
use anyhow::Ok;
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize)]
pub struct PulseBackgroundSettings {
    pub ring_count: ConstantParam<u32>,
    pub mode: PulseModes,
    pub shape: PulseShape,
    pub speed: ConstantParam<u8>,
    pub color: ColorParam,
    pub limit: ModulatedParam,
    pub rotation_speed: ModulatedParam,
    pub ring_spread: ModulatedParam,
}

impl PulseBackgroundSettings {
    pub fn new(_win_rect: &Rect) -> Self {
        Self {
            mode: PulseModes::default(),
            shape: PulseShape::default(),
            speed: ConstantParam::new(1, 1, 2, "Speed", "pulse_speed"),
            color: ColorParam::default(),
            limit: ModulatedParam::new(0.2, 0.1, 1.0, "Limit", "pulse_limit"),
            ring_count: ConstantParam::new(3, 1, 12, "Ring Count", "ring_count"),
            rotation_speed: ModulatedParam::new(1.0, 0.0, 6.0, "Rotation", "pulse_rotation"),
            ring_spread: ModulatedParam::new(1.0, 0.3, 3.0, "Ring Spread", "pulse_ring_spread"),
        }
    }
}

impl AnimatorSettings for PulseBackgroundSettings {
    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
        vec![
            &mut self.limit,
            &mut self.rotation_speed,
            &mut self.ring_spread,
        ]
    }

    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut Vec<Box<dyn Modulator>>) -> UpdateBehaviour {
        let mut change_type = UpdateBehaviour::None;

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

        ui.label("Shape:");
        ui.horizontal(|ui| {
            for options in PulseShape::iter() {
                if ui
                    .radio_value(&mut self.shape, options, format!("{}", options))
                    .changed()
                {
                    change_type = UpdateBehaviour::HotUpdate;
                };
            }
        });

        if self.speed.to_slider(ui) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.limit.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.ring_count.to_slider(ui) {
            change_type = UpdateBehaviour::NeedsReset;
        }

        if self.ring_spread.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.rotation_speed.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
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

        for index in 0..self.ring_count.value as usize {
            let k = Box::new(PulseBackground::new(
                self.mode,
                self.shape,
                self.color.clone().value_mapped(index),
                self.speed.value,
                *self.limit.value(),
                *self.rotation_speed.value(),
                *self.ring_spread.value(),
                index,
            ));

            animated_objects.push(k);
        }
        animated_objects
    }

    fn set_dimension(&mut self, _window_rect: &Rect) {}

    fn hot_update(&self, objects: &mut Vec<Box<dyn AnimatedObject>>) {
        for obj in objects.iter_mut() {
            if let Some(pulse_bg) = obj.as_any_mut().downcast_mut::<PulseBackground>() {
                pulse_bg.color = self.color.clone().value_mapped(pulse_bg.index);
                pulse_bg.speed = *&self.speed.value;
                pulse_bg.mode = self.mode;
                pulse_bg.shape = self.shape;
                pulse_bg.limit = *self.limit.value();
                pulse_bg.rotation_speed = *self.rotation_speed.value();
                pulse_bg.ring_spread = *self.ring_spread.value();
            }
        }
    }

    fn reset(&mut self) {
        self.ring_count.reset();
        self.speed.reset();
        self.limit.reset();
        self.rotation_speed.reset();
        self.ring_spread.reset();
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

pub struct PulseBackground {
    mode: PulseModes,
    pub shape: PulseShape,
    pub color: Rgba8,
    pub speed: u8,
    current_size_w: f32,
    current_size_h: f32,
    pub limit: f32,
    index: usize,
    pub rotation_speed: f32,
    pub ring_spread: f32,
    rotation: f32,
}

impl PulseBackground {
    fn new(
        mode: PulseModes,
        shape: PulseShape,
        color: Rgba8,
        speed: u8,
        limit: f32,
        rotation_speed: f32,
        ring_spread: f32,
        index: usize,
    ) -> Self {
        Self {
            mode,
            shape,
            speed,
            color,
            current_size_w: 20.0,
            current_size_h: 20.0,
            limit,
            index,
            rotation_speed,
            ring_spread,
            rotation: 0.0,
        }
    }
}

impl AnimatedObject for PulseBackground {
    fn update(&mut self, win_rect: &Rect, clock: &TimeCode) {
        let min_size = 20.0;
        let max_w = win_rect.w() * self.limit;
        let max_h = win_rect.h() * self.limit;

        self.rotation += clock.get_delta_time() * self.rotation_speed;

        match self.mode {
            PulseModes::Smooth => {
                let beat_cycle = (clock.get_beats() * self.speed as f32).fract();
                self.current_size_w = (beat_cycle * max_w).max(min_size);
                self.current_size_h = (beat_cycle * max_h).max(min_size);
            }
            PulseModes::Elastic => {
                let beat_phase =
                    (clock.get_beat_progress() * self.speed as f32) * std::f32::consts::PI;
                let normalized = (beat_phase.sin() + 1.0) * 0.5;
                let eased = normalized * normalized * (3.0 - 2.0 * normalized);
                self.current_size_w = min_size + eased * (max_w - min_size);
                self.current_size_h = min_size + eased * (max_h - min_size);
            }
        }
    }

    fn draw(&self, draw: &Draw) {
        let cx = 0.0;
        let cy = 0.0;

        let spread = self.ring_spread;

        let ring_size_w = self.current_size_w * spread * (1.0 + self.index as f32 * 0.3);
        let ring_size_h = self.current_size_h * spread * (1.0 + self.index as f32 * 0.3);
        let ring_color = self.color;
        let ring_rotation = self.rotation * (self.index as f32 + 1.0) * 0.15;

        let cos_r = ring_rotation.cos();
        let sin_r = ring_rotation.sin();
        let rotate = |p: Point2| -> Point2 {
            let dx = p.x - cx;
            let dy = p.y - cy;
            pt2(cx + dx * cos_r - dy * sin_r, cy + dx * sin_r + dy * cos_r)
        };

        match self.shape {
            PulseShape::Square => {
                let hw = ring_size_w * 0.5;
                let hh = ring_size_h * 0.5;
                let pts: Vec<Point2> = [
                    pt2(cx - hw, cy - hh),
                    pt2(cx + hw, cy - hh),
                    pt2(cx + hw, cy + hh),
                    pt2(cx - hw, cy + hh),
                    pt2(cx - hw, cy - hh),
                ]
                .iter()
                .map(|&p| rotate(p))
                .collect();
                draw.polyline().weight(2.0).color(ring_color).points(pts);
            }
            PulseShape::Circle => {
                let segments = 64usize;
                let rx = ring_size_w * 0.5;
                let ry = ring_size_h * 0.5;
                let mut pts: Vec<Point2> = (0..=segments)
                    .map(|i| {
                        let angle = i as f32 / segments as f32 * std::f32::consts::TAU;
                        rotate(pt2(cx + rx * angle.cos(), cy + ry * angle.sin()))
                    })
                    .collect();
                pts.push(pts[0]);
                draw.polyline().weight(2.0).color(ring_color).points(pts);
            }
            PulseShape::Diamond => {
                let hw = ring_size_w * 0.5;
                let hh = ring_size_h * 0.5;
                let pts: Vec<Point2> = [
                    pt2(cx, cy + hh),
                    pt2(cx + hw, cy),
                    pt2(cx, cy - hh),
                    pt2(cx - hw, cy),
                    pt2(cx, cy + hh),
                ]
                .iter()
                .map(|&p| rotate(p))
                .collect();
                draw.polyline().weight(2.0).color(ring_color).points(pts);
            }
        }
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Rect(Rect::from_x_y_w_h(
            0.0,
            0.0,
            self.current_size_w * self.ring_spread * (1.0 + self.index as f32 * 0.3),
            self.current_size_h * self.ring_spread * (1.0 + self.index as f32 * 0.3),
        ))
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn color(&self) -> Rgba8 {
        self.color
    }
}
