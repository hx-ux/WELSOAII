//! Custom styling for Welosa II egui interface
//!
//! This module provides a cohesive dark theme with consistent colors,
//! spacing, and typography across all UI elements.
use nannou::glam::bool;
use nannou_egui::egui::{Color32, Context, FontData, FontDefinitions, FontFamily, Style, Visuals};

use crate::ui::style_definitions::{custom_colors, custom_rounding};

/// Applies the custom dark theme to the given egui context
pub fn apply_custom_style(ctx: &Context, opacity: u8, transparent: bool) {
    let style = Style {
        visuals: setup_visuals(opacity, transparent),
        ..Default::default()
    };

    setup_fonts(ctx);
    ctx.set_style(style);
}

fn setup_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "sans".to_owned(),
        FontData::from_static(include_bytes!("../assets/GoogleSans.ttf")),
    );

    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "sans".to_owned());

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "sans".to_owned());

    ctx.set_fonts(fonts);
}

fn setup_visuals(opacity: u8, transparent: bool) -> Visuals {
    let mut visuals = Visuals::dark();

    if (transparent) {
        visuals.window_fill = Color32::TRANSPARENT;
        visuals.panel_fill = Color32::TRANSPARENT;
    } else {
        visuals.window_fill = Color32::from_rgba_premultiplied(12, 14, 18, opacity);
        visuals.panel_fill = Color32::from_rgba_premultiplied(12, 14, 18, opacity);
    }

    visuals.window_stroke.color = custom_colors::WINDOW_BORDER;
    visuals.window_stroke.width = 1.0;
    visuals.window_rounding = custom_rounding::window();

    // Widget fill states
    visuals.widgets.inactive.bg_fill = custom_colors::BUTTON_NORMAL;
    visuals.widgets.hovered.bg_fill = custom_colors::BUTTON_HOVER;
    visuals.widgets.active.bg_fill = custom_colors::BUTTON_ACTIVE;
    visuals.widgets.noninteractive.bg_fill = custom_colors::NONINTERACTIVE_BG;
    visuals.widgets.open.bg_fill = custom_colors::OPEN_BG;

    // Text / stroke colors
    visuals.widgets.inactive.fg_stroke.color = custom_colors::TEXT_NORMAL;
    visuals.widgets.inactive.fg_stroke.width = 0.8;
    visuals.widgets.hovered.fg_stroke.color = custom_colors::TEXT_HOVER;
    visuals.widgets.hovered.fg_stroke.width = 1.0;
    visuals.widgets.active.fg_stroke.color = custom_colors::TEXT_ACTIVE;
    visuals.widgets.active.fg_stroke.width = 1.0;
    visuals.widgets.open.fg_stroke.color = custom_colors::TEXT_OPEN;
    visuals.widgets.noninteractive.fg_stroke.color = custom_colors::TEXT_DIM;
    visuals.widgets.noninteractive.fg_stroke.width = 0.5;

    // Widget bg stroke (border outline)
    visuals.widgets.inactive.bg_stroke.color = Color32::from_rgb(65, 65, 65);
    visuals.widgets.inactive.bg_stroke.width = 0.5;
    visuals.widgets.hovered.bg_stroke.color = custom_colors::ACCENT;
    visuals.widgets.hovered.bg_stroke.width = 1.0;
    visuals.widgets.active.bg_stroke.color = custom_colors::ACCENT;
    visuals.widgets.active.bg_stroke.width = 1.0;

    visuals.selection.bg_fill = custom_colors::SELECTION_BG;
    visuals.selection.stroke.color = custom_colors::SELECTION_STROKE;
    visuals.selection.stroke.width = 1.0;

    visuals.faint_bg_color = custom_colors::FILL_BG;
    visuals.extreme_bg_color = custom_colors::EXTREME_BG;

    setup_widget_rounding(&mut visuals);

    visuals
}

fn setup_widget_rounding(visuals: &mut Visuals) {
    visuals.widgets.active.rounding = custom_rounding::active();
    visuals.widgets.inactive.rounding = custom_rounding::inactive();
    visuals.widgets.hovered.rounding = custom_rounding::hovered();
    visuals.widgets.open.rounding = custom_rounding::open();
    visuals.widgets.noninteractive.rounding = custom_rounding::noninteractive();
}
