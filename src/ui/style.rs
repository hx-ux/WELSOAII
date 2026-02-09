use nannou_egui::egui::{self};

pub fn apply_custom_style(ctx: &egui::Context, opacity: u8) {
    let mut style = egui::Style::default();

    style.visuals = egui::Visuals::dark();
    // widget bg color
    style.visuals.window_fill = egui::Color32::from_rgba_premultiplied(9, 10, 12, opacity);
    // border outline
    style.visuals.window_stroke.color = egui::Color32::from_rgba_premultiplied(29, 32, 38, opacity);
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 40, 60);
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(60, 80, 120);
    style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(20, 25, 35);

    style.visuals.faint_bg_color = egui::Color32::from_rgb(18, 22, 28);
    style.visuals.extreme_bg_color = egui::Color32::from_rgb(10, 12, 16);

    style.visuals.widgets.active.fg_stroke.color = egui::Color32::WHITE;
    style.visuals.widgets.inactive.fg_stroke.color = egui::Color32::GRAY;
    style.visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;

    style.visuals.selection.stroke.color = egui::Color32::WHITE;
    style.visuals.selection.stroke.width = 2.0;
    style.visuals.widgets.open.bg_fill = egui::Color32::from_rgb(50, 70, 100);
    style.visuals.widgets.open.fg_stroke.color = egui::Color32::WHITE;

    // Set rounding and padding
    style.visuals.widgets.active.rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.open.rounding = egui::Rounding::same(6.0);
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(6.0);

    ctx.set_style(style);
}
