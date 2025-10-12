extern crate nannou;
use nannou::{color::encoding::Srgb, prelude::*};
use nannou::{draw, prelude::*};

mod animator;
use crate::animator::Animator;
use crate::animator::AnimatorObject;

mod reciver;
use crate::reciver::ReciverGrid;

mod Utils;
use crate::Utils::AppMode;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    animators: Vec<AnimatorObject>,
    receivers: Vec<ReciverGrid>,
    // global settings
    app_mode: AppMode,
}

fn model(app: &App) -> Model {
    app.new_window().event(event).view(view).build().unwrap();

    let win_rect = app.window_rect();

    let _ramdomBalls: Animator = Animator {
        countObject: 10,
        multiColor: false,
    };
    let animators = _ramdomBalls.generateRandomBall(&win_rect);

    let main_receiver_rect = Rect::from_x_y_w_h(0.0, 0.0, 400.0, 300.0);
    let receiver_grid = ReciverGrid::new(main_receiver_rect, 10, 8); // 10 columns, 8 rows

    Model {
        animators,
        receivers: vec![receiver_grid],
        app_mode: AppMode::Preview,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let win_rect = _app.window_rect();
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
    draw.background().color(BLACK);

    match _model.app_mode {
        AppMode::Edit | AppMode::Preview => {
            for receiver in &_model.receivers {
                for cell in &receiver.cells {

                    // ReciverCell::
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

                    // Draw the cell's border
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

    draw.to_frame(_app, &frame).unwrap();
}
