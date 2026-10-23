use crate::{color::ColorPalette, modulator::Modulator, parameters::ModulatedParam};
use nannou::color::Rgba8;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
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
    pub fn ui(&mut self, ui: &mut egui::Ui, mods: &[Box<dyn Modulator>]) -> bool {
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
                changed |= self.col_r.slider_ui(ui, mods);
                changed |= self.col_g.slider_ui(ui, mods);
                changed |= self.col_b.slider_ui(ui, mods);
                changed |= self.col_a.slider_ui(ui, mods);
            }

            ColorMode::Palette => {
                egui::ComboBox::from_label("")
                    .selected_text(self.palette.to_string())
                    .show_ui(ui, |ui| {
                        for option in ColorPalette::iter() {
                            changed |= ui
                                .selectable_value(&mut self.palette, option, option.to_string())
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
                self.col_r.value() as u8,
                self.col_g.value() as u8,
                self.col_b.value() as u8,
                self.col_a.value() as u8,
            );
        }

        let palette = self.palette.as_slice();
        palette[index % palette.len()]
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

