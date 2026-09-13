use nannou_egui::egui::{Color32, Rounding};

pub mod custom_colors {
    use super::Color32;

    // Base surfaces
    pub const FILL_BG: Color32 = Color32::from_rgb(26, 26, 26);
    pub const EXTREME_BG: Color32 = Color32::from_rgb(10, 10, 10);

    // Widget states — charcoal-based
    pub const BUTTON_NORMAL: Color32 = Color32::from_rgb(50, 50, 50);
    pub const BUTTON_HOVER: Color32 = Color32::from_rgb(68, 68, 68);
    pub const BUTTON_ACTIVE: Color32 = Color32::from_rgb(255, 102, 0); //
    //
    // Ableton orange
    pub const NONINTERACTIVE_BG: Color32 = Color32::from_rgb(30, 30, 30);
    pub const OPEN_BG: Color32 = Color32::from_rgb(55, 55, 55);
    pub const WINDOW_BORDER: Color32 = Color32::from_rgb(55, 55, 55);

    // Accent
    pub const ACCENT: Color32 = Color32::from_rgb(255, 102, 0);
    pub const ACCENT_DIM: Color32 = Color32::from_rgb(160, 64, 0);

    // Text
    pub const TEXT_NORMAL: Color32 = Color32::from_gray(170);
    pub const TEXT_HOVER: Color32 = Color32::from_gray(220);
    pub const TEXT_ACTIVE: Color32 = Color32::from_gray(255);
    pub const TEXT_OPEN: Color32 = Color32::from_gray(200);
    pub const TEXT_DIM: Color32 = Color32::from_gray(100);

    // Selection
    pub const SELECTION_BG: Color32 = Color32::from_rgb(100, 40, 0);
    pub const SELECTION_STROKE: Color32 = Color32::from_rgb(255, 102, 0);

    // Slider track
    pub const SLIDER_TRACK_BG: Color32 = Color32::from_rgb(38, 38, 38);
    pub const SLIDER_FILL: Color32 = Color32::from_rgb(255, 102, 0);
    pub const SLIDER_GHOST_FILL: Color32 = Color32::from_rgba_premultiplied(200, 200, 200, 100);
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
