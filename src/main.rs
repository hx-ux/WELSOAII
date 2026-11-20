// External crate imports
extern crate nannou;
use nannou::prelude::*;

// Module imports
mod animator;
mod color;
mod modulator;
mod parameters;
mod receiver;
mod scene;
mod sender;
mod timecode;
mod ui;
mod utils;
// Re-exports for public API
pub use utils::AppMode;

use crate::animator::Animator;
// Core component imports
use crate::receiver::{LayoutMode, ReceiverGrid};
use crate::ui::performance_view::PerfStats;
use crate::utils::{AppLogger, AppSettings, Severity};
fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    animator: Animator,
    window: Entity,
    global_settings: AppSettings,
    device_modal_open: bool,
    settings_modal_open: bool,
    performance_view: PerfStats,
    logger: AppLogger,
    log_modal_open: bool,
}

fn model(app: &App) -> Model {
    let logger = AppLogger::default();

    let global_settings = AppSettings::load_or_default(&logger);

    app.set_update_rate(global_settings.framerate.value);

    let view_window_id = app
        .new_window()
        .title(AppSettings::APP_NAME)
        .size(
            global_settings.view_window_size.0,
            global_settings.view_window_size.1,
        )
        .view(view)
        .key_pressed(key_pressed)
        .primary()
        .build();
    logger.log("Application started", Severity::Info);

    let win_rect = app.window_rect();

    let receiver_grid = ReceiverGrid::new(
        Rect::from_x_y_w_h(0.0, 0.0, 400.0, 300.0),
        20,
        20,
        LayoutMode::Colum,
        logger.clone(),
    );

    let mut animator = Animator::new(&win_rect, receiver_grid, logger.clone());
    animator.init_all_layers(&win_rect);

    Model {
        animator,
        window: view_window_id,
        global_settings,
        device_modal_open: false,
        settings_modal_open: false,
        performance_view: PerfStats::new(),
        logger,
        log_modal_open: false,
    }
}

fn update(app: &App, model: &mut Model) {
    let win_rect = app.window_rect();

    let now_secs = app.time() as f64;
    let delta_secs = app.time_delta();
    model.performance_view.on_new_frame(now_secs, delta_secs);

    if model.global_settings.app_mode == AppMode::Edit {
        let ctx = app.egui_for_window(model.window);
        crate::ui::style_injector::apply_custom_style(
            &ctx,
            model.global_settings.control_windows_opacity.value,
            model.global_settings.fully_transparent,
        );

        let mut viewport_ui = egui::Ui::new(
            ctx.clone(),
            "viewport".into(),
            egui::UiBuilder::new()
                .layer_id(egui::LayerId::background())
                .max_rect(ctx.viewport_rect()),
        );

        egui::Panel::top("MENU").show_inside(&mut viewport_ui, |ui| {
            ui.horizontal(|ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.menu_button("Settings", |ui| {
                        if ui.button("Device").clicked() {
                            model.device_modal_open = true;
                            ui.close();
                        }
                        if ui.button("Settings").clicked() {
                            model.settings_modal_open = true;
                            ui.close();
                        }

                        if ui.button("Log").clicked() {
                            model.log_modal_open = true;
                            ui.close();
                        }
                    });
                    ui.separator();
                    model.animator.timecode.ui(ui);
                    ui.separator();
                    model.performance_view.ui(ui);
                });
            });
        });

        let window_width = model.global_settings.view_window_size.0 as f32;
        egui::Panel::left("control_panel")
            .resizable(true)
            .min_size(window_width * 0.2)
            .max_size(window_width)
            .default_size(window_width * 0.2)
            .show_animated_inside(&mut viewport_ui, true, |ui| {
                ui.add_space(1.0);
                model.animator.animator_layer_ui(ui, &win_rect);
                model.animator.animator_ui(ui);
            });

        egui::Panel::bottom("Modulators")
            .exact_size(200.0)
            .show_inside(&mut viewport_ui, |ui| {
                ui.add_space(1.0);
                ui.separator();
                ui.horizontal(|ui| {
                    model.animator.modulators_ui(ui);
                });
            });

        egui::Window::new("GLOBAL SETTINGS")
            .resizable(true)
            .default_open(true)
            .open(&mut model.settings_modal_open)
            .show(&ctx, |ui| {
                model.global_settings.ui(ui, &model.logger);
            });

        egui::Window::new("DEVICE")
            .resizable(true)
            .default_open(true)
            .open(&mut model.device_modal_open)
            .show(&ctx, |ui| {
                model.animator.grid.ui(ui);
            });

        egui::Window::new("LOG")
            .resizable(true)
            .default_open(true)
            .open(&mut model.log_modal_open)
            .show(&ctx, |ui| {
                model.logger.ui(ui);
            });
    }

    model.animator.update(&win_rect, delta_secs);
}

fn key_pressed(app: &App, model: &mut Model, key: KeyCode) {
    // Don't trigger shortcuts while the user is typing into an egui text field
    if model.global_settings.app_mode == AppMode::Edit
        && app
            .egui_for_window(model.window)
            .egui_wants_keyboard_input()
    {
        return;
    }

    let receiver = &mut model.animator.grid;

    match key {
        KeyCode::ArrowUp => {
            if receiver.edit_mode.value {
                receiver.move_by(vec2(0.0, 10.0));
            }
        }
        KeyCode::ArrowDown => {
            if receiver.edit_mode.value {
                receiver.move_by(vec2(0.0, -10.0));
            }
        }
        KeyCode::ArrowRight => {
            if receiver.edit_mode.value {
                receiver.move_by(vec2(10.0, 0.0));
            }
        }
        KeyCode::ArrowLeft => {
            if receiver.edit_mode.value {
                receiver.move_by(vec2(-10.0, 0.0));
            }
        }

        KeyCode::Equal | KeyCode::NumpadAdd => {
            if receiver.edit_mode.value {
                receiver.resize_by(vec2(10.0, 10.0))
            }
        }
        KeyCode::Minus | KeyCode::NumpadSubtract => {
            if receiver.edit_mode.value {
                receiver.resize_by(vec2(-10.0, -10.0))
            }
        }
        KeyCode::KeyP => model.global_settings.app_mode = AppMode::Presentation,
        KeyCode::KeyE => model.global_settings.app_mode = AppMode::Edit,
        _ => (),
    }
}

fn view(app: &App, model: &Model) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.animator.draw_animator(&draw);
    model.animator.draw_grid(&draw);
}
