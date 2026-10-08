use nannou_egui::egui::{self, vec2};

use crate::ui::style_definitions::custom_colors;

pub fn monospace_text_edit<'a>(text: &'a mut String, hint: &'a str) -> egui::TextEdit<'a> {
    egui::TextEdit::singleline(text)
        .hint_text(hint)
        .font(egui::TextStyle::Monospace)
        .desired_width(120.0)
        .code_editor()
}

pub struct GhostValueSlider<'a> {
    value: &'a mut f32,
    ghost_value: Option<f32>,
    lower: f32,
    upper: f32,
}

impl<'a> GhostValueSlider<'a> {
    pub fn new(value: &'a mut f32, ghost_value: Option<f32>, lower: f32, upper: f32) -> Self {
        Self {
            value,
            ghost_value,
            lower,
            upper,
        }
    }
}

impl<'a> egui::Widget for GhostValueSlider<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let height = 1.0_f32;
        let base_cords = vec2(ui.spacing().slider_width, height);

        let mut response = ui.allocate_response(base_cords, egui::Sense::drag());

        let base_rect = response.rect;

        let min = self.lower;
        let max = self.upper;
        let range_size = max - min;
        let mut value_frac = if range_size != 0.0 {
            (*self.value - min) / range_size
        } else {
            0.0
        };
        value_frac = value_frac.clamp(0.0, 1.0);

        let track_rect = base_rect.shrink(20.00);

        // Drag logic
        if response.dragged() {
            let delta = ui.input(|i| i.pointer.delta().x / track_rect.width());
            value_frac += delta;
            value_frac = value_frac.clamp(0.0, 1.0);
            *self.value = min + value_frac * range_size;
            response.changed = true;
        }

        let painter = ui.painter();
        let rounding = egui::Rounding::same(base_rect.height() / 2.0);

        if let Some(ghost_val) = self.ghost_value {
            let ghost_frac = (ghost_val - min) / range_size;
            let ghost_frac = ghost_frac.clamp(0.0, 1.0);

            let ghost_fill_rect = egui::Rect::from_min_size(
                base_rect.left_top(),
                egui::vec2(ghost_frac * base_rect.width(), base_rect.height()),
            );

            painter.rect_filled(ghost_fill_rect, rounding, custom_colors::SLIDER_GHOST_FILL);

            // let base_fill_rect = egui::Rect::from_min_size(
            //     base_rect.left_top(),
            //     egui::vec2(value_frac * base_rect.width(), base_rect.height() / 2.00),
            // );
            // // second indicatior for its value
            // painter.rect_filled(base_fill_rect, rounding, Color32::from_rgb(255, 0, 0));
        }

        response
    }
}
