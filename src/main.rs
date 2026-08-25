// External crate imports
extern crate nannou;
use nannou::prelude::*;
use nannou_egui::{self, Egui, egui};

// Module imports
mod animator;
mod color;
mod modulator;
mod parameters;
mod presets;
mod receiver;
mod timecode;
mod ui;
mod utils;

// Re-exports for public API
pub use utils::AppMode;

// Core component imports
use crate::animator::Animator;
use crate::animator::animation_type::UpdateBehaviour;
use crate::receiver::{LayoutMode, ReceiverGrid};
use crate::utils::GlobalSettings;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    animator: Animator,
    settings_egui: Egui,
    global_settings: GlobalSettings,
    perf_stats: ui::performance_view::PerfStats,
}

fn model(app: &App) -> Model {
    let global_settings = GlobalSettings::load_or_default();

    app.set_loop_mode(LoopMode::rate_fps(global_settings.framerate));

    let view_window_id = app
        .new_window()
        .title(GlobalSettings::APP_NAME)
        .size(
            global_settings.view_window_size.0,
            global_settings.view_window_size.1,
        )
        .view(view)
        .event(event)
        .raw_event(settings_window_event)
        .build()
        .unwrap();

    let window = app.window(view_window_id).unwrap();
    let settings_egui = Egui::from_window(&window);
    let win_rect = app.window_rect();

    let receiver_grid = ReceiverGrid::new(
        Rect::from_x_y_w_h(0.0, 0.0, 400.0, 300.0),
        20,
        20,
        false,
        LayoutMode::FollowColum,
    );

    let mut animator = Animator::new(&win_rect, receiver_grid);
    animator.reset(&win_rect);

    app.set_loop_mode(LoopMode::RefreshSync);

    Model {
        animator,
        settings_egui,
        global_settings,
        perf_stats: ui::performance_view::PerfStats::new(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let win_rect = _app.window_rect();

    let egui = &mut _model.settings_egui;
    egui.set_elapsed_time(_update.since_start);

    let ctx = egui.begin_frame();
    crate::ui::style_injector::apply_custom_style(
        &ctx,
        _model.global_settings.control_windows_opacity.value as u8,
    );

    // Record frame timing for the perf window (always, even in Presentation mode).
    let now_secs = _update.since_start.as_secs_f64();
    let delta_secs = _app.duration.since_prev_update.as_secs_f32();
    _model.perf_stats.on_new_frame(now_secs, delta_secs);

    egui::SidePanel::left("control_panel")
        .resizable(true)
        .min_width(_model.global_settings.view_window_size.1 as f32 * 0.2)
        .max_width(_model.global_settings.view_window_size.1 as f32)
        .default_width(_model.global_settings.view_window_size.1 as f32 * 0.3)
        .show_animated(&ctx, true, |ui| {
            ui.heading("Welosa II");
            ui.separator();
            ui.collapsing("Global", |ui| {
                _model.global_settings.ui(ui);
            });
            ui.separator();
            ui.collapsing("Time Code", |ui| {
                _model.animator.clock.ui(ui);
            });
            ui.separator();

            ui.collapsing("Device", |ui| {
                _model.animator.grid.ui(ui);
            });
            ui.separator();

            ui.collapsing("Performance", |ui| {
                _model.perf_stats.ui(ui);
            });
            ui.separator();

            ui.collapsing("Mod Matrix", |ui| {
                let current_beat = _model.animator.clock.get_beats();
                _model
                    .animator
                    .modulators
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, modulator)| {
                        ui.collapsing(format!("LFO {}", i), |ui| {
                            modulator.ui(ui, current_beat);
                        });
                    });
            });
            ui.separator();

            // always set hot update
            // // idk if this a good idea
            _model.animator.behaviour_hot_update();

            ui.collapsing("Animator", |ui| match _model.animator.ui(ui) {
                UpdateBehaviour::NeedsReset => _model.animator.reset(&win_rect),
                UpdateBehaviour::HotUpdate => _model.animator.behaviour_hot_update(),
                UpdateBehaviour::LoadPreset => {}
                UpdateBehaviour::SavePresets => _model.animator.save_preset(),
                UpdateBehaviour::None => {}
            });
        });

    _model
        .animator
        .update(&win_rect, _app.duration.since_prev_update.as_secs_f32());
}

fn settings_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.settings_egui.handle_raw_event(event);
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    let receiver = &mut _model.animator.grid;
    let win_rect = _app.window_rect();

    match event {
        KeyPressed(_key) => match _key {
            Key::Up => {
                if receiver.edit_mode {
                    receiver.move_by(vec2(0.0, 10.0))
                }
            }
            Key::Down => {
                if receiver.edit_mode {
                    receiver.move_by(vec2(0.0, -10.0))
                }
            }
            Key::Right => {
                if receiver.edit_mode {
                    receiver.move_by(vec2(10.0, 0.0));
                } else {
                    _model.animator.switch_animation_tye(1);
                    _model.animator.reset(&win_rect);
                }
            }
            Key::Left => {
                if receiver.edit_mode {
                    receiver.move_by(vec2(-10.0, 0.0));
                } else {
                    _model.animator.switch_animation_tye(-1);
                    _model.animator.reset(&win_rect);
                }
            }

            Key::Equals | Key::Plus => receiver.resize_by(vec2(10.0, 10.0)),
            Key::Minus => receiver.resize_by(vec2(-10.0, -10.0)),
            Key::P => _model.global_settings.app_mode = AppMode::Presentation,
            Key::E => _model.global_settings.app_mode = AppMode::Edit,
            _ => (),
        },
        MousePressed(_button) => {}
        MouseReleased(_button) => {}

        _other => {}
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();
    draw.background().color(BLACK);

    _model.animator.draw_animator(&draw);
    _model.animator.draw_grid(&draw);

    draw.to_frame(_app, &frame).unwrap();

    match _model.global_settings.app_mode {
        AppMode::Presentation => {}
        AppMode::Edit => _model.settings_egui.draw_to_frame(&frame).unwrap(),
    }
}
