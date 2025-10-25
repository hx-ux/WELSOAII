extern crate nannou;
use crate::reciver::ReciverGrid;
use nannou::{color::rgb::Rgb, prelude::*};
use nannou_egui::egui::{self, widgets};

use self::AnimationType::*;
use std::slice::Iter;
use crate::Utils::*;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AnimationType {
    BouncingBalls,
    GravityFountain,
    ScanLine,
}

impl AnimationType {
    pub fn iterator() -> Iter<'static, AnimationType> {
        static DIRECTIONS: [AnimationType; 3] = [BouncingBalls, GravityFountain, ScanLine];
        DIRECTIONS.iter()
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            AnimationType::BouncingBalls => "Bouncing Balls",
            AnimationType::GravityFountain => "Gravity Fountain",
            AnimationType::ScanLine => "Scan Line",
        }
    }
}

pub trait AnimatedObject {
    fn update(&mut self, win_rect: &Rect, delta_time: f32);
    fn draw(&self, draw: &Draw);
    fn is_dead(&self) -> bool {
        false
    }
    fn position(&self) -> Vec2;
    fn color(&self) -> Rgba;
    fn ui(&mut self, ui: &mut egui::Ui) -> bool;
}

pub struct BouncingBall {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub color: Rgba,
}

impl BouncingBall {
    pub fn new(win_rect: &Rect, color: Rgba) -> Self {
        let radius = random_range(10.0, 30.0);
        BouncingBall {
            position: vec2(
                random_range(win_rect.left() + radius, win_rect.right() - radius),
                random_range(win_rect.bottom() + radius, win_rect.top() - radius),
            ),
            velocity: vec2(random_range(-200.0, 200.0), random_range(-200.0, 200.0)),
            radius,
            color,
        }
    }
}

impl AnimatedObject for BouncingBall {
    fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        self.position += self.velocity * delta_time;

        if self.position.x < win_rect.left() + self.radius
            || self.position.x > win_rect.right() - self.radius
        {
            self.velocity.x *= -1.0;
        }
        if self.position.y < win_rect.bottom() + self.radius
            || self.position.y > win_rect.top() - self.radius
        {
            self.velocity.y *= -1.0;
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.radius)
            .color(self.color);
    }

    fn is_dead(&self) -> bool {
        false // Bouncing balls live forever.
    }
    fn position(&self) -> Vec2 {
        self.position
    }

    fn color(&self) -> Rgba {
        self.color
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        false
    }
}

pub struct GravityParticle {
    position: Vec2,
    init_pos: Vec2,
    velocity: Vec2,
    radius: f32,
    color: Rgba,
    life: f32, // Time to live in seconds
}

impl GravityParticle {
    const GRAVITY: f32 = 980.0;

    pub fn new(win_rect: &Rect, origin: Vec2, color: Rgba) -> Self {
        let angle = random_range(-PI / 4.0, PI / 4.0) - PI / 2.0;
        let speed = random_range(400.0, 800.0);
        let velocity = vec2(angle.cos(), angle.sin()) * speed;

        GravityParticle {
            position: origin,
            init_pos: origin,
            velocity,
            radius: random_range(5.0, 20.0),
            color,
            life: random_range(1.0, 3.0),
        }
    }
}

impl AnimatedObject for GravityParticle {
    fn update(&mut self, _win_rect: &Rect, delta_time: f32) {
        self.velocity.y -= Self::GRAVITY * delta_time;
        self.position += self.velocity * delta_time;
        // self.life -= delta_time;

        if _win_rect.bottom() >= self.position[1] || self.is_dead() {
            self.position = self.init_pos;
        }
    }

    fn draw(&self, draw: &Draw) {
        // Fade out as the particle dies
        let mut display_color = self.color;
        display_color.alpha = (self.life / 2.0).clamp(0.0, 1.0);

        draw.ellipse()
            .xy(self.position)
            .radius(self.radius)
            .color(display_color);
    }

    fn is_dead(&self) -> bool {
        self.life <= 0.0
    }
    fn position(&self) -> Vec2 {
        self.position
    }

    fn color(&self) -> Rgba {
        self.color
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        todo!()
    }
}

pub struct ScanLine {
    mode: i16,
    speed: f32,
    color: Rgba,
    position: Vec2,
    init_pos: Vec2,
    window_dimension: (f32, f32),
}

impl ScanLine {
    pub fn new(win_rect: &Rect, mode: i16, speed: f32, color: Rgba, position: Vec2) -> Self {
        ScanLine {
            mode: mode,
            speed: speed,
            color: color,
            position: position,
            init_pos: position,
            window_dimension: win_rect.w_h(),
        }
    }
}
/* ------------------------------------------- */

impl AnimatedObject for ScanLine {
    fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        self.window_dimension = win_rect.w_h();
        self.position.x += self.speed * delta_time;

        let half_width = 10.0;

        let left_bound = win_rect.left() + half_width;
        let right_bound = win_rect.right() - half_width;

        match self.mode {
            // Mode 0: Ping-Pong ("left to right and vice versa")
            0 => {
                if self.position.x > right_bound {
                    // Hit right edge
                    self.position.x = right_bound; // Clamp position
                    self.speed = -self.speed; // Reverse direction
                } else if self.position.x < left_bound {
                    // Hit left edge
                    self.position.x = left_bound; // Clamp position
                    self.speed = -self.speed; // Reverse direction
                }
            }
            // Mode 1: Wrap-around
            _ => {
                if self.position.x > right_bound && self.speed > 0.0 {
                    // Went off the right edge, wrap to left
                    self.position.x = left_bound;
                } else if self.position.x < left_bound && self.speed < 0.0 {
                    // Went off the left edge, wrap to right
                    self.position.x = right_bound;
                }
            }
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.rect()
            .xy(self.position)
            .height(self.window_dimension.1) // Use the stored window height
            .width(20.0)
            .color(self.color);
    }

    fn position(&self) -> Vec2 {
        // doesnt work, bc the positon is on the top
        // so the reciver doesnt recognize it
        self.position
    }

    fn color(&self) -> Rgba {
        self.color
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut reset = false;
        ui.heading("Scan Line");

        // Use a grid for a cleaner layout
        egui::Grid::new("scanline_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                // Color control
                ui.label("Color:");
                // ui.color_edit_button_rgba(&mut self.color);
                ui.end_row();

                // Speed control
                ui.label("Speed:");
                ui.add(
                    egui::DragValue::new(&mut self.speed)
                        .speed(1.0)
                        .clamp_range(-500.0..=500.0),
                );
                ui.end_row();

                // Mode control
                ui.label("Mode:");
                egui::ComboBox::from_label("")
                    .selected_text(if self.mode == 0 { "Ping-Pong" } else { "Wrap" })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.mode, 0, "Ping-Pong");
                        ui.selectable_value(&mut self.mode, 1, "Wrap");
                    });
                ui.end_row();
            });

        ui.add_space(10.0);

        // Reset button
        if ui.button("Reset Position").clicked() {
            self.position = self.init_pos;
            reset = true;
        }

        reset
    }
}
pub struct AnimatorSettings {
    pub count: u32,
    pub multicolor: bool,
    pub animation_type: AnimationType,
}

pub struct AnimatorNew {
    pub objects: Vec<Box<dyn AnimatedObject>>,
    pub settings: AnimatorSettings,
    pub grid: ReciverGrid,
    pub color: Rgba,
    egui_color: egui::Color32,
}

impl AnimatorNew {
    pub fn new(win_rect: &Rect) -> Self {
        AnimatorNew {
            objects: Vec::new(),
            settings: AnimatorSettings {
                count: 10,
                multicolor: true,
                animation_type: AnimationType::BouncingBalls,
            },
            grid: ReciverGrid::new(win_rect.pad(20.0), 10, 10),
            color: Rgba::new(1.0, 0.0, 0.0, 1.0),
            egui_color: egui::Color32::from_rgba_unmultiplied(100, 140, 200, 255),
        }
    }

    pub fn link_grid(&mut self, grid: ReciverGrid) {
        self.grid = grid;
    }

    /// Clears and generates a new set of objects based on current settings.
    pub fn reset(&mut self, win_rect: &Rect) {
        let fountain_origin = vec2(0.0, win_rect.top());

        self.objects.clear();

        for elements in 0..self.settings.count {

            let color = if self.settings.multicolor {
                Rgba::random()
            } else {
                self.color
            };

            let new_obj: Box<dyn AnimatedObject> = match self.settings.animation_type {
                AnimationType::BouncingBalls => Box::new(BouncingBall::new(win_rect, color)),
                AnimationType::GravityFountain => {
                    Box::new(GravityParticle::new(win_rect, fountain_origin, color))
                }
                AnimationType::ScanLine => {
                    Box::new(ScanLine::new(win_rect, 0, 20.0, color, fountain_origin))
                }
            };
            self.objects.push(new_obj);
        }
    }

    pub fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        // animate objects by time
        for obj in self.objects.iter_mut() {
            obj.update(win_rect, delta_time);
        }

        // Remove dead particles
        self.objects.retain(|obj| !obj.is_dead());

        for cell in self.grid.cells.iter_mut() {
            cell.reset();
        }

        for obj in &self.objects {
            let obj_pos = obj.position();
            for cell in self.grid.cells.iter_mut() {
                if cell.rect.contains(obj_pos) {
                    // TODO Animator can fill multiple Cells
                    cell.is_active = true;

                    cell.found_color = obj.color();

                    // cell.found_color.red += obj.color().red;
                    // cell.found_color.green += obj.color().green;
                    // cell.found_color.blue += obj.color().blue;
                }
            }
        }
    }

    pub fn draw(&self, draw: &Draw) {
        self.grid.draw(draw);
        for obj in &self.objects {
            obj.draw(draw);
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.label("Animator");

        ui.label("Animation Type");
        ui.horizontal(|ui| {
            for t in AnimationType::iterator() {
                if ui
                    .radio_value(&mut self.settings.animation_type, *t, t.as_str())
                    .clicked()
                {
                    changed = true;
                };
            }
        });

        // other controls
        if ui
            .add(egui::Slider::new(&mut self.settings.count, 1..=100).text("Object Count"))
            .changed()
        {
            changed = true;
        }

        if ui
            .checkbox(&mut self.settings.multicolor, "Multicolor")
            .clicked()
        {
            changed = true;
        }
        if ui.color_edit_button_srgba(&mut self.egui_color).changed() {
            self.color = Rgba::from_egui(self.egui_color);
            changed = true;
        }

        changed
    }
}
