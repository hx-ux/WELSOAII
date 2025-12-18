use super::{AnimatedObject, ObjectShape};
use nannou::prelude::*;
use nannou_egui::egui;

#[derive(Debug)]
pub struct ScanLineSettings {
    pub mode: i16,
    pub speed: f32,
    pub start_x: f32,
}

impl ScanLineSettings {
    pub fn factory(win_rect: &Rect) -> Self {
        Self {
            mode: 0,
            speed: 300.0,
            start_x: win_rect.left() + 10.0, // plus rect size
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        
        let mut changed = false;

        ui.heading("ScanLine Settings");
        egui::Grid::new("scanline_grid")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Speed:");
                changed |= ui
                    .add(
                        egui::DragValue::new(&mut self.speed)
                            .speed(5.0)
                            .clamp_range(-1000.0..=1000.0),
                    )
                    .changed();
                ui.end_row();

                ui.label("Mode:");
                let mode_str = if self.mode == 0 {
                    "Ping-Pong"
                } else {
                    "Wrap"
                };
                egui::ComboBox::from_label("")
                    .selected_text(mode_str)
                    .show_ui(ui, |ui| {
                        changed |= ui
                            .selectable_value(&mut self.mode, 0, "Ping-Pong")
                            .changed();
                        changed |= ui
                            .selectable_value(&mut self.mode, 1, "Wrap")
                            .changed();
                    });
                ui.end_row();
            });
            changed
    }
}

pub struct ScanLine {
    mode: i16,
    speed: f32,
    color: Rgba,
    position: Vec2,

    window_dimension: (f32, f32),
    width: f32,
}

impl ScanLine {
    pub fn new(scanline_settings: &ScanLineSettings, win_rect: &Rect, color: Rgba) -> Self {
        let pos = vec2(scanline_settings.start_x, 0.0); // Vertically centered

        ScanLine {
            mode: scanline_settings.mode,
            speed: scanline_settings.speed,
            color,
            position: pos,
            window_dimension: (win_rect.w(), win_rect.h()),
            width: 20.0,
        }
    }

    fn bounding_box(&self) -> Rect {
        Rect::from_x_y_w_h(
            self.position.x,
            self.position.y,
            self.width,
            self.window_dimension.1,
        )
    }
}

impl AnimatedObject for ScanLine {
    fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        self.window_dimension = win_rect.w_h();
        self.position.x += self.speed * delta_time;

        let half_width = self.width / 2.0;
        let left_bound = win_rect.left() + half_width;
        let right_bound = win_rect.right() - half_width;

        match self.mode {
            // Mode 0: Ping-Pong
            0 => {
                if self.position.x > right_bound {
                    self.position.x = right_bound;
                    self.speed *= -1.0;
                } else if self.position.x < left_bound {
                    self.position.x = left_bound;
                    self.speed *= -1.0;
                }
            }
            // Mode 1: Wrap-around
            _ => {
                if self.position.x > right_bound && self.speed > 0.0 {
                    self.position.x = left_bound;
                } else if self.position.x < left_bound && self.speed < 0.0 {
                    self.position.x = right_bound;
                }
            }
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.rect()
            .xy(self.position)
            .height(self.window_dimension.1)
            .width(self.width)
            .color(self.color);
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Rect(self.bounding_box())
    }

    fn color(&self) -> Rgba {
        self.color
    }
}
