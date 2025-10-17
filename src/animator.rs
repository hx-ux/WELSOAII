extern crate nannou;
use nannou::{color::encoding::Srgb, prelude::*};
use std::collections::HashMap;

pub struct AnimatorCore {}
impl AnimatorCore {
    fn color_map(pos: u32) -> Rgba {
        let _pos = pos % 5;

        let mut color_dict = HashMap::new();
        color_dict.insert(0, Rgba::new(1.0, 0.0, 0.0, 1.0)); // RED
        color_dict.insert(1, Rgba::new(0.0, 0.0, 1.0, 1.0)); // BLUE
        color_dict.insert(2, Rgba::new(0.0, 1.0, 0.0, 1.0)); // GREEN
        color_dict.insert(3, Rgba::new(1.0, 0.843, 0.0, 1.0)); // GOLD
        color_dict.insert(4, Rgba::new(0.275, 0.510, 0.706, 1.0)); // STEELBLUE

        return color_dict
            .get(&_pos)
            .copied()
            .unwrap_or(Rgba::new(1.0, 0.0, 0.0, 1.0));
    }

    fn gen_rand_col() -> Rgba {
        let col: nannou::color::Alpha<rgb::Rgb, f32> = Rgba::new(
            random_range(0.0, 1.0),
            random_range(0.0, 1.0),
            random_range(0.0, 1.0),
            1.0,
        );
        col
    }
}

pub struct AnimatorObject {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub color: Rgba,
}

impl AnimatorObject {
   
    // Singelton
    fn _randPositions(win_rect: &Rect, multiCol: bool) -> Self {
        let mut currColor = Rgba::new(1.0, 1.0, 1.0, 1.0);

        let radius = random_range(10.0, 25.0);
        if multiCol {
            currColor = AnimatorCore::gen_rand_col();
        };

        AnimatorObject {
            position: vec2(
                random_range(win_rect.left() + radius, win_rect.right() - radius),
                random_range(win_rect.bottom() + radius, win_rect.top() - radius),
            ),
            velocity: vec2(random_range(-2.0, 2.0), random_range(-2.0, 2.0)),
            radius,
            color: currColor,
        }
    }

    pub fn update(&mut self, win_rect: &Rect) {
        // Move by velocity
        self.position += self.velocity;

        // Bounce off walls
        if self.position.x < win_rect.left() + self.radius
            || self.position.x > win_rect.right() - self.radius
        {
            self.velocity.x *= -1.0;
        }
        if self.position.y < win_rect.bottom() + self.radius
            || self.position.y > win_rect.top() - self.radius
        {
            self.velocity.y *= -1.0;
        }
    }
}

pub struct Animator {
    pub countObjects: u32,
    pub multiColor: bool,
}
impl Animator {
    pub fn generateRandomBall(&self, _c_size: &Rect) -> Vec<AnimatorObject> {
        let animators = (0..self.countObjects)
            .map(|_| AnimatorObject::_randPositions(&_c_size, true))
            .collect();

        return animators;
    }
}
