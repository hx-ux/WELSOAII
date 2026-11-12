use crate::{
    animator::{
        AnimatedObject, AnimatorSettings, ObjectShape,
        animation_type::{AnimationType, MirageMode},
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
pub struct AcidMirageSettings {
    pub serpent_count: ConstantParam<u32>,
    pub trail_steps: ConstantParam<u32>,
    pub mode: MirageMode,
    pub flow_speed: ModulatedParam,
    pub turbulence: ModulatedParam,
    pub spread: ModulatedParam,
    pub tail_span: ModulatedParam,
    pub ribbon_width: ModulatedParam,
    pub beat_warp: ModulatedParam,
    pub mirror_symmetry: ConstantParam<bool>,
    pub outline: ConstantParam<bool>,

    pub color: ColorParam,
    #[serde(skip)]
    #[serde(default = "default_rect")]
    dimension: Rect,
    #[serde(skip)]
    pub animator: Vec<MirageSerpent>,
    pub visible: bool,
}

impl AcidMirageSettings {
    pub fn new(win_rect: &Rect) -> Self {
        Self {
            serpent_count: ConstantParam::new(14, 2, 36, "Serpents", "mirage_serpents"),
            trail_steps: ConstantParam::new(28, 8, 64, "Trail Steps", "mirage_trail_steps"),
            mode: MirageMode::default(),
            flow_speed: ModulatedParam::new(1.35, 0.1, 6.0, "Flow Speed", "mirage_speed"),
            turbulence: ModulatedParam::new(2.4, 0.3, 8.0, "Turbulence", "mirage_turbulence"),
            spread: ModulatedParam::new(0.78, 0.15, 1.25, "Field Spread", "mirage_spread"),
            tail_span: ModulatedParam::new(0.95, 0.15, 2.5, "Tail Span", "mirage_tail_span"),
            ribbon_width: ModulatedParam::new(4.0, 1.0, 18.0, "Ribbon Width", "mirage_width"),
            beat_warp: ModulatedParam::new(0.45, 0.0, 1.5, "Beat Warp", "mirage_beat_warp"),
            mirror_symmetry: ConstantParam::new(
                true,
                false,
                true,
                "Astral Mirror",
                "mirage_mirror",
            ),
            outline: ConstantParam::new(false, false, true, "Outline", "outline"),
            color: ColorParam::with_palette(ColorPalette::Neon),
            dimension: *win_rect,
            animator: Vec::new(),
            visible: true,
        }
    }
}

impl AnimatorSettings for AcidMirageSettings {
    fn control_ui(&mut self, ui: &mut egui::Ui, modulators: &[Box<dyn Modulator>]) {
        ui.label("Mirage Realm:");
        ui.horizontal_wrapped(|ui| {
            for option in MirageMode::iter() {
                if ui
                    .radio_value(&mut self.mode, option, option.to_string())
                    .changed()
                {
                    self.hot_update();
                }
            }
        });

        if self.serpent_count.slider_ui(ui) {
            self.hot_update();
        }

        if self.trail_steps.slider_ui(ui) {
            self.hot_update();
        }

        if self.flow_speed.slider_ui(ui, modulators) {
            self.hot_update();
        }

        if self.turbulence.slider_ui(ui, modulators) {
            self.hot_update();
        }

        if self.spread.slider_ui(ui, modulators) {
            self.hot_update();
        }

        if self.tail_span.slider_ui(ui, modulators) {
            self.hot_update();
        }

        if self.ribbon_width.slider_ui(ui, modulators) {
            self.hot_update();
        }

        if self.beat_warp.slider_ui(ui, modulators) {
            self.hot_update();
        }

        if self.mirror_symmetry.checkbox_ui(ui) {
            self.hot_update();
        }

        if self.outline.checkbox_ui(ui) {
            self.hot_update();
        }

        if self.color.ui(ui, modulators) {
            self.hot_update();
        }
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::AcidMirage
    }

    fn init(&mut self) {
        self.animator.clear();
        let total = self.serpent_count.value.max(1) as usize;
        for idx in 0..total {
            self.animator
                .push(MirageSerpent::from_settings(self, idx, total));
        }
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.dimension = *window_rect;
    }

    fn hot_update(&mut self) {
        let current = self.animator.len();
        let target = self.serpent_count.value.max(1) as usize;

        if target > current {
            let base_phase = self.animator.first().map(|s| s.phase).unwrap_or(0.0);
            for idx in current..target {
                let mut serpent = MirageSerpent::from_settings(self, idx, target);
                serpent.phase = base_phase;
                self.animator.push(serpent);
            }
        } else if target < current {
            self.animator.truncate(target);
        }

        let trail_steps = self.trail_steps.value.max(4) as usize;
        let flow_speed = self.flow_speed.value();
        let turbulence = self.turbulence.value().max(0.1);
        let spread = self.spread.value().max(0.05);
        let tail_span = self.tail_span.value().max(0.05);
        let ribbon_width = self.ribbon_width.value().max(0.5);
        let beat_warp = self.beat_warp.value().max(0.0);
        let mirror_symmetry = self.mirror_symmetry.value;
        let mode = self.mode;
        let outline = self.outline.value;

        for (idx, serpent) in self.animator.iter_mut().enumerate() {
            serpent.index = idx;
            serpent.total = target;
            serpent.trail_steps = trail_steps;
            serpent.mode = mode;
            serpent.flow_speed = flow_speed;
            serpent.turbulence = turbulence;
            serpent.spread = spread;
            serpent.tail_span = tail_span;
            serpent.ribbon_width = ribbon_width;
            serpent.beat_warp = beat_warp;
            serpent.mirror_symmetry = mirror_symmetry;
            serpent.color = self.color.value_mapped(idx);
            serpent.twin_color = self.color.value_mapped(idx + 3);
            serpent.visible = self.visible;
            serpent.oultine = self.outline.value;
        }
    }

    fn draw(&self, draw: &Draw) {
        for serpent in self.animator.iter() {
            serpent.draw(draw);
        }
    }

    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode) {
        for serpent in self.animator.iter_mut() {
            serpent.update(win_rect, timecode);
        }
    }

    fn get_objects(&self) -> Vec<&dyn AnimatedObject> {
        self.animator
            .iter()
            .map(|s| s as &dyn AnimatedObject)
            .collect()
    }

    fn get_objects_mut(&mut self) -> Vec<&mut dyn AnimatedObject> {
        self.animator
            .iter_mut()
            .map(|s| s as &mut dyn AnimatedObject)
            .collect()
    }

    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
        let mut params = vec![
            &mut self.flow_speed,
            &mut self.turbulence,
            &mut self.spread,
            &mut self.tail_span,
            &mut self.ribbon_width,
            &mut self.beat_warp,
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

pub struct MirageSerpent {
    pub index: usize,
    pub total: usize,
    pub trail_steps: usize,
    pub mode: MirageMode,
    pub flow_speed: f32,
    pub turbulence: f32,
    pub spread: f32,
    pub tail_span: f32,
    pub ribbon_width: f32,
    pub beat_warp: f32,
    pub mirror_symmetry: bool,
    pub color: Rgba8,
    pub twin_color: Rgba8,
    pub phase: f32,
    pub beat_pos: f32,
    pub beat_progress: f32,
    pub is_downbeat: bool,
    pub win_size: Vec2,
    pub head_pos: Vec2,
    pub active_radius: f32,
    pub oultine: bool,
    pub visible: bool,
}

impl MirageSerpent {
    pub fn from_settings(settings: &AcidMirageSettings, index: usize, total: usize) -> Self {
        Self {
            index,
            total: total.max(1),
            trail_steps: settings.trail_steps.value.max(4) as usize,
            mode: settings.mode,
            flow_speed: settings.flow_speed.value(),
            turbulence: settings.turbulence.value().max(0.1),
            spread: settings.spread.value().max(0.05),
            tail_span: settings.tail_span.value().max(0.05),
            ribbon_width: settings.ribbon_width.value().max(0.5),
            beat_warp: settings.beat_warp.value().max(0.0),
            mirror_symmetry: settings.mirror_symmetry.value,
            color: settings.color.value_mapped(index),
            twin_color: settings.color.value_mapped(index + 3),
            phase: 0.0,
            beat_pos: 0.0,
            beat_progress: 0.0,
            is_downbeat: false,
            win_size: settings.dimension.wh(),
            head_pos: vec2(0.0, 0.0),
            active_radius: 24.0,
            oultine: false,
            visible: settings.visible,
        }
    }

    fn sample_trajectory(&self, t: f32, idx_norm: f32) -> Point2 {
        let half_w = self.win_size.x * 0.5 * self.spread;
        let half_h = self.win_size.y * 0.5 * self.spread;
        let turb = self.turbulence;
        let entity_angle = idx_norm * TAU;

        // Beat-reactive warp envelope
        let beat_kick = (1.0 - self.beat_progress).powi(2) * self.beat_warp;
        let beat_harmonic = (self.beat_pos * PI * 0.5 + entity_angle).sin() * self.beat_warp * 0.25;
        let scale_boost = 1.0 + beat_kick * 0.22 + beat_harmonic;

        match self.mode {
            MirageMode::Kundalini => {
                // Braided caduceus / desert-serpent waves slithering across space
                let strand_dir = if self.index.is_multiple_of(2) {
                    1.0
                } else {
                    -1.0
                };
                let spine_t = t * 0.85 + entity_angle;
                let base_x = (spine_t).sin() * half_w * 0.88;
                let base_y = (spine_t * 0.5 * strand_dir + entity_angle).cos() * half_h * 0.72;

                // Secondary kundalini coil around the spine
                let coil_freq = turb * 2.2;
                let coil_amp = (0.18 + 0.12 * (t + entity_angle).sin()) * scale_boost;
                let coil_x = (t * coil_freq + entity_angle * 2.0).cos() * half_w * coil_amp;
                let coil_y =
                    (t * coil_freq * 1.3 - entity_angle).sin() * half_h * coil_amp * strand_dir;

                pt2(base_x + coil_x, base_y + coil_y)
            }
            MirageMode::HypnoVortex => {
                // 3D-tilted golden-spiral vortex breathing in and out
                let radial_cycle = (t * 0.65 + idx_norm * TAU).sin() * 0.5 + 0.5;
                let r_norm = (0.15 + 0.85 * radial_cycle) * scale_boost;
                let swirl_angle =
                    t * (1.2 + turb * 0.35) + entity_angle + (1.0 - radial_cycle) * turb * 1.8;

                // Toroidal vertical undulation
                let torus_z = (swirl_angle * 2.0 - t * 1.5).sin() * 0.22 * self.beat_warp;
                let x = swirl_angle.cos() * half_w * (r_norm + torus_z);
                let y = swirl_angle.sin() * half_h * (r_norm - torus_z);
                pt2(x, y)
            }
            MirageMode::AttractorWeb => {
                // Coupled strange-attractor / multi-ratio Lissajous knots
                let freq_a = 1.0 + (turb * 0.35).sin() * 0.5;
                let freq_b = 1.5 + (turb * 0.25).cos() * 0.5;
                let phase_a = t * freq_a + entity_angle;
                let phase_b = t * freq_b + entity_angle * 1.618;

                let x =
                    (phase_a.sin() * 0.68 + (phase_b * 1.7).cos() * 0.32) * half_w * scale_boost;
                let y = (phase_b.cos() * 0.68 + (phase_a * 2.3 - turb).sin() * 0.32)
                    * half_h
                    * scale_boost;
                pt2(x, y)
            }
        }
    }
}

impl AnimatedObject for MirageSerpent {
    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode) {
        self.win_size = win_rect.wh();
        self.phase += timecode.get_delta_time() * self.flow_speed;
        self.beat_pos = timecode.get_beats();
        self.beat_progress = timecode.get_beat_progress();
        self.is_downbeat = timecode.get_beat_counter().0 == 1;

        let idx_norm = self.index as f32 / self.total.max(1) as f32;
        self.head_pos = self.sample_trajectory(self.phase, idx_norm);

        let beat_flare = (1.0 - self.beat_progress).powi(2) * self.beat_warp * 16.0;
        self.active_radius = (self.ribbon_width * 4.0 + beat_flare).clamp(12.0, 64.0);
    }

    fn draw(&self, draw: &Draw) {
        if !self.visible {
            return;
        }

        let idx_norm = self.index as f32 / self.total.max(1) as f32;
        let next_idx_norm =
            ((self.index + 1) % self.total.max(1)) as f32 / self.total.max(1) as f32;
        let steps = self.trail_steps.max(4);

        let trail_pts: Vec<Point2> = (0..=steps)
            .map(|i| {
                let frac = i as f32 / steps as f32;
                let sample_t = self.phase - frac * self.tail_span;
                self.sample_trajectory(sample_t, idx_norm)
            })
            .collect();

        let base_alpha = self.color.alpha as f32;
        let aura_col = Rgba8::new(
            self.color.red,
            self.color.green,
            self.color.blue,
            (base_alpha * 0.26) as u8,
        );
        let twin_aura_col = Rgba8::new(
            self.twin_color.red,
            self.twin_color.green,
            self.twin_color.blue,
            (base_alpha * 0.45) as u8,
        );

        // draw outline
        if self.oultine {
            draw.polyline()
                .weight(self.ribbon_width * 2.8)
                .color(aura_col)
                .points(trail_pts.iter().copied());
        }

        // 2. Crisp core serpent ribbon
        draw.polyline()
            .weight(self.ribbon_width)
            .color(self.color)
            .points(trail_pts.iter().copied());

        // Mirror
        if self.mirror_symmetry {
            let mirror_pts: Vec<Point2> = trail_pts.iter().map(|&p| pt2(-p.x, -p.y)).collect();
            draw.polyline()
                .weight((self.ribbon_width * 0.85).max(0.8))
                .color(twin_aura_col)
                .points(mirror_pts);
        }
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Circle(self.head_pos, self.active_radius)
    }

    fn color(&self) -> Rgba8 {
        self.color
    }
}
