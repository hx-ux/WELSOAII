use nannou::prelude::*;
use super::{AnimatedObject, ObjectShape}; 

pub struct BouncingBall {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub color: Rgba,
}

impl BouncingBall {
    
    pub fn new(win_rect: &Rect, color: Rgba) -> Self {
        let radius = random_range(10.0, 30.0);
        BouncingBall {
            position: vec2(
                random_range(win_rect.left() + radius, win_rect.right() - radius),
                random_range(win_rect.bottom() + radius, win_rect.top() - radius),
            ),
            velocity: vec2(random_range(-200.0, 200.0), random_range(-200.0, 200.0)),
            radius,
            color,
        }
    }
}

impl AnimatedObject for BouncingBall {
    fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        self.position += self.velocity * delta_time;

        // Bounce off window edges
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

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.radius)
            .color(self.color);
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Circle(self.position, self.radius)
    }

    fn color(&self) -> Rgba {
        self.color
    }
}