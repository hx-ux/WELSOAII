use std::sync::OnceLock;

use nannou::prelude::Srgba;
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
    #[strum(to_string = "Playa")]
    Playa,
    #[strum(to_string = "Aggro")]
    Aggro,
    #[strum(to_string = "Vaporwave")]
    Vaporwave,
    #[strum(to_string = "Brutalist")]
    Brutalist,
}

impl ColorPalette {
    // Allocate all these pallets on the heap only once
    pub fn as_slice(&self) -> &'static [Srgba] {
        match self {
            ColorPalette::Breeze => {
                static BREEZE: OnceLock<Vec<Srgba>> = OnceLock::new();
                BREEZE.get_or_init(|| {
                    vec![
                        Srgba::rgba_u8(2, 82, 89, 255),
                        Srgba::rgba_u8(0, 112, 114, 255),
                        Srgba::rgba_u8(242, 147, 36, 255),
                        Srgba::rgba_u8(216, 79, 4, 255),
                        Srgba::rgba_u8(244, 226, 221, 255),
                    ]
                })
            }
            ColorPalette::Dolphin => {
                static DOLPHIN: OnceLock<Vec<Srgba>> = OnceLock::new();
                DOLPHIN.get_or_init(|| {
                    vec![
                        Srgba::rgba_u8(242, 121, 222, 255),
                        Srgba::rgba_u8(191, 132, 216, 255),
                        Srgba::rgba_u8(132, 119, 216, 255),
                        Srgba::rgba_u8(181, 179, 242, 255),
                        Srgba::rgba_u8(186, 194, 242, 255),
                    ]
                })
            }
            ColorPalette::Sunset => {
                static SUNSET: OnceLock<Vec<Srgba>> = OnceLock::new();
                SUNSET.get_or_init(|| {
                    vec![
                        Srgba::rgba_u8(255, 94, 77, 255),
                        Srgba::rgba_u8(255, 149, 128, 255),
                        Srgba::rgba_u8(255, 196, 107, 255),
                        Srgba::rgba_u8(255, 112, 67, 255),
                        Srgba::rgba_u8(128, 39, 108, 255),
                    ]
                })
            }
            ColorPalette::Forest => {
                static FOREST: OnceLock<Vec<Srgba>> = OnceLock::new();
                FOREST.get_or_init(|| {
                    vec![
                        Srgba::rgba_u8(20, 68, 41, 255),
                        Srgba::rgba_u8(45, 115, 76, 255),
                        Srgba::rgba_u8(92, 148, 87, 255),
                        Srgba::rgba_u8(164, 186, 102, 255),
                        Srgba::rgba_u8(232, 217, 164, 255),
                    ]
                })
            }
            ColorPalette::Neon => {
                static NEON: OnceLock<Vec<Srgba>> = OnceLock::new();
                NEON.get_or_init(|| {
                    vec![
                        Srgba::rgba_u8(57, 255, 20, 255),
                        Srgba::rgba_u8(0, 245, 255, 255),
                        Srgba::rgba_u8(255, 0, 204, 255),
                        Srgba::rgba_u8(255, 251, 0, 255),
                        Srgba::rgba_u8(255, 84, 0, 255),
                    ]
                })
            }
            ColorPalette::Ember => {
                static EMBER: OnceLock<Vec<Srgba>> = OnceLock::new();
                EMBER.get_or_init(|| {
                    vec![
                        Srgba::rgba_u8(43, 9, 6, 255),
                        Srgba::rgba_u8(103, 20, 17, 255),
                        Srgba::rgba_u8(176, 58, 46, 255),
                        Srgba::rgba_u8(224, 120, 44, 255),
                        Srgba::rgba_u8(251, 210, 120, 255),
                    ]
                })
            }
            ColorPalette::Ice => {
                static ICE: OnceLock<Vec<Srgba>> = OnceLock::new();
                ICE.get_or_init(|| {
                    vec![
                        Srgba::rgba_u8(10, 36, 99, 255),
                        Srgba::rgba_u8(38, 84, 124, 255),
                        Srgba::rgba_u8(84, 145, 178, 255),
                        Srgba::rgba_u8(152, 210, 232, 255),
                        Srgba::rgba_u8(228, 246, 255, 255),
                    ]
                })
            }
            PastelHorizon => {
                static PASTEL_HORIZON: OnceLock<Vec<Srgba>> = OnceLock::new();
                PASTEL_HORIZON.get_or_init(|| {
                    vec![
                        Srgba::rgba_u8(47, 37, 86, 255),
                        Srgba::rgba_u8(88, 76, 132, 255),
                        Srgba::rgba_u8(130, 124, 175, 255),
                        Srgba::rgba_u8(174, 171, 211, 255),
                        Srgba::rgba_u8(206, 207, 240, 255),
                        Srgba::rgba_u8(255, 255, 255, 255),
                        Srgba::rgba_u8(201, 255, 191, 255),
                        Srgba::rgba_u8(120, 216, 183, 255),
                        Srgba::rgba_u8(52, 173, 161, 255),
                        Srgba::rgba_u8(37, 125, 155, 255),
                        Srgba::rgba_u8(37, 188, 198, 255),
                        Srgba::rgba_u8(160, 255, 240, 255),
                        Srgba::rgba_u8(132, 165, 148, 255),
                        Srgba::rgba_u8(207, 211, 175, 255),
                        Srgba::rgba_u8(255, 242, 209, 255),
                        Srgba::rgba_u8(255, 217, 174, 255),
                        Srgba::rgba_u8(255, 165, 130, 255),
                        Srgba::rgba_u8(255, 128, 126, 255),
                        Srgba::rgba_u8(255, 84, 112, 255),
                        Srgba::rgba_u8(152, 88, 104, 255),
                        Srgba::rgba_u8(183, 132, 130, 255),
                        Srgba::rgba_u8(221, 164, 159, 255),
                        Srgba::rgba_u8(224, 189, 193, 255),
                        Srgba::rgba_u8(119, 137, 239, 255),
                        Srgba::rgba_u8(94, 193, 255, 255),
                        Srgba::rgba_u8(186, 230, 255, 255),
                        Srgba::rgba_u8(226, 211, 255, 255),
                        Srgba::rgba_u8(190, 155, 255, 255),
                        Srgba::rgba_u8(153, 104, 226, 255),
                        Srgba::rgba_u8(255, 155, 182, 255),
                        Srgba::rgba_u8(255, 204, 220, 255),
                        Srgba::rgba_u8(255, 234, 246, 255),
                    ]
                })
            }
            ColorPalette::Playa => {
                static PLAYA: OnceLock<Vec<Srgba>> = OnceLock::new();
                PLAYA.get_or_init(|| {
                    vec![
                        Srgba::rgba_u8(255, 0, 128, 255),  // Acid Magenta
                        Srgba::rgba_u8(255, 102, 0, 255),  // Burning Flame Orange
                        Srgba::rgba_u8(255, 230, 0, 255),  // Solar Gold
                        Srgba::rgba_u8(0, 255, 180, 255),  // Bioluminescent Mint
                        Srgba::rgba_u8(0, 210, 255, 255),  // Laser Cyan
                        Srgba::rgba_u8(138, 43, 255, 255), // Ultraviolet
                        Srgba::rgba_u8(255, 42, 255, 255), // Electric Fuchsia
                        Srgba::rgba_u8(180, 255, 0, 255),  // Psychedelic Lime
                    ]
                })
            }
            ColorPalette::Aggro => {
                static AGGRO: OnceLock<Vec<Srgba>> = OnceLock::new();
                AGGRO.get_or_init(|| {
                    vec![
                        Srgba::rgba_u8(255, 0, 34, 255),    // Laser Crimson
                        Srgba::rgba_u8(255, 56, 0, 255),    // Riot Flame Orange
                        Srgba::rgba_u8(255, 238, 0, 255),   // Hazard Strobe Yellow
                        Srgba::rgba_u8(150, 0, 18, 255),    // Arterial Blood Red
                        Srgba::rgba_u8(255, 0, 96, 255),    // Violator Magenta
                        Srgba::rgba_u8(196, 255, 0, 255),   // Venom Acid Chartreuse
                        Srgba::rgba_u8(255, 245, 245, 255), // White-Hot Arc
                    ]
                })
            }
            ColorPalette::Vaporwave => {
                static VAPORWAVE: OnceLock<Vec<Srgba>> = OnceLock::new();
                VAPORWAVE.get_or_init(|| {
                    vec![
                        Srgba::rgba_u8(255, 113, 206, 255), // Neon Bubblegum Pink
                        Srgba::rgba_u8(1, 205, 254, 255),   // Cyber Mall Cyan
                        Srgba::rgba_u8(5, 255, 161, 255),   // Seafoam VHS Mint
                        Srgba::rgba_u8(185, 103, 255, 255), // Heliotrope Synth Purple
                        Srgba::rgba_u8(255, 251, 150, 255), // Sunset Grid Cream-Yellow
                        Srgba::rgba_u8(255, 108, 149, 255), // Outrun Coral Rose
                        Srgba::rgba_u8(98, 70, 234, 255),   // CRT Twilight Indigo
                    ]
                })
            }

            ColorPalette::Brutalist => {
                static BRUTALIST: OnceLock<Vec<Srgba>> = OnceLock::new();
                BRUTALIST.get_or_init(|| {
                    vec![
                        Srgba::rgba_u8(235, 235, 230, 255), // Stark Chalk Concrete
                        Srgba::rgba_u8(148, 152, 156, 255), // Raw Béton Brut Slab
                        Srgba::rgba_u8(68, 72, 76, 255),    // Cast Iron Charcoal
                        Srgba::rgba_u8(255, 69, 0, 255),    // International Safety Orange
                        Srgba::rgba_u8(212, 208, 196, 255), // Weathered Limestone
                        Srgba::rgba_u8(255, 212, 0, 255),   // Industrial Warning Yellow
                        Srgba::rgba_u8(0, 87, 255, 255),    // Structural Blueprint Cobalt
                    ]
                })
            }
        }
    }
}
