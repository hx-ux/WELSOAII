//! Custom styling for Welosa II egui interface
//!
//! This module provides a cohesive dark theme with consistent colors,
//! spacing, and typography across all UI elements.

use nannou_egui::egui::{
    Color32, Context, FontFamily, FontId, Rounding, Style, TextStyle, Vec2, Visuals,
};

/// Color constants for the application theme
mod colors {
    use super::Color32;

    pub const WINDOW_BG: Color32 = Color32::from_rgba_premultiplied(12, 14, 18, 255);
    pub const PANEL_BG: Color32 = Color32::from_rgba_premultiplied(12, 14, 18, 255);
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
mod typography {
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
mod spacing {
    use super::Vec2;

    pub const ITEM_SPACING: Vec2 = Vec2::new(6.0, 5.0);
    pub const BUTTON_PADDING: Vec2 = Vec2::new(8.0, 4.0);
    pub const INDENT: f32 = 12.0;
    pub const SLIDER_WIDTH: f32 = 150.0;
}

/// Rounding constants for visual consistency
mod rounding {
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

/// Applies the custom dark theme to the given egui context
pub fn apply_custom_style(ctx: &Context, opacity: u8) {
    let mut style = Style {
        visuals: create_visuals(opacity),
        ..Default::default()
    };

    apply_text_styles(&mut style);
    apply_spacing(&mut style);

    ctx.set_style(style);
}

fn create_visuals(opacity: u8) -> Visuals {
    let mut visuals = Visuals::dark();

    visuals.window_fill = Color32::from_rgba_premultiplied(12, 14, 18, opacity);
    visuals.panel_fill = Color32::from_rgba_premultiplied(12, 14, 18, opacity);

    visuals.window_stroke.color = colors::WINDOW_BORDER;
    visuals.window_stroke.width = 1.0;
    visuals.window_rounding = rounding::window();

    visuals.widgets.inactive.bg_fill = colors::BUTTON_NORMAL;
    visuals.widgets.hovered.bg_fill = colors::BUTTON_HOVER;
    visuals.widgets.active.bg_fill = colors::BUTTON_ACTIVE;
    visuals.widgets.noninteractive.bg_fill = colors::NONINTERACTIVE_BG;
    visuals.widgets.open.bg_fill = colors::OPEN_BG;

    visuals.widgets.inactive.fg_stroke.color = colors::TEXT_NORMAL;
    visuals.widgets.hovered.fg_stroke.color = colors::TEXT_HOVER;
    visuals.widgets.active.fg_stroke.color = colors::TEXT_ACTIVE;
    visuals.widgets.open.fg_stroke.color = colors::TEXT_OPEN;

    visuals.selection.bg_fill = colors::SELECTION_BG;
    visuals.selection.stroke.color = colors::SELECTION_STROKE;
    visuals.selection.stroke.width = 1.0;

    visuals.faint_bg_color = colors::FILL_BG;
    visuals.extreme_bg_color = colors::EXTREME_BG;

    apply_widget_rounding(&mut visuals);

    visuals
}

fn apply_widget_rounding(visuals: &mut Visuals) {
    visuals.widgets.active.rounding = rounding::active();
    visuals.widgets.inactive.rounding = rounding::inactive();
    visuals.widgets.hovered.rounding = rounding::hovered();
    visuals.widgets.open.rounding = rounding::open();
    visuals.widgets.noninteractive.rounding = rounding::noninteractive();
}

fn apply_text_styles(style: &mut Style) {
    style
        .text_styles
        .insert(TextStyle::Heading, typography::heading());
    style
        .text_styles
        .insert(TextStyle::Body, typography::body());
    style
        .text_styles
        .insert(TextStyle::Button, typography::button());
    style
        .text_styles
        .insert(TextStyle::Monospace, typography::monospace());
    style
        .text_styles
        .insert(TextStyle::Small, typography::small());
}

fn apply_spacing(style: &mut Style) {
    style.spacing.item_spacing = spacing::ITEM_SPACING;
    style.spacing.button_padding = spacing::BUTTON_PADDING;
    style.spacing.indent = spacing::INDENT;
    style.spacing.slider_width = spacing::SLIDER_WIDTH;
}
