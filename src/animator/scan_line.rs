use super::{AnimatedObject, ObjectShape};
use crate::{
    animator::{
        animation_type::{AnimationType, ModeHelper, ScanLineModes}, AnimatorSettings
    }, utils::ColorHelpers
};
use nannou::prelude::*;
use nannou_egui::egui;

#[derive(Debug)]
pub struct ScanLineSettings {
    mode: ScanLineModes,
    speed: f32,
    width: f32,
    color: Rgba,
    dimension: Rect,
}

impl AnimatorSettings for ScanLineSettings {
    fn new(win_rect: &Rect) -> Self {
        Self {
            mode: ScanLineModes::PingPong,
            speed: 300.0,
            width: 20.0,
            color: Rgba::red(),
            dimension: win_rect.clone(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading(self.get_ani_type().as_str());
        ui.add_space(5.0);

        ui.label("Speed");
        changed |= ui
            .add(egui::Slider::new(&mut self.speed, -1000.0..=1000.0).text("Speed"))
            .changed();
        ui.add_space(5.0);

        ui.label("Width");
        changed |= ui
            .add(egui::Slider::new(&mut self.width, 5.0..=20.0).text("Width"))
            .changed();
        ui.add_space(5.0);

        ui.label("Mode:");
        ui.horizontal(|ui| {
            for options in ScanLineModes::iterator() {
                if ui
                    .radio_value(&mut self.mode, *options, options.as_str())
                    .changed()
                {
                    changed = true;
                };
            }
        });

        changed
    }

    fn get_ani_type(&self) -> AnimationType {
        AnimationType::ScanLine
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        let mut animated_objects: Vec<Box<dyn AnimatedObject>> = Vec::new();

        animated_objects.push(Box::new(ScanLine::new(
            self.mode,
            self.speed,
            self.color,
            &self.dimension,
            self.width,
        )));

        animated_objects
    }
}

pub struct ScanLine {
    mode: ScanLineModes,
    speed: f32,
    color: Rgba,
    position: Vec2,
    // TODO as rect
    window_dimension: (f32, f32),
    width: f32,
}

impl ScanLine {
    pub fn new(mode: ScanLineModes, speed: f32, color: Rgba, win_rect: &Rect, width: f32) -> Self {
        // TODO init pos on the left side
        let pos = vec2(0.0, 0.0);

        ScanLine {
            mode,
            speed,
            color,
            position: pos,
            window_dimension: (win_rect.w(), win_rect.h()),
            width,
        }
    }
}

impl AnimatedObject for ScanLine {
    fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        self.window_dimension = win_rect.w_h();

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
            .height(self.window_dimension.1)
            .width(self.width)
            .color(self.color);
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Rect(Rect::from_x_y_w_h(
            self.position.x,
            self.position.y,
            self.width,
            self.window_dimension.1,
        ))
    }

    fn color(&self) -> Rgba {
        self.color
    }
}
