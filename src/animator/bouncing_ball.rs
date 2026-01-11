use crate::animator::animator_structs::AnimationParam;
use crate::animator::presets_manager::PresetManager;
// use crate::animator::presets_manager::PresetManager;
use crate::animator::{animation_type::ModeHelper, animator_structs::RangeHolder};
use crate::utils::ColorHelpers;
use serde::Serialize;

use super::{AnimatedObject, AnimatorSettings, ObjectShape};
use crate::animator::animation_type::AnimationType;
use nannou::prelude::*;
use nannou_egui::egui;

fn default_rect() -> Rect {
    Rect::from_w_h(800.0, 600.0)
}

#[derive(Serialize)]
pub struct BouncingBallSettings {
    ball_count: AnimationParam<u32>,
    speed: AnimationParam<f32>,
    radius: AnimationParam<f32>,
    ball_vel_range_x: RangeHolder<f32>,
    ball_vel_range_y: RangeHolder<f32>,
    #[serde(skip)]
    #[serde(default = "default_rect")]
    dimension: Rect,
    color: AnimationParam<Rgba>,
    #[serde(skip)]
     presets:PresetManager,

}

impl AnimatorSettings for BouncingBallSettings {
    fn new(win_rect: &Rect) -> Self {
        Self {
            ball_count: AnimationParam::new(20, 1, 200, "Ball_Count"),
            speed: AnimationParam::new(1.0, 1.0, 5.0, "Speed"),
            dimension: *win_rect,
            radius: AnimationParam::new(10.0, 6.0, 30.0, "Radius"),

            ball_vel_range_x: RangeHolder {
                lower: -100.0,
                upper: 100.0,
            },
            ball_vel_range_y: RangeHolder {
                lower: -100.0,
                upper: 100.0,
            },
            color: AnimationParam::new_without_range(Rgba::red(), "Color"),
            presets:PresetManager::new(AnimationType::BouncingBalls)
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading(self.animation_type().as_str());
        ui.add_space(5.0);

        ui.label("Ball Count");
        changed |= self.ball_count.to_slider(ui);

        ui.add_space(5.0);
        ui.label("Speed");
        changed |= self.speed.to_slider(ui);

        ui.add_space(5.0);
        ui.label("Radius");
        changed |= self.radius.to_slider(ui);

        ui.add_space(5.0);
        ui.label("Velocity Range (X-axis)");

        changed |= ui
            .horizontal(|ui| {
                let c1 = ui
                    .add(egui::DragValue::new(&mut self.ball_vel_range_x.lower).speed(1.0))
                    .changed();
                let c2 = ui
                    .add(egui::DragValue::new(&mut self.ball_vel_range_x.upper).speed(1.0))
                    .changed();

                // Ensure lower <= upper
                if self.ball_vel_range_x.lower > self.ball_vel_range_x.upper {
                    std::mem::swap(
                        &mut self.ball_vel_range_x.lower,
                        &mut self.ball_vel_range_x.upper,
                    );
                }

                let _ = ui.label("Random Range");
                c1 || c2
            })
            .inner;
        ui.add_space(5.0);

        ui.label("Velocity Range (Y-axis)");
        changed |= ui
            .horizontal(|ui| {
                let c1 = ui
                    .add(egui::DragValue::new(&mut self.ball_vel_range_y.lower).speed(1.0))
                    .changed();
                let c2 = ui
                    .add(egui::DragValue::new(&mut self.ball_vel_range_y.upper).speed(1.0))
                    .changed();

                // Ensure lower <= upper
                if self.ball_vel_range_y.lower > self.ball_vel_range_y.upper {
                    std::mem::swap(
                        &mut self.ball_vel_range_y.lower,
                        &mut self.ball_vel_range_y.upper,
                    );
                }

                let _ = ui.label("Random Range");
                c1 || c2
            })
            .inner;
        ui.add_space(5.0);

        ui.add_space(5.0);
        ui.label("Ball Color");
        changed |= self.color.to_color_picker(ui);

        // Preset Management UI
        ui.separator();
        ui.add_space(5.0);
       self.presets.ui(ui);


        changed
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::BouncingBalls
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        let mut animated_objects: Vec<Box<dyn AnimatedObject>> = Vec::new();
        for _ in 0..self.ball_count.value {
            let new_obj = Box::new(BouncingBall::new(
                &self.dimension,
                self.color.value,
                self.radius.value,
                &self.ball_vel_range_x,
                &self.ball_vel_range_y,
                self.speed.value,
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
