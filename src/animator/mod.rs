extern crate nannou;
use anyhow::Result;
use strum::IntoEnumIterator;

use crate::{
    animator::{
        animation_type::{AnimationType, UpdateBehaviour},
        animators::{WaveLinesSettings, bouncing_ball, pulse_background, scan_line, wave_lines},
    },
    connect_bouncing_balls_modulations, connect_pulse_bg_modulations,
    connect_scan_line_modulations, connect_wave_lines_modulations,
    receiver::ReceiverGrid,
    timecode::TimeCode,
};
use nannou::prelude::*;
use nannou_egui::egui::{self};

pub mod animation_type;
mod animators;

use crate::modulator::ModMatrix;
use crate::modulator::ModTarget;

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
    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut ModMatrix) -> UpdateBehaviour;
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
    fn connect_modulations(&mut self, mod_matrix: &mut ModMatrix);
    fn save_preset(&mut self) -> Result<()>;
}

pub struct Animator {
    pub objects: Vec<Box<dyn AnimatedObject>>,
    pub grid: ReceiverGrid,
    pub animation_type: AnimationType,
    pub clock: TimeCode,
    pub mod_matrix: ModMatrix,
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
        let mut mod_matrix = ModMatrix::default();

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
        with_current_settings!(self, save_preset);
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
        self.bouncing_ball_settings.speed.ghost_value = None;
        self.bouncing_ball_settings.radius.ghost_value = None;

        self.scanline_settings.speed.ghost_value = None;
        self.scanline_settings.width.ghost_value = None;

        self.pulse_settings.speed.ghost_value = None;
        self.pulse_settings.limit.ghost_value = None;
        self.pulse_settings.rotation_speed.ghost_value = None;

        self.wave_lines_settings.amplitude.ghost_value = None;
        self.wave_lines_settings.frequency.ghost_value = None;
        self.wave_lines_settings.speed.ghost_value = None;
        self.wave_lines_settings.thickness.ghost_value = None;
        self.wave_lines_settings.phase_spread.ghost_value = None;
    }

    fn apply_modulations(&mut self) {
        self.clear_mod_ghosts();

        if self.mod_matrix.routes.is_empty() || !self.mod_matrix.enabled {
            return;
        }
        let beat_pos = self.clock.get_beats();

        match self.animation_type {
            AnimationType::BouncingBalls => {
                let speed = self.bouncing_ball_settings.speed.value
                    * self
                        .mod_matrix
                        .calc_modulation(beat_pos, ModTarget::BouncingSpeed);

                let radius = self.bouncing_ball_settings.radius.value
                    * self
                        .mod_matrix
                        .calc_modulation(beat_pos, ModTarget::BouncingRadius);

                for obj in self.objects.iter_mut() {
                    if let Some(ball) = obj
                        .as_any_mut()
                        .downcast_mut::<bouncing_ball::BouncingBall>()
                    {
                        ball.speed = speed;
                        ball.radius = radius.max(0.5);
                    }
                }

                if self.mod_matrix.has_target(ModTarget::BouncingSpeed) {
                    self.bouncing_ball_settings.speed.ghost_value = Some(speed);
                }
                if self.mod_matrix.has_target(ModTarget::BouncingRadius) {
                    self.bouncing_ball_settings.radius.ghost_value = Some(radius);
                }
            }
            AnimationType::ScanLine => {
                let speed = self.scanline_settings.speed.value
                    * self
                        .mod_matrix
                        .calc_modulation(beat_pos, ModTarget::ScanSpeed);
                let width = self.scanline_settings.width.value
                    * self
                        .mod_matrix
                        .calc_modulation(beat_pos, ModTarget::ScanWidth);

                for obj in self.objects.iter_mut() {
                    if let Some(scan_line) = obj.as_any_mut().downcast_mut::<scan_line::ScanLine>()
                    {
                        let current_direction = if scan_line.speed == 0.0 {
                            1.0
                        } else {
                            scan_line.speed.signum()
                        };
                        scan_line.speed = speed.abs() * current_direction;
                        scan_line.width = width.max(1.0);
                    }
                }

                if self.mod_matrix.has_target(ModTarget::ScanSpeed) {
                    self.scanline_settings.speed.ghost_value = Some(speed);
                }
                if self.mod_matrix.has_target(ModTarget::ScanWidth) {
                    self.scanline_settings.width.ghost_value = Some(width);
                }
            }
            AnimationType::PulseBackground => {
                let speed = self.pulse_settings.speed.value
                    * self
                        .mod_matrix
                        .calc_modulation(beat_pos, ModTarget::PulseSpeed);
                let limit = self.pulse_settings.limit.value
                    * self
                        .mod_matrix
                        .calc_modulation(beat_pos, ModTarget::PulseLimit);
                let ring_count = (self.pulse_settings.ring_count.value as f32
                    * self
                        .mod_matrix
                        .calc_modulation(beat_pos, ModTarget::PulseRingCount))
                .round()
                .clamp(1.0, 32.0) as usize;
                let rotation_speed = self.pulse_settings.rotation_speed.value
                    * self
                        .mod_matrix
                        .calc_modulation(beat_pos, ModTarget::PulseRotation);

                for obj in self.objects.iter_mut() {
                    if let Some(pulse) = obj
                        .as_any_mut()
                        .downcast_mut::<pulse_background::PulseBackground>()
                    {
                        pulse.speed = speed.max(0.0);
                        pulse.limit = limit.clamp(0.01, 1.0);
                        pulse.ring_count = ring_count;
                        pulse.rotation_speed = rotation_speed;
                    }
                }

                if self.mod_matrix.has_target(ModTarget::PulseSpeed) {
                    self.pulse_settings.speed.ghost_value = Some(speed);
                }
                if self.mod_matrix.has_target(ModTarget::PulseLimit) {
                    self.pulse_settings.limit.ghost_value = Some(limit);
                }

                if self.mod_matrix.has_target(ModTarget::PulseRotation) {
                    self.pulse_settings.rotation_speed.ghost_value = Some(rotation_speed);
                }
            }
            AnimationType::WaveLines => {
                let amplitude = self.wave_lines_settings.amplitude.value
                    * self
                        .mod_matrix
                        .calc_modulation(beat_pos, ModTarget::WaveAmplitude);
                let frequency = self.wave_lines_settings.frequency.value
                    * self
                        .mod_matrix
                        .calc_modulation(beat_pos, ModTarget::WaveFrequency);
                let speed = self.wave_lines_settings.speed.value
                    * self
                        .mod_matrix
                        .calc_modulation(beat_pos, ModTarget::WaveSpeed);
                let thickness = self.wave_lines_settings.thickness.value
                    * self
                        .mod_matrix
                        .calc_modulation(beat_pos, ModTarget::WaveThickness);
                let phase_spread = self.wave_lines_settings.phase_spread.value
                    * self
                        .mod_matrix
                        .calc_modulation(beat_pos, ModTarget::WavePhaseSpread);

                for obj in self.objects.iter_mut() {
                    if let Some(line) = obj.as_any_mut().downcast_mut::<wave_lines::WaveLine>() {
                        line.amplitude_base = amplitude.max(0.0);
                        line.frequency = frequency.max(0.0001);
                        line.speed = speed.max(0.0);
                        line.thickness = thickness.max(0.5);
                        line.phase_spread = phase_spread;
                    }
                }

                if self.mod_matrix.has_target(ModTarget::WaveAmplitude) {
                    self.wave_lines_settings.amplitude.ghost_value = Some(amplitude);
                }
                if self.mod_matrix.has_target(ModTarget::WaveFrequency) {
                    self.wave_lines_settings.frequency.ghost_value = Some(frequency);
                }
                if self.mod_matrix.has_target(ModTarget::WaveSpeed) {
                    self.wave_lines_settings.speed.ghost_value = Some(speed);
                }
                if self.mod_matrix.has_target(ModTarget::WaveThickness) {
                    self.wave_lines_settings.thickness.ghost_value = Some(thickness);
                }
                if self.mod_matrix.has_target(ModTarget::WavePhaseSpread) {
                    self.wave_lines_settings.phase_spread.ghost_value = Some(phase_spread);
                }
            }
        }
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
