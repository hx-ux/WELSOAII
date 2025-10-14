extern crate nannou;
use nannou::image::flat::View;
use nannou::{color::encoding::Srgb, prelude::*};
use nannou::{draw, prelude::*};
use nannou_egui::{self, Egui, egui};

mod animator;
use crate::animator::Animator;
use crate::animator::AnimatorObject;

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
    animators: Vec<AnimatorObject>,
    receivers: Vec<ReciverGrid>,
    currReciver: ReciverGrid,
    app_mode: AppMode,
    settings_egui: Egui,
    global_settings: GlobalSettings,
}

fn settings_window_event(app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    println!("window b: {:?}", event);
    model.settings_egui.handle_raw_event(event);
}

fn model(app: &App) -> Model {
    let global_settings = GlobalSettings::default();
    app.set_loop_mode(LoopMode::rate_fps(global_settings.framerate));

    let view_window = app
        .new_window()
        .title("main")
        //.always_on_top(true)
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

    let _ramdomBalls: Animator = Animator {
        countObject: 10,
        multiColor: false,
    };

    let animators = _ramdomBalls.generateRandomBall(&win_rect);

    let main_receiver_rect = Rect::from_x_y_w_h(0.0, 0.0, 400.0, 300.0);
    let receiver_grid = ReciverGrid::new(main_receiver_rect, 10, 8); // 10 columns, 8 rows
    let currReciver = receiver_grid.clone();

    Model {
        animators,
        receivers: vec![receiver_grid],
        currReciver,
        app_mode: AppMode::Preview,
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

    egui::Window::new("Global Settings").show(&ctx, |ui| {
        ui.label("Framerate");
        let framerate_response = ui.add(egui::Slider::new(
            &mut _model.global_settings.framerate,
            1.0..=60.0,
        ));

        if framerate_response.changed() {
            print!("what");
            _app.set_loop_mode(LoopMode::rate_fps(_model.global_settings.framerate));
        }
    });

    egui::Window::new("Test2").show(&ctx, |ui| {
  ui.label("Framerate");
     

    });

    _model
        .receivers
        .iter_mut()
        .for_each(|receiver| receiver.cells.iter_mut().for_each(|cell| cell.reset()));

    for animator in &mut _model.animators {
        animator.update(&win_rect);

        for receiver in &mut _model.receivers {
            for cell in &mut receiver.cells {
                if cell.rect.contains(animator.position) {
                    cell.is_active = true;
                    cell.found_color = animator.color;
                }
            }
        }
    }
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    if _model.receivers.is_empty() {
        return;
    }
    let receiver = &mut _model.receivers[0];

    println!("{:?}", event);

    match event {
        KeyPressed(_key) => match _key {
            Key::Up => receiver.move_by(vec2(0.0, 10.0)),
            Key::Down => receiver.move_by(vec2(0.0, -10.0)),
            Key::Left => receiver.move_by(vec2(-10.0, 0.0)),
            Key::Right => receiver.move_by(vec2(10.0, 0.0)),
            Key::Equals | Key::Plus => receiver.resize_by(vec2(10.0, 10.0)),
            Key::Minus => receiver.resize_by(vec2(-10.0, -10.0)),
            Key::P => _model.app_mode = AppMode::Presentation,
            Key::E => _model.app_mode = AppMode::Edit,
            Key::R => _model.app_mode = AppMode::Preview,
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

    match frame.window_id() {
        id if id == _model.view_window => {
            match _model.app_mode {
                AppMode::Edit | AppMode::Preview => {
                    draw.background().color(BLACK);
                    for receiver in &_model.receivers {
                        for cell in &receiver.cells {
                            let color = if cell.is_active {
                                Rgba::new(
                                    cell.found_color.red,
                                    cell.found_color.green,
                                    cell.found_color.blue,
                                    0.5,
                                )
                            } else {
                                Rgba::new(1.0, 1.0, 1.0, 0.1)
                            };

                            draw.rect()
                                .xy(cell.rect.xy())
                                .wh(cell.rect.wh())
                                .color(color);

                            draw.rect()
                                .xy(cell.rect.xy())
                                .wh(cell.rect.wh())
                                .no_fill()
                                .stroke_weight(1.0)
                                .stroke(SNOW);
                        }
                    }
                }
                AppMode::Presentation => {}
            }

            for animator in &_model.animators {
                draw.ellipse()
                    .xy(animator.position)
                    .radius(animator.radius)
                    .color(animator.color);
            }
        }

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
