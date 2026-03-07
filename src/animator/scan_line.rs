use super::{AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour};
use crate::animator::{
    animation_type::{AnimationType, ScanLineModes},
    animator_structs::AnimationParam,
    modulation::{ModMatrix, ModTarget},
    presets_manager::PresetManager,
};
use crate::color::ColorParam;

use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize)]
pub struct ScanLineSettings {
    mode: ScanLineModes,
    pub speed: AnimationParam<f32>,
    pub width: AnimationParam<f32>,
    color: ColorParam,
    multi_line_count: AnimationParam<u32>,
    #[serde(skip)]
    height: f32,
    #[serde(skip)]
    begin_pos: f32,
    #[serde(skip)]
    presets: PresetManager<ScanLineSettings>,
}

impl AnimatorSettings for ScanLineSettings {
    fn new(win_rect: &Rect) -> Self {
        Self {
            mode: ScanLineModes::default(),
            speed: AnimationParam::new_modulate(300.0, 0.0, 1000.0, "speed", ModTarget::ScanSpeed),
            width: AnimationParam::new_modulate(20.0, 5.0, 100.0, "width", ModTarget::ScanWidth),
            color: ColorParam::default(),
            multi_line_count: AnimationParam::new(1, 1, 10, "line_count"),
            height: win_rect.h(),
            begin_pos: win_rect.left(),
            presets: PresetManager::new_animator(AnimationType::ScanLine),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut ModMatrix) -> UpdateBehaviour {
        let mut change_type = UpdateBehaviour::None;

        ui.heading(format!("{}", self.animation_type()));
        ui.add_space(5.0);

        ui.label("Speed");
        if self.speed.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }
        ui.add_space(5.0);

        ui.label("Width");
        if self.width.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }
        ui.add_space(5.0);

        ui.label("Mode:");

        ui.horizontal(|ui| {
            for options in ScanLineModes::iter() {
                if ui
                    .radio_value(&mut self.mode, options, format!("{}", options))
                    .changed()
                {
                    change_type = UpdateBehaviour::NeedsReset;
                };
            }
        });

        if self.multi_line_count.to_slider(ui) {
            change_type = UpdateBehaviour::NeedsReset;
        }

        if self.color.ui(ui) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.presets.ui(ui) {
            change_type = UpdateBehaviour::LoadPreset;
        }
        change_type
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::ScanLine
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        let mut animated_objects: Vec<Box<dyn AnimatedObject>> = Vec::new();

        for index in 0..self.multi_line_count.value {
            animated_objects.push(Box::new(ScanLine::new(
                self.mode,
                self.speed.value,
                self.color.clone().value_mapped(index as usize),
                self.width.value,
                self.height,
                self.begin_pos,
                index as usize,
            )));
        }

        animated_objects
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.height = window_rect.h();
        self.begin_pos = window_rect.left();
    }

    fn hot_update(&self, objects: &mut Vec<Box<dyn AnimatedObject>>) {
        for obj in objects.iter_mut() {
            if let Some(scan_line) = obj.as_any_mut().downcast_mut::<ScanLine>() {
                scan_line.color = self.color.clone().value_mapped(scan_line.index);
                scan_line.width = self.width.value;
                scan_line.mode = self.mode;
                scan_line.height = self.height;
                // Update speed, preserving direction
                let direction = scan_line.speed.signum();
                scan_line.speed = self.speed.value.abs() * direction;
            }
        }
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        self.presets.save_to_file(self, None)?;
        Ok(())
    }

    fn reset(&mut self) {}
}

pub struct ScanLine {
    mode: ScanLineModes,
    pub speed: f32,
    pub color: Rgba8,
    position: Vec2,
    height: f32,
    pub width: f32,
    index: usize,
    // pub beat_snap: f32,
    phase_offset: f32,
}

impl ScanLine {
    pub fn new(
        mode: ScanLineModes,
        speed: f32,
        color: Rgba8,
        width: f32,
        height: f32,
        begin_pos: f32,
        index: usize,
    ) -> Self {
        let half_width = width / 2.0;
        let phase_offset = (index as f32 * std::f32::consts::PI * 2.0) / 10.0;
        let position = vec2(begin_pos + half_width + (index as f32 * 50.0), 0.0);

        ScanLine {
            mode,
            speed,
            color,
            position,
            height,
            width,
            index,
            phase_offset,
        }
    }
}

impl AnimatedObject for ScanLine {
    fn update(
        &mut self,
        win_rect: &Rect,
        delta_time: f32,
        clock: &crate::animator::timecode::TimeCode,
    ) {
        // Beat-synced speed modulation with snap
        let beat_progress = clock.get_beat_progress();
        let beat_pulse = ((beat_progress + self.phase_offset) * std::f32::consts::PI * 2.0).sin();
        // let speed_multiplier = 1.0 + (beat_pulse * self.beat_snap);

        self.position.x += self.speed * delta_time;


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

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
