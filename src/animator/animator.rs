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
    parameters::ModulatedParam,
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
    pub trail_persistence: ModulatedParam,
    pub last_retention: f32,
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
            trail_persistence: ModulatedParam::new(
                0.0,
                0.0,
                0.97,
                "Trail Persistence",
                "trail_persistence",
            ),
            last_retention: 0.0,
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

        for animations in self.active_animations.iter_mut() {
            animations.update(win_rect, &self.timecode);
        }

        let trail_val = self.trail_persistence.value().clamp(0.0, 0.97);
        if self.force_clear_frames > 0 {
            self.force_clear_frames -= 1;
            self.last_retention = 0.0;
            for cell in self.grid.cells.iter_mut() {
                cell.reset();
            }
        } else if trail_val <= 0.001 {
            self.last_retention = 0.0;
            for cell in self.grid.cells.iter_mut() {
                cell.reset();
            }
        } else {
            let retention = trail_val.powf((delta_time * 60.0).clamp(0.2, 4.0));
            self.last_retention = retention;
            for cell in self.grid.cells.iter_mut() {
                cell.decay(retention);
            }
        }

        for layer in self.active_animations.iter() {
            if !layer.get_visibility() {
                continue;
            }

            let blend_mode = layer.get_blend_mode();
            let wireframe = layer.get_polygon_mode().is_wireframe();

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

                        let cell = &mut self.grid.cells[idx];
                        if obj_shape.intersects_cell(&cell.rect, wireframe) {
                            cell.blend(obj_color, blend_mode);
                        }
                    }
                }
            }
        }
        self.grid.update_led_buffer_and_send();
    }

    pub fn draw_animator(&self, draw: &Draw, win_rect: &Rect) {
        if self.last_retention <= 0.001 {
            draw.background().color(BLACK);
        } else {
            // Persistent Bevy framebuffer decay quad for motion trails
            let fade_alpha = (1.0 - self.last_retention).clamp(0.015, 1.0);
            draw.z(-1.0)
                .color_blend(BLEND_NORMAL)
                .polygon_mode(PolygonMode::Fill)
                .rect()
                .wh(win_rect.wh())
                .color(Srgba::new(0.0, 0.0, 0.0, fade_alpha));
        }

        for (idx, ani) in self.active_animations.iter().enumerate() {
            if !ani.get_visibility() {
                continue;
            }
            let layer_z = (idx as f32 + 1.0) * 0.05;
            let layer_draw = draw
                .z(layer_z)
                .color_blend(ani.get_blend_mode().to_wgpu_blend())
                .polygon_mode(ani.get_polygon_mode().to_wgpu_polygon_mode());
            ani.draw(&layer_draw);
        }
    }

    fn clear_mod_ghosts(&mut self) {
        self.trail_persistence.ghost_value = None;
        for effect in &mut self.active_animations {
            effect.reset_modulations();
        }
    }

    fn apply_modulations(&mut self) {
        self.clear_mod_ghosts();
        let beat_pos = self.timecode.get_beats();
        self.trail_persistence.modulate(beat_pos, &self.modulators);
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
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("COMPOSITOR & TRAILS")
                    .size(13.0)
                    .color(Color32::WHITE),
            );
            if ui
                .button(format!("{} Clear", ICON_CLEAR))
                .on_hover_text("Clear persistent motion trails on screen and WLED grid")
                .clicked()
            {
                self.request_clear();
            }
        });
        self.trail_persistence.slider_ui(ui, &self.modulators);

        ui.add_space(4.0);
        ui.separator();

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
        });

        ui.separator();

        let total_layers = self.active_animations.len();
        ui.vertical(|ui| {
            for index in 0..total_layers {
                let is_selected = self.current_animation_index == Some(index);
                let anim_name = self.active_animations[index].animation_type().to_string();
                let is_visible = self.active_animations[index].get_visibility();
                let blend_badge = self.active_animations[index].get_blend_mode().short_label();
                let poly_mode = self.active_animations[index].get_polygon_mode();

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

                    let badge_suffix = if poly_mode.is_wireframe() {
                        format!(" [{}:{}]", blend_badge, poly_mode.short_label())
                    } else {
                        format!(" [{}]", blend_badge)
                    };

                    let label = egui::RichText::new(format!(
                        "{}{}",
                        anim_name.to_uppercase(),
                        badge_suffix
                    ))
                    .color(if is_selected {
                        egui::Color32::from_rgb(255, 102, 0)
                    } else {
                        egui::Color32::from_gray(150)
                    });

                    if ui.button(label).clicked() {
                        self.current_animation_index = Some(index);
                    }

                    if index > 0
                        && ui
                            .add(
                                egui::Button::new(egui::RichText::new(ICON_CARET_UP))
                                    .min_size(egui::vec2(12.0, 12.0)),
                            )
                            .on_hover_text("Move layer earlier in blend order")
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

                    let eye_icon = if is_visible {
                        ICON_EYE_ACTIVE
                    } else {
                        ICON_EYE_DISABLED
                    };
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new(eye_icon))
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
