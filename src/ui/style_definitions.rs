/// Color constants for the application theme
use nannou_egui::egui::{Color32, Rounding};
pub mod custom_colors {
    use super::Color32;

    pub const FILL_BG: Color32 = Color32::from_rgb(18, 21, 26);
    pub const EXTREME_BG: Color32 = Color32::from_rgb(8, 10, 13);
    pub const BUTTON_NORMAL: Color32 = Color32::from_rgb(24, 28, 34);
    pub const BUTTON_HOVER: Color32 = Color32::from_rgb(34, 40, 48);
    pub const BUTTON_ACTIVE: Color32 = Color32::from_rgb(40, 46, 56);
    pub const NONINTERACTIVE_BG: Color32 = Color32::from_rgb(18, 21, 26);
    pub const OPEN_BG: Color32 = Color32::from_rgb(30, 35, 43);
    pub const WINDOW_BORDER: Color32 = Color32::from_rgb(44, 48, 58);
    pub const TEXT_NORMAL: Color32 = Color32::from_gray(185);
    pub const TEXT_HOVER: Color32 = Color32::from_gray(235);
    pub const TEXT_ACTIVE: Color32 = Color32::from_gray(245);
    pub const TEXT_OPEN: Color32 = Color32::from_gray(220);
    pub const SELECTION_BG: Color32 = Color32::from_rgb(55, 70, 90);
    pub const SELECTION_STROKE: Color32 = Color32::from_rgb(220, 230, 255);
}

pub mod custom_rounding {
    use super::Rounding;

    pub fn active() -> Rounding {
        Rounding::same(4.0)
    }
    pub fn inactive() -> Rounding {
        Rounding::same(4.0)
    }
    pub fn hovered() -> Rounding {
        Rounding::same(4.0)
    }
    pub fn open() -> Rounding {
        Rounding::same(4.0)
    }
    pub fn noninteractive() -> Rounding {
        Rounding::same(4.0)
    }
    pub fn window() -> Rounding {
        Rounding::same(5.0)
    }
}
