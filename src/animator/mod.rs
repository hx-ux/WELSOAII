use crate::{
    animator::{
        animation_type::{AnimationType, UpdateBehaviour},
        animators::{WaveLinesSettings, bouncing_ball, pulse_background, scan_line},
    },
    parameters::ModulatedParam,
    receiver::ReceiverGrid,
    timecode::TimeCode,
};
use anyhow::Result;
use nannou::prelude::*;
use nannou_egui::egui::{self};
pub mod animation_type;
mod animators;
use crate::modulator::Modulator;

use bouncing_ball::BouncingBallSettings;
use pulse_background::PulseBackgroundSettings;
use scan_line::ScanLineSettings;

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
}

pub trait AnimatorSettings {
    fn control_ui(&mut self, ui: &mut egui::Ui, mods: &mut Modulator) -> UpdateBehaviour;
    fn color_ui(&mut self, ui: &mut egui::Ui) -> UpdateBehaviour;
    fn animation_type(&self) -> AnimationType;
    fn init(&mut self);
    fn set_dimension(&mut self, _window_rect: &Rect) {}
    fn hot_update(&mut self);
    fn reset(&mut self);

    // Returns references to the internal concrete objects
    fn get_objects(&self) -> Vec<&dyn AnimatedObject>;
    fn get_objects_mut(&mut self) -> Vec<&mut dyn AnimatedObject>;

    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
        Vec::new()
    }

    fn connect_modulations(&mut self, mod_matrix: &mut Modulator) {
        for param in self.modulated_params_mut() {
            param.connect_modulation(mod_matrix);
        }
    }

    fn update_modulations(&mut self, beat_pos: f32, mod_matrix: &Modulator) {
        for param in self.modulated_params_mut() {
            param.modulate(beat_pos, mod_matrix);
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
    pub grid: ReceiverGrid,
    pub active_index: usize,
    pub clock: TimeCode,
    pub mod_matrix: Modulator,
    pub effects: Vec<Box<dyn AnimatorSettings>>,
}

impl Animator {
    pub fn new(win_rect: &Rect, grid: ReceiverGrid) -> Self {
        let mut effects: Vec<Box<dyn AnimatorSettings>> = vec![
            Box::new(BouncingBallSettings::new(win_rect)),
            Box::new(PulseBackgroundSettings::new(win_rect)),
            Box::new(ScanLineSettings::new(win_rect)),
            Box::new(WaveLinesSettings::new(win_rect)),
        ];

        let mut mod_matrix = Modulator::default();
        for effect in effects.iter_mut() {
            effect.connect_modulations(&mut mod_matrix);
        }

        Animator {
            active_index: 0,
            clock: TimeCode::new(),
            mod_matrix,
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

    pub fn reset(&mut self, win_rect: &Rect) {
        if let Some(effect) = self.effects.get_mut(self.active_index) {
            effect.set_dimension(win_rect);
            effect.init();
        }
    }

    pub fn behaviour_hot_update(&mut self) {
        if let Some(effect) = self.effects.get_mut(self.active_index) {
            effect.hot_update();
        }
    }

    pub fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        let synced_delta = self.clock.update(delta_time);
        self.apply_modulations();

        let effect = match self.effects.get_mut(self.active_index) {
            Some(e) => e,
            None => return,
        };

        // Update objects
        for obj in effect.get_objects_mut() {
            obj.update(win_rect, synced_delta, &self.clock);
        }

        // Reset grid
        for cell in self.grid.cells.iter_mut() {
            cell.reset();
        }

        // Spatial collision
        for obj in effect.get_objects() {
            let obj_shape = obj.shape();
            let obj_color = obj.color();

            let (min_col, max_col, min_row, max_row) = match &obj_shape {
                ObjectShape::Circle(pos, radius) => self.grid.get_cell_range(
                    pos.x - radius,
                    pos.x + radius,
                    pos.y - radius,
                    pos.y + radius,
                ),
                ObjectShape::Rect(r) => {
                    self.grid
                        .get_cell_range(r.left(), r.right(), r.bottom(), r.top())
                }
            };

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

                    let intersects = match &obj_shape {
                        ObjectShape::Circle(pos, radius) => {
                            let closest_x = pos.x.clamp(cell.rect.left(), cell.rect.right());
                            let closest_y = pos.y.clamp(cell.rect.bottom(), cell.rect.top());
                            ((pos.x - closest_x).powi(2) + (pos.y - closest_y).powi(2))
                                < (radius * radius)
                        }
                        ObjectShape::Rect(r) => {
                            r.left() < cell.rect.right()
                                && r.right() > cell.rect.left()
                                && r.top() > cell.rect.bottom()
                                && r.bottom() < cell.rect.top()
                        }
                    };

                    if intersects {
                        cell.is_active = true;
                        cell.display_color = obj_color;
                    }
                }
            }
        }

        self.grid.update_led_buffer_and_send();
    }

    pub fn draw_animator(&self, draw: &Draw) {
        if let Some(effect) = self.effects.get(self.active_index) {
            for obj in effect.get_objects() {
                obj.draw(draw);
            }
        }
    }

    fn apply_modulations(&mut self) {
        for effect in &mut self.effects {
            effect.reset_modulations();
        }
        if self.mod_matrix.routes.is_empty() || !self.mod_matrix.enabled {
            return;
        }
        let beat_pos = self.clock.get_beats();
        for effect in &mut self.effects {
            effect.update_modulations(beat_pos, &self.mod_matrix);
        }
    }

    pub fn draw_grid(&self, draw: &Draw) {
        self.grid.draw(draw);
    }

    pub fn control_ui(&mut self, ui: &mut egui::Ui) -> UpdateBehaviour {
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
            effect.control_ui(ui, &mut self.mod_matrix)
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
