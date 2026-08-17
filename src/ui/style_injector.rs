//! Custom styling for Welosa II egui interface
//!
//! This module provides a cohesive dark theme with consistent colors,
//! spacing, and typography across all UI elements.
use nannou_egui::egui::{Color32, Context, Style, TextStyle, Visuals};

use crate::ui::style_definitions::{
    custom_colors, custom_rounding, custom_spacing, custom_typography,
};

/// Applies the custom dark theme to the given egui context
pub fn apply_custom_style(ctx: &Context, opacity: u8) {
    let mut style = Style {
        visuals: setup_visuals(opacity),
        ..Default::default()
    };

    inject_text_styles(&mut style);
    inject_spacing(&mut style);

    ctx.set_style(style);
}

// inject style into controls
fn setup_visuals(opacity: u8) -> Visuals {
    let mut visuals = Visuals::dark();

    visuals.window_fill = Color32::from_rgba_premultiplied(12, 14, 18, opacity);
    visuals.panel_fill = Color32::from_rgba_premultiplied(12, 14, 18, opacity);

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

    inject_widget_rounding(&mut visuals);

    visuals
}

fn inject_widget_rounding(visuals: &mut Visuals) {
    visuals.widgets.active.rounding = custom_rounding::active();
    visuals.widgets.inactive.rounding = custom_rounding::inactive();
    visuals.widgets.hovered.rounding = custom_rounding::hovered();
    visuals.widgets.open.rounding = custom_rounding::open();
    visuals.widgets.noninteractive.rounding = custom_rounding::noninteractive();
}

fn inject_text_styles(style: &mut Style) {
    style
        .text_styles
        .insert(TextStyle::Heading, custom_typography::heading());
    style
        .text_styles
        .insert(TextStyle::Body, custom_typography::body());
    style
        .text_styles
        .insert(TextStyle::Button, custom_typography::button());
    style
        .text_styles
        .insert(TextStyle::Monospace, custom_typography::monospace());
    style
        .text_styles
        .insert(TextStyle::Small, custom_typography::small());
}

fn inject_spacing(style: &mut Style) {
    style.spacing.item_spacing = custom_spacing::ITEM_SPACING;
    style.spacing.button_padding = custom_spacing::BUTTON_PADDING;
    style.spacing.indent = custom_spacing::INDENT;
    style.spacing.slider_width = custom_spacing::SLIDER_WIDTH;
}
