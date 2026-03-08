//! Common styled controls for egui UI

use nannou_egui::egui;
use std::ops::RangeInclusive;

/// Styled single-line text edit with monospace font and code editor look
pub fn styled_text_edit<'a>(text: &'a mut String, hint: &'a str) -> egui::TextEdit<'a> {
    egui::TextEdit::singleline(text)
        .hint_text(hint)
        .font(egui::TextStyle::Monospace)
        .desired_width(120.0)
        .code_editor()
}

/// Styled slider with consistent look for egui UI
pub fn single_slider_styled<'a, T>(
    value: &'a mut T,
    range: RangeInclusive<T>,
    text: &'a str,
) -> egui::Slider<'a>
where
    T: egui::emath::Numeric + Copy,
{
    egui::Slider::new(value, range)
        .text(text)
        .show_value(true)
        .smart_aim(true)
        .trailing_fill(true)
        .smallest_positive(0.01)
        .trailing_fill(true)
}
pub struct DualSlider<'a> {
    value: &'a mut f32,
    ghost_value: Option<f32>,
    range: std::ops::RangeInclusive<f32>,
    text: &'a str,
}

impl<'a> egui::Widget for DualSlider<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let sense = egui::Sense::drag();
        let height = 18.0;
        let (rect, mut response) =
            ui.allocate_at_least(egui::vec2(ui.available_width(), height), sense);
        response = response.on_hover_text(self.text);

        let min = *self.range.start();
        let max = *self.range.end();
        let range_size = max - min;
        let mut value_frac = if range_size != 0.0 {
            (*self.value - min) / range_size
        } else {
            0.0
        };
        value_frac = value_frac.clamp(0.0, 1.0);

        if response.dragged() {
            let delta = ui.input(|i| i.pointer.delta().x / rect.width());
            value_frac += delta;
            value_frac = value_frac.clamp(0.0, 1.0);
            *self.value = min + value_frac * range_size;
            response.changed = true;
        }

        let painter = ui.painter();
        let rounding = egui::Rounding::same(9.0);

        // Background track
        painter.rect_filled(rect, rounding, egui::Color32::from_gray(50));

        // Ghost fill (readonly background)
        if let Some(ghost_val) = self.ghost_value {
            let ghost_frac = if range_size != 0.0 {
                (ghost_val - min) / range_size
            } else {
                0.0
            };
            let ghost_frac = ghost_frac.clamp(0.0, 1.0);
            let ghost_fill_rect = egui::Rect::from_min_size(
                rect.left_top(),
                egui::vec2(ghost_frac * rect.width(), rect.height()),
            );
            painter.rect_filled(
                ghost_fill_rect,
                rounding,
                egui::Color32::LIGHT_BLUE.gamma_multiply(0.5),
            );
        }

        // Value fill (interactive foreground)
        let value_fill_rect = egui::Rect::from_min_size(
            rect.left_top(),
            egui::vec2(value_frac * rect.width(), rect.height()),
        );
        painter.rect_filled(value_fill_rect, rounding, egui::Color32::WHITE);

        // Knob (interactive)
        let knob_x = rect.left() + value_frac * rect.width();
        let knob_center = egui::pos2(knob_x, rect.center().y);
        let knob_radius = 8.0;
        painter.circle_filled(knob_center, knob_radius, egui::Color32::WHITE);
        painter.circle_stroke(
            knob_center,
            knob_radius,
            egui::Stroke::new(1.5, egui::Color32::BLACK),
        );

        // Value label
        let value_str = format!("{:.2}", *self.value);
        painter.text(
            rect.right_center() + egui::vec2(-50.0, 0.0),
            egui::Align2::RIGHT_CENTER,
            &value_str,
            egui::FontId::proportional(11.0),
            egui::Color32::WHITE,
        );

        response
    }
}

pub fn styled_dual_slider<'a>(
    value: &'a mut f32,
    ghost_value: Option<f32>,
    range: std::ops::RangeInclusive<f32>,
    text: &'a str,
) -> DualSlider<'a> {
    DualSlider {
        value,
        ghost_value,
        range,
        text,
    }
}
