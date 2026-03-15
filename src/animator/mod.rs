use crate::{
    animator::{
        animation_type::{AnimationType, UpdateBehaviour},
        animators::{bouncing_ball, pulse_background, scan_line, WaveLinesSettings},
    },
    connect_bouncing_balls_modulations, connect_pulse_bg_modulations,
    connect_scan_line_modulations, connect_wave_lines_modulations,
    receiver::ReceiverGrid,
    timecode::TimeCode,
};
use anyhow::Result;
use nannou::prelude::*;
use nannou_egui::egui::{self};
use strum::IntoEnumIterator;
pub mod animation_type;
mod animators;
use crate::modulator::Modulator;

use bouncing_ball::BouncingBallSettings;
use pulse_background::PulseBackgroundSettings;
use scan_line::ScanLineSettings;

macro_rules! with_current_settings {
    ($self:expr, $method:ident $(, $args:expr)*) => {
        match $self.animation_type {
            AnimationType::BouncingBalls => $self.bouncing_ball_settings.$method($($args),*),
            AnimationType::ScanLine => $self.scanline_settings.$method($($args),*),
            AnimationType::PulseBackground => $self.pulse_settings.$method($($args),*),
            AnimationType::WaveLines => $self.wave_lines_settings.$method($($args),*),
        }
    };
}

// An animated object, which every animator does emit
pub enum ObjectShape {
    Circle(Vec2, f32),
    Rect(Rect),
}

pub trait AnimatedObject {
    fn update(&mut self, win_rect: &Rect, delta_time: f32, clock: &TimeCode);
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
//DispatchFromDyn

pub trait AnimatorSettings {
    fn new(win_rect: &Rect) -> Self;
    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut Modulator) -> UpdateBehaviour;
    fn animation_type(&self) -> AnimationType;
    fn create(&self) -> Vec<Box<dyn AnimatedObject>>;
    fn set_dimension(&mut self, _window_rect: &Rect) {}
    // Custom Logic for Hot reloading the animator without resetting
    fn hot_update(&self, objects: &mut Vec<Box<dyn AnimatedObject>>);
    // Rest all parameter values to its definded standards
    fn reset(&mut self);
    fn force_update(&self) -> UpdateBehaviour {
        UpdateBehaviour::NeedsReset
    }

    fn connect_modulations(&mut self, mod_matrix: &mut Modulator);
    fn update_modulations(&mut self, beat_pos: f32, mod_matrix: &Modulator);
    fn reset_modulations(&mut self);
    fn save_preset(&mut self) -> Result<()>;
}

pub struct Animator {
    pub objects: Vec<Box<dyn AnimatedObject>>,
    pub grid: ReceiverGrid,
    pub animation_type: AnimationType,
    pub clock: TimeCode,
    pub mod_matrix: Modulator,
    bouncing_ball_settings: BouncingBallSettings,
    scanline_settings: ScanLineSettings,
    pulse_settings: PulseBackgroundSettings,
    wave_lines_settings: WaveLinesSettings,
}

impl Animator {
    pub fn new(win_rect: &Rect, grid: ReceiverGrid) -> Self {
        let bouncing_ball_settings = BouncingBallSettings::new(win_rect);
        let scanline_settings = ScanLineSettings::new(win_rect);
        let pulse_settings = PulseBackgroundSettings::new(win_rect);
        let wave_lines_settings = WaveLinesSettings::new(win_rect);
        let mut mod_matrix = Modulator::default();

        connect_bouncing_balls_modulations!(bouncing_ball_settings, &mut mod_matrix);
        connect_scan_line_modulations!(scanline_settings, mod_matrix);
        connect_pulse_bg_modulations!(pulse_settings, &mut mod_matrix);
        connect_wave_lines_modulations!(wave_lines_settings, mod_matrix);

        Animator {
            objects: Vec::new(),
            animation_type: AnimationType::BouncingBalls,
            clock: TimeCode::new(),
            mod_matrix,
            grid,
            bouncing_ball_settings,
            scanline_settings,
            pulse_settings,
            wave_lines_settings,
        }
    }

    pub fn switch_animation_tye(&mut self, dir: i8) {
        let mut new_index = self.animation_type as usize;
        let count = AnimationType::iter().count();
        if dir == 1 {
            new_index = (new_index + 1) % count;
        } else if dir == -1 {
            new_index = if new_index == 0 {
                count - 1
            } else {
                new_index - 1
            };
        }
        self.animation_type = AnimationType::from(new_index);
    }

    /// Clears and repopulates the animations objects based on current settings.
    pub fn reset(&mut self, win_rect: &Rect) {
        self.objects.clear();
        // keep settings in sync with the current window
        // if reset in bind to on_window_resize
        self.scanline_settings.set_dimension(win_rect);
        self.bouncing_ball_settings.set_dimension(win_rect);
        self.pulse_settings.set_dimension(win_rect);
        self.wave_lines_settings.set_dimension(win_rect);

        self.objects = with_current_settings!(self, create);
    }
    /// Apply hot updates to existing objects without recreating them
    pub fn behaviour_hot_update(&mut self) {
        with_current_settings!(self, hot_update, &mut self.objects);
    }

    pub fn save_preset(&mut self) {
        let _ = with_current_settings!(self, save_preset);
    }

    pub fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        // Update the master clock
        let synced_delta = self.clock.update(delta_time);

        // Live modulation mapped to current animator parameters (synth-style matrix)
        self.apply_modulations();

        // Update all objects with clock reference
        for obj in self.objects.iter_mut() {
            obj.update(win_rect, synced_delta, &self.clock);
        }

        // Remove dead objects ()
        self.objects.retain(|obj| !obj.is_dead());

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

        // Build the LED buffer and send once per update.
        self.grid.update_led_buffer_and_send();
    }

    // draws the created shapes from the animators
    pub fn draw_animator(&self, draw: &Draw) {
        for obj in &self.objects {
            obj.draw(draw);
        }
    }

    fn clear_mod_ghosts(&mut self) {
        self.bouncing_ball_settings.reset_modulations();
        self.scanline_settings.reset_modulations();
        self.pulse_settings.reset_modulations();
        self.wave_lines_settings.reset_modulations();
    }

    fn apply_modulations(&mut self) {
        self.clear_mod_ghosts();
        if self.mod_matrix.routes.is_empty() || !self.mod_matrix.enabled {
            return;
        }
        let beat_pos = self.clock.get_beats();

        self.bouncing_ball_settings
            .update_modulations(beat_pos, &self.mod_matrix);
        self.scanline_settings
            .update_modulations(beat_pos, &self.mod_matrix);
        self.pulse_settings
            .update_modulations(beat_pos, &self.mod_matrix);
        self.wave_lines_settings
            .update_modulations(beat_pos, &self.mod_matrix);
    }

    pub fn draw_grid(&self, draw: &Draw) {
        self.grid.draw(draw);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> UpdateBehaviour {
        let mut change_type = UpdateBehaviour::None;
        // --- General Settings ---
        egui::ComboBox::from_label("")
            .selected_text(format!("{:?}", self.animation_type))
            .show_ui(ui, |ui| {
                for option in AnimationType::iter() {
                    if ui
                        .selectable_value(&mut self.animation_type, option, format!("{}", option))
                        .clicked()
                    {
                        change_type = UpdateBehaviour::NeedsReset;
                    }
                }
            });

        // --- Show controls for each animator settings
        let settings_change = match self.animation_type {
            AnimationType::BouncingBalls => {
                self.bouncing_ball_settings.ui(ui, &mut self.mod_matrix)
            }
            AnimationType::ScanLine => self.scanline_settings.ui(ui, &mut self.mod_matrix),
            AnimationType::PulseBackground => self.pulse_settings.ui(ui, &mut self.mod_matrix),
            AnimationType::WaveLines => self.wave_lines_settings.ui(ui, &mut self.mod_matrix),
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
