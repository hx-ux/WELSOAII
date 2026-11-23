use crate::{
    animator::{
        AnimatedObject, AnimatorSettings, LayerTrailState, ObjectShape,
        animation_type::{
            AnimationType, LayerBlendMode, LayerPolygonMode, PulseModes, PulseShape,
        },
        default_trail_persistence,
    },
    color::ColorParam,
    modulator::Modulator,
    parameters::{ConstantParam, ModulatedParam},
    timecode::TimeCode,
};
use anyhow::Ok;
use nannou::prelude::*;
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
    #[serde(default = "default_trail_persistence")]
    pub trail_persistence: ModulatedParam,
    #[serde(skip)]
    pub animator: Vec<PulseBackgroundAnimator>,
    #[serde(skip)]
    pub trails: LayerTrailState,
    pub visible: bool,
    #[serde(default)]
    pub blend_mode: LayerBlendMode,
    #[serde(default)]
    pub polygon_mode: LayerPolygonMode,
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
            trail_persistence: default_trail_persistence(),
            animator: Vec::new(),
            trails: LayerTrailState::default(),
            visible: true,
            blend_mode: LayerBlendMode::default(),
            polygon_mode: LayerPolygonMode::default(),
        }
    }
}

impl AnimatorSettings for PulseBackgroundSettings {
    fn control_ui(&mut self, ui: &mut egui::Ui, modulators: &[Box<dyn Modulator>]) {
        ui.horizontal(|ui| {
            for options in PulseModes::iter() {
                if ui
                    .radio_value(&mut self.mode, options, format!("{}", options))
                    .changed()
                {
                    self.init();
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
                    self.init();
                };
            }
        });

        if self.speed.slider_ui(ui) {
            self.hot_update();
        }
        if self.limit.slider_ui(ui, modulators) {
            self.hot_update();
        }

        if self.ring_count.slider_ui(ui) {
            self.init();
        }

        if self.ring_spread.slider_ui(ui, modulators) {
            self.hot_update();
        }

        if self.rotation_speed.slider_ui(ui, modulators) {
            self.hot_update();
        }

        if self.trail_persistence.slider_ui(ui, modulators) {
            self.hot_update();
        }

        if self.color.ui(ui, modulators) {
            self.hot_update();
        }
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::PulseBackground
    }

    fn init(&mut self) {
        self.animator.clear();
        for index in 0..self.ring_count.value as usize {
            self.animator
                .push(PulseBackgroundAnimator::from_settings(self, index));
        }
    }

    fn set_dimension(&mut self, _window_rect: &Rect) {}

    fn hot_update(&mut self) {
        let limit = self.limit.value();
        let rotation_speed = self.rotation_speed.value();
        let ring_spread = self.ring_spread.value();
        for obj in self.animator.iter_mut() {
            obj.color = self.color.value_mapped(obj.index);
            obj.speed = self.speed.value;
            obj.mode = self.mode;
            obj.shape = self.shape;
            obj.limit = limit;
            obj.rotation_speed = rotation_speed;
            obj.ring_spread = ring_spread;
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
            &mut self.limit,
            &mut self.rotation_speed,
            &mut self.ring_spread,
            &mut self.trail_persistence,
        ];

        params.extend(self.color.modulated_params_mut());
        params
    }

    fn snapshot(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn set_visibility(&mut self, state: bool) {
        self.visible = state;
        self.hot_update();
    }

    fn get_visibility(&self) -> bool {
        self.visible
    }

    fn get_blend_mode(&self) -> LayerBlendMode {
        self.blend_mode
    }

    fn set_blend_mode(&mut self, mode: LayerBlendMode) {
        self.blend_mode = mode;
    }

    fn get_polygon_mode(&self) -> LayerPolygonMode {
        self.polygon_mode
    }

    fn set_polygon_mode(&mut self, mode: LayerPolygonMode) {
        self.polygon_mode = mode;
    }

    fn get_trail_persistence(&self) -> f32 {
        self.trail_persistence.value()
    }

    fn trails(&self) -> &LayerTrailState {
        &self.trails
    }

    fn trails_mut(&mut self) -> &mut LayerTrailState {
        &mut self.trails
    }
}

pub struct PulseBackgroundAnimator {
    mode: PulseModes,
    pub shape: PulseShape,
    pub color: Srgba,
    pub speed: u8,
    current_size_w: f32,
    current_size_h: f32,
    pub limit: f32,
    index: usize,
    pub rotation_speed: f32,
    pub ring_spread: f32,
    rotation: f32,
    visible: bool,
}

impl PulseBackgroundAnimator {
    fn from_settings(settings: &PulseBackgroundSettings, index: usize) -> Self {
        Self {
            mode: settings.mode,
            shape: settings.shape,
            speed: settings.speed.value,
            color: settings.color.value_mapped(index),
            current_size_w: 20.0,
            current_size_h: 20.0,
            limit: settings.limit.value(),
            index,
            rotation_speed: settings.rotation_speed.value(),
            ring_spread: settings.ring_spread.value(),
            rotation: 0.0,
            visible: settings.visible,
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
        if !self.visible {
            return;
        }
        let pts = self.ring_points(64);
        draw.polyline().weight(2.0).color(self.color).points(pts);
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Polyline {
            points: self.ring_points(32),
            thickness: 3.0,
        }
    }

    fn color(&self) -> Srgba {
        self.color
    }
}

impl PulseBackgroundAnimator {
    fn ring_points(&self, circle_segments: usize) -> Vec<Point2> {
        let cx = 0.0;
        let cy = 0.0;
        let spread = self.ring_spread;
        let ring_size_w = self.current_size_w * spread * (1.0 + self.index as f32 * 0.3);
        let ring_size_h = self.current_size_h * spread * (1.0 + self.index as f32 * 0.3);
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
                [
                    pt2(cx - hw, cy - hh),
                    pt2(cx + hw, cy - hh),
                    pt2(cx + hw, cy + hh),
                    pt2(cx - hw, cy + hh),
                    pt2(cx - hw, cy - hh),
                ]
                .iter()
                .map(|&p| rotate(p))
                .collect()
            }
            PulseShape::Circle => {
                let rx = ring_size_w * 0.5;
                let ry = ring_size_h * 0.5;
                let mut pts: Vec<Point2> = (0..=circle_segments)
                    .map(|i| {
                        let angle = i as f32 / circle_segments as f32 * std::f32::consts::TAU;
                        rotate(pt2(cx + rx * angle.cos(), cy + ry * angle.sin()))
                    })
                    .collect();
                pts.push(pts[0]);
                pts
            }
            PulseShape::Diamond => {
                let hw = ring_size_w * 0.5;
                let hh = ring_size_h * 0.5;
                [
                    pt2(cx, cy + hh),
                    pt2(cx + hw, cy),
                    pt2(cx, cy - hh),
                    pt2(cx - hw, cy),
                    pt2(cx, cy + hh),
                ]
                .iter()
                .map(|&p| rotate(p))
                .collect()
            }
        }
    }
}
