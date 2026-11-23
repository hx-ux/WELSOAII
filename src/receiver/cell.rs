use crate::animator::animation_type::LayerBlendMode;
use nannou::{geom::Rect, prelude::Srgba};

/// Represents a single cell, which can collide with an animator object
#[derive(Clone)]
pub struct GridCell {
    pub rect: Rect,
    pub display_color: Srgba,
    pub is_active: bool,
    pub pos: u32,
}

impl GridCell {
    pub fn new_from_rect(rect: Rect, pos: u32) -> Self {
        GridCell {
            rect,
            is_active: false,
            display_color: Srgba::rgba_u8(0, 0, 0, 0),
            pos,
        }
    }

    pub fn reset(&mut self) {
        self.is_active = false;
        self.display_color = Srgba::rgba_u8(0, 0, 0, 0);
    }

    /// Applies frame-rate-independent trail retention decay to the cell's color.
    pub fn decay(&mut self, retention: f32) {
        if !self.is_active || retention <= 0.001 {
            self.reset();
            return;
        }
        self.display_color.red *= retention;
        self.display_color.green *= retention;
        self.display_color.blue *= retention;
        self.display_color.alpha *= retention;

        if self.display_color.red < 0.004
            && self.display_color.green < 0.004
            && self.display_color.blue < 0.004
        {
            self.reset();
        }
    }

    /// Composites a source color onto this cell using the layer's blend mode.
    pub fn blend(&mut self, src_color: Srgba, mode: LayerBlendMode) {
        self.display_color = mode.blend_srgba(self.display_color, src_color);
        self.update_active_flag();
    }

    /// Composites a premultiplied layer cell color onto this cell using the layer's blend mode.
    pub fn blend_premultiplied(&mut self, src_premul: Srgba, mode: LayerBlendMode) {
        self.display_color = mode.blend_premultiplied(self.display_color, src_premul);
        self.update_active_flag();
    }

    fn update_active_flag(&mut self) {
        self.is_active = self.display_color.red > 0.002
            || self.display_color.green > 0.002
            || self.display_color.blue > 0.002;
        if self.is_active && self.display_color.alpha < 0.01 {
            self.display_color.alpha = 1.0;
        }
    }

    pub fn get_send_color(&self) -> Srgba {
        if self.is_active {
            return self.display_color;
        }
        Srgba::rgba_u8(0, 0, 0, 0)
    }

    pub fn get_display_color(&self) -> Srgba {
        if self.is_active {
            return self.display_color;
        }
        Srgba::rgba_u8(0, 0, 0, 0)
    }
}
