extern crate nannou;

use nannou::{
    color::{encoding::Srgb, rgb::Rgb},
    prelude::*,
};

pub struct ReciverCell {
    pub rect: Rect,
    pub found_color: Rgba,
    pub is_active: bool,
    pub draw_color: Rgba,
}

impl ReciverCell {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        ReciverCell {
            rect: Rect::from_x_y_w_h(x, y, w, h),
            is_active: false,
            found_color: Self::get_stan_colr(),
            draw_color: Self::get_stan_colr(),
        }
    }
    pub fn new_from_rect(r: Rect) -> Self {
        ReciverCell {
            rect: r,
            is_active: false,
            found_color: Self::get_stan_colr(),
            draw_color: Self::get_stan_colr(),
        }
    }

    fn get_stan_colr() -> Rgba {
        Rgba::new(1.0, 1.0, 1.0, 0.1)
    }

    pub fn reset(&mut self) {
        self.is_active = false;
    }
}

pub struct ReciverGrid {
    main_rect: Rect, // The bounding box for the entire grid
    pub cells: Vec<ReciverCell>,
    cols: usize,
    rows: usize,
    pub col: Rgba,
}

impl ReciverGrid {
    pub fn new(main_rect: Rect, cols: usize, rows: usize) -> Self {
        let mut grid = ReciverGrid {
            main_rect,
            cells: Vec::new(),
            cols,
            rows,
            col: Rgba::new(1.0, 1.0, 1.0, 0.1),
        };
        grid.update_cells(); // Generate the cells
        return grid;
    }

    pub fn update_cells(&mut self) {
        self.cells.clear();
        let cell_w = self.main_rect.w() / self.cols as f32;
        let cell_h = self.main_rect.h() / self.rows as f32;
        let start_x = self.main_rect.left() + cell_w / 2.0;
        let start_y = self.main_rect.top() - cell_h / 2.0;

        for r in 0..self.rows {
            for c in 0..self.cols {
                let x = start_x + c as f32 * cell_w;
                let y = start_y - r as f32 * cell_h;
                let cell_rect = Rect::from_x_y_w_h(x, y, cell_w, cell_h);
                self.cells.push(ReciverCell::new_from_rect(cell_rect));
            }
        }
    }

    pub fn move_by(&mut self, offset: Vec2) {
     //   self.main_rect.set_xy(self.main_rect.xy() + offset);
        self.update_cells();
    }

    // Resizes the whole receiver grid by an amount.
   pub fn resize_by(&mut self, amount: Vec2) {
        let new_size = (self.main_rect.wh() + amount).max(vec2(20.0, 20.0));
        // Re-create the rectangle with the new size while keeping the same center point.
        self.main_rect = Rect::from_x_y_w_h(
            self.main_rect.x(),
            self.main_rect.y(),
            new_size.x,
            new_size.y,
        );
        self.update_cells();
    }
}
