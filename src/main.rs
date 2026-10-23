// External crate imports
extern crate nannou;
use nannou::prelude::*;
use nannou_egui::{self, Egui, egui};

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
    egui: Egui,
    global_settings: AppSettings,
    device_modal_open: bool,
    settings_modal_open: bool,
    performance_view: PerfStats,
    logger: AppLogger,
    log_modal_open: bool,
    scene_manager_open: bool,
}

fn model(app: &App) -> Model {
    let logger = AppLogger::default();

    let global_settings = AppSettings::load_or_default(&logger);

    app.set_loop_mode(LoopMode::rate_fps(global_settings.framerate));

    let view_window_id = app
        .new_window()
        .title(AppSettings::APP_NAME)
        .size(
            global_settings.view_window_size.0,
            global_settings.view_window_size.1,
        )
        .view(view)
        .event(event)
        .raw_event(settings_window_event)
        .build()
        .unwrap();
    logger.log("Application started", Severity::Info);

    let window = app.window(view_window_id).unwrap();
    let settings_egui = Egui::from_window(&window);
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
        egui: settings_egui,
        global_settings,
        device_modal_open: false,
        settings_modal_open: false,
        performance_view: PerfStats::new(),
        logger,
        log_modal_open: false,
        scene_manager_open: false,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let win_rect = app.window_rect();

    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();
    crate::ui::style_injector::apply_custom_style(
        &ctx,
        model.global_settings.control_windows_opacity.value,
        model.global_settings.fully_transparent,
    );

    let now_secs = update.since_start.as_secs_f64();
    let delta_secs = app.duration.since_prev_update.as_secs_f32();
    model.performance_view.on_new_frame(now_secs, delta_secs);

    egui::TopBottomPanel::top("MENU").show(&ctx, |ui| {
        ui.horizontal(|ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Settings", |ui| {
                    if ui.button("Device").clicked() {
                        model.device_modal_open = true;
                        ui.close_menu();
                    }
                    if ui.button("Settings").clicked() {
                        model.settings_modal_open = true;
                        ui.close_menu();
                    }

                    if ui.button("Log").clicked() {
                        model.log_modal_open = true;
                        ui.close_menu();
                    }

                    if ui.button("Scene").clicked() {
                        model.scene_manager_open = true;
                        ui.close_menu();
                    }
                });
                ui.separator();
                model.animator.timecode.ui(ui);
                ui.separator();
                model.performance_view.ui(ui);
            });
        });
    });

    egui::SidePanel::left("control_panel")
        .resizable(true)
        .min_width(model.global_settings.view_window_size.1 as f32 * 0.2)
        .max_width(model.global_settings.view_window_size.1 as f32)
        .default_width(model.global_settings.view_window_size.1 as f32 * 0.1)
        .show_animated(&ctx, true, |ui| {
            ui.add_space(1.0);
            model.animator.animator_layer_ui(ui, &win_rect);
            model.animator.animator_ui(ui);
        });

    egui::TopBottomPanel::bottom("Modulators")
        .exact_height(200.0)
        .show(&ctx, |ui| {
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

    egui::Window::new("SCENE")
        .resizable(true)
        .default_open(true)
        .open(&mut model.log_modal_open)
        .show(&ctx, |ui| {
            model.logger.ui(ui);
        });

    model
        .animator
        .update(&win_rect, app.duration.since_prev_update.as_secs_f32());
}

fn settings_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    let receiver = &mut model.animator.grid;

    match event {
        KeyPressed(key) => match key {
            Key::Up => {
                if receiver.edit_mode {
                    receiver.move_by(vec2(0.0, 10.0));
                }
            }
            Key::Down => {
                if receiver.edit_mode {
                    receiver.move_by(vec2(0.0, -10.0));
                }
            }
            Key::Right => {
                if receiver.edit_mode {
                    receiver.move_by(vec2(10.0, 0.0));
                }
            }
            Key::Left => {
                if receiver.edit_mode {
                    receiver.move_by(vec2(-10.0, 0.0));
                }
            }

            Key::Equals | Key::Plus => receiver.resize_by(vec2(10.0, 10.0)),
            Key::Minus => receiver.resize_by(vec2(-10.0, -10.0)),
            Key::P => model.global_settings.app_mode = AppMode::Presentation,
            Key::E => model.global_settings.app_mode = AppMode::Edit,
            _ => (),
        },
        MousePressed(_button) => {}
        MouseReleased(_button) => {}

        _other => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.animator.draw_animator(&draw);
    model.animator.draw_grid(&draw);

    draw.to_frame(app, &frame).unwrap();

    match model.global_settings.app_mode {
        AppMode::Presentation => {}
        AppMode::Edit => model.egui.draw_to_frame(&frame).unwrap(),
    }
}
