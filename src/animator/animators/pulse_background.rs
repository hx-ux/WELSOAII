use crate::parameters::ConstantParam;
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
    pub speed: ModulatedParam,
    pub color: ColorParam,
    pub limit: ModulatedParam,
    pub rotation_speed: ModulatedParam,
    pub ring_spread: ModulatedParam,
    pub offset_x: ModulatedParam,
    pub offset_y: ModulatedParam,
    pub beat_flash: bool,
}

impl PulseBackgroundSettings {
    pub fn new(_win_rect: &Rect) -> Self {
        Self {
            mode: PulseModes::default(),
            shape: PulseShape::default(),
            speed: ModulatedParam::new(100.0, 1.0, 300.0, "Speed", "pulse_speed"),
            color: ColorParam::default(),
            limit: ModulatedParam::new(0.8, 0.05, 1.0, "Limit", "pulse_limit"),
            ring_count: ConstantParam::new(3, 1, 12, "Ring Count", "ring_count"),
            rotation_speed: ModulatedParam::new(0.5, 0.0, 6.0, "Rotation", "pulse_rotation"),
            ring_spread: ModulatedParam::new(1.0, 0.3, 3.0, "Ring Spread", "pulse_ring_spread"),
            offset_x: ModulatedParam::new(0.0, -500.0, 500.0, "Offset X", "pulse_offset_x"),
            offset_y: ModulatedParam::new(0.0, -500.0, 500.0, "Offset Y", "pulse_offset_y"),
            beat_flash: false,
        }
    }
}

impl AnimatorSettings for PulseBackgroundSettings {
    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
        vec![
            &mut self.speed,
            &mut self.limit,
            &mut self.rotation_speed,
            &mut self.ring_spread,
            &mut self.offset_x,
            &mut self.offset_y,
        ]
    }

    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut Vec<Box<dyn Modulator>>) -> UpdateBehaviour {
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

        if self.speed.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.limit.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.ring_count.to_slider(ui) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.ring_spread.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.rotation_speed.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.offset_x.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.offset_y.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        ui.horizontal(|ui| {
            ui.label("Beat Flash:");
            if ui.checkbox(&mut self.beat_flash, "").changed() {
                change_type = UpdateBehaviour::HotUpdate;
            }
        });

        if self.color.ui(ui) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        change_type
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::PulseBackground
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        vec![Box::new(PulseBackground::new(
            self.mode,
            self.shape,
            self.color.clone().value_mapped(0),
            *self.speed.value(),
            *self.limit.value(),
            self.ring_count.value,
            *self.rotation_speed.value(),
            *self.ring_spread.value(),
            *self.offset_x.value(),
            *self.offset_y.value(),
            self.beat_flash,
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
                pulse_bg.shape = self.shape;
                pulse_bg.limit = *self.limit.value();
                pulse_bg.ring_count = self.ring_count.value as usize;
                pulse_bg.rotation_speed = *self.rotation_speed.value();
                pulse_bg.ring_spread = *self.ring_spread.value();
                pulse_bg.offset_x = *self.offset_x.value();
                pulse_bg.offset_y = *self.offset_y.value();
                pulse_bg.beat_flash = self.beat_flash;
            }
        }
    }

    fn reset(&mut self) {
        self.ring_count.reset();
        self.speed.reset();
        self.limit.reset();
        self.rotation_speed.reset();
        self.ring_spread.reset();
        self.offset_x.reset();
        self.offset_y.reset();
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

pub struct PulseBackground {
    mode: PulseModes,
    pub shape: PulseShape,
    pub color: Rgba8,
    pub speed: f32,
    current_size_w: f32,
    current_size_h: f32,
    pub limit: f32,
    index: usize,
    pub ring_count: usize,
    pub rotation_speed: f32,
    pub ring_spread: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub beat_flash: bool,
    rotation: f32,
    flash_alpha: f32,
    last_beat_floor: u32,
}

impl PulseBackground {
    fn new(
        mode: PulseModes,
        shape: PulseShape,
        color: Rgba8,
        speed: f32,
        limit: f32,
        ring_count: u32,
        rotation_speed: f32,
        ring_spread: f32,
        offset_x: f32,
        offset_y: f32,
        beat_flash: bool,
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
            ring_count: ring_count as usize,
            rotation_speed,
            ring_spread,
            offset_x,
            offset_y,
            beat_flash,
            rotation: 0.0,
            flash_alpha: 0.0,
            last_beat_floor: 0,
        }
    }
}

impl AnimatedObject for PulseBackground {
    fn update(&mut self, win_rect: &Rect, delta_time: f32, clock: &TimeCode) {
        let min_size = 20.0;
        let max_w = win_rect.w() * self.limit;
        let max_h = win_rect.h() * self.limit;

        self.rotation += delta_time * self.rotation_speed;

        // Beat flash
        let beat_floor = clock.get_beats() as u32;
        if beat_floor != self.last_beat_floor && self.beat_flash {
            self.last_beat_floor = beat_floor;
            self.flash_alpha = 1.0;
        }
        self.flash_alpha = (self.flash_alpha - delta_time * 4.0).max(0.0);

        match self.mode {
            PulseModes::Smooth => {
                let beat_cycle = clock.get_beats().fract();
                self.current_size_w = (beat_cycle * max_w).max(min_size);
                self.current_size_h = (beat_cycle * max_h).max(min_size);
            }
            PulseModes::Elastic => {
                let beat_phase = clock.get_beat_progress() * std::f32::consts::PI * 2.0;
                let normalized = (beat_phase.sin() + 1.0) * 0.5;
                let eased = normalized * normalized * (3.0 - 2.0 * normalized);
                self.current_size_w = min_size + eased * (max_w - min_size);
                self.current_size_h = min_size + eased * (max_h - min_size);
            }
        }
    }

    fn draw(&self, draw: &Draw) {
        let cx = self.offset_x;
        let cy = self.offset_y;

        // Beat flash overlay
        if self.flash_alpha > 0.0 {
            let flash_alpha_u8 = (self.flash_alpha * 255.0) as u8;
            let flash_color = rgba8(255, 255, 255, flash_alpha_u8);
            draw.rect()
                .x(cx)
                .y(cy)
                .width(self.current_size_w * 2.0)
                .height(self.current_size_h * 2.0)
                .color(flash_color);
        }

        for i in (0..self.ring_count).rev() {
            let ring_progress = (i as f32 + 1.0) / self.ring_count as f32;
            let spread = self.ring_spread;
            let ring_size_w = self.current_size_w * ring_progress * spread;
            let ring_size_h = self.current_size_h * ring_progress * spread;

            let alpha = (self.color.alpha as f32 * (1.0 - ring_progress * 0.55)) as u8;
            let mut ring_color = self.color;
            ring_color.alpha = alpha;

            let ring_rotation = self.rotation * (i as f32 + 1.0) * 0.15;

            match self.shape {
                PulseShape::Square => {
                    draw.rect()
                        .x(cx)
                        .y(cy)
                        .width(ring_size_w)
                        .height(ring_size_h)
                        .rotate(ring_rotation)
                        .color(ring_color);
                }
                PulseShape::Circle => {
                    // Use ellipse for Circle shape
                    let r = (ring_size_w + ring_size_h) * 0.5;
                    draw.ellipse()
                        .x(cx)
                        .y(cy)
                        .width(ring_size_w)
                        .height(ring_size_h)
                        .rotate(ring_rotation)
                        .color(ring_color);
                    let _ = r;
                }
                PulseShape::Diamond => {
                    let hw = ring_size_w * 0.5;
                    let hh = ring_size_h * 0.5;
                    let pts = [
                        pt2(cx, cy + hh),
                        pt2(cx + hw, cy),
                        pt2(cx, cy - hh),
                        pt2(cx - hw, cy),
                        pt2(cx, cy + hh),
                    ];
                    // Rotate manually
                    let cos_r = ring_rotation.cos();
                    let sin_r = ring_rotation.sin();
                    let rot_pts: Vec<Point2> = pts
                        .iter()
                        .map(|p| {
                            let dx = p.x - cx;
                            let dy = p.y - cy;
                            pt2(
                                cx + dx * cos_r - dy * sin_r,
                                cy + dx * sin_r + dy * cos_r,
                            )
                        })
                        .collect();
                    draw.polyline()
                        .weight(2.0)
                        .color(ring_color)
                        .points(rot_pts);
                }
            }
        }
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Rect(Rect::from_x_y_w_h(
            self.offset_x,
            self.offset_y,
            self.current_size_w * self.ring_spread,
            self.current_size_h * self.ring_spread,
        ))
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn color(&self) -> Rgba8 {
        self.color
    }
}
