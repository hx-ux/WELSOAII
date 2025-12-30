use super::{AnimatedObject, ObjectShape};
use nannou::prelude::*;
use nannou_egui::egui;


#[derive(Debug)]
pub struct GravityFountainSettings {
    pub origin: Vec2,
    pub min_angle: f32,
    pub max_angle: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub min_radius: f32,
    pub max_radius: f32,
    pub min_life: f32,
    pub max_life: f32,
}

impl GravityFountainSettings {
    
    pub fn new(win_rect: &Rect) -> Self {
        Self {
            origin: vec2(0.0, win_rect.h()*0.2),
            min_angle: -PI / 3.0,
            max_angle: PI / 3.0,
            min_speed: 400.0,
            max_speed: 800.0,
            min_radius: 5.0,
            max_radius: 20.0,
            min_life: 1.0,
            max_life: 3.0,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading("Fountain Settings");
        egui::Grid::new("gravity_settings")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Origin Y:");
                changed |= ui
                    .add(
                        egui::DragValue::new(&mut self.origin.y)
                            .speed(1.0),
                    )
                    .changed();
                ui.end_row();

                ui.label("Speed (min/max):");
                ui.horizontal(|ui| {
                    changed |= ui
                        .add(
                            egui::DragValue::new(&mut self.min_speed)
                                .speed(10.0),
                        )
                        .changed();
                    changed |= ui
                        .add(
                            egui::DragValue::new(&mut self.max_speed)
                                .speed(10.0),
                        )
                        .changed();
                });
                ui.end_row();

                ui.label("Radius (min/max):");
                ui.horizontal(|ui| {
                    changed |= ui
                        .add(
                            egui::DragValue::new(&mut self.min_radius)
                                .speed(0.5),
                        )
                        .changed();
                    changed |= ui
                        .add(
                            egui::DragValue::new(&mut self.max_radius)
                                .speed(0.5),
                        )
                        .changed();
                });
                ui.end_row();

                ui.label("Life (min/max):");
                ui.horizontal(|ui| {
                    changed |= ui
                        .add(
                            egui::DragValue::new(&mut self.min_life)
                                .speed(0.1),
                        )
                        .changed();
                    changed |= ui
                        .add(
                            egui::DragValue::new(&mut self.max_life)
                                .speed(0.1),
                        )
                        .changed();
                });
                ui.end_row();
            });
        changed
    }
}

pub struct GravityParticle {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    color: Rgba,
    life: f32, 
    max_life: f32,
}

impl GravityParticle {
    const GRAVITY: f32 = -980.0; // Gravity points down

    pub fn new(origin: Vec2, velocity: Vec2, radius: f32, color: Rgba, life: f32) -> Self {
        GravityParticle {
            position: origin,
            velocity,
            radius,
            color,
            life,
            max_life: life, 
        }
    }

    /// Creates a new particle based on fountain settings.
    pub fn factory(settings: &GravityFountainSettings, color: Rgba) -> Self {
        
        let angle = random_range(settings.min_angle, settings.max_angle) + PI / 2.0; // Point up
        let speed = random_range(settings.min_speed, settings.max_speed);
        let velocity = vec2(angle.cos(), angle.sin()) * speed;
        let radius = random_range(settings.min_radius, settings.max_radius);
        let life = random_range(settings.min_life, settings.max_life);

        GravityParticle::new(settings.origin, velocity, radius, color, life)
    }
}

impl AnimatedObject for GravityParticle {
    fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        self.velocity.y += Self::GRAVITY * delta_time;
        self.position += self.velocity * delta_time;
        self.life -= delta_time;

        // Kill particle if it hits the floor
        if self.position.y < win_rect.bottom() - self.radius {
            self.life = 0.0;
        }
    }

    fn draw(&self, draw: &Draw) {
        // Fade out as the particle dies
        let mut display_color = self.color;

        if self.max_life > 0.0 {
            display_color.alpha = (self.life / self.max_life).clamp(0.0, 1.0);
        } else {

            display_color.alpha = if self.life > 0.0 { 1.0 } else { 0.0 };
        }

        draw.ellipse()
            .xy(self.position)
            .radius(self.radius)
            .color(display_color);
    }

    fn is_dead(&self) -> bool {
        self.life <= 0.0
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Circle(self.position, self.radius)
    }

    fn color(&self) -> Rgba {
        self.color
    }
}
