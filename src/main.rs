extern crate nannou;
use nannou::prelude::*;
use nannou_egui::{self, Egui, egui};
mod ui;
mod animator;
mod receiver;
mod utils;
mod color;
use animator::Animator;
use animator::UpdateBehaviour;
use receiver::{LayoutMode, ReceiverGrid};
pub use utils::AppMode;
use utils::GlobalSettings;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    animators: Animator,
    settings_egui: Egui,
    global_settings: GlobalSettings,
}

fn settings_window_event(app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.settings_egui.handle_raw_event(event);
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

    let win_rect: Rect = app.window_rect();

    let receiver_grid = ReceiverGrid::new(
        Rect::from_x_y_w_h(0.0, 0.0, 400.0, 300.0),
        20,
        20,
        true,
        LayoutMode::FollowColum,
    );

    let mut animator = Animator::new(&win_rect, receiver_grid);
    animator.reset(&win_rect);

    app.set_loop_mode(LoopMode::RefreshSync);

    Model {
        animators: animator,
        settings_egui,
        global_settings,
    }
}


fn update(_app: &App, _model: &mut Model, _update: Update) {
    let win_rect = _app.window_rect();

    let egui = &mut _model.settings_egui;
    egui.set_elapsed_time(_update.since_start);

    let ctx = egui.begin_frame();

    crate::ui::style::apply_custom_style(&ctx,_model.global_settings.window_opacity.value);

    egui::Window::new("Global Settings").show(&ctx, |ui| {
        _model.global_settings.ui(ui);
    });

    egui::Window::new("Device").show(&ctx, |ui| {
        _model.animators.grid.ui(ui);
    });

    egui::Window::new("Animator Controls").show(&ctx, |ui| match _model.animators.ui(ui) {
        UpdateBehaviour::NeedsReset => _model.animators.reset(&win_rect),
        UpdateBehaviour::HotUpdate => _model.animators.behaviour_hot_update(),
        UpdateBehaviour::LoadPreset => {}
        UpdateBehaviour::SavePrest => _model.animators.save_preset(),
        UpdateBehaviour::None => {}
    });

    _model
        .animators
        .update(&win_rect, _app.duration.since_prev_update.as_secs_f32());
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    let receiver = &mut _model.animators.grid;

    match event {
        KeyPressed(_key) => match _key {
            Key::Up => receiver.move_by(vec2(0.0, 10.0)),
            Key::Down => receiver.move_by(vec2(0.0, -10.0)),
            Key::Left => receiver.move_by(vec2(-10.0, 0.0)),
            Key::Right => receiver.move_by(vec2(10.0, 0.0)),
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

    _model.animators.draw_animator(&draw);
    _model.animators.draw_grid(&draw);
   
    draw.to_frame(_app, &frame).unwrap();

    match _model.global_settings.app_mode {
        AppMode::Presentation => {}
        AppMode::Edit => _model.settings_egui.draw_to_frame(&frame).unwrap(),
    }
}
