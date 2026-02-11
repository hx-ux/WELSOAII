use crate::animator::animator_structs::AnimationParam;
use crate::animator::animator_structs::RangeHolder;
use crate::animator::modulation;
use crate::animator::modulation::ModMatrix;
use crate::animator::presets_manager::PresetManager;
use crate::animator::timecode::TimeCode;
use crate::color::ColorParam;
use anyhow::Ok;
use serde::{Deserialize, Serialize};

use super::{AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour};
use crate::animator::animation_type::AnimationType;
use nannou::prelude::*;
use nannou_egui::egui;

fn default_rect() -> Rect {
    Rect::from_w_h(800.0, 600.0)
}

#[derive(Serialize, Deserialize)]
pub struct BouncingBallSettings {
    pub ball_count: AnimationParam<u32>,
    pub speed: AnimationParam<f32>,
    pub radius: AnimationParam<f32>,
    ball_vel_range_x: RangeHolder<f32>,
    ball_vel_range_y: RangeHolder<f32>,
    #[serde(skip)]
    #[serde(default = "default_rect")]
    dimension: Rect,
    color: ColorParam,
    #[serde(skip)]
    presets: PresetManager<BouncingBallSettings>,
}

impl AnimatorSettings for BouncingBallSettings {
    fn new(win_rect: &Rect) -> Self {
        Self {
            ball_count: AnimationParam::new(20, 1, 400, "Ball Count"),
            speed: AnimationParam::new_modulate(
                1.0,
                1.0,
                5.0,
                "Speed",
                modulation::ModTarget::BouncingSpeed,
            ),
            dimension: *win_rect,
            radius: AnimationParam::new_modulate(
                10.0,
                6.0,
                30.0,
                "radius",
                modulation::ModTarget::BouncingRadius,
            ),
            ball_vel_range_x: RangeHolder::new(-100.0, 100.0),
            ball_vel_range_y: RangeHolder::new(-100.0, 100.0),
            color: ColorParam::default(),
            presets: PresetManager::new_animator(AnimationType::BouncingBalls),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut ModMatrix) -> UpdateBehaviour {
        let mut change_type = UpdateBehaviour::None;

        ui.heading(format!("{}", self.animation_type()));

        if self.ball_count.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.radius.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.speed.to_slider_modulate(ui, mods) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if ui
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
            .inner
        {
            change_type = UpdateBehaviour::HotUpdate;
        }
        ui.add_space(5.0);

        ui.label("Velocity Range (Y-axis)");
        if ui
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
            .inner
        {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.color.ui(ui) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        if self.presets.ui(ui) {
            change_type = UpdateBehaviour::LoadPreset
        }

        change_type
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::BouncingBalls
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        let mut animated_objects: Vec<Box<dyn AnimatedObject>> = Vec::new();

        for index in 0..self.ball_count.value as usize {
            let new_obj = Box::new(BouncingBall::new(
                &self.dimension,
                self.color.clone().value_mapped(index),
                self.radius.value,
                &self.ball_vel_range_x,
                &self.ball_vel_range_y,
                self.speed.value,
                index,
            ));
            animated_objects.push(new_obj);
        }
        animated_objects
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.dimension = *window_rect;
    }

    fn hot_update(&self, objects: &mut Vec<Box<dyn AnimatedObject>>) {
        let current_count = objects.len();
        let target_count = self.ball_count.value as usize;

        // Adjust ball count
        if target_count > current_count {
            // Add new balls
            for index in current_count..target_count {
                let new_obj = Box::new(BouncingBall::new(
                    &self.dimension,
                    self.color.clone().value_mapped(index),
                    self.radius.value,
                    &self.ball_vel_range_x,
                    &self.ball_vel_range_y,
                    self.speed.value,
                    index,
                ));
                objects.push(new_obj);
            }
        } else if target_count < current_count {
            // Remove excess balls
            objects.truncate(target_count);
        }

        // HOT UPDATE
        // Update existing balls with new parameters
        for obj in objects.iter_mut() {
            if let Some(ball) = obj.as_any_mut().downcast_mut::<BouncingBall>() {
                ball.speed = self.speed.value;
                ball.radius = self.radius.value;
                ball.color = self.color.clone().value_mapped(ball.index);

                // Re-randomize velocity within new range
                // ball.velocity = vec2(
                //     random_range(self.ball_vel_range_x.lower, self.ball_vel_range_x.upper),
                //     random_range(self.ball_vel_range_y.lower, self.ball_vel_range_y.upper),
                // );
            }
        }
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        self.presets.save_to_file(self, None)?;
        Ok(())
    }

    fn reset(&mut self) {
        self.ball_count.reset();
        self.speed.reset();
        self.radius.reset();
    }
}

impl BouncingBallSettings {}

pub struct BouncingBall {
    pub speed: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub color: Rgba8,
    pub index: usize,
}

impl BouncingBall {
    pub fn new(
        win_rect: &Rect,
        color: Rgba8,
        radius: f32,
        horizontal_velocity: &RangeHolder<f32>,
        vertical_velocity: &RangeHolder<f32>,
        speed: f32,
        index: usize,
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
            index,
        }
    }
}

impl AnimatedObject for BouncingBall {
    fn update(&mut self, win_rect: &Rect, delta_time: f32, clock: &TimeCode) {
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
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn color(&self) -> Rgba8 {
        self.color
    }

    fn is_dead(&self) -> bool {
        false
    }
}
