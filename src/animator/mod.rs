extern crate nannou;
use anyhow::Result;

use crate::{
    animator::animation_type::{AnimationType, ModeHelper},
    receiver::ReceiverGrid,
};
use nannou::prelude::*;
use nannou_egui::egui;

pub mod animation_type;
pub mod animator_structs;
pub mod bouncing_ball;
pub mod gravity_fountain;
pub mod presets_manager;
pub mod pulse_background;
pub mod scan_line;

use bouncing_ball::BouncingBallSettings;
use gravity_fountain::GravityFountainSettings;
use presets_manager::PresetManager;
use pulse_background::PulseBackgroundSettings;
use scan_line::ScanLineSettings;

pub enum ObjectShape {
    Circle(Vec2, f32),
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
    fn animation_type(&self) -> AnimationType;
    fn create(&self) -> Vec<Box<dyn AnimatedObject>>;
    fn set_dimension(&mut self, window_rect: &Rect) {}

    fn save(&self, filename: &str, animation_type: AnimationType) -> Result<bool>
    where
        Self: serde::Serialize,
    {
        let json = serde_json::to_string_pretty(self)?;
        PresetManager::save_to_file(filename, &animation_type, json)?;
        Ok(true)
    }
}

pub struct Animator {
    pub objects: Vec<Box<dyn AnimatedObject>>,
    pub grid: ReceiverGrid,
    pub curr_an_type: AnimationType,

    bouncing_ball_settings: BouncingBallSettings,
    gravity_fountain_settings: GravityFountainSettings,
    scanline_settings: ScanLineSettings,
    pulse_bg_settings: PulseBackgroundSettings,
}

impl Animator {
    pub fn new(win_rect: &Rect, grid: ReceiverGrid) -> Self {
        let bouncing_ball_settings = BouncingBallSettings::new(win_rect);
        let scanline_settings = ScanLineSettings::new(win_rect);
        let gravity_settings = GravityFountainSettings::new(win_rect);
        let pulse_settings = PulseBackgroundSettings::new(win_rect);

        Animator {
            objects: Vec::new(),
            curr_an_type: AnimationType::BouncingBalls,
            grid,
            bouncing_ball_settings,
            gravity_fountain_settings: gravity_settings,
            scanline_settings,
            pulse_bg_settings: pulse_settings,
        }
    }
    /// Clears and repopulates objects based on current settings.
    pub fn reset(&mut self, win_rect: &Rect) {
        self.objects.clear();
        // keep settings in sync with the current window
        self.gravity_fountain_settings.set_dimension(win_rect);
        self.bouncing_ball_settings.set_dimension(win_rect);

        self.objects = match self.curr_an_type {
            AnimationType::BouncingBalls => self.bouncing_ball_settings.create(),
            AnimationType::GravityFountain => self.gravity_fountain_settings.create(),
            AnimationType::ScanLine => self.scanline_settings.create(),
            AnimationType::PulseBackground => self.pulse_bg_settings.create(),
        };
    }

    pub fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        // Update all objects
        for obj in self.objects.iter_mut() {
            obj.update(win_rect, delta_time);
        }

        // Remove dead objects ()
        self.objects.retain(|obj| !obj.is_dead());

        // Restart animation loop if all particles are gone (for GravityFountain)
        if self.objects.is_empty() && self.curr_an_type == AnimationType::GravityFountain {
            self.objects = self.gravity_fountain_settings.create();
        }

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
                changed |= self.gravity_fountain_settings.ui(ui);
            }
            AnimationType::ScanLine => {
                changed |= self.scanline_settings.ui(ui);
            }
            AnimationType::PulseBackground => {
                changed |= self.pulse_bg_settings.ui(ui);
            }
        }
        changed
    }
}
