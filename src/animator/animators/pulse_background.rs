use crate::{
    animator::{
        AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour,
        animation_type::{AnimationType, PulseModes, PulseShape},
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
    #[serde(skip)]
    pub animator: Vec<PulseBackgroundAnimator>, // Refactored to concrete type
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
            animator: Vec::new(),
        }
    }
}

impl AnimatorSettings for PulseBackgroundSettings {
    fn control_ui(
        &mut self,
        ui: &mut egui::Ui,
        modulators: &mut Vec<Box<dyn Modulator>>,
    ) -> UpdateBehaviour {
        let mut update = UpdateBehaviour::None;

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

        ui.label("Shape:");
        ui.horizontal(|ui| {
            for options in PulseShape::iter() {
                if ui
                    .radio_value(&mut self.shape, options, format!("{}", options))
                    .changed()
                {
                    update = UpdateBehaviour::HotUpdate;
                };
            }
        });

        if self.speed.to_slider(ui) {
            update = UpdateBehaviour::HotUpdate;
        }
        if self.limit.to_slider_modulate(ui, modulators) {
            update = UpdateBehaviour::HotUpdate;
        }

        if self.ring_count.to_slider(ui) {
            update = UpdateBehaviour::NeedsReset;
            self.init();
        }

        if self.ring_spread.to_slider_modulate(ui, modulators) {
            update = UpdateBehaviour::HotUpdate;
        }

        if self.rotation_speed.to_slider_modulate(ui, modulators) {
            update = UpdateBehaviour::HotUpdate;
        }

        update
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::PulseBackground
    }

    fn init(&mut self) {
        self.animator.clear();
        for index in 0..self.ring_count.value as usize {
            self.animator.push(PulseBackgroundAnimator::new(
                self.mode,
                self.shape,
                self.color.clone().value_mapped(index),
                self.speed.value,
                *self.limit.value(),
                *self.rotation_speed.value(),
                *self.ring_spread.value(),
                index,
            ));
        }
    }

    fn set_dimension(&mut self, _window_rect: &Rect) {}

    fn hot_update(&mut self) {
        for obj in self.animator.iter_mut() {
            obj.color = self.color.clone().value_mapped(obj.index);
            obj.speed = *&self.speed.value;
            obj.mode = self.mode;
            obj.shape = self.shape;
            obj.limit = *self.limit.value();
            obj.rotation_speed = *self.rotation_speed.value();
            obj.ring_spread = *self.ring_spread.value();
        }
    }

    fn reset(&mut self) {
        self.ring_count.reset();
        self.speed.reset();
        self.limit.reset();
        self.rotation_speed.reset();
        self.ring_spread.reset();
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
        vec![
            &mut self.limit,
            &mut self.rotation_speed,
            &mut self.ring_spread,
        ]
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn update_modulations(&mut self, beat_pos: f32, modulators: &mut Vec<Box<dyn Modulator>>) {
        for param in self.modulated_params_mut() {
            param.modulate(beat_pos, modulators);
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

impl PulseBackgroundAnimator {
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

impl AnimatedObject for PulseBackgroundAnimator {
    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode) {
        let min_size = 20.0;
        let max_w = win_rect.w() * self.limit;
        let max_h = win_rect.h() * self.limit;

        self.rotation += timecode.get_delta_time() * self.rotation_speed;

        match self.mode {
            PulseModes::Smooth => {
                let beat_cycle = (timecode.get_beats() * self.speed as f32).fract();
                self.current_size_w = (beat_cycle * max_w).max(min_size);
                self.current_size_h = (beat_cycle * max_h).max(min_size);
            }
            PulseModes::Elastic => {
                let beat_phase =
                    (timecode.get_beat_progress() * self.speed as f32) * std::f32::consts::PI;
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
        ObjectShape::Rect(Rect::from_w_h(self.current_size_w, self.current_size_h))
    }

    fn color(&self) -> Rgba8 {
        self.color
    }
}
