use nannou::color::{Rgba, Rgba8};
use std::slice::Iter;

use nannou::math::clamp;
use nannou::rand::random_range;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum ColorPalette {
    Breeze,
    Dolphin,
}

impl ColorPalette {
    fn breeze_palette() -> Vec<Rgba8> {
        vec![
            Rgba8::new(15, 194, 192, 255),
            Rgba8::new(12, 171, 168, 255),
            Rgba8::new(0, 143, 140, 255),
            Rgba8::new(1, 89, 88, 255),
            Rgba8::new(2, 53, 53, 255),
        ]
    }
    fn dolphin_palette() -> Vec<Rgba8> {
        vec![
            Rgba8::new(242, 121, 222, 255),
            Rgba8::new(191, 132, 216, 255),
            Rgba8::new(132, 119, 216, 255),
            Rgba8::new(181, 179, 242, 255),
            Rgba8::new(186, 194, 242, 255),
        ]
    }

    pub fn as_vec(&self) -> Vec<Rgba8> {
        match self {
            ColorPalette::Breeze => Self::breeze_palette(),
            ColorPalette::Dolphin => Self::dolphin_palette(),
        }
    }

    fn iterator() -> Iter<'static, ColorPalette> {
        static PALETTE: [ColorPalette; 2] = [ColorPalette::Breeze, ColorPalette::Dolphin];
        PALETTE.iter()
    }
    fn as_str(&self) -> &'static str {
        match self {
            ColorPalette::Breeze => "Breeze",
            ColorPalette::Dolphin => "Warm",
        }
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        ColorPalette::Breeze
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum ColorMode {
    Solid,
    Palette,
    Random,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ColorParam {
    pub single_color: Rgba8,
    pub mode: ColorMode,
    #[serde(skip)]
    pub random: Vec<Rgba8>,
    pub palette: ColorPalette,
}

impl ColorParam {
    pub fn new(mode: ColorMode) -> Self {
        Self {
            single_color: Rgba8::new(255, 0, 0, 255),
            mode,
            random: Self::random(),
            palette: ColorPalette::default(),
        }
    }
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            changed |= ui
                .radio_value(&mut self.mode, ColorMode::Solid, "Solid")
                .changed();
            changed |= ui
                .radio_value(&mut self.mode, ColorMode::Palette, "Palette")
                .changed();
        });

        match self.mode {
            ColorMode::Solid => {
                ui.label("Solid Color");
                changed |= ui
                    .add(egui::Slider::new(&mut self.single_color.red, 0..=255).text("R"))
                    .changed();
                changed |= ui
                    .add(egui::Slider::new(&mut self.single_color.green, 0..=255).text("G"))
                    .changed();
                changed |= ui
                    .add(egui::Slider::new(&mut self.single_color.blue, 0..=255).text("B"))
                    .changed();
                changed |= ui
                    .add(egui::Slider::new(&mut self.single_color.alpha, 0..=255).text("A"))
                    .changed();

                let (color_preview, _) =
                    ui.allocate_exact_size(egui::vec2(60.0, 24.0), egui::Sense::hover());
                ui.vertical(|ui| {
                    ui.painter().rect_filled(
                        color_preview,
                        4.0,
                        egui::Color32::from_rgba_premultiplied(
                            self.single_color.red,
                            self.single_color.green,
                            self.single_color.blue,
                            self.single_color.alpha,
                        ),
                    );
                });
                self.mode = ColorMode::Solid;
            }
            ColorMode::Palette => {
                ui.horizontal(|ui| {
                    for options in ColorPalette::iterator() {
                        if ui
                            .radio_value(&mut self.palette, *options, options.as_str())
                            .changed()
                        {};
                    }
                });

                ui.label("Palette mode (not implemented)");
                self.mode = ColorMode::Palette;
            }
            ColorMode::Random => {}
        }

        changed
    }

    // pub fn value(self) -> Vec<Rgba8> {
    //     match self.mode {
    //         ColorMode::Solid => ColorData::Single(self.single_color).as_vec(),
    //         ColorMode::Palette => self.random.as_vec(),
    //         ColorMode::Random => self.random.as_vec(),
    //     }
    // }

    pub fn value_mapped(self, index: usize) -> Rgba8 {
        if self.mode == ColorMode::Solid {
            return self.single_color;
        }

        let other = match self.mode {
            ColorMode::Solid => todo!(),
            ColorMode::Palette => self.random,
            ColorMode::Random => self.palette.as_vec(),
        };

        let mapped_index = clamp(index % other.len(), 0, other.len());
        other[mapped_index]
    }

    fn random() -> Vec<Rgba8> {
        let mut temp: Vec<Rgba8> = Vec::new();
        for _ in 0..8 {
            temp.push(Rgba::new(
                random_range(0, 255),
                random_range(0, 255),
                random_range(0, 255),
                random_range(0, 255),
            ));
        }
        temp
        // ColorData::Array(remp)
    }
}

impl Default for ColorParam {
    fn default() -> Self {
        Self {
            single_color: Rgba8::new(255, 0, 0, 255),
            mode: ColorMode::Solid,
            random: Self::random(),
            palette: ColorPalette::default(),
        }
    }
}
// impl ColorHelpers for Rgba {
//     fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
//         let c = v * s;
//         let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
//         let m = v - c;

//         let (r_prime, g_prime, b_prime) = if (0.0..60.0).contains(&h) {
//             (c, x, 0.0)
//         } else if (60.0..120.0).contains(&h) {
//             (x, c, 0.0)
//         } else if (120.0..180.0).contains(&h) {
//             (0.0, c, x)
//         } else if (180.0..240.0).contains(&h) {
//             (0.0, x, c)
//         } else if (240.0..300.0).contains(&h) {
//             (x, 0.0, c)
//         } else {
//             (c, 0.0, x)
//         };

//         (
//             ((r_prime + m) * 255.0) as u8,
//             ((g_prime + m) * 255.0) as u8,
//             ((b_prime + m) * 255.0) as u8,
//         )
//     }
// }
