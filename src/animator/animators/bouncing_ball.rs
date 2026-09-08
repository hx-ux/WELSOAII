use crate::{
    animator::{AnimatedObject, AnimatorSettings, ObjectShape, animation_type::AnimationType},
    color::ColorParam,
    modulator::Modulator,
    parameters::{ConstantParam, ModulatedParam},
    timecode::TimeCode,
};

use anyhow::Ok;
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};

fn default_rect() -> Rect {
    Rect::from_w_h(800.0, 600.0)
}

#[derive(Serialize, Deserialize)]
pub struct BouncingBallSettings {
    pub ball_count: ConstantParam<u32>,
    pub speed: ModulatedParam,
    pub radius: ModulatedParam,
    ball_vel_range_x: ConstantParam<f32>,
    ball_vel_range_y: ConstantParam<f32>,
    #[serde(skip)]
    #[serde(default = "default_rect")]
    dimension: Rect,
    color: ColorParam,
    #[serde(skip)]
    animator: Vec<BouncingBallAnimator>, // Refactored to concrete type
}

impl BouncingBallSettings {
    pub fn new(win_rect: &Rect) -> Self {
        Self {
            ball_count: ConstantParam::new(20, 1, 400, "Ball Count", "ball_count"),
            speed: ModulatedParam::new(5.0, 1.0, 20.0, "Speed", "bounce_speed"),
            dimension: *win_rect,
            radius: ModulatedParam::new(10.0, 4.0, 40.0, "Radius", "bounce_radius"),
            ball_vel_range_x: ConstantParam::new(10.0, 1.0, 200.0, "Range X", "range_x"),
            ball_vel_range_y: ConstantParam::new(15.0, 1.0, 200.0, "Range Y", "range_y"),
            color: ColorParam::default(),
            animator: Vec::new(),
        }
    }
}

impl AnimatorSettings for BouncingBallSettings {
    fn control_ui(&mut self, ui: &mut egui::Ui, mods: &mut Vec<Box<dyn Modulator>>) {
        if self.ball_count.to_slider(ui) {
            self.hot_update();
        }
        if self.radius.to_slider_modulate(ui, mods) {
            self.hot_update();
        }
        if self.speed.to_slider_modulate(ui, mods) {
            self.hot_update();
        }

        if ui.horizontal(|ui| self.ball_vel_range_x.to_drag(ui)).inner {
            self.init();
        }

        if ui.horizontal(|ui| self.ball_vel_range_y.to_drag(ui)).inner {
            self.init();
        }

        if self.color.ui(ui) {
            self.hot_update();
        }
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::BouncingBalls
    }

    fn init(&mut self) {
        self.animator.clear();

        for index in 0..self.ball_count.value as usize {
            let new_obj = BouncingBallAnimator::new(
                &self.dimension,
                self.color.clone().value_mapped(index),
                *self.radius.value(),
                self.ball_vel_range_x.value,
                self.ball_vel_range_y.value,
                *self.speed.value(),
                index,
            );
            self.animator.push(new_obj);
        }
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.dimension = *window_rect;
    }

    fn hot_update(&mut self) {
        let current_count = self.animator.len();
        let target_count = self.ball_count.value as usize;

        if target_count > current_count {
            for index in current_count..target_count {
                let new_obj = BouncingBallAnimator::new(
                    &self.dimension,
                    self.color.clone().value_mapped(index),
                    *self.radius.value(),
                    self.ball_vel_range_x.value,
                    self.ball_vel_range_y.value,
                    *self.speed.value(),
                    index,
                );
                self.animator.push(new_obj);
            }
        } else if target_count < current_count {
            self.animator.truncate(target_count);
        }

        // Update existing balls with new parameters directly
        for ball in self.animator.iter_mut() {
            ball.speed = *self.speed.value();
            ball.radius = *self.radius.value();
            ball.color = self.color.clone().value_mapped(ball.index);
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
        vec![&mut self.speed, &mut self.radius]
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn color_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| self.color.ui(ui));
    }
}

pub struct BouncingBallAnimator {
    pub speed: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub color: Rgba8,
    pub index: usize,
}

impl BouncingBallAnimator {
    fn randomize_ball_spread(h: f32, w: f32) -> Vec2 {
        let vx = random_range(-h, h);
        let vy = random_range(-w, w);
        vec2(vx, vy)
    }

    fn randomize_ball_position(win: &Rect, r: f32) -> Vec2 {
        vec2(
            random_range(win.left() + r, win.right() - r),
            random_range(win.bottom() + r, win.top() - r),
        )
    }

    pub fn new(
        win_rect: &Rect,
        color: Rgba8,
        radius: f32,
        horizontal_velocity: f32,
        vertical_velocity: f32,
        speed: f32,
        index: usize,
    ) -> Self {
        BouncingBallAnimator {
            position: Self::randomize_ball_position(win_rect, radius),
            velocity: Self::randomize_ball_spread(horizontal_velocity, vertical_velocity),
            radius,
            color,
            speed,
            index,
        }
    }
}

impl AnimatedObject for BouncingBallAnimator {
    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode) {
        self.position += self.velocity * timecode.get_delta_time() * self.speed;

        let min_x = win_rect.left() + self.radius;
        let max_x = win_rect.right() - self.radius;
        let min_y = win_rect.bottom() + self.radius;
        let max_y = win_rect.top() - self.radius;

        if self.position.x < min_x {
            self.position.x = min_x;
            self.velocity.x = self.velocity.x.abs();
        } else if self.position.x > max_x {
            self.position.x = max_x;
            self.velocity.x = -self.velocity.x.abs();
        }

        if self.position.y < min_y {
            self.position.y = min_y;
            self.velocity.y = self.velocity.y.abs();
        } else if self.position.y > max_y {
            self.position.y = max_y;
            self.velocity.y = -self.velocity.y.abs();
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

    fn color(&self) -> Rgba8 {
        self.color
    }
    fn is_dead(&self) -> bool {
        false
    }
}
