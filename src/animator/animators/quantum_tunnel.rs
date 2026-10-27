use crate::{
    animator::{AnimatedObject, AnimatorSettings, ObjectShape, animation_type::AnimationType},
    color::ColorParam,
    modulator::Modulator,
    parameters::{ConstantParam, ModulatedParam},
    preset::PresetState,
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
    pub line_weight: ModulatedParam,
    pub recursive_line_height: ConstantParam<bool>,
    #[serde(skip)]
    #[serde(default = "default_rect")]
    dimension: Rect,
    color: ColorParam,
    #[serde(skip)]
    animator: Vec<QuantumTunnelAnimator>,
    visible: bool,
    #[serde(skip)]
    preset: PresetState,
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
            animator: Vec::new(),
            line_weight: ModulatedParam::new(1.0, 1.0, 10.0, "Line Weight", "line_weight"),

            recursive_line_height: ConstantParam::new(
                false,
                false,
                true,
                "Recursive Weight",
                "recursive_weight",
            ),
            visible: true,
            preset: PresetState::default(),
        }
    }
}

impl AnimatorSettings for QuantumTunnelSettings {
    fn control_ui(&mut self, ui: &mut egui::Ui, mods: &[Box<dyn Modulator>]) {
        if self.ring_count.slider_ui(ui) {
            self.init();
        }

        if self.speed.slider_ui(ui, mods) {
            self.hot_update();
        }

        if self.depth.slider_ui(ui, mods) {
            self.hot_update();
        }

        if self.twist.slider_ui(ui, mods) {
            self.hot_update();
        }

        if self.line_weight.slider_ui(ui, mods) {
            self.hot_update();
        }

        if self.recursive_line_height.checkbox_ui(ui) {
            self.init();
        }

        if self.color.ui(ui, mods) {
            self.hot_update();
        }
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::QuantumTunnel
    }

    fn init(&mut self) {
        self.animator.clear();

        let total = self.ring_count.value as usize;
        for index in 0..total {
            self.animator
                .push(QuantumTunnelAnimator::from_settings(self, index, total));
        }
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.dimension = *window_rect;
    }

    fn hot_update(&mut self) {
        let current_count = self.animator.len();
        let target_count = self.ring_count.value as usize;

        if target_count > current_count {
            for index in current_count..target_count {
                self.animator.push(QuantumTunnelAnimator::from_settings(
                    self,
                    index,
                    target_count,
                ));
            }
        } else if target_count < current_count {
            self.animator.truncate(target_count);
        }

        let speed = self.speed.value();
        let depth = self.depth.value();
        let twist = self.twist.value();
        let line_weight = self.line_weight.value();
        for obj in self.animator.iter_mut() {
            obj.speed = speed;
            obj.color = self.color.value_mapped(obj.index);
            obj.depth = depth;
            obj.twist = twist;
            obj.line_weight = line_weight;
            obj.visible = self.visible;
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
            &mut self.speed,
            &mut self.depth,
            &mut self.twist,
            &mut self.line_weight,
        ];

        params.extend(self.color.modulated_params_mut());
        params
    }

    fn set_visibility(&mut self, state: bool) {
        self.visible = state;
        self.hot_update();
    }

    fn get_visibility(&self) -> bool {
        self.visible
    }
    fn preset_state(&self) -> &PresetState {
        &self.preset
    }
    fn preset_state_mut(&mut self) -> &mut PresetState {
        &mut self.preset
    }
    fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    fn apply_json(&mut self, json: &str) -> anyhow::Result<()> {
        Ok(())
    }
}

pub struct QuantumTunnelAnimator {
    pub position: Vec2,
    pub speed: f32,
    pub depth: f32,
    pub twist: f32,
    pub color: Rgba8,
    pub index: usize,
    pub current_z: f32,
    pub center: Vec2,
    pub line_weight: f32,
    pub r_weight: bool,
    visible: bool,
}

impl QuantumTunnelAnimator {
    pub fn from_settings(settings: &QuantumTunnelSettings, index: usize, total: usize) -> Self {
        let depth = settings.depth.value();
        let z = (index as f32 / total.max(1) as f32) * depth;
        Self {
            position: vec2(0.0, 0.0),
            speed: settings.speed.value(),
            depth,
            twist: settings.twist.value(),
            color: settings.color.value_mapped(index),
            index,
            current_z: z,
            center: vec2(settings.dimension.x(), settings.dimension.y()),
            line_weight: settings.line_weight.value(),
            r_weight: settings.recursive_line_height.value,
            visible: settings.visible,
        }
    }
}

impl AnimatedObject for QuantumTunnelAnimator {
    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode) {
        self.center = vec2(win_rect.x(), win_rect.y());

        self.current_z -= self.speed * timecode.get_delta_time();
        if self.current_z <= 0.0 {
            self.current_z += self.depth;
        } else if self.current_z > self.depth {
            self.current_z -= self.depth;
        }

        let z_normalized = self.current_z / self.depth;
        let time = timecode.get_beats();

        let angle = time * self.twist + (z_normalized * std::f32::consts::PI * 4.0);
        let offset_x = angle.cos() * (1.0 - z_normalized) * win_rect.w() * 0.2;
        let offset_y = angle.sin() * (1.0 - z_normalized) * win_rect.h() * 0.2;

        self.position = self.center + vec2(offset_x, offset_y);
    }

    fn draw(&self, draw: &Draw) {
        let z_normalized = self.current_z / self.depth;
        let size = (1.0 - z_normalized).powi(2) * 400.0;

        let mut line_weight = self.line_weight;

        if self.r_weight {
            line_weight = (z_normalized) * self.line_weight * 2.0;
        }

        if self.visible {
            draw.ellipse()
                .xy(self.position)
                .radius(size)
                .no_fill()
                .stroke_weight(line_weight)
                .stroke_color(self.color);
        }
    }

    fn shape(&self) -> ObjectShape {
        let z_normalized = self.current_z / self.depth;
        let size = (1.0 - z_normalized).powi(2) * 400.0;
        ObjectShape::Circle(self.position, size)
    }

    fn color(&self) -> Rgba8 {
        self.color
    }
}
