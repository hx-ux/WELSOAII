use crate::{
    animator::{
        AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour,
        animation_type::AnimationType,
    },
    color::ColorParam,
    modulator::Modulator,
    parameters::{ConstantParam, ModulatedParam},
    timecode::TimeCode,
};

use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};

fn default_rect() -> Rect {
    Rect::from_w_h(800.0, 600.0)
}

#[derive(Serialize, Deserialize)]
pub struct QuantumTunnelSettings {
    pub ring_count: ConstantParam<u32>,
    pub speed: ModulatedParam,
    pub depth: ModulatedParam,
    pub twist: ModulatedParam,
    #[serde(skip)]
    #[serde(default = "default_rect")]
    dimension: Rect,
    color: ColorParam,
}

impl QuantumTunnelSettings {
    pub fn new(win_rect: &Rect) -> Self {
        Self {
            ring_count: ConstantParam::new(30, 5, 100, "Ring Count", "ring_count"),
            speed: ModulatedParam::new(2.0, 0.1, 10.0, "Speed", "tunnel_speed"),
            depth: ModulatedParam::new(10.0, 1.0, 50.0, "Depth", "tunnel_depth"),
            twist: ModulatedParam::new(0.5, -3.0, 3.0, "Twist", "tunnel_twist"),
            dimension: *win_rect,
            color: ColorParam::default(),
        }
    }
}

impl AnimatorSettings for QuantumTunnelSettings {
    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
        vec![&mut self.speed, &mut self.depth, &mut self.twist]
    }

    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut Vec<Box<dyn Modulator>>) -> UpdateBehaviour {
        let mut change_type = UpdateBehaviour::None;

        ui.heading(format!("{}", self.animation_type()));

        if self.ring_count.to_slider(ui) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.speed.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.depth.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.twist.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.color.ui(ui) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        change_type
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::QuantumTunnel
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        let mut animated_objects: Vec<Box<dyn AnimatedObject>> = Vec::new();

        for index in 0..self.ring_count.value as usize {
            animated_objects.push(Box::new(TunnelRing::new(
                &self.dimension,
                self.color.clone().value_mapped(index),
                *self.speed.value(),
                *self.depth.value(),
                *self.twist.value(),
                index,
                self.ring_count.value as usize,
            )));
        }
        animated_objects
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.dimension = *window_rect;
    }

    fn hot_update(&self, objects: &mut Vec<Box<dyn AnimatedObject>>) {
        let current_count = objects.len();
        let target_count = self.ring_count.value as usize;

        if target_count > current_count {
            for index in current_count..target_count {
                objects.push(Box::new(TunnelRing::new(
                    &self.dimension,
                    self.color.clone().value_mapped(index),
                    *self.speed.value(),
                    *self.depth.value(),
                    *self.twist.value(),
                    index,
                    target_count,
                )));
            }
        } else if target_count < current_count {
            objects.truncate(target_count);
        }

        for obj in objects.iter_mut() {
            if let Some(ring) = obj.as_any_mut().downcast_mut::<TunnelRing>() {
                ring.speed = *self.speed.value();
                ring.depth = *self.depth.value();
                ring.twist = *self.twist.value();
                ring.total = target_count;
                ring.color = self.color.clone().value_mapped(ring.index);
            }
        }
    }

    fn reset(&mut self) {
        self.ring_count.reset();
        self.speed.reset();
        self.depth.reset();
        self.twist.reset();
    }
}

pub struct TunnelRing {
    pub position: Vec2,
    pub speed: f32,
    pub depth: f32,
    pub twist: f32,
    pub color: Rgba8,
    pub index: usize,
    pub total: usize,
    pub current_z: f32,
    pub center: Vec2,
}

impl TunnelRing {
    pub fn new(
        win_rect: &Rect,
        color: Rgba8,
        speed: f32,
        depth: f32,
        twist: f32,
        index: usize,
        total: usize,
    ) -> Self {
        let z = (index as f32 / total as f32) * depth;
        Self {
            position: vec2(0.0, 0.0),
            speed,
            depth,
            twist,
            color,
            index,
            total,
            current_z: z,
            center: vec2(win_rect.x(), win_rect.y()),
        }
    }
}

impl AnimatedObject for TunnelRing {
    fn update(&mut self, win_rect: &Rect, clock: &TimeCode) {
        self.center = vec2(win_rect.x(), win_rect.y());

        self.current_z -= self.speed * clock.get_delta_time();
        if self.current_z <= 0.0 {
            self.current_z += self.depth;
        } else if self.current_z > self.depth {
            self.current_z -= self.depth;
        }

        let z_normalized = self.current_z / self.depth;
        let time = clock.get_beats();

        let angle = time * self.twist + (z_normalized * std::f32::consts::PI * 4.0);
        let offset_x = angle.cos() * (1.0 - z_normalized) * win_rect.w() * 0.2;
        let offset_y = angle.sin() * (1.0 - z_normalized) * win_rect.h() * 0.2;

        self.position = self.center + vec2(offset_x, offset_y);
    }

    fn draw(&self, draw: &Draw) {
        let z_normalized = self.current_z / self.depth;
        let size = (1.0 - z_normalized).powi(2) * 400.0;
        let line_weight = (1.0 - z_normalized) * 10.0 + 1.0;

        let mut c = self.color;
        // avoid subtraction with overflow
        let alpha = (255.0 * (1.0 - z_normalized)) as u8;
        c = Rgba8::new(c.red, c.green, c.blue, alpha);

        draw.ellipse()
            .xy(self.position)
            .radius(size)
            .no_fill()
            .stroke_weight(line_weight)
            .stroke_color(c);
    }

    fn shape(&self) -> ObjectShape {
        let z_normalized = self.current_z / self.depth;
        let size = (1.0 - z_normalized).powi(2) * 400.0;
        ObjectShape::Circle(self.position, size)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn color(&self) -> Rgba8 {
        self.color
    }
}
