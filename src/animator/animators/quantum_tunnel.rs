use crate::{
    animator::{
        AnimatedObject, AnimatorSettings, ObjectShape,
        animation_type::{AnimationType, TunnelShape},
    },
    color::{ColorPalette, ColorParam},
    modulator::Modulator,
    parameters::{ConstantParam, ModulatedParam},
    timecode::TimeCode,
};

use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

fn default_rect() -> Rect {
    Rect::from_w_h(800.0, 600.0)
}

#[derive(Serialize, Deserialize)]
pub struct QuantumTunnelSettings {
    pub shape_mode: TunnelShape,
    pub ring_count: ConstantParam<u32>,
    pub speed: ModulatedParam,
    pub depth: ModulatedParam,
    pub twist: ModulatedParam,
    pub warp_ripple: ModulatedParam,
    pub curve_amp: ModulatedParam,
    pub line_weight: ModulatedParam,
    pub recursive_line_height: ConstantParam<bool>,
    pub vortex_ribs: ConstantParam<bool>,
    #[serde(skip)]
    #[serde(default = "default_rect")]
    dimension: Rect,
    color: ColorParam,
    #[serde(skip)]
    animator: Vec<QuantumTunnelAnimator>,
    visible: bool,
}

impl QuantumTunnelSettings {
    pub fn new(win_rect: &Rect) -> Self {
        Self {
            shape_mode: TunnelShape::default(),
            ring_count: ConstantParam::new(36, 5, 100, "Ring Count", "ring_count"),
            speed: ModulatedParam::new(3.2, 0.1, 12.0, "Warp Speed", "tunnel_speed"),
            depth: ModulatedParam::new(12.0, 2.0, 50.0, "Depth", "tunnel_depth"),
            twist: ModulatedParam::new(1.8, -4.0, 4.0, "Vortex Twist", "tunnel_twist"),
            warp_ripple: ModulatedParam::new(0.34, 0.0, 1.0, "Star Warp", "tunnel_ripple"),
            curve_amp: ModulatedParam::new(0.28, 0.0, 0.8, "Wormhole Curve", "tunnel_curve"),
            dimension: *win_rect,
            color: ColorParam::with_palette(ColorPalette::Sunset),
            animator: Vec::new(),
            line_weight: ModulatedParam::new(3.2, 1.0, 12.0, "Line Weight", "line_weight"),
            recursive_line_height: ConstantParam::new(
                true,
                false,
                true,
                "Perspective Weight",
                "recursive_weight",
            ),
            vortex_ribs: ConstantParam::new(
                true,
                false,
                true,
                "Warp Streaks & Halo",
                "tunnel_vortex_ribs",
            ),
            visible: true,
        }
    }
}

impl AnimatorSettings for QuantumTunnelSettings {
    fn control_ui(&mut self, ui: &mut egui::Ui, mods: &[Box<dyn Modulator>]) {
        ui.label("Portal Geometry:");
        ui.horizontal_wrapped(|ui| {
            for option in TunnelShape::iter() {
                if ui
                    .radio_value(&mut self.shape_mode, option, option.to_string())
                    .changed()
                {
                    self.hot_update();
                }
            }
        });

        if self.ring_count.slider_ui(ui) {
            self.hot_update();
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
        if self.warp_ripple.slider_ui(ui, mods) {
            self.hot_update();
        }
        if self.curve_amp.slider_ui(ui, mods) {
            self.hot_update();
        }
        if self.line_weight.slider_ui(ui, mods) {
            self.hot_update();
        }
        if self.recursive_line_height.checkbox_ui(ui) {
            self.hot_update();
        }
        if self.vortex_ribs.checkbox_ui(ui) {
            self.hot_update();
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

        let total = self.ring_count.value.max(1) as usize;
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
        let target_count = self.ring_count.value.max(1) as usize;

        if target_count != current_count {
            self.animator.clear();
            for index in 0..target_count {
                self.animator.push(QuantumTunnelAnimator::from_settings(
                    self,
                    index,
                    target_count,
                ));
            }
        }

        let shape_mode = self.shape_mode;
        let speed = self.speed.value();
        let new_depth = self.depth.value().max(0.5);
        let twist = self.twist.value();
        let warp_ripple = self.warp_ripple.value();
        let curve_amp = self.curve_amp.value();
        let line_weight = self.line_weight.value();
        let r_weight = self.recursive_line_height.value;
        let vortex_ribs = self.vortex_ribs.value;

        for (idx, obj) in self.animator.iter_mut().enumerate() {
            let z_norm = (obj.current_z / obj.depth.max(0.01)).clamp(0.0, 1.0);
            obj.index = idx;
            obj.shape_mode = shape_mode;
            obj.speed = speed;
            obj.depth = new_depth;
            obj.current_z = z_norm * new_depth;
            obj.twist = twist;
            obj.warp_ripple = warp_ripple;
            obj.curve_amp = curve_amp;
            obj.line_weight = line_weight;
            obj.r_weight = r_weight;
            obj.vortex_ribs = vortex_ribs;
            obj.color = self.color.value_mapped(idx);
            obj.twin_color = self.color.value_mapped(idx + 2);
            obj.visible = self.visible;
        }
    }

    fn draw(&self, draw: &Draw) {
        if !self.visible {
            return;
        }

        // Draw rings from deepest vanishing point (highest current_z) to closest foreground
        let mut order: Vec<usize> = (0..self.animator.len()).collect();
        order.sort_by(|&a, &b| {
            self.animator[b]
                .current_z
                .partial_cmp(&self.animator[a].current_z)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for idx in order {
            self.animator[idx].draw(draw);
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
            &mut self.warp_ripple,
            &mut self.curve_amp,
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
}

pub struct QuantumTunnelAnimator {
    pub shape_mode: TunnelShape,
    pub position: Vec2,
    pub vanishing_point: Vec2,
    pub win_size: Vec2,
    pub speed: f32,
    pub depth: f32,
    pub twist: f32,
    pub warp_ripple: f32,
    pub curve_amp: f32,
    pub color: Rgba8,
    pub twin_color: Rgba8,
    pub index: usize,
    pub current_z: f32,
    pub center: Vec2,
    pub line_weight: f32,
    pub r_weight: bool,
    pub vortex_ribs: bool,
    pub beats: f32,
    pub beat_kick: f32,
    visible: bool,
}

impl QuantumTunnelAnimator {
    pub fn from_settings(settings: &QuantumTunnelSettings, index: usize, total: usize) -> Self {
        let depth = settings.depth.value().max(0.5);
        let z = (index as f32 / total.max(1) as f32) * depth;
        Self {
            shape_mode: settings.shape_mode,
            position: vec2(0.0, 0.0),
            vanishing_point: vec2(0.0, 0.0),
            win_size: settings.dimension.wh(),
            speed: settings.speed.value(),
            depth,
            twist: settings.twist.value(),
            warp_ripple: settings.warp_ripple.value(),
            curve_amp: settings.curve_amp.value(),
            color: settings.color.value_mapped(index),
            twin_color: settings.color.value_mapped(index + 2),
            index,
            current_z: z,
            center: vec2(settings.dimension.x(), settings.dimension.y()),
            line_weight: settings.line_weight.value(),
            r_weight: settings.recursive_line_height.value,
            vortex_ribs: settings.vortex_ribs.value,
            beats: 0.0,
            beat_kick: 0.0,
            visible: settings.visible,
        }
    }

    fn current_radius(&self) -> f32 {
        let z_norm = (self.current_z / self.depth.max(0.01)).clamp(0.0, 1.0);
        let proximity = 1.0 - z_norm;
        let persp = proximity.powf(2.15);
        let max_r = (self.win_size.x.hypot(self.win_size.y) * 0.56).max(320.0);
        let beat_wave = ((self.beats * TAU - z_norm * TAU * 2.0).sin() * 0.5 + 0.5).powi(3)
            * (0.35 + self.beat_kick * 0.65);
        (4.0 + persp * max_r) * (1.0 + beat_wave * 0.14)
    }

    fn build_portal_contour(&self, radius: f32, rot: f32, z_norm: f32) -> Vec<Point2> {
        let (sin_r, cos_r) = rot.sin_cos();
        let to_world = |x: f32, y: f32| -> Point2 {
            pt2(
                self.position.x + x * cos_r - y * sin_r,
                self.position.y + x * sin_r + y * cos_r,
            )
        };

        let morph = self.warp_ripple;
        match self.shape_mode {
            TunnelShape::SacredStar => {
                let segments = 80usize;
                let points = 8.0;
                (0..=segments)
                    .map(|i| {
                        let a = i as f32 / segments as f32 * TAU;
                        let petals = (a * points).cos() * morph * 0.30;
                        let sub = (a * points * 2.0 - rot * 1.5).sin() * morph * 0.12;
                        let r = (radius * (1.0 + petals + sub)).max(2.0);
                        to_world(r * a.cos(), r * a.sin())
                    })
                    .collect()
            }
            TunnelShape::HyperCircle => {
                let segments = 72usize;
                (0..=segments)
                    .map(|i| {
                        let a = i as f32 / segments as f32 * TAU;
                        let wave = (a * 6.0 + z_norm * 10.0 - self.beats * 2.0).sin() * morph * 0.22;
                        let r = (radius * (1.0 + wave)).max(2.0);
                        to_world(r * a.cos(), r * a.sin())
                    })
                    .collect()
            }
            TunnelShape::CrystalHex => {
                let sides = 6usize;
                let sub = 10usize;
                let mut pts = Vec::with_capacity(sides * sub + 1);
                for s in 0..sides {
                    let a0 = s as f32 / sides as f32 * TAU;
                    let a1 = (s + 1) as f32 / sides as f32 * TAU;
                    let p0 = vec2(radius * a0.cos(), radius * a0.sin());
                    let p1 = vec2(radius * a1.cos(), radius * a1.sin());
                    for k in 0..sub {
                        let t = k as f32 / sub as f32;
                        let base = p0.lerp(p1, t);
                        let facet = 1.0 - (t * PI).sin() * morph * 0.25;
                        pts.push(to_world(base.x * facet, base.y * facet));
                    }
                }
                if let Some(&first) = pts.first() {
                    pts.push(first);
                }
                pts
            }
        }
    }
}

impl AnimatedObject for QuantumTunnelAnimator {
    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode) {
        self.center = vec2(win_rect.x(), win_rect.y());
        self.win_size = win_rect.wh();
        self.beats = timecode.get_beats();
        self.beat_kick = (1.0 - timecode.get_beat_progress()).powi(3);

        let speed_boost = 1.0 + self.beat_kick * 0.28;
        self.current_z -= self.speed * speed_boost * timecode.get_delta_time();
        let depth = self.depth.max(0.5);
        self.current_z = self.current_z.rem_euclid(depth);

        let z_normalized = (self.current_z / depth).clamp(0.0, 1.0);
        let proximity = 1.0 - z_normalized;

        // Curving 3D wormhole spine that meanders in the distance and opens around the viewer
        let spine_phase = self.beats * 0.65 + z_normalized * TAU * 0.85;
        let curve_env = z_normalized.powf(1.1) * (1.0 - proximity.powi(3) * 0.65) * self.curve_amp;
        let offset_x = spine_phase.cos() * win_rect.w() * curve_env;
        let offset_y = (spine_phase * 1.5).sin() * win_rect.h() * curve_env;

        let vp_phase = self.beats * 0.65 + TAU * 0.85;
        self.vanishing_point = self.center
            + vec2(
                vp_phase.cos() * win_rect.w() * self.curve_amp,
                (vp_phase * 1.5).sin() * win_rect.h() * self.curve_amp,
            );
        self.position = self.center + vec2(offset_x, offset_y);
    }

    fn draw(&self, draw: &Draw) {
        if !self.visible {
            return;
        }

        let z_normalized = (self.current_z / self.depth.max(0.01)).clamp(0.0, 1.0);
        let proximity = 1.0 - z_normalized;
        let radius = self.current_radius();

        let beat_wave = ((self.beats * TAU - z_normalized * TAU * 2.0).sin() * 0.5 + 0.5).powi(3);
        let line_weight = if self.r_weight {
            (0.45 + proximity.powf(1.35) * self.line_weight * 1.75) * (1.0 + beat_wave * 0.25)
        } else {
            self.line_weight * (1.0 + beat_wave * 0.2)
        };

        // Smooth depth fade so rings emerge gracefully from the singularity
        let depth_fade = (proximity * 3.8).clamp(0.10, 1.0);
        let base_a = self.color.alpha as f32 * depth_fade;
        let rot = self.twist * (z_normalized * TAU * 1.2 - self.beats * 0.45)
            + self.index as f32 * 0.14;

        let pts = self.build_portal_contour(radius, rot, z_normalized);

        if self.vortex_ribs {
            // 1. Wide neon wormhole aura
            draw.polyline()
                .weight(line_weight * 2.5)
                .color(Rgba8::new(
                    self.color.red,
                    self.color.green,
                    self.color.blue,
                    (base_a * 0.24) as u8,
                ))
                .points(pts.iter().copied());

            // 2. 3D perspective warp streaks & vertex sparks
            let rib_count = if self.shape_mode == TunnelShape::CrystalHex {
                6
            } else {
                8
            };
            let step = (pts.len() / rib_count).max(1);
            let streak_pull = (0.06 + proximity * 0.11).min(0.22);
            let streak_a = (base_a * (0.28 + beat_wave * 0.35).min(0.85)) as u8;
            let spark_a = (base_a * (0.45 + beat_wave * 0.5).min(1.0)) as u8;

            for i in (0..pts.len().saturating_sub(1)).step_by(step) {
                let v = pts[i];
                let inner = v.lerp(self.vanishing_point, streak_pull);
                draw.line()
                    .start(v)
                    .end(inner)
                    .weight((line_weight * 0.65).max(0.8))
                    .color(Rgba8::new(
                        self.twin_color.red,
                        self.twin_color.green,
                        self.twin_color.blue,
                        streak_a,
                    ));

                if proximity > 0.18 {
                    draw.ellipse()
                        .xy(v)
                        .radius((line_weight * 0.75 + beat_wave * 2.0).clamp(1.2, 6.0))
                        .color(Rgba8::new(255, 250, 245, spark_a));
                }
            }
        }

        // 3. Crisp primary portal ring
        draw.polyline()
            .weight(line_weight.max(0.6))
            .color(Rgba8::new(
                self.color.red,
                self.color.green,
                self.color.blue,
                base_a as u8,
            ))
            .points(pts);
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Circle(self.position, self.current_radius())
    }

    fn color(&self) -> Rgba8 {
        self.color
    }
}
