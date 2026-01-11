use super::{AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour};
use crate::animator::{
    animation_type::{AnimationType, ModeHelper, ScanLineModes},
    animator_structs::AnimationParam,
    presets_manager::PresetManager,
};
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScanLineSettings {
    mode: ScanLineModes,
    speed: AnimationParam<f32>,
    width: AnimationParam<f32>,
    color: AnimationParam<Rgba>,
    #[serde(skip)]
    height: f32,
    #[serde(skip)]
    begin_pos: f32,
    #[serde(skip)]
    presets: PresetManager,
}

impl AnimatorSettings for ScanLineSettings {
    fn new(win_rect: &Rect) -> Self {
        Self {
            mode: ScanLineModes::default(),
            speed: AnimationParam::new(300.0, 0.0, 1000.0, "Speed"),
            width: AnimationParam::new(20.0, 5.0, 20.0, "Width"),
            color: AnimationParam::new_without_range(Rgba::new(1.0, 0.0, 0.0, 1.0), "Color"),
            height: win_rect.h(),
            begin_pos: win_rect.left(),
            presets: PresetManager::new(AnimationType::ScanLine),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> UpdateBehaviour {
        let mut change_type = UpdateBehaviour::None;

        ui.heading(self.animation_type().as_str());
        ui.add_space(5.0);

        ui.label("Speed");
        if self.speed.to_slider(ui) {
            change_type = UpdateBehaviour::HotUpdate;
        }
        ui.add_space(5.0);

        ui.label("Width");
        if self.width.to_slider(ui) {
            change_type = UpdateBehaviour::HotUpdate;
        }
        ui.add_space(5.0);

        ui.label("Mode:");

        ui.horizontal(|ui| {
            for options in ScanLineModes::iterator() {
                if ui
                    .radio_value(&mut self.mode, *options, options.as_str())
                    .changed()
                {
                    change_type = UpdateBehaviour::NeedsReset;
                };
            }
        });

        if self.color.to_color_picker(ui) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        ui.separator();
        ui.add_space(5.0);
        self.presets.ui(ui);
        change_type
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::ScanLine
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        let mut animated_objects: Vec<Box<dyn AnimatedObject>> = Vec::new();

        animated_objects.push(Box::new(ScanLine::new(
            self.mode,
            self.speed.value,
            self.color.value,
            self.width.value,
            self.height,
            self.begin_pos,
        )));

        animated_objects
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.height = window_rect.h();
        self.begin_pos = window_rect.left();
    }

    fn update_behaviour(&self, objects: &mut Vec<Box<dyn AnimatedObject>>) {
        for obj in objects.iter_mut() {
            if let Some(scan_line) = obj.as_any_mut().downcast_mut::<ScanLine>() {
                scan_line.color = self.color.value;
                scan_line.width = self.width.value;
                scan_line.mode = self.mode;
                scan_line.height = self.height;

                // Update speed, preserving direction
                let direction = scan_line.speed.signum();
                scan_line.speed = self.speed.value.abs() * direction;
            }
        }
    }
}

pub struct ScanLine {
    mode: ScanLineModes,
    speed: f32,
    color: Rgba,
    position: Vec2,
    height: f32,
    width: f32,
}

impl ScanLine {
    pub fn new(
        mode: ScanLineModes,
        speed: f32,
        color: Rgba,
        width: f32,
        height: f32,
        begin_pos: f32,
    ) -> Self {
        let half_width = width / 2.0;
        let position = vec2(begin_pos + half_width, 0.0);

        ScanLine {
            mode,
            speed,
            color,
            position,
            height,
            width,
        }
    }
}

impl AnimatedObject for ScanLine {
    fn update(&mut self, win_rect: &Rect, delta_time: f32) {
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

    fn color(&self) -> Rgba {
        self.color
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
