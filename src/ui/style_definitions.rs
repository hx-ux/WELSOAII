/// Color constants for the application theme
use nannou_egui::egui::{Color32, FontFamily, FontId, Rounding, Vec2};
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

/// Typography configuration
pub mod custom_typography {
    use super::{FontFamily, FontId};

    pub const HEADING_SIZE: f32 = 14.0;
    pub const BODY_SIZE: f32 = 12.0;
    pub const BUTTON_SIZE: f32 = 11.0;
    pub const MONOSPACE_SIZE: f32 = 13.0;
    pub const SMALL_SIZE: f32 = 9.0;

    pub fn heading() -> FontId {
        FontId::new(HEADING_SIZE, FontFamily::Monospace)
    }

    pub fn body() -> FontId {
        FontId::new(BODY_SIZE, FontFamily::Monospace)
    }

    pub fn button() -> FontId {
        FontId::new(BUTTON_SIZE, FontFamily::Monospace)
    }

    pub fn monospace() -> FontId {
        FontId::new(MONOSPACE_SIZE, FontFamily::Monospace)
    }
    pub fn small() -> FontId {
        FontId::new(SMALL_SIZE, FontFamily::Monospace)
    }
}

/// Spacing constants for consistent layout
pub mod custom_spacing {
    use super::Vec2;

    pub const ITEM_SPACING: Vec2 = Vec2::new(6.0, 5.0);
    pub const BUTTON_PADDING: Vec2 = Vec2::new(8.0, 4.0);
    pub const INDENT: f32 = 12.0;
    pub const SLIDER_WIDTH: f32 = 150.0;
}

/// Rounding constants for visual consistency
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
