use nannou::color::Rgba8;
use nannou::math::clamp;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
// use std::slice::Iter;
use strum_macros::{Display, EnumIter};
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum ColorPalette {
    #[strum(to_string = "Breeze")]
    Breeze,
    #[strum(to_string = "Dolphin")]
    Dolphin,
}

impl ColorPalette {
    fn breeze_palette() -> Vec<Rgba8> {
        vec![
            Rgba8::new(2, 82, 89, 255),
            Rgba8::new(0, 112, 114, 255),
            Rgba8::new(242, 147, 36, 255),
            Rgba8::new(216, 79, 4, 255),
            Rgba8::new(244, 226, 221, 255),
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
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ColorParam {
    pub single_color: Rgba8,
    pub mode: ColorMode,
    pub palette: ColorPalette,
}

impl ColorParam {
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
                let _ = egui::ComboBox::from_label("Palette")
                    .selected_text(format!("{}", format!("{}", self.palette.clone())))
                    .show_ui(ui, |ui| {
                        for option in ColorPalette::iter() {
                            if ui
                                .selectable_value(&mut self.palette, option, format!(""))
                                .clicked()
                            {}
                        }
                    });

                ui.label("Palette mode (not implemented)");
                self.mode = ColorMode::Palette;
            }
        }

        changed
    }

    pub fn value_mapped(self, index: usize) -> Rgba8 {
        if self.mode == ColorMode::Solid {
            return self.single_color;
        }

        let other = match self.mode {
            ColorMode::Solid => todo!(),
            ColorMode::Palette => self.palette.as_vec(),
        };

        let mapped_index = clamp(index % other.len(), 0, other.len());
        other[mapped_index]
    }
}

impl Default for ColorParam {
    fn default() -> Self {
        Self {
            single_color: Rgba8::new(255, 0, 0, 255),
            mode: ColorMode::Solid,
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
