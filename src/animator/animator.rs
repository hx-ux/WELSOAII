use crate::{
    animator::{
        AnimatorSettings,
        animation_type::{AnimationType, LayerBlendMode, LayerPolygonMode},
        animators::{
            bouncing_ball::BouncingBallSettings, pulse_background::PulseBackgroundSettings,
            quantum_tunnel::QuantumTunnelSettings, scan_line::ScanLineSettings,
            wave_lines::WaveLinesSettings,
        },
    },
    modulator::{Modulator, wave_modulator::WaveModulator},
    receiver::ReceiverGrid,
    timecode::TimeCode,
    ui::icons::{ICON_CARET_DOWN, ICON_CARET_UP, ICON_CLEAR, ICON_EYE_ACTIVE, ICON_EYE_DISABLED},
    utils::{AppLogger, Severity},
};
use nannou::prelude::egui::Color32;
use nannou::prelude::*;
use strum::IntoEnumIterator;

pub struct Animator {
    pub grid: ReceiverGrid,
    pub timecode: TimeCode,
    pub modulators: Vec<Box<dyn Modulator>>,
    pub active_animations: Vec<Box<dyn AnimatorSettings>>,
    pub current_animation_index: Option<usize>,
    pub force_clear_frames: u8,
    pub logger: AppLogger,
}

impl Animator {
    pub fn new(win_rect: &Rect, grid: ReceiverGrid, logger: AppLogger) -> Self {
        let active_animations: Vec<Box<dyn AnimatorSettings>> =
            vec![Box::new(BouncingBallSettings::new(win_rect))];

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
            force_clear_frames: 2,
            logger,
        }
    }

    pub fn add_animator(&mut self, win_rect: &Rect, animation_type: AnimationType) {
        let mut animator: Box<dyn AnimatorSettings> = match animation_type {
            AnimationType::BouncingBalls => Box::new(BouncingBallSettings::new(win_rect)),
            AnimationType::PulseBackground => Box::new(PulseBackgroundSettings::new(win_rect)),
            AnimationType::ScanLine => Box::new(ScanLineSettings::new(win_rect)),
            AnimationType::WaveLines => Box::new(WaveLinesSettings::new(win_rect)),
            AnimationType::QuantumTunnel => Box::new(QuantumTunnelSettings::new(win_rect)),
        };

        animator.init();
        self.active_animations.push(animator);
        self.current_animation_index = Some(self.active_animations.len() - 1);
        self.logger.log(
            format!("Added animation layer: {}", animation_type),
            Severity::Info,
        );
    }

    pub fn init_all_layers(&mut self, win_rect: &Rect) {
        for ani in self.active_animations.iter_mut() {
            ani.set_dimension(win_rect);
            ani.init();
        }
    }

    pub fn request_clear(&mut self) {
        self.force_clear_frames = self.force_clear_frames.max(2);
        for ani in self.active_animations.iter_mut() {
            ani.clear_trails();
        }
        for cell in self.grid.cells.iter_mut() {
            cell.reset();
        }
    }

    pub fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        self.timecode.update(delta_time);
        self.apply_modulations();

        if self.grid.take_needs_clear() {
            self.force_clear_frames = self.force_clear_frames.max(2);
        }

        let force_clear = if self.force_clear_frames > 0 {
            self.force_clear_frames -= 1;
            true
        } else {
            false
        };

        for cell in self.grid.cells.iter_mut() {
            cell.reset();
        }

        let dt_factor = (delta_time * 60.0).clamp(0.2, 4.0);

        for layer in self.active_animations.iter_mut() {
            let trail_val = layer.get_trail_persistence().clamp(0.0, 0.97);
            let retention = if force_clear || trail_val <= 0.001 {
                0.0
            } else {
                trail_val.powf(dt_factor)
            };

            layer.step_trails(retention, &self.grid.cells);
            layer.update(win_rect, &self.timecode);

            if !layer.get_visibility() {
                continue;
            }

            let blend_mode = layer.get_blend_mode();
            let wireframe = layer.get_polygon_mode().is_wireframe();

            let mut hit_cells: Vec<(usize, Srgba)> = Vec::new();
            for obj in layer.get_objects() {
                let obj_shape = obj.shape();
                let obj_color = obj.color();

                let (left, right, bottom, top) = obj_shape.bounds();
                let (min_col, max_col, min_row, max_row) =
                    self.grid.get_cell_range(left, right, bottom, top);

                for row in min_row..=max_row {
                    for col in min_col..=max_col {
                        let idx = self.grid.get_cell_index(row, col);
                        if idx >= self.grid.cells.len() {
                            continue;
                        }

                        if obj_shape.intersects_cell(&self.grid.cells[idx].rect, wireframe) {
                            hit_cells.push((idx, obj_color));
                        }
                    }
                }
            }

            {
                let layer_cells = &mut layer.trails_mut().grid_cells;
                for (idx, obj_color) in hit_cells {
                    if let Some(cell) = layer_cells.get_mut(idx) {
                        cell.blend(obj_color, blend_mode);
                    }
                }
            }

            for (idx, layer_cell) in layer.trails().grid_cells.iter().enumerate() {
                if layer_cell.is_active
                    && let Some(dst_cell) = self.grid.cells.get_mut(idx)
                {
                    dst_cell.blend_premultiplied(layer_cell.display_color, blend_mode);
                }
            }
        }
        self.grid.update_led_buffer_and_send();
    }

    pub fn draw_animator(&self, draw: &Draw, _win_rect: &Rect) {
        draw.background().color(BLACK);

        for (idx, ani) in self.active_animations.iter().enumerate() {
            if !ani.get_visibility() {
                continue;
            }
            let layer_z = (idx as f32 + 1.0) * 0.05;
            let layer_draw = draw
                .z(layer_z)
                .color_blend(ani.get_blend_mode().to_wgpu_blend())
                .polygon_mode(ani.get_polygon_mode().to_wgpu_polygon_mode());
            ani.draw_trails(&layer_draw);
            ani.draw(&layer_draw);
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
            effect.update_modulations(beat_pos, &self.modulators);
        }
    }

    pub fn draw_grid(&self, draw: &Draw) {
        self.grid.draw(draw);
    }

    pub fn modulators_ui(&mut self, ui: &mut egui::Ui) {
        let current_beat = self.timecode.get_beats();
        for modulator in self.modulators.iter_mut() {
            modulator.ui(ui, current_beat);
        }
    }

    pub fn animator_layer_ui(&mut self, ui: &mut egui::Ui, win_rect: &Rect) {
        let mut index_to_remove = None;
        let mut swap_indices: Option<(usize, usize)> = None;

        ui.horizontal_top(|ui| {
            ui.label(
                egui::RichText::new("LAYERS")
                    .size(15.0)
                    .color(Color32::WHITE),
            );
            ui.add_space(5.0);
            ui.menu_button(egui::RichText::new("+"), |ui| {
                for anim_type in AnimationType::iter() {
                    if ui
                        .button(egui::RichText::new(anim_type.to_string().to_uppercase()))
                        .clicked()
                    {
                        self.add_animator(win_rect, anim_type);
                        ui.close();
                    }
                }
            });
            if ui
                .button(format!("{} Clear", ICON_CLEAR))
                .on_hover_text("Clear persistent motion trails on screen and WLED grid")
                .clicked()
            {
                self.request_clear();
            }
        });

        ui.separator();

        let total_layers = self.active_animations.len();
        ui.vertical(|ui| {
            for index in 0..total_layers {
                let is_selected = self.current_animation_index == Some(index);
                let anim_name = self.active_animations[index].animation_type().to_string();
                let is_visible = self.active_animations[index].get_visibility();

                ui.horizontal(|ui| {
                    let indicator_color = if is_selected {
                        egui::Color32::from_rgb(255, 102, 0)
                    } else {
                        egui::Color32::from_gray(55)
                    };

                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(3.0, 14.0), egui::Sense::hover());
                    ui.painter()
                        .rect_filled(rect, egui::CornerRadius::ZERO, indicator_color);

                    if index > 0
                        && ui
                            .add(
                                egui::Button::new(egui::RichText::new(ICON_CARET_UP))
                                    .min_size(egui::vec2(12.0, 12.0)),
                            )
                            .clicked()
                    {
                        swap_indices = Some((index, index - 1));
                    }

                    if index + 1 < total_layers
                        && ui
                            .add(
                                egui::Button::new(egui::RichText::new(ICON_CARET_DOWN))
                                    .min_size(egui::vec2(12.0, 12.0)),
                            )
                            .on_hover_text("Move layer later in blend order")
                            .clicked()
                    {
                        swap_indices = Some((index, index + 1));
                    }

                    let label = egui::RichText::new(format!("{}", anim_name.to_uppercase(),))
                        .color(if is_selected {
                            egui::Color32::from_rgb(255, 102, 0)
                        } else {
                            egui::Color32::from_gray(150)
                        });

                    if ui.button(label).clicked() {
                        self.current_animation_index = Some(index);
                    }

                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new(if is_visible {
                                ICON_EYE_ACTIVE
                            } else {
                                ICON_EYE_DISABLED
                            }))
                            .min_size(egui::vec2(12.0, 12.0)),
                        )
                        .clicked()
                    {
                        self.active_animations[index].set_visibility(!is_visible);
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

            if let Some((a, b)) = swap_indices {
                self.active_animations.swap(a, b);
                if self.current_animation_index == Some(a) {
                    self.current_animation_index = Some(b);
                } else if self.current_animation_index == Some(b) {
                    self.current_animation_index = Some(a);
                }
            }

            if let Some(remove_idx) = index_to_remove {
                let removed = self.active_animations.remove(remove_idx);
                self.logger.log(
                    format!("Removed animation layer: {}", removed.animation_type()),
                    Severity::Info,
                );

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

    pub fn animator_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            if let Some(index) = self.current_animation_index {
                if let Some(animator) = self.active_animations.get_mut(index) {
                    ui.add(egui::Separator::default().spacing(4.0));
                    ui.label(
                        egui::RichText::new("BEVY RENDERING")
                            .size(12.5)
                            .color(Color32::WHITE),
                    );

                    let mut current_blend = animator.get_blend_mode();
                    ui.horizontal(|ui| {
                        ui.label("Blend:");
                        egui::ComboBox::from_id_salt(format!("layer_blend_{index}"))
                            .selected_text(current_blend.to_string())
                            .show_ui(ui, |ui| {
                                for mode in LayerBlendMode::iter() {
                                    if ui
                                        .selectable_value(
                                            &mut current_blend,
                                            mode,
                                            mode.to_string(),
                                        )
                                        .changed()
                                    {
                                        animator.set_blend_mode(current_blend);
                                    }
                                }
                            });
                    });

                    let mut current_poly = animator.get_polygon_mode();
                    ui.horizontal(|ui| {
                        ui.label("Raster:");
                        for mode in LayerPolygonMode::iter() {
                            if ui
                                .radio_value(&mut current_poly, mode, mode.to_string())
                                .changed()
                            {
                                animator.set_polygon_mode(current_poly);
                            }
                        }
                    });

                    ui.add(egui::Separator::default().spacing(4.0));
                    animator.control_ui(ui, &self.modulators);
                }
            } else {
                ui.label(egui::RichText::new("SELECT A LAYER"));
            }
        });
    }
}
