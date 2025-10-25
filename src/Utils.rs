use nannou::color::rgb::Rgb;
use nannou::color::Rgba;

use nannou::rand::random_range;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum AppMode {
    Presentation,
    Edit,
    Preview,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct GlobalSettings {
    pub framerate: f64,
    pub view_window_size: (u32, u32),
    pub settings_window_size: (u32, u32),
    pub app_mode: AppMode,
}

impl GlobalSettings {
    pub fn load_or_default(path: &str) -> Self {
        let result = std::fs::read_to_string(path);

        Self {
            framerate: 60.0,
            view_window_size: (1000, 1000),
            settings_window_size: (800, 400),
            app_mode: AppMode::Preview,
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json_string)?;
        println!("Successfully saved settings to '{}'", path);
        Ok(())
    }
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.label("Framerate");
        let framerate_response = ui.add(egui::Slider::new(&mut self.framerate, 1.0..=60.0));
        ui.separator();
        ui.horizontal(|ui| {
            changed |= ui
                .radio_value(&mut self.app_mode, AppMode::Presentation, "Presentation")
                .changed();
            changed |= ui
                .radio_value(&mut self.app_mode, AppMode::Edit, "Edit")
                .changed();
            changed |= ui
                .radio_value(&mut self.app_mode, AppMode::Preview, "Preview")
                .changed();
        });

        if framerate_response.changed() {
            changed = true;
        }

        changed
    }
}

pub trait ColorHelpers {
    fn to_rgb(&self) -> Rgb;
    fn placeholder() -> Rgba;
    fn to_sendable(&self) -> (u8, u8, u8);
    fn from_egui(col: egui::Color32) -> Rgba;
    fn random() -> Rgba;
    fn standard() -> Rgba;
    fn hsv_to_rgb(h:f32,v:f32,s:f32) ->(u8, u8, u8);
}

impl ColorHelpers for Rgba {
    fn to_rgb(&self) -> Rgb {
        Rgb::from_components((self.red, self.green, self.blue))
    }

    fn placeholder() -> Rgba {
        Rgba::new(1.0, 0.0, 0.0, 1.0)
    }

    fn from_egui(col: egui::Color32) -> Rgba {
        let _r = col.r() as f32 / 255.0;
        let _g = col.g() as f32 / 255.0;
        let _b = col.b() as f32 / 255.0;
        let _a = col.a() as f32 / 255.0;
        Rgba::new(_r, _g, _b, _a)
    }

    fn random() -> Rgba {
        Rgba::new(
            random_range(0.0, 1.0),
            random_range(0.0, 1.0),
            random_range(0.0, 1.0),
            1.0,
        )
    }

    fn standard() -> Rgba {
        Rgba::new(1.0, 1.0, 1.0, 0.1)
    }
    
    fn to_sendable(&self) -> (u8, u8, u8) {
        let r = (self.red * 255.0) as u8;
        let g = (self.green * 255.0) as u8;
        let b = (self.blue * 255.0) as u8;
        (r, g, b)
    }

    fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {

        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r_prime, g_prime, b_prime) = if (0.0..60.0).contains(&h) {
            (c, x, 0.0)
        } else if (60.0..120.0).contains(&h) {
            (x, c, 0.0)
        } else if (120.0..180.0).contains(&h) {
            (0.0, c, x)
        } else if (180.0..240.0).contains(&h) {
            (0.0, x, c)
        } else if (240.0..300.0).contains(&h) {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        (
            ((r_prime + m) * 255.0) as u8,
            ((g_prime + m) * 255.0) as u8,
            ((b_prime + m) * 255.0) as u8,
        )
    }
}
