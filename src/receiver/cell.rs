use nannou::{geom::Rect, prelude::Srgba};

/// Represents a single cell, which can collide with an animator object
#[derive(Clone)]
pub struct GridCell {
    pub rect: Rect,
    pub display_color: Srgba,
    pub is_active: bool,
    pub pos_string: String,
}

impl GridCell {
    pub fn new_from_rect(rect: Rect, pos: u32) -> Self {
        GridCell {
            rect,
            is_active: false,
            display_color: Srgba::rgba_u8(0, 0, 0, 0),
            pos_string: pos.to_string(),
        }
    }

    pub fn reset(&mut self) {
        self.is_active = false;
        self.display_color = Srgba::rgba_u8(0, 0, 0, 0);
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
