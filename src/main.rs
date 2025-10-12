extern crate nannou;
use nannou::{color::encoding::Srgb, prelude::*};
// use nannou::{draw, prelude::*};

mod animator;
use crate::animator::Animator;
use crate::animator::AnimatorObject;

mod reciver;
use crate::reciver::Reciver;
use crate::reciver::ReciverObject;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    animators: Vec<AnimatorObject>,
    receivers: Reciver,
}

fn model(_app: &App) -> Model {
    let win_rect = _app.window_rect();

    let _ramdomBalls: Animator = Animator {
        countObject: 10,
        multiColor: false,
    };
    let animators = _ramdomBalls.generateRandomBall(&win_rect);

    let receivers = vec![
        ReciverObject::new(-200.0, 0.0, 100.0, 100.0),
        ReciverObject::new(200.0, 0.0, 100.0, 100.0),
        ReciverObject::new(0.0, 150.0, 150.0, 80.0),
    ];

    let r: Reciver = Reciver { revicer: receivers };

    Model {
        animators,
        receivers: r,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let win_rect = _app.window_rect();

    _model.receivers.revicer.iter_mut().for_each(|t| t.reset());

    for animator in &mut _model.animators {
        animator.update(&win_rect);

        for t in &mut _model.receivers.revicer {
            if t.rect.contains(animator.position) {
                t.is_active = true;
                t.targetColor = animator.color;

                // t.active_color = animator.color;
            }
        }
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();
    draw.background().color(BLUE);

    for receiver in &_model.receivers.revicer {
        // White with low opacity (alpha = 0.1)
        let transparent_white = Rgba::new(1.0, 1.0, 1.0, 0.1);
        let currColor = Rgba::new(
            receiver.targetColor.red,
            receiver.targetColor.green,
            receiver.targetColor.blue,
            0.5, // new opacity
        );

        let color = if receiver.is_active {
            currColor
        } else {
            transparent_white
        };

        draw.rect()
            .xy(receiver.rect.xy())
            .wh(receiver.rect.wh())
            .color(color);

        // Add a stroke to show the receiver's boundary clearly.
        draw.rect()
            .xy(receiver.rect.xy())
            .wh(receiver.rect.wh())
            .no_fill()
            .stroke_weight(2.0)
            .stroke(SNOW);
    }

    for animator in &_model.animators {
        draw.ellipse()
            .xy(animator.position)
            .radius(animator.radius)
            .color(animator.color);
    }

    draw.to_frame(_app, &frame).unwrap();
}
