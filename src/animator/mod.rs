use crate::{
    animator::{
        animation_type::AnimationType,
        animators::{WaveLinesSettings, bouncing_ball, pulse_background, scan_line},
    },
    modulator::wave_modulator::WaveModulator,
    parameters::ModulatedParam,
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

pub enum ObjectShape {
    Circle(Vec2, f32),
    Rect(Rect),
}

pub trait AnimatedObject {
    fn update(&mut self, win_rect: &Rect, clock: &TimeCode);
    fn draw(&self, draw: &Draw);
    fn is_dead(&self) -> bool {
        false
    }
    fn shape(&self) -> ObjectShape;
    fn color(&self) -> Rgba8;
}

pub trait AnimatorSettings {
    fn control_ui(&mut self, ui: &mut egui::Ui, mods: &mut Vec<Box<dyn Modulator>>);
    fn color_ui(&mut self, ui: &mut egui::Ui);

    fn animation_type(&self) -> AnimationType;
    fn init(&mut self);
    fn set_dimension(&mut self, _window_rect: &Rect) {}
    fn hot_update(&mut self);
    fn draw(&self, draw: &Draw);
    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode);
    // Returns references to the internal concrete objects
    fn get_objects(&self) -> Vec<&dyn AnimatedObject>;
    fn get_objects_mut(&mut self) -> Vec<&mut dyn AnimatedObject>;

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
    pub grid: ReceiverGrid,
    pub timecode: TimeCode,
    pub modulators: Vec<Box<dyn Modulator>>,
    pub active_animations: Vec<Box<dyn AnimatorSettings>>,
    pub current_animation_index: Option<usize>,
}

impl Animator {
    pub fn new(win_rect: &Rect, grid: ReceiverGrid) -> Self {
        let mut active_animations: Vec<Box<dyn AnimatorSettings>> = Vec::new();
        active_animations.push(Box::new(BouncingBallSettings::new(win_rect)));

        let modulators: Vec<Box<dyn Modulator>> = vec![
            Box::new(WaveModulator::new(0)),
            Box::new(WaveModulator::new(1)),
        ];

        Animator {
            timecode: TimeCode::new(),
            modulators,
            grid,
            active_animations,
            current_animation_index: Some(0),
        }
    }

    pub fn add_animator(&mut self, win_rect: &Rect, animation_type: AnimationType) {
        match animation_type {
            AnimationType::BouncingBalls => {
                let mut ani = Box::new(BouncingBallSettings::new(win_rect));
                ani.init();
                self.active_animations.push(ani);
            }
            AnimationType::PulseBackground => {
                let mut ani = Box::new(PulseBackgroundSettings::new(win_rect));
                ani.init();
                self.active_animations.push(ani);
            }
            AnimationType::ScanLine => {
                let mut ani = Box::new(ScanLineSettings::new(win_rect));
                ani.init();
                self.active_animations.push(ani);
            }
            AnimationType::WaveLines => {
                let mut ani = Box::new(WaveLinesSettings::new(win_rect));
                ani.init();
                self.active_animations
                    .push(Box::new(WaveLinesSettings::new(win_rect)));
            }
        }
        self.current_animation_index = Some(self.active_animations.iter().len() - 1);
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
            animations.update(win_rect, &self.timecode);
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

    fn clear_mod_ghosts(&mut self) {
        for effect in &mut self.active_animations {
            effect.reset_modulations();
        }
    }

    fn apply_modulations(&mut self) {
        self.clear_mod_ghosts();
        let beat_pos = self.timecode.get_beats();
        for effect in &mut self.active_animations {
            effect.update_modulations(beat_pos, &mut self.modulators);
        }
    }

    pub fn draw_grid(&self, draw: &Draw) {
        self.grid.draw(draw);
    }

    pub fn modulators_ui(&mut self, ui: &mut egui::Ui, win_rect: &Rect) {
        for g in self.modulators.iter_mut() {
            g.ui(ui, 0.0);
        }
    }

    pub fn animator_layer_ui(&mut self, ui: &mut egui::Ui, win_rect: &Rect) {
        let mut index_to_remove = None;

        ui.vertical(|ui| {
            ui.label(egui::RichText::new("LAYERS"));

            ui.add_space(2.0);
            ui.menu_button(egui::RichText::new("+"), |ui| {
                for direction in AnimationType::iter() {
                    if ui
                        .button(egui::RichText::new(format!("{}", direction).to_uppercase()))
                        .clicked()
                    {
                        self.add_animator(win_rect, direction);
                        ui.close_menu();
                    }
                }
            });

            for index in 0..self.active_animations.len() {
                let is_selected = self.current_animation_index == Some(index);
                let anim_name = format!("{}", self.active_animations[index].animation_type());

                ui.horizontal(|ui| {
                    let indicator_color = if is_selected {
                        egui::Color32::from_rgb(255, 102, 0)
                    } else {
                        egui::Color32::from_gray(55)
                    };

                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(3.0, 14.0), egui::Sense::hover());
                    ui.painter()
                        .rect_filled(rect, egui::Rounding::ZERO, indicator_color);

                    let label =
                        egui::RichText::new(anim_name.to_uppercase()).color(if is_selected {
                            egui::Color32::from_rgb(255, 102, 0)
                        } else {
                            egui::Color32::from_gray(150)
                        });

                    if ui.button(label).clicked() {
                        self.current_animation_index = Some(index);
                    }

                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new("×"))
                                .min_size(egui::vec2(12.0, 12.0)),
                        )
                        .clicked()
                    {
                        index_to_remove = Some(index);
                    }
                });
            }

            if let Some(remove_idx) = index_to_remove {
                self.active_animations.remove(remove_idx);

                if let Some(current_idx) = self.current_animation_index {
                    if current_idx == remove_idx {
                        self.current_animation_index = if self.active_animations.is_empty() {
                            None
                        } else {
                            Some(remove_idx.saturating_sub(1))
                        };
                    } else if current_idx > remove_idx {
                        self.current_animation_index = Some(current_idx - 1);
                    }
                }
            }
        });
    }

    pub fn control_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            if let Some(index) = self.current_animation_index {
                if let Some(animator) = self.active_animations.get_mut(index) {
                    ui.label(egui::RichText::new(
                        format!("{}", animator.animation_type()).to_uppercase(),
                    ));
                    ui.add(egui::Separator::default().spacing(4.0));
                    animator.control_ui(ui, &mut self.modulators);
                    animator.color_ui(ui);
                }
            } else {
                ui.label(egui::RichText::new("SELECT A LAYER"));
            }
        });
    }
}
