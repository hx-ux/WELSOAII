//! Common styled controls for egui UI
use std::ops::RangeInclusive;

use nannou_egui::egui;
/// Styled single-line text edit with monospace font and code editor look
pub fn styled_text_edit<'a>(text: &'a mut String, hint: &'a str) -> egui::TextEdit<'a> {
    egui::TextEdit::singleline(text)
        .hint_text(hint)
        .font(egui::TextStyle::Monospace)
        .desired_width(120.0)
        .code_editor()
}

/// Styled slider with consistent look for egui UI
pub fn styled_slider<'a, T>(
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

/// Minimal set of Material Symbols icons we bind in code.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IconKind {
    Add,
    Delete,
    Reset,
}

impl IconKind {
    /// Unicode codepoint from the Google Material Symbols font. The values correspond to the
    /// upstream `MaterialSymbolsRounded` TTF file.
    pub fn glyph(&self) -> &'static str {
        match self {
            IconKind::Add => "\u{e145}",    // add
            IconKind::Delete => "\u{e872}", // delete
            IconKind::Reset => "\u{e5d5}",  // refresh (good reset metaphor)
        }
    }
}

/// A compact icon-only button using the Material Symbols font.
///
/// Ensure `install_material_symbols` has been called once on the egui context before using this.
pub fn icon_button(ui: &mut egui::Ui, icon: IconKind, tooltip: Option<&str>) -> egui::Response {
    // Keep this close to egui's font_book pattern: set family + size on RichText.
    let text = egui::RichText::new(icon.glyph())
        // .family(egui::FontFamily::Name(MATERIAL_SYMBOLS_FAMILY.into()))
        .size(20.0);

    let response = ui.add_sized(
        egui::vec2(30.0, 30.0),
        egui::Button::new(text).rounding(6.0),
    );

    if let Some(tt) = tooltip {
        response.on_hover_text(tt)
    } else {
        response
    }
}

