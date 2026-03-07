use nannou::color::Rgba8;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum ColorPalette {
    #[strum(to_string = "Breeze")]
    Breeze,
    #[strum(to_string = "Dolphin")]
    Dolphin,
    #[strum(to_string = "Sunset")]
    Sunset,
    #[strum(to_string = "Forest")]
    Forest,
    #[strum(to_string = "Neon")]
    Neon,
    #[strum(to_string = "Ember")]
    Ember,
    #[strum(to_string = "Ice")]
    Ice,
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

    fn sunset_palette() -> Vec<Rgba8> {
        vec![
            Rgba8::new(255, 94, 77, 255),
            Rgba8::new(255, 149, 128, 255),
            Rgba8::new(255, 196, 107, 255),
            Rgba8::new(255, 112, 67, 255),
            Rgba8::new(128, 39, 108, 255),
        ]
    }

    fn forest_palette() -> Vec<Rgba8> {
        vec![
            Rgba8::new(20, 68, 41, 255),
            Rgba8::new(45, 115, 76, 255),
            Rgba8::new(92, 148, 87, 255),
            Rgba8::new(164, 186, 102, 255),
            Rgba8::new(232, 217, 164, 255),
        ]
    }

    fn neon_palette() -> Vec<Rgba8> {
        vec![
            Rgba8::new(57, 255, 20, 255),
            Rgba8::new(0, 245, 255, 255),
            Rgba8::new(255, 0, 204, 255),
            Rgba8::new(255, 251, 0, 255),
            Rgba8::new(255, 84, 0, 255),
        ]
    }

    fn ember_palette() -> Vec<Rgba8> {
        vec![
            Rgba8::new(43, 9, 6, 255),
            Rgba8::new(103, 20, 17, 255),
            Rgba8::new(176, 58, 46, 255),
            Rgba8::new(224, 120, 44, 255),
            Rgba8::new(251, 210, 120, 255),
        ]
    }

    fn ice_palette() -> Vec<Rgba8> {
        vec![
            Rgba8::new(10, 36, 99, 255),
            Rgba8::new(38, 84, 124, 255),
            Rgba8::new(84, 145, 178, 255),
            Rgba8::new(152, 210, 232, 255),
            Rgba8::new(228, 246, 255, 255),
        ]
    }

    pub fn as_vec(&self) -> Vec<Rgba8> {
        match self {
            ColorPalette::Breeze => Self::breeze_palette(),
            ColorPalette::Dolphin => Self::dolphin_palette(),
            ColorPalette::Sunset => Self::sunset_palette(),
            ColorPalette::Forest => Self::forest_palette(),
            ColorPalette::Neon => Self::neon_palette(),
            ColorPalette::Ember => Self::ember_palette(),
            ColorPalette::Ice => Self::ice_palette(),
        }
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        ColorPalette::Breeze
    }
}
