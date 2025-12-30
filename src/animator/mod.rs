extern crate nannou;
use crate::{animator::animation_type::AnimationType, reciver::ReciverGrid};
use nannou::prelude::*;
use nannou_egui::egui;

pub mod bouncing_ball;
pub mod gravity_fountain;
pub mod pulse_background;
pub mod scan_line; // TODO
pub mod animation_type;
use bouncing_ball::BouncingBall;
use gravity_fountain::{GravityFountainSettings, GravityParticle};
use pulse_background::{PulseBackground, PulseBackgroundSettings};
use scan_line::{ScanLine, ScanLineSettings};
use crate::Utils::*;


pub enum ObjectShape {
    Circle(Vec2, f32), // (position, radius)
    Rect(Rect),
}

pub trait AnimatedObject {
    fn update(&mut self, win_rect: &Rect, delta_time: f32);
    fn draw(&self, draw: &Draw);
    fn is_dead(&self) -> bool {
        false
    }

    fn shape(&self) -> ObjectShape;
    fn color(&self) -> Rgba;
}

pub struct AnimatorSettings {
    pub count: u32,
    pub multicolor: bool,
    pub curr_animation_type: AnimationType,
}

// These are the global settings, which apply to every animator unless overwritten
pub struct Animator {
    pub objects: Vec<Box<dyn AnimatedObject>>,
    pub settings: AnimatorSettings,
    pub grid: ReciverGrid,
    pub color: Rgba,
    egui_color: egui::Color32,

    // Settings for each animation type
    gravity_settings: GravityFountainSettings,
    scanline_settings: ScanLineSettings,
    pulse_settings: PulseBackgroundSettings,
}

impl Animator {
    pub fn new(win_rect: &Rect) -> Self {
        let scanline_settings = ScanLineSettings::factory(win_rect);
        let gravity_settings = GravityFountainSettings::factory(win_rect);
        let pulse_settings = PulseBackgroundSettings::default();

        Animator {
            objects: Vec::new(),
            settings: AnimatorSettings {
                count: 10,
                multicolor: true,
                curr_animation_type: AnimationType::BouncingBalls,
            },
            grid: ReciverGrid::new(win_rect.pad(20.0), 10, 10,true),
            color: Rgba::new(1.0, 0.0, 0.0, 1.0),
            egui_color: egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255),
            gravity_settings,
            scanline_settings,
            pulse_settings,
        }
    }

    pub fn link_grid(&mut self, grid: ReciverGrid) {
        self.grid = grid;
    }

    /// Clears and repopulates objects based on current settings.
    pub fn reset(&mut self, win_rect: &Rect) {
        // Update settings that depend on window size
        // self.scanline_settings = ScanLineSettings::factory(win_rect);
        // self.gravity_settings = GravityFountainSettings::factory(win_rect);


        self.objects.clear();

        for _ in 0..self.settings.count {
            
            let color = if self.settings.multicolor {
                Rgba::random()
            } else {
                self.color
            };

            let new_obj: Box<dyn AnimatedObject> = match self.settings.curr_animation_type {
                AnimationType::BouncingBalls => Box::new(BouncingBall::new(win_rect, color)),

                AnimationType::GravityFountain => {
                    Box::new(GravityParticle::factory(&self.gravity_settings, color))
                }
                AnimationType::ScanLine => {
                    Box::new(ScanLine::new(&self.scanline_settings, win_rect, color))
                }
                AnimationType::PulseBackground => Box::new(PulseBackground::factory(
                    &self.pulse_settings,
                    win_rect,
                    color,
                )),
            };
            self.objects.push(new_obj);
        }
    }

    pub fn update(&mut self, win_rect: &Rect, delta_time: f32) {

        // Update all objects
        for obj in self.objects.iter_mut() {
            obj.update(win_rect, delta_time);
        }

        // Remove dead objects
        self.objects.retain(|obj| !obj.is_dead());

        // TODO in Animator class
        // Respawn particles for GravityFountain to maintain count
        if self.settings.curr_animation_type == AnimationType::GravityFountain {
            let dead_count = self.settings.count as i32 - self.objects.len() as i32;
            for _ in 0..dead_count {
                let color = if self.settings.multicolor {
                    Rgba::random()
                } else {
                    self.color
                };
                self.objects.push(Box::new(GravityParticle::factory(
                    &self.gravity_settings,
                    color,
                )));
            }
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

    pub fn draw_animator(&self, draw: &Draw) {
        for obj in &self.objects {
            obj.draw(draw);
        }
    }
    pub fn draw_grid(&self, draw: &Draw) {
        self.grid.draw(draw);
    }

    /// Draws the main UI panel and animation-specific settings.
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.separator();

        // --- General Settings ---
        ui.label("Animation Type");
        ui.horizontal(|ui| {
            for options in AnimationType::iterator() {
                if ui
                    .radio_value(
                        &mut self.settings.curr_animation_type,
                        *options,
                        options.as_str(),
                    )
                    .changed()
                {
                    changed = true;
                };
            }
        });

        ui.add_space(5.0);

        if ui
            .add(egui::Slider::new(&mut self.settings.count, 1..=300).text("Object Count"))
            .changed()
        {
            changed = true;
        }

        ui.horizontal(|ui| {
            if ui
                .checkbox(&mut self.settings.multicolor, "Multicolor")
                .changed()
            {
                changed = true;
            }

            ui.add_enabled_ui(!self.settings.multicolor, |ui| {
                if ui.color_edit_button_srgba(&mut self.egui_color).changed() {
                    self.color = Rgba::from_egui(self.egui_color);
                    changed = true;
                }
            });
        });

        ui.separator();

        // --- Animation-Specific Settings ---
        match self.settings.curr_animation_type {
            AnimationType::BouncingBalls => {
                ui.label("No settings for Bouncing Balls.");
            }

            AnimationType::GravityFountain => {
                ui.heading("Fountain Settings");
                egui::Grid::new("gravity_settings")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Origin Y:");
                        changed |= ui
                            .add(
                                egui::DragValue::new(&mut self.gravity_settings.origin.y)
                                    .speed(1.0),
                            )
                            .changed();
                        ui.end_row();

                        ui.label("Speed (min/max):");
                        ui.horizontal(|ui| {
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut self.gravity_settings.min_speed)
                                        .speed(10.0),
                                )
                                .changed();
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut self.gravity_settings.max_speed)
                                        .speed(10.0),
                                )
                                .changed();
                        });
                        ui.end_row();

                        ui.label("Radius (min/max):");
                        ui.horizontal(|ui| {
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut self.gravity_settings.min_radius)
                                        .speed(0.5),
                                )
                                .changed();
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut self.gravity_settings.max_radius)
                                        .speed(0.5),
                                )
                                .changed();
                        });
                        ui.end_row();

                        ui.label("Life (min/max):");
                        ui.horizontal(|ui| {
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut self.gravity_settings.min_life)
                                        .speed(0.1),
                                )
                                .changed();
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut self.gravity_settings.max_life)
                                        .speed(0.1),
                                )
                                .changed();
                        });
                        ui.end_row();
                    });
            }
            AnimationType::ScanLine => {
                // changed = self.scanline_settings.ui(ui);
                ui.heading("ScanLine Settings");
                egui::Grid::new("scanline_grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Speed:");
                        changed |= ui
                            .add(
                                egui::DragValue::new(&mut self.scanline_settings.speed)
                                    .speed(5.0)
                                    .clamp_range(-1000.0..=1000.0),
                            )
                            .changed();
                        ui.end_row();

                        ui.label("Mode:");
                        let mode_str = if self.scanline_settings.mode == 0 {
                            "Ping-Pong"
                        } else {
                            "Wrap"
                        };
                        egui::ComboBox::from_label("")
                            .selected_text(mode_str)
                            .show_ui(ui, |ui| {
                                changed |= ui
                                    .selectable_value(
                                        &mut self.scanline_settings.mode,
                                        0,
                                        "Ping-Pong",
                                    )
                                    .changed();
                                changed |= ui
                                    .selectable_value(&mut self.scanline_settings.mode, 1, "Wrap")
                                    .changed();
                            });
                        ui.end_row();
                    });
            }
            AnimationType::PulseBackground => {
                ui.label("Pulse effect not implemented.");
            }
        }

        changed
    }
}
