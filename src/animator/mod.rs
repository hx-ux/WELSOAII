extern crate nannou;
use crate::{animator::animation_type::AnimationType, reciver::ReciverGrid};
use nannou::prelude::*;
use nannou_egui::egui;

pub mod animation_type;
pub mod animator_structs;
pub mod bouncing_ball;
pub mod gravity_fountain;
pub mod pulse_background;
pub mod scan_line;

use bouncing_ball::{BouncingBall, BouncingBallSettings};
use gravity_fountain::{GravityFountainSettings, GravityParticle};
use pulse_background::{PulseBackground, PulseBackgroundSettings};
use scan_line::{ScanLine, ScanLineSettings};

pub enum ObjectShape {
    Circle(Vec2, f32), // (position, radius)
    Rect(Rect),
}

pub trait AnimatedObject {
    fn update(&mut self, win_rect: &Rect, delta_time: f32);
    fn draw(&self, draw: &Draw);
    // partial obsolete
    fn is_dead(&self) -> bool {
        false
    }
    fn shape(&self) -> ObjectShape;
    fn color(&self) -> Rgba;
}

pub trait AnimatorSettings {
    fn new(win_rect: &Rect) -> Self;
    fn ui(&mut self, ui: &mut egui::Ui) -> bool;
    fn get_ani_type(&self) -> AnimationType;
    fn create(&self) -> Vec<Box<dyn AnimatedObject>>;
    fn set_dimension(&mut self, window_rect: &Rect) {}
}

// These are the global settings
pub struct Animator {
    pub objects: Vec<Box<dyn AnimatedObject>>,
    pub grid: ReciverGrid,
    pub color: Rgba,
    pub curr_an_type: AnimationType,
    egui_color: egui::Color32,

    // Settings for each animation type
    bouncing_ball_settings: BouncingBallSettings,
    gravity_settings: GravityFountainSettings,
    scanline_settings: ScanLineSettings,
    pulse_settings: PulseBackgroundSettings,
}

impl Animator {
    pub fn new(win_rect: &Rect, grid: ReciverGrid) -> Self {
        let bouncing_ball_settings = BouncingBallSettings::new(win_rect);
        let scanline_settings = ScanLineSettings::new(win_rect);
        let gravity_settings = GravityFountainSettings::new(win_rect);
        let pulse_settings = PulseBackgroundSettings::default();

        Animator {
            objects: Vec::new(),
            curr_an_type: AnimationType::BouncingBalls,
            grid,
            color: Rgba::new(1.0, 0.0, 0.0, 1.0),
            egui_color: egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255),

            bouncing_ball_settings,
            gravity_settings,
            scanline_settings,
            pulse_settings,
        }
    }
    /// Clears and repopulates objects based on current settings.
    pub fn reset(&mut self, win_rect: &Rect) {
        self.objects.clear();
        // keep settings in sync with the current window
        self.bouncing_ball_settings.set_dimension(win_rect);
        // self.objects =self.curr_an_type.cre
        self.objects = self.bouncing_ball_settings.create();
    }

    pub fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        // Update all objects
        for obj in self.objects.iter_mut() {
            obj.update(win_rect, delta_time);
        }

        // Remove dead objects (OBSOLETE)
        self.objects.retain(|obj| !obj.is_dead());

        // Reset grid cells
        for cell in self.grid.cells.iter_mut() {
            cell.reset();
        }

        for obj in &self.objects {
            let obj_shape = obj.shape();
            let obj_color = obj.color();

            for cell in self.grid.cells.iter_mut() {
                if cell.is_active {
                    continue;
                }
                // Extended Collsion detection
                let intersects = match obj_shape {
                    ObjectShape::Circle(pos, radius) => {
                        let closest_x = pos.x.clamp(cell.rect.left(), cell.rect.right());
                        let closest_y = pos.y.clamp(cell.rect.bottom(), cell.rect.top());
                        let distance_sq = (pos.x - closest_x).powi(2) + (pos.y - closest_y).powi(2);
                        distance_sq < (radius * radius)
                    }
                    ObjectShape::Rect(obj_rect) => {
                        obj_rect.left() < cell.rect.right()
                            && obj_rect.right() > cell.rect.left()
                            && obj_rect.top() > cell.rect.bottom()
                            && obj_rect.bottom() < cell.rect.top()
                    }
                };

                if intersects {
                    cell.is_active = true;
                    cell.display_color = obj_color;
                }
            }
        }
    }

    // draws the created shapes from the animators
    pub fn draw_animator(&self, draw: &Draw) {
        for obj in &self.objects {
            obj.draw(draw);
        }
    }
    // draws the grid
    pub fn draw_grid(&self, draw: &Draw) {
        self.grid.draw(draw);
    }

    /// Draws the main UI panel
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.separator();

        // --- General Settings ---
        ui.label("Animation Type");
        // Radio buttons for selecting the animation type
        ui.horizontal(|ui| {
            for options in AnimationType::iterator() {
                if ui
                    .radio_value(&mut self.curr_an_type, *options, options.as_str())
                    .changed()
                {
                    changed = true;
                };
            }
        });

        ui.add_space(5.0);

        ui.separator();

        // --- Show controls of each animator settings
        match self.curr_an_type {
            AnimationType::BouncingBalls => {
                changed |= self.bouncing_ball_settings.ui(ui);
            }
            AnimationType::GravityFountain => {
                changed |= self.gravity_settings.ui(ui);
            }
            AnimationType::ScanLine => {
                changed |= self.scanline_settings.ui(ui);
            }
            AnimationType::PulseBackground => {
                changed |= self.pulse_settings.ui(ui);
            }
        }
        changed
    }
}
