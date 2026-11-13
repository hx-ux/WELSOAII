use crate::{
    animator::{
        AnimatedObject, AnimatorSettings, ObjectShape,
        animation_type::{AnimationType, PulseModes, PulseShape},
    },
    color::{ColorPalette, ColorParam},
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
    pub line_weight: ModulatedParam,
    pub petal_morph: ModulatedParam,
    pub kaleidoscope: ConstantParam<bool>,
    #[serde(skip)]
    pub animator: Vec<PulseBackgroundAnimator>,
    pub visible: bool,
}

impl PulseBackgroundSettings {
    pub fn new(_win_rect: &Rect) -> Self {
        Self {
            mode: PulseModes::default(),
            shape: PulseShape::default(),
            speed: ConstantParam::new(1, 1, 4, "Speed (x Beat)", "pulse_speed"),
            color: ColorParam::with_palette(ColorPalette::Vaporwave),
            limit: ModulatedParam::new(0.85, 0.15, 1.4, "Max Scale", "pulse_limit"),
            ring_count: ConstantParam::new(9, 1, 20, "Ring Count", "ring_count"),
            rotation_speed: ModulatedParam::new(1.35, 0.0, 6.0, "Rotation", "pulse_rotation"),
            ring_spread: ModulatedParam::new(1.15, 0.3, 2.5, "Ring Spread", "pulse_ring_spread"),
            line_weight: ModulatedParam::new(3.2, 1.0, 14.0, "Line Weight", "pulse_line_weight"),
            petal_morph: ModulatedParam::new(0.42, 0.0, 1.0, "Cymatic Morph", "pulse_morph"),
            kaleidoscope: ConstantParam::new(
                true,
                false,
                true,
                "Sacred Kaleidoscope",
                "pulse_kaleidoscope",
            ),
            animator: Vec::new(),
            visible: true,
        }
    }
}

impl AnimatorSettings for PulseBackgroundSettings {
    fn control_ui(&mut self, ui: &mut egui::Ui, modulators: &[Box<dyn Modulator>]) {
        ui.label("Pulse Mode:");
        ui.horizontal_wrapped(|ui| {
            for options in PulseModes::iter() {
                if ui
                    .radio_value(&mut self.mode, options, format!("{}", options))
                    .changed()
                {
                    self.hot_update();
                };
            }
        });

        ui.label("Sacred Geometry:");
        ui.horizontal_wrapped(|ui| {
            for options in PulseShape::iter() {
                if ui
                    .radio_value(&mut self.shape, options, format!("{}", options))
                    .changed()
                {
                    self.hot_update();
                };
            }
        });

        if self.ring_count.slider_ui(ui) {
            self.hot_update();
        }
        if self.speed.slider_ui(ui) {
            self.hot_update();
        }
        if self.limit.slider_ui(ui, modulators) {
            self.hot_update();
        }
        if self.ring_spread.slider_ui(ui, modulators) {
            self.hot_update();
        }
        if self.rotation_speed.slider_ui(ui, modulators) {
            self.hot_update();
        }
        if self.line_weight.slider_ui(ui, modulators) {
            self.hot_update();
        }
        if self.petal_morph.slider_ui(ui, modulators) {
            self.hot_update();
        }
        if self.kaleidoscope.checkbox_ui(ui) {
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
        let total = self.ring_count.value.max(1) as usize;
        for index in 0..total {
            self.animator
                .push(PulseBackgroundAnimator::from_settings(self, index, total));
        }
    }

    fn set_dimension(&mut self, _window_rect: &Rect) {}

    fn hot_update(&mut self) {
        let current = self.animator.len();
        let target = self.ring_count.value.max(1) as usize;

        if target > current {
            let base_rot = self.animator.first().map(|a| a.rotation).unwrap_or(0.0);
            for index in current..target {
                let mut ring = PulseBackgroundAnimator::from_settings(self, index, target);
                ring.rotation = base_rot;
                self.animator.push(ring);
            }
        } else if target < current {
            self.animator.truncate(target);
        }

        let limit = self.limit.value();
        let rotation_speed = self.rotation_speed.value();
        let ring_spread = self.ring_spread.value();
        let line_weight = self.line_weight.value();
        let petal_morph = self.petal_morph.value();
        let kaleidoscope = self.kaleidoscope.value;

        for (idx, obj) in self.animator.iter_mut().enumerate() {
            obj.index = idx;
            obj.total_rings = target;
            obj.color = self.color.value_mapped(idx);
            obj.twin_color = self.color.value_mapped(idx + 2);
            obj.speed = self.speed.value;
            obj.mode = self.mode;
            obj.shape = self.shape;
            obj.limit = limit;
            obj.rotation_speed = rotation_speed;
            obj.ring_spread = ring_spread;
            obj.line_weight = line_weight;
            obj.petal_morph = petal_morph;
            obj.kaleidoscope = kaleidoscope;
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
            &mut self.line_weight,
            &mut self.petal_morph,
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
}

pub struct PulseBackgroundAnimator {
    mode: PulseModes,
    pub shape: PulseShape,
    pub color: Rgba8,
    pub twin_color: Rgba8,
    pub speed: u8,
    current_size_w: f32,
    current_size_h: f32,
    pub limit: f32,
    index: usize,
    total_rings: usize,
    pub rotation_speed: f32,
    pub ring_spread: f32,
    pub line_weight: f32,
    pub petal_morph: f32,
    pub kaleidoscope: bool,
    rotation: f32,
    beat_kick: f32,
    visible: bool,
}

impl PulseBackgroundAnimator {
    fn from_settings(settings: &PulseBackgroundSettings, index: usize, total_rings: usize) -> Self {
        Self {
            mode: settings.mode,
            shape: settings.shape,
            speed: settings.speed.value,
            color: settings.color.value_mapped(index),
            twin_color: settings.color.value_mapped(index + 2),
            current_size_w: 60.0,
            current_size_h: 60.0,
            limit: settings.limit.value(),
            index,
            total_rings: total_rings.max(1),
            rotation_speed: settings.rotation_speed.value(),
            ring_spread: settings.ring_spread.value(),
            line_weight: settings.line_weight.value(),
            petal_morph: settings.petal_morph.value(),
            kaleidoscope: settings.kaleidoscope.value,
            rotation: 0.0,
            beat_kick: 0.0,
            visible: settings.visible,
        }
    }

    fn build_contour(&self, rx: f32, ry: f32, rot: f32, morph: f32) -> Vec<Point2> {
        let (sin_r, cos_r) = rot.sin_cos();
        let rotate_pt = |x: f32, y: f32| -> Point2 { pt2(x * cos_r - y * sin_r, x * sin_r + y * cos_r) };

        match self.shape {
            PulseShape::Lotus => {
                let segments = 96usize;
                (0..=segments)
                    .map(|i| {
                        let a = i as f32 / segments as f32 * TAU;
                        let petals = (a * 8.0).cos() * (0.14 + 0.24 * morph);
                        let harmonic = (a * 16.0 - rot * 2.0).sin() * 0.10 * morph;
                        let r_mod = (1.0 + petals + harmonic).max(0.15);
                        rotate_pt(rx * r_mod * a.cos(), ry * r_mod * a.sin())
                    })
                    .collect()
            }
            PulseShape::Star => {
                let segments = 96usize;
                let points = 8.0;
                (0..=segments)
                    .map(|i| {
                        let a = i as f32 / segments as f32 * TAU;
                        let star_wave = (a * points).cos();
                        let sharp = star_wave.signum() * star_wave.abs().powf(0.65);
                        let inner_indent = 0.28 + 0.25 * morph;
                        let r_mod = (1.0 - inner_indent * 0.5 + sharp * inner_indent * 0.5
                            + (a * points * 2.0 + rot).sin() * 0.06 * morph)
                            .max(0.15);
                        rotate_pt(rx * r_mod * a.cos(), ry * r_mod * a.sin())
                    })
                    .collect()
            }
            PulseShape::Hexagon => {
                let sides = 6usize;
                let sub = 12usize;
                let mut pts = Vec::with_capacity(sides * sub + 1);
                for s in 0..sides {
                    let a0 = s as f32 / sides as f32 * TAU;
                    let a1 = (s + 1) as f32 / sides as f32 * TAU;
                    let p0 = vec2(rx * a0.cos(), ry * a0.sin());
                    let p1 = vec2(rx * a1.cos(), ry * a1.sin());
                    for k in 0..sub {
                        let t = k as f32 / sub as f32;
                        let base = p0.lerp(p1, t);
                        let bow = 1.0 + (t * PI).sin() * (morph * 0.26 - 0.08);
                        pts.push(rotate_pt(base.x * bow, base.y * bow));
                    }
                }
                if let Some(&first) = pts.first() {
                    pts.push(first);
                }
                pts
            }
            PulseShape::Diamond => {
                let corners = [
                    vec2(0.0, ry),
                    vec2(rx, 0.0),
                    vec2(0.0, -ry),
                    vec2(-rx, 0.0),
                ];
                let sub = 16usize;
                let mut pts = Vec::with_capacity(4 * sub + 1);
                for c in 0..4 {
                    let p0 = corners[c];
                    let p1 = corners[(c + 1) % 4];
                    for k in 0..sub {
                        let t = k as f32 / sub as f32;
                        let base = p0.lerp(p1, t);
                        // Morph curves flat diamond edges into a hypocycloid 4-star
                        let curve = 1.0 - (t * PI).sin() * morph * 0.34;
                        pts.push(rotate_pt(base.x * curve, base.y * curve));
                    }
                }
                if let Some(&first) = pts.first() {
                    pts.push(first);
                }
                pts
            }
            PulseShape::Square => {
                let corners = [
                    vec2(-rx, -ry),
                    vec2(rx, -ry),
                    vec2(rx, ry),
                    vec2(-rx, ry),
                ];
                let sub = 16usize;
                let mut pts = Vec::with_capacity(4 * sub + 1);
                for c in 0..4 {
                    let p0 = corners[c];
                    let p1 = corners[(c + 1) % 4];
                    for k in 0..sub {
                        let t = k as f32 / sub as f32;
                        let base = p0.lerp(p1, t);
                        let wave = 1.0 + (t * TAU * 1.5).sin() * morph * 0.16;
                        pts.push(rotate_pt(base.x * wave, base.y * wave));
                    }
                }
                if let Some(&first) = pts.first() {
                    pts.push(first);
                }
                pts
            }
            PulseShape::Circle => {
                let segments = 80usize;
                (0..=segments)
                    .map(|i| {
                        let a = i as f32 / segments as f32 * TAU;
                        let r_mod = 1.0 + morph * 0.22 * (a * 6.0 + rot * 2.0).sin();
                        rotate_pt(rx * r_mod * a.cos(), ry * r_mod * a.sin())
                    })
                    .collect()
            }
        }
    }
}

impl AnimatedObject for PulseBackgroundAnimator {
    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode) {
        let min_size = 24.0;
        let max_w = win_rect.w() * self.limit;
        let max_h = win_rect.h() * self.limit;
        let idx_norm = self.index as f32 / self.total_rings.max(1) as f32;

        self.rotation += timecode.get_delta_time() * self.rotation_speed;
        self.beat_kick = (1.0 - timecode.get_beat_progress()).powi(3);
        let beats = timecode.get_beats() * self.speed as f32;

        match self.mode {
            PulseModes::RippleWave => {
                let wave = (beats * TAU - idx_norm * TAU * 1.2).sin() * 0.5 + 0.5;
                let base_ring = 0.16 + idx_norm * 0.72 * self.ring_spread;
                let scale = (base_ring + wave * 0.26 * self.ring_spread)
                    * (1.0 + self.beat_kick * 0.14);
                self.current_size_w = (max_w * scale).max(min_size);
                self.current_size_h = (max_h * scale).max(min_size);
            }
            PulseModes::Heartbeat => {
                let phase = (beats - idx_norm * 0.35).rem_euclid(1.0);
                let lub = (-((phase - 0.10) * 9.5).powi(2)).exp();
                let dub = 0.68 * (-((phase - 0.32) * 11.5).powi(2)).exp();
                let cardiac = (lub + dub).clamp(0.0, 1.25);
                let scale = (0.18 + idx_norm * 0.70 * self.ring_spread) * (0.62 + 0.55 * cardiac);
                self.current_size_w = (max_w * scale).max(min_size);
                self.current_size_h = (max_h * scale).max(min_size);
            }
            PulseModes::Elastic => {
                let beat_phase = (beats - idx_norm * 0.42) * PI;
                let normalized = (beat_phase.sin() + 1.0) * 0.5;
                let eased = normalized * normalized * (3.0 - 2.0 * normalized);
                let ring_mul = (0.24 + idx_norm * 0.76 * self.ring_spread).max(0.15);
                self.current_size_w = (min_size + eased * (max_w - min_size)) * ring_mul;
                self.current_size_h = (min_size + eased * (max_h - min_size)) * ring_mul;
            }
            PulseModes::Smooth => {
                let beat_cycle = (beats + idx_norm).rem_euclid(1.0);
                let scale = beat_cycle * self.ring_spread;
                self.current_size_w = (scale * max_w).max(min_size);
                self.current_size_h = (scale * max_h).max(min_size);
            }
        }
    }

    fn draw(&self, draw: &Draw) {
        if !self.visible {
            return;
        }

        let rx = self.current_size_w * 0.5;
        let ry = self.current_size_h * 0.5;
        let spin_dir = if self.kaleidoscope && !self.index.is_multiple_of(2) {
            -1.0
        } else {
            1.0
        };
        let ring_rotation =
            self.rotation * spin_dir * (1.0 + self.index as f32 * 0.18) + self.index as f32 * 0.35;

        let pts = self.build_contour(rx, ry, ring_rotation, self.petal_morph);
        let base_a = self.color.alpha as f32;
        let weight = self.line_weight * (1.0 + self.beat_kick * 0.25);

        // 1. Soft neon bloom aura
        let aura_col = Rgba8::new(
            self.color.red,
            self.color.green,
            self.color.blue,
            (base_a * 0.25) as u8,
        );
        draw.polyline()
            .weight(weight * 2.8)
            .color(aura_col)
            .points(pts.iter().copied());

        // 2. Counter-rotating sacred kaleidoscope twin & chakra vertex jewels
        if self.kaleidoscope {
            let twin_pts = self.build_contour(
                rx * 0.84,
                ry * 0.84,
                -ring_rotation * 1.15 + PI * 0.125,
                self.petal_morph * 0.85,
            );
            let twin_col = Rgba8::new(
                self.twin_color.red,
                self.twin_color.green,
                self.twin_color.blue,
                (base_a * 0.46) as u8,
            );
            draw.polyline()
                .weight((weight * 0.68).max(1.0))
                .color(twin_col)
                .points(twin_pts);

            // Luminous vertex jewels along the primary symmetry peaks
            let step = (pts.len() / 8).max(1);
            let jewel_r = (weight * 0.85 + self.beat_kick * 2.5).clamp(1.8, 7.5);
            let jewel_a = (base_a * (0.58 + self.beat_kick * 0.35).min(1.0)) as u8;
            for i in (0..pts.len().saturating_sub(1)).step_by(step) {
                draw.ellipse()
                    .xy(pts[i])
                    .radius(jewel_r)
                    .color(Rgba8::new(255, 250, 245, jewel_a));
            }
        }

        // 3. Crisp primary sacred ring
        draw.polyline()
            .weight(weight)
            .color(self.color)
            .points(pts);
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Rect(Rect::from_w_h(self.current_size_w, self.current_size_h))
    }

    fn color(&self) -> Rgba8 {
        self.color
    }
}
