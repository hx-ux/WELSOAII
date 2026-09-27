use crate::{color::ColorPalette, modulator::Modulator, parameters::ModulatedParam};
use nannou::color::Rgba8;
use nannou::math::clamp;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum ColorMode {
    Solid,
    Palette,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ColorParam {
    pub col_r: ModulatedParam,
    pub col_g: ModulatedParam,
    pub col_b: ModulatedParam,
    pub col_a: ModulatedParam,
    pub mode: ColorMode,
    pub palette: ColorPalette,
}

impl Default for ColorParam {
    fn default() -> Self {
        Self {
            col_r: ModulatedParam::new(255.0, 0.0, 255.0, "Red", "red"),
            col_g: ModulatedParam::new(0.0, 0.0, 255.0, "Green", "green"),
            col_b: ModulatedParam::new(0.0, 0.0, 255.0, "Blue", "blue"),
            col_a: ModulatedParam::new(255.0, 0.0, 255.0, "Alpha", "alpha"),
            mode: ColorMode::Solid,
            palette: ColorPalette::default(),
        }
    }
}

impl ColorParam {
    pub fn ui(&mut self, ui: &mut egui::Ui, mods: &mut Vec<Box<dyn Modulator>>) -> bool {
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
                changed |= self.col_r.to_slider(ui, mods);
                changed |= self.col_g.to_slider(ui, mods);
                changed |= self.col_b.to_slider(ui, mods);
                changed |= self.col_a.to_slider(ui, mods);

                // let (color_preview, _) =
                //     ui.allocate_exact_size(egui::vec2(60.0, 24.0), egui::Sense::hover());
                // ui.vertical(|ui| {
                //     ui.painter().rect_filled(
                //         color_preview,
                //         4.0,
                //         egui::Color32::from_rgba_premultiplied(
                //             self.col_r.value() as u8,
                //             self.col_g.base as u8,
                //             self.col_b.base as u8,
                //             self.col_a.base as u8,
                //         ),
                //     );
                // });
            }

            ColorMode::Palette => {
                let _ = egui::ComboBox::from_label("")
                    .selected_text(format!("{}", self.palette).to_string())
                    .show_ui(ui, |ui| {
                        for option in ColorPalette::iter() {
                            changed |= ui
                                .selectable_value(&mut self.palette, option, format!("{}", option))
                                .changed();
                        }
                    });
            }
        }

        changed
    }

    pub fn value_mapped(&self, index: usize) -> Rgba8 {
        if self.mode == ColorMode::Solid {
            return Rgba8::new(
                self.col_r.value().clone() as u8,
                self.col_g.value().clone() as u8,
                self.col_b.value().clone() as u8,
                self.col_a.value().clone() as u8,
            );
        }

        let palette = self.palette.as_slice();
        palette[clamp(index % palette.len(), 0, palette.len() - 1)]
    }

    pub fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
        vec![
            &mut self.col_r,
            &mut self.col_g,
            &mut self.col_b,
            &mut self.col_a,
        ]
    }
}
