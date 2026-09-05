use std::sync::OnceLock;

use nannou::color::Rgba8;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

use crate::color::ColorPalette::PastelHorizon;
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, EnumIter, Display, Default)]
pub enum ColorPalette {
    #[strum(to_string = "Breeze")]
    #[default]
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
    #[strum(to_string = "Horizon")]
    PastelHorizon,
}

impl ColorPalette {
    // Allocate all these pallets on the heap only once
    pub fn as_slice(&self) -> &'static [Rgba8] {
        match self {
            ColorPalette::Breeze => {
                static BREEZE: OnceLock<Vec<Rgba8>> = OnceLock::new();
                BREEZE.get_or_init(|| {
                    vec![
                        Rgba8::new(2, 82, 89, 255),
                        Rgba8::new(0, 112, 114, 255),
                        Rgba8::new(242, 147, 36, 255),
                        Rgba8::new(216, 79, 4, 255),
                        Rgba8::new(244, 226, 221, 255),
                    ]
                })
            }
            ColorPalette::Dolphin => {
                static DOLPHIN: OnceLock<Vec<Rgba8>> = OnceLock::new();
                DOLPHIN.get_or_init(|| {
                    vec![
                        Rgba8::new(242, 121, 222, 255),
                        Rgba8::new(191, 132, 216, 255),
                        Rgba8::new(132, 119, 216, 255),
                        Rgba8::new(181, 179, 242, 255),
                        Rgba8::new(186, 194, 242, 255),
                    ]
                })
            }
            ColorPalette::Sunset => {
                static SUNSET: OnceLock<Vec<Rgba8>> = OnceLock::new();
                SUNSET.get_or_init(|| {
                    vec![
                        Rgba8::new(255, 94, 77, 255),
                        Rgba8::new(255, 149, 128, 255),
                        Rgba8::new(255, 196, 107, 255),
                        Rgba8::new(255, 112, 67, 255),
                        Rgba8::new(128, 39, 108, 255),
                    ]
                })
            }
            ColorPalette::Forest => {
                static FOREST: OnceLock<Vec<Rgba8>> = OnceLock::new();
                FOREST.get_or_init(|| {
                    vec![
                        Rgba8::new(20, 68, 41, 255),
                        Rgba8::new(45, 115, 76, 255),
                        Rgba8::new(92, 148, 87, 255),
                        Rgba8::new(164, 186, 102, 255),
                        Rgba8::new(232, 217, 164, 255),
                    ]
                })
            }
            ColorPalette::Neon => {
                static NEON: OnceLock<Vec<Rgba8>> = OnceLock::new();
                NEON.get_or_init(|| {
                    vec![
                        Rgba8::new(57, 255, 20, 255),
                        Rgba8::new(0, 245, 255, 255),
                        Rgba8::new(255, 0, 204, 255),
                        Rgba8::new(255, 251, 0, 255),
                        Rgba8::new(255, 84, 0, 255),
                    ]
                })
            }
            ColorPalette::Ember => {
                static EMBER: OnceLock<Vec<Rgba8>> = OnceLock::new();
                EMBER.get_or_init(|| {
                    vec![
                        Rgba8::new(43, 9, 6, 255),
                        Rgba8::new(103, 20, 17, 255),
                        Rgba8::new(176, 58, 46, 255),
                        Rgba8::new(224, 120, 44, 255),
                        Rgba8::new(251, 210, 120, 255),
                    ]
                })
            }
            ColorPalette::Ice => {
                static ICE: OnceLock<Vec<Rgba8>> = OnceLock::new();
                ICE.get_or_init(|| {
                    vec![
                        Rgba8::new(10, 36, 99, 255),
                        Rgba8::new(38, 84, 124, 255),
                        Rgba8::new(84, 145, 178, 255),
                        Rgba8::new(152, 210, 232, 255),
                        Rgba8::new(228, 246, 255, 255),
                    ]
                })
            }
            PastelHorizon => {
                static PASTEL_HORIZON: OnceLock<Vec<Rgba8>> = OnceLock::new();
                PASTEL_HORIZON.get_or_init(|| {
                    vec![
                        Rgba8::new(47, 37, 86, 255),
                        Rgba8::new(88, 76, 132, 255),
                        Rgba8::new(130, 124, 175, 255),
                        Rgba8::new(174, 171, 211, 255),
                        Rgba8::new(206, 207, 240, 255),
                        Rgba8::new(255, 255, 255, 255),
                        Rgba8::new(201, 255, 191, 255),
                        Rgba8::new(120, 216, 183, 255),
                        Rgba8::new(52, 173, 161, 255),
                        Rgba8::new(37, 125, 155, 255),
                        Rgba8::new(37, 188, 198, 255),
                        Rgba8::new(160, 255, 240, 255),
                        Rgba8::new(132, 165, 148, 255),
                        Rgba8::new(207, 211, 175, 255),
                        Rgba8::new(255, 242, 209, 255),
                        Rgba8::new(255, 217, 174, 255),
                        Rgba8::new(255, 165, 130, 255),
                        Rgba8::new(255, 128, 126, 255),
                        Rgba8::new(255, 84, 112, 255),
                        Rgba8::new(152, 88, 104, 255),
                        Rgba8::new(183, 132, 130, 255),
                        Rgba8::new(221, 164, 159, 255),
                        Rgba8::new(224, 189, 193, 255),
                        Rgba8::new(119, 137, 239, 255),
                        Rgba8::new(94, 193, 255, 255),
                        Rgba8::new(186, 230, 255, 255),
                        Rgba8::new(226, 211, 255, 255),
                        Rgba8::new(190, 155, 255, 255),
                        Rgba8::new(153, 104, 226, 255),
                        Rgba8::new(255, 155, 182, 255),
                        Rgba8::new(255, 204, 220, 255),
                        Rgba8::new(255, 234, 246, 255),
                    ]
                })
            }
        }
    }
}
