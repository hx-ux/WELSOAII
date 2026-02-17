use nannou_egui::egui::{self, FontFamily, FontId, TextStyle};

pub fn apply_custom_style(ctx: &egui::Context, opacity: u8) {
    let mut style = egui::Style::default();

    style.visuals = egui::Visuals::dark();
    style.visuals.window_fill = egui::Color32::from_rgba_premultiplied(12, 14, 18, opacity);
    style.visuals.panel_fill = egui::Color32::from_rgba_premultiplied(12, 14, 18, opacity);
    style.visuals.window_stroke.color = egui::Color32::from_rgb(44, 48, 58);
    style.visuals.window_stroke.width = 1.0;

    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(24, 28, 34);
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(34, 40, 48);
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(40, 46, 56);
    style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(18, 21, 26);
    style.visuals.widgets.open.bg_fill = egui::Color32::from_rgb(30, 35, 43);

    style.visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_gray(185);
    style.visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_gray(235);
    style.visuals.widgets.active.fg_stroke.color = egui::Color32::from_gray(245);
    style.visuals.widgets.open.fg_stroke.color = egui::Color32::from_gray(220);

    style.visuals.selection.bg_fill = egui::Color32::from_rgb(55, 70, 90);
    style.visuals.selection.stroke.color = egui::Color32::from_rgb(220, 230, 255);
    style.visuals.selection.stroke.width = 1.0;

    style.visuals.faint_bg_color = egui::Color32::from_rgb(18, 21, 26);
    style.visuals.extreme_bg_color = egui::Color32::from_rgb(8, 10, 13);

    style.visuals.widgets.active.rounding = egui::Rounding::same(4.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(4.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(4.0);
    style.visuals.widgets.open.rounding = egui::Rounding::same(4.0);
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(4.0);
    style.visuals.window_rounding = egui::Rounding::same(5.0);

    style.spacing.item_spacing = egui::vec2(6.0, 5.0);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);
    style.spacing.indent = 12.0;
    style.spacing.slider_width = 150.0;

    style
        .text_styles
        .insert(TextStyle::Heading, FontId::new(16.0, FontFamily::Monospace));
    style
        .text_styles
        .insert(TextStyle::Body, FontId::new(12.0, FontFamily::Monospace));
    style
        .text_styles
        .insert(TextStyle::Button, FontId::new(11.0, FontFamily::Monospace));
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::new(13.0, FontFamily::Monospace),
    );
    style
        .text_styles
        .insert(TextStyle::Small, FontId::new(9.0, FontFamily::Monospace));

    ctx.set_style(style);
}
