extern crate nannou;
use nannou::{color::encoding::Srgb, prelude::*};
use nannou_egui::{self, Egui, egui};

mod animator;
use crate::animator::AnimatorNew;

mod reciver;
use crate::reciver::ReciverGrid;

mod Utils;
use crate::Utils::AppMode;
use crate::Utils::GlobalSettings;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    view_window: WindowId,
    settings_window: WindowId,

    animators: AnimatorNew,
    // animators: Vec<AnimatorObject>,
    //  receivers: Vec<ReciverGrid>,
    // currReciver: ReciverGrid,
    settings_egui: Egui,
    global_settings: GlobalSettings,
}

fn settings_window_event(app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    println!("window b: {:?}", event);
    model.settings_egui.handle_raw_event(event);
}

fn model(app: &App) -> Model {
    let global_settings = GlobalSettings::load_or_default("");

    let serialized = serde_json::to_string(&global_settings).unwrap();
    print!("{}", serialized);

    app.set_loop_mode(LoopMode::rate_fps(global_settings.framerate));

    let view_window = app
        .new_window()
        .title("main")
        // .always_on_top(false)
        .size(
            global_settings.view_window_size.0,
            global_settings.view_window_size.1,
        )
        .view(view)
        .event(event)
        .build()
        .unwrap();

    let settings_window = app
        .new_window()
        .title("settings")
        // .always_on_top(true)
        .size(
            global_settings.settings_window_size.0,
            global_settings.settings_window_size.1,
        )
        .view(view)
        .raw_event(settings_window_event)
        .build()
        .unwrap();

    let settings_window_ref = app.window(settings_window).unwrap();

    let egui = Egui::from_window(&settings_window_ref);
    let win_rect: Rect = app.window_rect();

    let mut animator = AnimatorNew::new(&win_rect);
    animator.reset(&win_rect);

    let main_receiver_rect = Rect::from_x_y_w_h(0.0, 0.0, 400.0, 300.0);
    let receiver_grid = ReciverGrid::new(main_receiver_rect, 10, 8); // 10 columns, 8 rows

    animator.link_grid(receiver_grid);

    Model {
        animators: animator,
        //    receivers: vec![receiver_grid],
        // currReciver,
        settings_egui: egui,
        view_window,
        settings_window,
        global_settings,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let win_rect = _app.window_rect();

    let egui = &mut _model.settings_egui;
    let ctx = egui.begin_frame();

    egui::Window::new("Global Settings").show(&ctx, |ui| if _model.global_settings.ui(ui) {});

    egui::Window::new("Grid").show(&ctx, |ui| if (_model.animators.grid.ui(ui)) {});

    egui::Window::new("Animator Controls").show(&ctx, |ui| {
        if _model.animators.ui(ui) {
            _model.animators.reset(&win_rect);
        }
    });

    // IDK
    //  _model.animators.grid.cells.reset
    //     .receivers
    //     .iter_mut()
    //     .for_each(|receiver| receiver.cells.iter_mut().for_each(|cell| cell.reset()));

    _model
        .animators
        .update(&win_rect, _app.duration.since_prev_update.as_secs_f32());
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    // if _model.receivers.is_empty() {
    //     return;
    // }
    let receiver = &mut _model.animators.grid;

    // println!("{:?}", event);

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
            Key::R => _model.global_settings.app_mode = AppMode::Preview,
            _ => (),
        },
        MousePressed(_button) => {
            // do_something();
        }
        MouseReleased(_button) => {}

        _other => {}
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();

    _model.animators.draw(&draw);
    
    match frame.window_id() {
        id if id == _model.view_window => match _model.global_settings.app_mode {
            AppMode::Edit | AppMode::Preview => {
                draw.background().color(BLACK);
                _model.animators.grid.draw(&draw);
            }
            AppMode::Presentation => {}
        },

        id if id == _model.settings_window => {
            draw.background().color(DARKGRAY);

            draw.to_frame(_app, &frame).unwrap();
            _model.settings_egui.draw_to_frame(&frame).unwrap();
        }

        _ => (),
    }
    draw.to_frame(_app, &frame).unwrap();
    if frame.window_id() == _model.settings_window {
        _model.settings_egui.draw_to_frame(&frame).unwrap();
    }
}
