use super::{AnimatedObject, AnimatorSettings, ObjectShape};
use crate::animator::animation_type::ModeHelper;
use crate::animator::animator_structs::AnimationParam;
use crate::animator::presets_manager::PresetManager;
use crate::animator::{animation_type::AnimationType};
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};

#[derive( Serialize, Deserialize, Default)]
pub struct GravityFountainSettings {
    origin_x: AnimationParam<f32>,
    origin_y: AnimationParam<f32>,
    ball_count: AnimationParam<u32>,
    speed: AnimationParam<f32>,
    spread: AnimationParam<f32>,
    radius: AnimationParam<f32>,
    color: AnimationParam<Rgba>,
    angle_min: AnimationParam<f32>, // In degrees
    angle_max: AnimationParam<f32>, // In degrees

    #[serde(skip)]
    presets: PresetManager,
}

impl AnimatorSettings for GravityFountainSettings {
    fn new(win_rect: &Rect) -> Self {
        Self {
            origin_x: AnimationParam::new(0.0, 0.0, win_rect.w(), "origin_X"),
            origin_y: AnimationParam::new(win_rect.h() * 0.2, 0.0, win_rect.h(), "origin_Y"),
            ball_count: AnimationParam::new(20, 1, 200, "ball_Count"),
            spread: AnimationParam::new(20.0, 1.0, 200.0, "spread"),
            speed: AnimationParam::new(-150.0, -500.0, 500.0, "speed"),
            radius: AnimationParam::new(5.0, 1.0, 20.0, "radius"),
            color: AnimationParam::new_without_range(Rgba::new(1.0, 0.0, 0.0, 1.0), "color"),
            angle_min: AnimationParam::new(-60.0, -180.0, 180.0, "Min Angle"),
            angle_max: AnimationParam::new(60.0, -180.0, 180.0, "Max Angle"),
            presets: PresetManager::new(AnimationType::GravityFountain)
           
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading(self.animation_type().as_str());
        ui.add_space(5.0);

        ui.label("Ball Count");
        changed |= self.ball_count.to_slider(ui);

        ui.add_space(5.0);
        ui.label("Origin X/Y:");
        changed |= ui
            .horizontal(|ui| {
                let c1 = self.origin_x.to_slider(ui);
                let c2 = self.origin_y.to_slider(ui);
                c1 || c2
            })
            .inner;
        ui.add_space(5.0);

        ui.label("Speed");
        changed |= self.speed.to_slider(ui);
        ui.add_space(5.0);

        ui.label("Spread");
        changed |= self.spread.to_slider(ui);
        ui.add_space(5.0);

        ui.label("Radius:");
        changed |= self.radius.to_slider(ui);

        ui.add_space(5.0);

        changed |= self.color.to_color_picker(ui);

        ui.add_space(5.0);
        ui.label("Spray Angle Range (degrees):");
        ui.label("(-90° = left, 0° = up, 90° = right)");
        changed |= ui
            .horizontal(|ui| {
                ui.label("Min:");
                let c1 = self.angle_min.to_slider(ui);
                ui.label("Max:");
                let c2 = self.angle_max.to_slider(ui);

                // Ensure min <= max
                if self.angle_min.value > self.angle_max.value {
                    std::mem::swap(&mut self.angle_min.value, &mut self.angle_max.value);
                }

                c1 || c2
            })
            .inner;

        changed
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::GravityFountain
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        let mut animated_objects: Vec<Box<dyn AnimatedObject>> = Vec::new();
        for _ in 0..self.ball_count.value {
            let gravity_particle = Box::new(GravityParticle::new(
                Vec2::new(self.origin_x.value, self.origin_y.value),
                self.speed.value,
                self.radius.value,
                self.color.value,
                self.spread.value,
                self.angle_min.value.to_radians(),
                self.angle_max.value.to_radians(),
            ));

            animated_objects.push(gravity_particle);
        }

        animated_objects
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        // self.dimension = Some(*window_rect);
    }
}

pub struct GravityParticle {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    color: Rgba,
    speed: f32,
    is_dead: bool,
    spread: f32,
}

impl GravityParticle {
    pub fn new(
        origin: Vec2,
        speed: f32,
        radius: f32,
        color: Rgba,
        spread: f32,
        angle_min: f32, // in radians
        angle_max: f32, // in radians
    ) -> Self {
        let angle = random_range(angle_min, angle_max) + PI / 2.0;

        let velocity = vec2(angle.cos(), angle.sin()) * spread;

        GravityParticle {
            position: origin,
            velocity,
            radius,
            color,
            is_dead: false,
            speed,
            spread,
        }
    }
}

impl AnimatedObject for GravityParticle {
    fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        self.velocity.y += self.speed * delta_time;
        self.position += self.velocity * delta_time * self.spread;

        // Kill particle if it hits the floor
        if self.position.y < win_rect.bottom() - self.radius {
            self.is_dead = true;
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.radius)
            .color(self.color);
    }

    fn is_dead(&self) -> bool {
        self.is_dead
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Circle(self.position, self.radius)
    }

    fn color(&self) -> Rgba {
        self.color
    }
}
