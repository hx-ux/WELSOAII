use crate::animator::AnimatedObject;
use crate::animator::AnimatorSettings;
use crate::animator::ObjectShape;
use crate::animator::animation_type::AnimationType;
use crate::animator::animation_type::ScanLineModes;
use crate::color::ColorParam;
use crate::modulator::Modulator;
use crate::parameters::ConstantParam;
use crate::parameters::ModulatedParam;
use crate::timecode::TimeCode;
use anyhow::Ok;
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize)]
pub struct ScanLineSettings {
    line_count: ConstantParam<u8>,
    mode: ScanLineModes,
    pub speed: ModulatedParam,
    pub width: ModulatedParam,
    pub wobble_amp: ModulatedParam,
    pub wobble_freq: ModulatedParam,
    pub tilt: ModulatedParam,
    color: ColorParam,
    #[serde(skip)]
    height: f32,
    #[serde(skip)]
    begin_pos: f32,
    #[serde(skip)]
    pub animator: Vec<ScanLineAnimator>,
}

impl ScanLineSettings {
    pub fn new(win_rect: &Rect) -> Self {
        Self {
            line_count: ConstantParam::new(1, 1, 20, "Line Count", "line_count"),
            mode: ScanLineModes::default(),
            speed: ModulatedParam::new(300.0, 0.0, 1000.0, "Speed", "scan_speed"),
            width: ModulatedParam::new(20.0, 5.0, 200.0, "Width", "scan_width"),
            wobble_amp: ModulatedParam::new(0.0, 0.0, 300.0, "Wobble Amp", "scan_wobble_amp"),
            wobble_freq: ModulatedParam::new(2.0, 0.1, 20.0, "Wobble Freq", "scan_wobble_freq"),
            tilt: ModulatedParam::new(0.0, -1.57, 1.57, "Tilt", "scan_tilt"),
            color: ColorParam::default(),
            height: win_rect.h(),
            begin_pos: win_rect.left(),
            animator: vec![],
        }
    }
}

impl AnimatorSettings for ScanLineSettings {
    fn control_ui(&mut self, ui: &mut egui::Ui, modulators: &mut Vec<Box<dyn Modulator>>) {
        if self.line_count.to_slider(ui) {
            self.init();
        }

        if self.speed.to_slider_modulate(ui, modulators) {
            self.hot_update();
        }

        if self.width.to_slider_modulate(ui, modulators) {
            self.hot_update();
        }

        if self.wobble_amp.to_slider_modulate(ui, modulators) {
            self.hot_update();
        }

        if self.wobble_freq.to_slider_modulate(ui, modulators) {
            self.hot_update();
        }

        if self.tilt.to_slider_modulate(ui, modulators) {
            self.hot_update();
        }

        ui.label("Mode:");
        ui.horizontal(|ui| {
            for options in ScanLineModes::iter() {
                if ui
                    .radio_value(&mut self.mode, options, format!("{}", options))
                    .changed()
                {
                    self.init();
                };
            }
        });

        if self.color.ui(ui) {
            self.hot_update();
        }
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::ScanLine
    }

    fn init(&mut self) {
        self.animator.clear();

        for index in 0..self.line_count.value as usize {
            let g = ScanLineAnimator::new(
                self.mode,
                *self.speed.value(),
                self.color.clone().value_mapped(index as usize),
                *self.width.value(),
                self.height,
                self.begin_pos,
                *self.wobble_amp.value(),
                *self.wobble_freq.value(),
                *self.tilt.value(),
                index,
            );
            self.animator.push(g);
        }
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.height = window_rect.h();
        self.begin_pos = window_rect.left();
    }

    fn hot_update(&mut self) {
        for obj in self.animator.iter_mut() {
            obj.color = self.color.clone().value_mapped(obj.index);
            obj.width = *self.width.value();
            obj.mode = self.mode;
            obj.height = self.height;
            obj.wobble_amp = *self.wobble_amp.value();
            obj.wobble_freq = *self.wobble_freq.value();
            obj.tilt = *self.tilt.value();
            obj.speed = self.speed.value().abs() * obj.speed.signum();
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
        vec![
            &mut self.speed,
            &mut self.width,
            &mut self.wobble_amp,
            &mut self.wobble_freq,
            &mut self.tilt,
        ]
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn color_ui(&mut self, ui: &mut egui::Ui) {
        self.color.ui(ui);
    }
}

pub struct ScanLineAnimator {
    mode: ScanLineModes,
    pub speed: f32,
    pub color: Rgba8,
    position: Vec2,
    height: f32,
    pub width: f32,
    pub wobble_amp: f32,
    pub wobble_freq: f32,
    pub tilt: f32,
    time: f32,
    index: usize,
    phase_offset: f32,
}

impl ScanLineAnimator {
    pub fn new(
        mode: ScanLineModes,
        speed: f32,
        color: Rgba8,
        width: f32,
        height: f32,
        begin_pos: f32,
        wobble_amp: f32,
        wobble_freq: f32,
        tilt: f32,
        index: usize,
    ) -> Self {
        let half_width = width / 2.0;
        let phase_offset = (index as f32 * std::f32::consts::PI * 2.0) / 10.0;
        let position = vec2(begin_pos + half_width + (index as f32 * 50.0), 0.0);

        ScanLineAnimator {
            mode,
            speed,
            color,
            position,
            height,
            width,
            wobble_amp,
            wobble_freq,
            tilt,
            time: 0.0,
            index,
            phase_offset,
        }
    }
}

impl AnimatedObject for ScanLineAnimator {
    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode) {
        self.time += timecode.get_delta_time();
        self.position.x += self.speed * timecode.get_delta_time();

        // Y wobble — oscillate the vertical center
        self.position.y =
            (self.time * self.wobble_freq + self.phase_offset).sin() * self.wobble_amp;

        let half_width = self.width / 2.0;
        let left_bound = win_rect.left() + half_width;
        let right_bound = win_rect.right() - half_width;

        match self.mode {
            ScanLineModes::PingPong => {
                if self.position.x > right_bound {
                    self.position.x = right_bound;
                    self.speed *= -1.0;
                } else if self.position.x < left_bound {
                    self.position.x = left_bound;
                    self.speed *= -1.0;
                }
            }
            ScanLineModes::WrapAround => {
                if self.position.x > right_bound && self.speed > 0.0 {
                    self.position.x = left_bound;
                } else if self.position.x < left_bound && self.speed < 0.0 {
                    self.position.x = right_bound;
                }
            }
        }
    }
    fn draw(&self, draw: &Draw) {
        draw.rect()
            .xy(self.position)
            .height(self.height)
            .width(self.width)
            .rotate(self.tilt)
            .color(self.color);
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Rect(Rect::from_x_y_w_h(
            self.position.x,
            self.position.y,
            self.width,
            self.height,
        ))
    }

    fn color(&self) -> Rgba8 {
        self.color
    }
}
