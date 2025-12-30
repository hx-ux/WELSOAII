use super::{AnimatedObject, ObjectShape};
use nannou::prelude::*;
use nannou_egui::egui;

pub struct PulseBackgroundSettings {
    #[allow(dead_code)]
    pub mode: i16,
    #[allow(dead_code)]
    pub speed: f32,
}

impl Default for PulseBackgroundSettings {
    fn default() -> Self {
        Self {
            mode: 0,
            speed: 4.0,
        }
    }
}

impl PulseBackgroundSettings {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        ui.label("Pulse effect");
        false
    }
}

pub struct PulseBackground {
    color: Rgba,
    current_alpha: f32,
    time: f32,
    window_dimension: (f32, f32),
}

impl PulseBackground {
    fn new(color: Rgba, win_rect: &Rect) -> Self {
        Self {
            color,
            current_alpha: 0.0,
            time: 0.0,
            window_dimension: win_rect.w_h(),
        }
    }
    pub fn factory(settings: &PulseBackgroundSettings, win_rect: &Rect, color: Rgba) -> Self {
        let _ = settings; // Settings not used yet for PulseBackground
        Self::new(color, win_rect)
    }
}

impl AnimatedObject for PulseBackground {
    fn update(&mut self, _win_rect: &Rect, delta_time: f32) {
        self.time += delta_time;
        self.current_alpha = (self.time * 2.0).sin() * 0.5 + 0.5; // 0.0 to 1.0
    }

    fn draw(&self, draw: &Draw) {
        let mut display_color = self.color;
        display_color.alpha = self.current_alpha;

        draw.rect()
            .x(0.0)
            .y(0.0)
            .w(self.window_dimension.0)
            .h(self.window_dimension.1)
            .color(display_color);
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Rect(Rect::from_w_h(0.0, 0.0)) // Empty rect
    }

    fn color(&self) -> Rgba {
        let mut c = self.color;
        c.alpha = self.current_alpha;
        c
    }
}
