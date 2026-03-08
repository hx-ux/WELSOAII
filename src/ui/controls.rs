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
pub struct DualSlider<'a, T> {
    value: &'a mut T,
    ghost_value: Option<T>,
    range: std::ops::RangeInclusive<T>,
    text: &'a str,
}

impl<'a, T> egui::Widget for DualSlider<'a, T>
where
    T: egui::emath::Numeric + Copy,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add_space(4.0);

        // Top: interactive value slider (your styled one)
        let value_slider = single_slider_styled(self.value, self.range.clone(), self.text);
        let response = ui.add(value_slider);

        // Bottom: ghost viz (standard disabled slider)
        if let Some(mut ghost) = self.ghost_value {
            let ghost_slider = egui::Slider::new(&mut ghost, self.range.clone()).show_value(false);
            ui.add_enabled(false, ghost_slider);
        }

        response
    }
}

pub fn styled_dual_slider<'a, T>(
    value: &'a mut T,
    ghost_value: Option<T>,
    range: std::ops::RangeInclusive<T>,
    text: &'a str,
) -> DualSlider<'a, T>
where
    T: egui::emath::Numeric + Copy,
{
    DualSlider {
        value,
        ghost_value,
        range,
        text,
    }
}
