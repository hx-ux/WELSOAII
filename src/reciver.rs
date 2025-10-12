extern crate nannou;

use nannou::{color::encoding::Srgb, prelude::*};

pub struct ReciverObject {
    pub rect: Rect,

    pub targetColor: Rgba,
    pub is_active: bool,
}

impl ReciverObject {
    // const TRANSPARENT_WHITE: Rgba = Rgba::new(1.0, 1.0, 1.0, 0.1);
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        ReciverObject {
            rect: Rect::from_x_y_w_h(x, y, w, h),
            is_active: false,
            targetColor: Rgba::new(1.0, 1.0, 1.0, 0.1),
        }
    }

    pub fn reset(&mut self) {
        self.is_active = false;
    }
}

pub struct Reciver {
    pub revicer: Vec<ReciverObject>,
    //pub name: str,
}

impl Reciver {
    pub fn new(list: Vec<ReciverObject>) -> Self {
        Reciver { revicer: list }
    }
}
