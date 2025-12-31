use crate::{
    animator::{
        animation_type::{AnimationType, ModeHelper}, animator_structs::RangeHolder, AnimatorSettings
    }, Utils::ColorHelpers
};

use super::{AnimatedObject, ObjectShape};
use nannou::prelude::*;
use nannou_egui::egui;

pub struct BouncingBallSettings {
    ball_count: u32,
    speed: f32,
    radius: f32,
    ball_vel_range_x: RangeHolder<f32>,
    ball_vel_range_y: RangeHolder<f32>,
    dimension: Rect,
    color: Rgba,
}

impl AnimatorSettings for BouncingBallSettings {
    fn new(win_rect: &Rect) -> Self {
        Self {
            ball_count: 20,
            dimension: win_rect.to_owned(),
            radius: 10.0,
            ball_vel_range_x: RangeHolder {
                lower: -100.0,
                upper: 100.0,
            },
            ball_vel_range_y: RangeHolder {
                lower: -100.0,
                upper: 100.0,
            },
            speed: 1.0,
            color: Rgba::red(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        // let mut e_color = self.color.to_egui().clone();

        // if ui.color_edit_button_rgba_unmultiplied(&mut self.color).changed() {
        //     self.color = Rgba::from_egui(e_color);
        //     changed = true;
        // }

        ui.heading(self.get_ani_type().as_str());
        ui.add_space(5.0);

        // Object Count
        changed |= ui
            .add(egui::Slider::new(&mut self.ball_count, 1..=300).text("Ball Count"))
            .changed();
        ui.add_space(5.0);

        // Speed
        changed |= ui
            .add(egui::Slider::new(&mut self.speed, 1.0..=5.0).text("Speed"))
            .changed();

        // Radius
        changed |= ui
            .add(egui::Slider::new(&mut self.radius, 6.0..=30.0).text("Radius"))
            .changed();
        ui.add_space(5.0);

        ui.label("Velocity Range (X-axis)");
        // TODO Limit range
        changed |= ui
            .horizontal(|ui| {
                let c1 = ui
                    .add(egui::DragValue::new(&mut self.ball_vel_range_x.lower).speed(1.0))
                    .changed();
                let c2 = ui
                    .add(egui::DragValue::new(&mut self.ball_vel_range_x.upper).speed(1.0))
                    .changed();
                let _ = ui.label("Random Range");
                c1 || c2
            })
            .inner;
        ui.add_space(5.0);

        ui.label("Velocity Range (Y-axis)");
        // TODO Limit range
        changed |= ui
            .horizontal(|ui| {
                let c1 = ui
                    .add(egui::DragValue::new(&mut self.ball_vel_range_y.lower).speed(1.0))
                    .changed();
                let c2 = ui
                    .add(egui::DragValue::new(&mut self.ball_vel_range_y.upper).speed(1.0))
                    .changed();
                let _ = ui.label("Random Range");
                c1 || c2
            })
            .inner;
        ui.add_space(5.0);

        changed
    }

    fn get_ani_type(&self) -> AnimationType {
        AnimationType::BouncingBalls
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        let mut animated_objects: Vec<Box<dyn AnimatedObject>> = Vec::new();
        for _ in 0..self.ball_count {
            let new_obj = Box::new(BouncingBall::new(
                &self.dimension,
                self.color,
                self.radius,
                &self.ball_vel_range_x,
                &self.ball_vel_range_y,
                self.speed,
            ));
            animated_objects.push(new_obj);
        }
        animated_objects
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.dimension = *window_rect;
    }
}

pub struct BouncingBall {
    pub speed: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub color: Rgba,
}

impl BouncingBall {
    pub fn new(
        win_rect: &Rect,
        color: Rgba,
        radius: f32,
        horizontal_velocity: &RangeHolder<f32>,
        vertical_velocity: &RangeHolder<f32>,
        speed: f32,
    ) -> Self {
        BouncingBall {
            position: vec2(
                random_range(win_rect.left() + radius, win_rect.right() - radius),
                random_range(win_rect.bottom() + radius, win_rect.top() - radius),
            ),
            velocity: vec2(
                random_range(horizontal_velocity.lower, horizontal_velocity.upper),
                random_range(vertical_velocity.lower, vertical_velocity.upper),
            ),
            radius,
            color,
            speed,
        }
    }
}

impl AnimatedObject for BouncingBall {
    fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        self.position += self.velocity * delta_time * self.speed;

        // Bounce off window edges and clamp position within bounds
        let min_x = win_rect.left() + self.radius;
        let max_x = win_rect.right() - self.radius;
        let min_y = win_rect.bottom() + self.radius;
        let max_y = win_rect.top() - self.radius;

        if self.position.x < min_x {
            self.position.x = min_x;
            self.velocity.x *= -1.0;
        } else if self.position.x > max_x {
            self.position.x = max_x;
            self.velocity.x *= -1.0;
        }

        if self.position.y < min_y {
            self.position.y = min_y;
            self.velocity.y *= -1.0;
        } else if self.position.y > max_y {
            self.position.y = max_y;
            self.velocity.y *= -1.0;
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.radius)
            .color(self.color);
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Circle(self.position, self.radius)
    }

    fn color(&self) -> Rgba {
        self.color
    }
}
