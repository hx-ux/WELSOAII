use crate::animator::AnimatedObject;
use crate::animator::AnimatorSettings;
use crate::animator::ObjectShape;
use crate::animator::animation_type::AnimationType;
use crate::animator::animation_type::ScanLineModes;
use crate::color::ColorParam;
use crate::modulator::Modulator;
use crate::parameters::ConstantParam;
use crate::parameters::ModulatedParam;
use crate::preset;
use crate::preset::PresetState;
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
    visible: bool,
    #[serde(skip)]
    preset: PresetState,
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
            visible: true,
            preset: PresetState::default(),
        }
    }
}

impl AnimatorSettings for ScanLineSettings {
    fn control_ui(&mut self, ui: &mut egui::Ui, modulators: &[Box<dyn Modulator>]) {
        if self.line_count.slider_ui(ui) {
            self.init();
        }

        if self.speed.slider_ui(ui, modulators) {
            self.hot_update();
        }

        if self.width.slider_ui(ui, modulators) {
            self.hot_update();
        }

        if self.wobble_amp.slider_ui(ui, modulators) {
            self.hot_update();
        }

        if self.wobble_freq.slider_ui(ui, modulators) {
            self.hot_update();
        }

        if self.tilt.slider_ui(ui, modulators) {
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

        if self.color.ui(ui, modulators) {
            self.hot_update();
        }
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::ScanLine
    }

    fn init(&mut self) {
        self.animator.clear();

        for index in 0..self.line_count.value as usize {
            self.animator
                .push(ScanLineAnimator::from_settings(self, index));
        }
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.height = window_rect.h();
        self.begin_pos = window_rect.left();
    }

    fn hot_update(&mut self) {
        let width = self.width.value();
        let wobble_amp = self.wobble_amp.value();
        let wobble_freq = self.wobble_freq.value();
        let tilt = self.tilt.value();
        let speed_abs = self.speed.value().abs();

        for obj in self.animator.iter_mut() {
            obj.color = self.color.value_mapped(obj.index);
            obj.width = width;
            obj.mode = self.mode;
            obj.height = self.height;
            obj.wobble_amp = wobble_amp;
            obj.wobble_freq = wobble_freq;
            obj.tilt = tilt;
            obj.speed = speed_abs * obj.speed.signum();
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
            &mut self.width,
            &mut self.wobble_amp,
            &mut self.wobble_freq,
            &mut self.tilt,
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
        let l: Self = serde_json::from_str(json)?;

        Ok(())
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
    visible: bool,
}

impl ScanLineAnimator {
    pub fn from_settings(settings: &ScanLineSettings, index: usize) -> Self {
        let width = settings.width.value();
        let half_width = width / 2.0;
        let phase_offset = (index as f32 * std::f32::consts::PI * 2.0) / 10.0;
        let position = vec2(settings.begin_pos + half_width + (index as f32 * 50.0), 0.0);

        ScanLineAnimator {
            mode: settings.mode,
            speed: settings.speed.value(),
            color: settings.color.value_mapped(index),
            position,
            height: settings.height,
            width,
            wobble_amp: settings.wobble_amp.value(),
            wobble_freq: settings.wobble_freq.value(),
            tilt: settings.tilt.value(),
            time: 0.0,
            index,
            phase_offset,
            visible: settings.visible,
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
        if self.visible {
            draw.rect()
                .xy(self.position)
                .height(self.height)
                .width(self.width)
                .rotate(self.tilt)
                .color(self.color);
        }
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
