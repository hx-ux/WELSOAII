extern crate nannou;
use nannou::{draw, prelude::*};

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame){
    let draw = _app.draw();
    // set background to blue
    draw.background().color(BLUE);
    // put everything on the frame
    draw.to_frame(_app, &frame).unwrap();
}
