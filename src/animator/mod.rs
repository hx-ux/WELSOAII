use crate::{
    animator::{
        animation_type::{AnimationType, UpdateBehaviour},
        animators::{WaveLinesSettings, bouncing_ball, pulse_background, scan_line},
    },
    modulator::{Modulator, wave_modulator::WaveModulator},
    parameters::ModulatedParam,
    receiver::ReceiverGrid,
    timecode::TimeCode,
};
use anyhow::Result;
use nannou::prelude::*;
use nannou_egui::egui::{self};
pub mod animation_type;
mod animators;

use bouncing_ball::BouncingBallSettings;
use pulse_background::PulseBackgroundSettings;
use scan_line::ScanLineSettings;

// An animated object, which every animator does emit
pub enum ObjectShape {
    Circle(Vec2, f32),
    Rect(Rect),
}

pub trait AnimatedObject {
    fn update(&mut self, win_rect: &Rect, delta_time: f32, clock: &TimeCode);
    fn draw(&self, draw: &Draw);
    fn is_dead(&self) -> bool {
        false
    }
    fn shape(&self) -> ObjectShape;
    fn color(&self) -> Rgba8;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub trait AnimatorSettings {
    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut Vec<Box<dyn Modulator>>) -> UpdateBehaviour;
    fn animation_type(&self) -> AnimationType;
    fn create(&self) -> Vec<Box<dyn AnimatedObject>>;
    fn set_dimension(&mut self, _window_rect: &Rect) {}
    fn hot_update(&self, objects: &mut Vec<Box<dyn AnimatedObject>>);
    fn reset(&mut self);
    fn force_update(&self) -> UpdateBehaviour {
        UpdateBehaviour::NeedsReset
    }

    /// Provides references to all modulated parameters of this effect.
    /// Default implementations for modulations use this to eliminate boilerplate.
    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
        Vec::new()
    }

    fn update_modulations(&mut self, beat_pos: f32, modulators: &mut Vec<Box<dyn Modulator>>) {
        for param in self.modulated_params_mut() {
            param.modulate(beat_pos, modulators);
        }
    }

    fn reset_modulations(&mut self) {
        for param in self.modulated_params_mut() {
            param.ghost_value = None;
        }
    }

    fn save_preset(&mut self) -> Result<()> {
        Ok(())
    }
}

pub struct Animator {
    pub objects: Vec<Box<dyn AnimatedObject>>,
    pub grid: ReceiverGrid,
    pub active_index: usize,
    pub clock: TimeCode,
    pub modulators: Vec<Box<dyn Modulator>>,
    pub effects: Vec<Box<dyn AnimatorSettings>>,
}

impl Animator {
    pub fn new(win_rect: &Rect, grid: ReceiverGrid) -> Self {
        let effects: Vec<Box<dyn AnimatorSettings>> = vec![
            Box::new(BouncingBallSettings::new(win_rect)),
            Box::new(PulseBackgroundSettings::new(win_rect)),
            Box::new(ScanLineSettings::new(win_rect)),
            Box::new(WaveLinesSettings::new(win_rect)),
        ];

        let modulators: Vec<Box<dyn Modulator>> = vec![
            Box::new(WaveModulator::new()),
            Box::new(WaveModulator::new()),
        ];

        Animator {
            objects: Vec::new(),
            active_index: 0,
            clock: TimeCode::new(),
            modulators,
            grid,
            effects,
        }
    }

    pub fn animation_type(&self) -> AnimationType {
        self.effects
            .get(self.active_index)
            .map(|e| e.animation_type())
            .unwrap_or(AnimationType::BouncingBalls)
    }

    pub fn switch_animation_tye(&mut self, dir: i8) {
        if self.effects.is_empty() {
            return;
        }
        let count = self.effects.len();
        if dir == 1 {
            self.active_index = (self.active_index + 1) % count;
        } else if dir == -1 {
            self.active_index = if self.active_index == 0 {
                count - 1
            } else {
                self.active_index - 1
            };
        }
    }

    /// Clears and repopulates the animation objects based on current settings.
    pub fn reset(&mut self, win_rect: &Rect) {
        self.objects.clear();
        if let Some(effect) = self.effects.get_mut(self.active_index) {
            effect.set_dimension(win_rect);
            self.objects = effect.create();
        }
    }

    /// Apply hot updates to existing objects without recreating them
    pub fn behaviour_hot_update(&mut self) {
        let objects = &mut self.objects;
        if let Some(effect) = self.effects.get(self.active_index) {
            effect.hot_update(objects);
        }
    }

    pub fn save_preset(&mut self) {
        if let Some(effect) = self.effects.get_mut(self.active_index) {
            let _ = effect.save_preset();
        }
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

        // Remove dead objects
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

    pub fn draw_animator(&self, draw: &Draw) {
        for obj in &self.objects {
            obj.draw(draw);
        }
    }

    fn clear_mod_ghosts(&mut self) {
        for effect in &mut self.effects {
            effect.reset_modulations();
        }
    }

    fn apply_modulations(&mut self) {
        self.clear_mod_ghosts();
        let beat_pos = self.clock.get_beats();
        for effect in &mut self.effects {
            effect.update_modulations(beat_pos, &mut self.modulators);
        }
    }

    pub fn draw_grid(&self, draw: &Draw) {
        self.grid.draw(draw);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> UpdateBehaviour {
        let mut change_type = UpdateBehaviour::None;
        let current_type = self.animation_type();

        egui::ComboBox::from_label("")
            .selected_text(format!("{}", current_type))
            .show_ui(ui, |ui| {
                for (idx, effect) in self.effects.iter().enumerate() {
                    let type_name = format!("{}", effect.animation_type());
                    if ui
                        .selectable_label(idx == self.active_index, type_name)
                        .clicked()
                        && self.active_index != idx
                    {
                        self.active_index = idx;
                        change_type = UpdateBehaviour::NeedsReset;
                    }
                }
            });

        let settings_change = if let Some(effect) = self.effects.get_mut(self.active_index) {
            effect.ui(ui, &mut self.modulators)
        } else {
            UpdateBehaviour::None
        };

        if change_type == UpdateBehaviour::NeedsReset {
            change_type
        } else if settings_change != UpdateBehaviour::None {
            settings_change
        } else {
            UpdateBehaviour::None
        }
    }
}
