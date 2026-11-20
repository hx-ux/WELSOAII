use nannou::prelude::egui::{Color32, CornerRadius};

pub mod custom_colors {
    use super::Color32;

    #[allow(dead_code)]
    pub const SURFACE: Color32 = Color32::from_rgb(46, 46, 56);

    // Widget states — charcoal-based
    pub const BUTTON_NORMAL: Color32 = Color32::from_rgb(50, 50, 50);
    pub const BUTTON_HOVER: Color32 = Color32::from_rgb(68, 68, 68);
    pub const BUTTON_ACTIVE: Color32 = Color32::from_rgb(255, 102, 0); //
    //
    // Ableton orange
    pub const NONINTERACTIVE_BG: Color32 = Color32::from_rgb(30, 30, 30);
    pub const OPEN_BG: Color32 = Color32::from_rgb(55, 55, 55);

    // Accent
    pub const ACCENT: Color32 = Color32::from_rgb(255, 102, 0);

    // Text
    pub const TEXT_NORMAL: Color32 = Color32::from_gray(170);
    pub const TEXT_HOVER: Color32 = Color32::from_gray(220);
    pub const TEXT_ACTIVE: Color32 = Color32::from_gray(255);
    pub const TEXT_OPEN: Color32 = Color32::from_gray(200);
    pub const TEXT_DIM: Color32 = Color32::from_gray(100);

    // Selection
    pub const SELECTION_BG: Color32 = Color32::from_rgb(100, 40, 0);
    pub const SELECTION_STROKE: Color32 = Color32::from_rgb(255, 102, 0);

    pub const SLIDER_GHOST_FILL: Color32 = Color32::from_rgba_unmultiplied_const(200, 200, 200, 100);
}

pub mod custom_rounding {
    use super::CornerRadius;

    pub fn active() -> CornerRadius {
        CornerRadius::same(4)
    }
    pub fn inactive() -> CornerRadius {
        CornerRadius::same(4)
    }
    pub fn hovered() -> CornerRadius {
        CornerRadius::same(4)
    }
    pub fn open() -> CornerRadius {
        CornerRadius::same(4)
    }
    pub fn noninteractive() -> CornerRadius {
        CornerRadius::same(4)
    }
    pub fn window() -> CornerRadius {
        CornerRadius::same(5)
    }
}
