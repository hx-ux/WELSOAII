extern crate nannou;
use anyhow::Result;
use strum::IntoEnumIterator;

use crate::{
    animator::animation_type::{AnimationType},
    receiver::ReceiverGrid,
};
use nannou::prelude::*;
use nannou_egui::egui;

pub mod animation_type;
pub mod animator_structs;
pub mod bouncing_ball;
pub mod timecode;
pub mod gravity_fountain;
pub mod presets_manager;
pub mod pulse_background;
pub mod scan_line;

use bouncing_ball::BouncingBallSettings;
use timecode::TimeCode;
use gravity_fountain::GravityFountainSettings;
use pulse_background::PulseBackgroundSettings;
use scan_line::ScanLineSettings;

#[derive(Debug, PartialEq)]
// Defines, how the animators behave, if an Param is changed
pub enum UpdateBehaviour {
    None,
    // Resets the current animator and its object.
    // Mainly used for switching between Animators
    // Does call Animator::new()
    NeedsReset,
    // Hot updates, which affect the animator in the next frame(s)
    // Does not call Animator::new()
    HotUpdate,
    //
    LoadPreset,
    //
    SavePrest,
}

// An animated object, which every animator does emit
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
    fn color(&self) -> Rgba8;
    // For downcasting to concrete types
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub trait AnimatorSettings {
    fn new(win_rect: &Rect) -> Self;
    fn ui(&mut self, ui: &mut egui::Ui) -> UpdateBehaviour;
    fn animation_type(&self) -> AnimationType;
    fn create(&self) -> Vec<Box<dyn AnimatedObject>>;
    fn set_dimension(&mut self, window_rect: &Rect) {}
    // Custom Logic for Hot reloading the animator without resetting
    fn hot_update(&self, objects: &mut Vec<Box<dyn AnimatedObject>>);
    // Rest all parameter values to its definded standards
    fn reset(&mut self);
    fn force_update(&self) -> UpdateBehaviour {
        UpdateBehaviour::NeedsReset
    }
    fn save_preset(&mut self) -> Result<()>;
}

pub struct Animator {
    pub objects: Vec<Box<dyn AnimatedObject>>,
    pub grid: ReceiverGrid,
    pub curr_an_type: AnimationType,
    pub clock: TimeCode,
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
            clock: TimeCode::new(),
            grid,
            bouncing_ball_settings,
            gravity_fountain_settings: gravity_settings,
            scanline_settings,
            pulse_bg_settings: pulse_settings,
        }
    }

    /// Clears and repopulates the animations objects based on current settings.
    pub fn reset(&mut self, win_rect: &Rect) {
        self.objects.clear();
        // keep settings in sync with the current window
        // if reset in bind to on_window_resize
        self.scanline_settings.set_dimension(win_rect);
        self.gravity_fountain_settings.set_dimension(win_rect);
        self.bouncing_ball_settings.set_dimension(win_rect);
        self.pulse_bg_settings.set_dimension(win_rect);

        self.objects = match self.curr_an_type {
            AnimationType::BouncingBalls => self.bouncing_ball_settings.create(),
            AnimationType::GravityFountain => self.gravity_fountain_settings.create(),
            AnimationType::ScanLine => self.scanline_settings.create(),
            AnimationType::PulseBackground => self.pulse_bg_settings.create(),
        };
    }
    /// Apply hot updates to existing objects without recreating them
    pub fn behaviour_hot_update(&mut self) {
        match self.curr_an_type {
            AnimationType::BouncingBalls => {
                self.bouncing_ball_settings
                    .hot_update(&mut self.objects);
            }
            AnimationType::GravityFountain => {
                self.gravity_fountain_settings
                    .hot_update(&mut self.objects);
            }
            AnimationType::ScanLine => {
                self.scanline_settings.hot_update(&mut self.objects);
            }
            AnimationType::PulseBackground => {
                self.pulse_bg_settings.hot_update(&mut self.objects);
            }
        }
    }

    pub fn save_preset(&mut self) {
        print!("save");
        match self.curr_an_type {
            AnimationType::BouncingBalls => {
                let _ = self.bouncing_ball_settings.save_preset();
            }
            AnimationType::GravityFountain => {
                let _ = self.gravity_fountain_settings.save_preset();
            }
            AnimationType::ScanLine => {
                let _ = self.scanline_settings.save_preset();
            }
            AnimationType::PulseBackground => {
                let _ = self.pulse_bg_settings.save_preset();
            }
        }
    }

    pub fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        // Update the master clock
        self.clock.update(delta_time);

        // Update all objects
        for obj in self.objects.iter_mut() {
            obj.update(win_rect, delta_time);
        }

        // Remove dead objects ()
        self.objects.retain(|obj| !obj.is_dead());

        // TODO Refactor
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

            // Spatial optimization: calculate which cells could possibly intersect
            let (min_col, max_col, min_row, max_row) = match obj_shape {
                ObjectShape::Circle(pos, radius) => {
                    let left = pos.x - radius;
                    let right = pos.x + radius;
                    let bottom = pos.y - radius;
                    let top = pos.y + radius;

                    self.grid.get_cell_range(left, right, bottom, top)
                }
                ObjectShape::Rect(obj_rect) => self.grid.get_cell_range(
                    obj_rect.left(),
                    obj_rect.right(),
                    obj_rect.bottom(),
                    obj_rect.top(),
                ),
            };

            // Only check cells in the relevant range
            for row in min_row..=max_row {
                for col in min_col..=max_col {
                    let idx = self.grid.get_cell_index(row, col);
                    if idx >= self.grid.cells.len() {
                        continue;
                    }

                    let cell = &mut self.grid.cells[idx];
                    if cell.is_active {
                        continue;
                    }

                    // Collision detection
                    let intersects = match obj_shape {
                        ObjectShape::Circle(pos, radius) => {
                            let closest_x = pos.x.clamp(cell.rect.left(), cell.rect.right());
                            let closest_y = pos.y.clamp(cell.rect.bottom(), cell.rect.top());
                            let distance_sq =
                                (pos.x - closest_x).powi(2) + (pos.y - closest_y).powi(2);
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

    pub fn ui(&mut self, ui: &mut egui::Ui) -> UpdateBehaviour {
        let mut change_type = UpdateBehaviour::None;

        // to refactor to own window
        self.clock.ui(ui);
        // self.signal.ui(ui);
        ui.separator();

        ui.separator();
        // --- General Settings ---
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.curr_an_type))
            .show_ui(ui, |ui| {
                for option in AnimationType::iter() {
                    if ui
                        .selectable_value(&mut self.curr_an_type, option, format!("{}", option))
                        .clicked()
                    {
                        change_type = UpdateBehaviour::NeedsReset;
                    }
                }
            });

        // --- Show controls for each animator settings
        let settings_change = match self.curr_an_type {
            AnimationType::BouncingBalls => self.bouncing_ball_settings.ui(ui),
            AnimationType::GravityFountain => self.gravity_fountain_settings.ui(ui),
            AnimationType::ScanLine => self.scanline_settings.ui(ui),
            AnimationType::PulseBackground => self.pulse_bg_settings.ui(ui),
        };

        // Prioritize NeedsReset over CanHotUpdate
        if change_type == UpdateBehaviour::NeedsReset {
            change_type
        } else if settings_change != UpdateBehaviour::None {
            settings_change
        } else {
            UpdateBehaviour::None
        }
    }
}
