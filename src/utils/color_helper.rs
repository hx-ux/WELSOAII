use nannou::color::Rgba;

use nannou::rand::random_range;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};



pub trait ColorHelpers {
    fn from_egui(col: egui::Color32) -> Rgba;
    fn random() -> Rgba {
        Rgba::new(
            random_range(0.0, 1.0),
            random_range(0.0, 1.0),
            random_range(0.0, 1.0),
            1.0,
        )
    }
    fn almost_transparent() -> Rgba {
        Rgba::new(1.0, 1.0, 1.0, 0.1)
    }
    fn hsv_to_rgb(h: f32, v: f32, s: f32) -> (u8, u8, u8);
    fn red() -> Rgba {
        Rgba::new(1.0, 0.0, 0.0, 1.0)
    }
}

impl ColorHelpers for Rgba {
    fn from_egui(col: egui::Color32) -> Rgba {
        let r = col.r() as f32 / 255.0;
        let g = col.g() as f32 / 255.0;
        let b = col.b() as f32 / 255.0;
        let a = col.a() as f32 / 255.0;
        Rgba::new(r, g, b, a)
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
