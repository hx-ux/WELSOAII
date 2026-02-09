use nannou::color::Rgba8;
use strum_macros::{Display, EnumIter};
use serde::{Deserialize, Serialize};
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
