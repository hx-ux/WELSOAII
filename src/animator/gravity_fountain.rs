use super::{AnimatedObject, ObjectShape};
use crate::{
    Utils::ColorHelpers,
    animator::{AnimatorSettings, animation_type::AnimationType, animator_structs::RangeHolder},
};
use nannou::prelude::*;
use nannou_egui::egui;

#[derive(Debug)]
pub struct GravityFountainSettings {
    origin: Vec2,
    velocity: Vec2,
    ball_count: u32,
    speed: f32,
    spread: f32,
    radius: f32,
    dimension: Rect,
    color: Rgba,
}

impl AnimatorSettings for GravityFountainSettings {
    fn new(win_rect: &Rect) -> Self {
        Self {
            origin: vec2(0.0, win_rect.h() * 0.2),
            velocity: vec2(0.0, -10.0),
            spread: 50.0,
            ball_count: 20,
            speed: 4.0,
            radius: 5.0,
            dimension: win_rect.to_owned(),
            color: Rgba::red(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading(self.get_ani_type().as_str());
        ui.add_space(5.0);

        ui.label("Ball Count");
        changed |= ui
            .add(egui::Slider::new(&mut self.ball_count, 1..=200).text("Spread"))
            .changed();
        ui.add_space(5.0);

        ui.label("Origin X/Y:");
        changed |= ui
            .horizontal(|ui| {
                let c1 = ui
                    .add(egui::DragValue::new(&mut self.origin.x).speed(1.0))
                    .changed();
                let c2 = ui
                    .add(egui::DragValue::new(&mut self.origin.y).speed(1.0))
                    .changed();
                c1 || c2
            })
            .inner;
        ui.add_space(5.0);

        ui.label("Speed");
        changed |= ui
            .add(egui::Slider::new(&mut self.speed, 1.0..=10.0).text("Speed"))
            .changed();
        ui.add_space(5.0);

        ui.label("Spread");
        changed |= ui
            .add(egui::Slider::new(&mut self.spread, 1.0..=200.0).text("Spread"))
            .changed();
        ui.add_space(5.0);

        ui.label("Radius:");
        changed |= ui
            .add(egui::Slider::new(&mut self.radius, 1.0..=10.0).text("Radius"))
            .changed();

        ui.add_space(5.0);

        changed
    }

    fn get_ani_type(&self) -> AnimationType {
        AnimationType::GravityFountain
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        let mut animated_objects: Vec<Box<dyn AnimatedObject>> = Vec::new();
        for _ in 0..self.ball_count {
            let particleBox = Box::new(GravityParticle::new(
                self.origin,
                self.velocity,
                self.speed,
                self.radius,
                self.color,
                self.spread,
            ));

            animated_objects.push(particleBox);
        }

        animated_objects
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.dimension = *window_rect;
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
    const GRAVITY: f32 = -980.0; // Gravity points down
    // TODO values always constant, no way to change
    const FALL_ANGLE: RangeHolder<f32> = RangeHolder {
        lower: -PI / 3.0,
        upper: PI / 3.0,
    };

    pub fn new(
        origin: Vec2,
        velocity: Vec2,
        speed: f32,
        radius: f32,
        color: Rgba,
        spread: f32,
    ) -> Self {
        let angle = random_range(Self::FALL_ANGLE.lower, Self::FALL_ANGLE.upper) + PI / 2.0;

        let velocity = vec2(angle.cos(), angle.sin()) * spread;

        GravityParticle {
            position: origin,
            velocity,
            radius,
            color,
            is_dead: false, // life,
            speed,
            spread,
        }
    }
}

impl AnimatedObject for GravityParticle {
    fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        self.velocity.y += Self::GRAVITY * delta_time * (self.speed / 10.0);
        self.position += self.velocity * delta_time;

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
