use nannou_egui::egui::{self, vec2};
use std::ops::RangeInclusive;

use crate::ui::style_definitions::custom_colors;

/// Styled single-line text edit with monospace font and code editor look
pub fn monospace_text_edit<'a>(text: &'a mut String, hint: &'a str) -> egui::TextEdit<'a> {
    egui::TextEdit::singleline(text)
        .hint_text(hint)
        .font(egui::TextStyle::Monospace)
        .desired_width(120.0)
        .code_editor()
}

/// Styled slider with consistent look for egui UI
pub fn single_slider_styled<'a, T>(value: &'a mut T, range: RangeInclusive<T>) -> egui::Slider<'a>
where
    T: egui::emath::Numeric + Copy,
{
    egui::Slider::new(value, range)
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
        let base_cords = vec2(ui.spacing().slider_width, 12.0);
        let mut response = ui.allocate_response(base_cords, egui::Sense::drag());
        let base_rect = response.rect;

        let min = *self.range.start();
        let max = *self.range.end();
        let range_size = max - min;
        let mut value_frac = if range_size != 0.0 {
            (*self.value - min) / range_size
        } else {
            0.0
        };
        value_frac = value_frac.clamp(0.0, 1.0);

        // Drag logic
        let padding = 6.0;
        let track_rect = base_rect.shrink(padding);

        if response.dragged() {
            let delta = ui.input(|i| i.pointer.delta().x / track_rect.width());
            value_frac += delta;
            value_frac = value_frac.clamp(0.0, 1.0);
            *self.value = min + value_frac * range_size;
            response.changed = true;
        }

        let painter = ui.painter();
        let rounding = egui::Rounding::ZERO;

        // ── Background track ──────────────────────────────────────────────
        painter.rect_filled(base_rect, rounding, custom_colors::SLIDER_TRACK_BG);

        if let Some(ghost_val) = self.ghost_value {
            // Ghost fill (modulated value) — semi-transparent white overlay
            let ghost_frac = if range_size != 0.0 {
                (ghost_val - min) / range_size
            } else {
                0.0
            };
            let ghost_frac = ghost_frac.clamp(0.0, 1.0);
            let ghost_fill_rect = egui::Rect::from_min_size(
                base_rect.left_top(),
                egui::vec2(ghost_frac * base_rect.width(), base_rect.height()),
            );
            // Draw base orange fill behind ghost
            let base_fill_rect = egui::Rect::from_min_size(
                base_rect.left_top(),
                egui::vec2(value_frac * base_rect.width(), base_rect.height()),
            );
            painter.rect_filled(base_fill_rect, rounding, custom_colors::ACCENT_DIM);
            painter.rect_filled(ghost_fill_rect, rounding, custom_colors::SLIDER_GHOST_FILL);
        } else {
            // Value fill (foreground)
            let value_fill_rect = egui::Rect::from_min_size(
                base_rect.left_top(),
                egui::vec2(value_frac * base_rect.width(), base_rect.height()),
            );
            painter.rect_filled(value_fill_rect, rounding, custom_colors::SLIDER_FILL);
        }

        // ── Value — always visible ────────────────────────────────────────
        let text = format!("{:.2}", *self.value);
        painter.text(
            base_rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::new(9.0, egui::FontFamily::Monospace),
            egui::Color32::from_rgb(255, 255, 255),
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
