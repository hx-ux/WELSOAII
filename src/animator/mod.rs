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
use nannou_egui::egui::{self, RichText};
use strum::IntoEnumIterator;
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
    fn color_ui(&mut self, ui: &mut egui::Ui);

    fn animation_type(&self) -> AnimationType;
    fn init(&mut self);
    fn set_dimension(&mut self, _window_rect: &Rect) {}
    fn hot_update(&mut self);
    fn reset(&mut self);
    fn draw(&self, draw: &Draw);
    fn update(&mut self, win_rect: &Rect, delta_time: f32, timecode: &TimeCode);
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
    pub timecode: TimeCode,
    pub mod_matrix: Modulator,
    pub active_animations: Vec<Box<dyn AnimatorSettings>>,
    pub serl_ani_index: Option<usize>,
}

impl Animator {
    pub fn new(win_rect: &Rect, grid: ReceiverGrid) -> Self {
        let mut active_animations: Vec<Box<dyn AnimatorSettings>> = Vec::new();
        active_animations.push(Box::new(BouncingBallSettings::new(win_rect)));

        let mut mod_matrix = Modulator::default();

        for effect in active_animations.iter_mut() {
            effect.connect_modulations(&mut mod_matrix);
        }

        Animator {
            timecode: TimeCode::new(),
            mod_matrix,
            grid,
            active_animations,
            serl_ani_index: Some(0),
        }
    }

    pub fn add_animator(&mut self, win_rect: &Rect, animation_type: AnimationType) {
        match animation_type {
            AnimationType::BouncingBalls => {
                self.active_animations
                    .push(Box::new(BouncingBallSettings::new(win_rect)));
            }
            AnimationType::PulseBackground => {
                self.active_animations
                    .push(Box::new(PulseBackgroundSettings::new(win_rect)));
            }
            AnimationType::ScanLine => {
                self.active_animations
                    .push(Box::new(ScanLineSettings::new(win_rect)));
            }
            AnimationType::WaveLines => {
                self.active_animations
                    .push(Box::new(WaveLinesSettings::new(win_rect)));
            }
        }
        self.serl_ani_index = Some(self.active_animations.iter().len() - 1);
    }

    pub fn reset(&mut self, win_rect: &Rect) {
        for ani in self.active_animations.iter_mut() {
            ani.set_dimension(win_rect);
            ani.init();
        }
    }

    pub fn behaviour_hot_update(&mut self) {
        for ani in self.active_animations.iter_mut() {
            ani.hot_update();
        }
    }

    pub fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        self.timecode.update(delta_time);
        self.apply_modulations();

        for animations in self.active_animations.iter_mut() {
            animations.update(win_rect, delta_time, &self.timecode);
        }

        for cell in self.grid.cells.iter_mut() {
            cell.reset();
        }

        for all in self.active_animations.iter() {
            for obj in all.get_objects() {
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
        }
        self.grid.update_led_buffer_and_send();
    }

    pub fn draw_animator(&self, draw: &Draw) {
        for ani in self.active_animations.iter() {
            ani.draw(draw);
        }
    }

    fn apply_modulations(&mut self) {
        for effect in &mut self.active_animations.iter_mut() {
            effect.reset_modulations();
        }
        if self.mod_matrix.routes.is_empty() || !self.mod_matrix.enabled {
            return;
        }
        let beat_pos = self.timecode.get_beats();
        for effect in &mut self.active_animations.iter_mut() {
            effect.update_modulations(beat_pos, &self.mod_matrix);
        }
    }

    pub fn draw_grid(&self, draw: &Draw) {
        self.grid.draw(draw);
    }

    pub fn animator_layer_ui(&mut self, ui: &mut egui::Ui, win_rect: &Rect) {
        ui.menu_button("My sub-menu", |ui| {
            for direction in AnimationType::iter() {
                if ui.button(format!("{}", direction)).clicked() {
                    self.add_animator(win_rect, direction);
                    ui.close_menu();
                }
            }
        });

        ui.vertical(|ui| {
            for index in 0..self.active_animations.iter().count() {
                let mut current = false;
                let mut entry_text = egui::RichText::new(format!(
                    "{}: {}",
                    index,
                    self.active_animations[index].animation_type()
                ));

                if let Some(index) = self.serl_ani_index {
                    if index == index {
                        current = true;
                    }
                }

                entry_text = entry_text.color(egui::Color32::WHITE);

                if current {
                    entry_text = entry_text.color(egui::Color32::GREEN);
                }

                ui.horizontal_top(|ui| {
                    if ui.button(entry_text).clicked() {
                        self.serl_ani_index = Some(index.clone());
                    }

                    if current == false {
                        if ui.button("DEL").clicked() {
                            self.active_animations.remove(0);
                        }
                    }
                });
            }
        });
    }

    pub fn control_ui(&mut self, ui: &mut egui::Ui) -> UpdateBehaviour {
        let mut change_type = UpdateBehaviour::None;
        ui.separator();
        ui.vertical(|ui| {
            if let Some(index) = self.serl_ani_index {
                if let Some(animator) = self.active_animations.get_mut(index) {
                    ui.heading(format!("Controls: {}", animator.animation_type()));
                    ui.separator();
                    change_type = animator.control_ui(ui, &mut self.mod_matrix);
                }
            } else {
                ui.label("Select an animation to edit.");
            }
        });

        change_type
    }
}
