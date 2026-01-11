extern crate nannou;
use crate::receiver::ReceiverDevice;
use crate::utils::ColorHelpers;
use nannou::prelude::*;
use nannou_egui::egui;
use std::cell::RefCell;

#[derive(Clone)]
/// the single cell, which interacts with the animator
pub struct GridCell {
    pub pos: u32,
    pub rect: Rect,
    pub display_color: Rgba,
    pub is_active: bool,
}

impl GridCell {
    pub fn new_from_rect(rect: Rect, pos: u32) -> Self {
        GridCell {
            rect,
            is_active: false,
            display_color: Rgba::almost_transparent(),
            pos,
        }
    }

    pub fn reset(&mut self) {
        self.is_active = false;
        self.display_color = Rgba::almost_transparent();
    }

    pub fn get_send_color(&self) -> Rgba {
        if self.is_active {
            return self.display_color;
        }
        Rgba::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn get_display_color(&self) -> Rgba {
        if self.is_active {
            return self.display_color;
        }
        Rgba::almost_transparent()
    }
}

#[derive(Clone)]
pub struct ReceiverGrid {
    main_rect: Rect,
    pub cells: Vec<GridCell>,
    cols: u32,
    rows: u32,
    device: ReceiverDevice,
    show_debug_info: bool,
    led_buffer: RefCell<Vec<u8>>, // Pre-allocated buffer for LED data
}

impl ReceiverGrid {
    pub fn new(main_rect: Rect, cols: u32, rows: u32, debug: bool) -> Self {

        let cell_count = (rows * cols) as usize;

        // Pre-allocate LED buffer (3 bytes per cell: RGB)
        let led_buffer = RefCell::new(vec![0u8; cell_count * 3]);

        let mut grid = ReceiverGrid {
            main_rect,
            cells:ReceiverGrid::create_grid(&main_rect, rows, cols),
            cols,
            rows,
            device: ReceiverDevice::new("192.168.178.102", "Leds 1", cell_count),
            show_debug_info: debug,
            led_buffer,
        };

        grid.update_cells();
        grid
    }

    pub fn update_cells(&mut self) {
        // self.cells.clear();
        // self.create_grid();
    }

    fn create_grid(dimension: &Rect, rows: u32, cols: u32) -> Vec<GridCell> {

        let mut grid: Vec<GridCell> = Vec::new();
        if cols == 0 || rows == 0 {
            return grid;
        }

        let cell_w = dimension.w() / cols as f32;
        let cell_h = dimension.h() / rows as f32;
        let start_x = dimension.left() + cell_w / 2.0;
        let start_y = dimension.top() - cell_h / 2.0;

        let mut _pos = 0;
        for r in 0..rows {
            for c in 0..cols {
                let x = start_x + c as f32 * cell_w;
                let y = start_y - r as f32 * cell_h;
                let cell_rect = Rect::from_x_y_w_h(x, y, cell_w, cell_h);
                grid.push(GridCell::new_from_rect(cell_rect, _pos));
                _pos += 1;
            }
        }

        grid
    }

    pub fn draw(&self, draw: &Draw) {
        // Reuse pre-allocated buffer 
        let mut led_buffer = self.led_buffer.borrow_mut();

        // check if buffer has changed 
        let buffer_len = self.cells.len() * 3;

        if led_buffer.len() != buffer_len {
            led_buffer.resize(buffer_len, 0);
        }

        // Fill LED buffer and draw cells 
        for (idx, cell) in self.cells.iter().enumerate() {
            let cell_send_col = cell.get_send_color();
            
            let base_idx = idx * 3;
            led_buffer[base_idx] = (cell_send_col.red * 255.0) as u8;
            led_buffer[base_idx + 1] = (cell_send_col.green * 255.0) as u8;
            led_buffer[base_idx + 2] = (cell_send_col.blue * 255.0) as u8;

            // Draw filled cell 
            draw.rect()
                .xy(cell.rect.xy())
                .wh(cell.rect.wh())
                .stroke_color(SNOW)
                .stroke_weight(1.0)
                .color(cell.get_display_color());

            // Draw the position number on each cell 
            if self.show_debug_info {
                draw.text(&cell.pos.to_string())
                        .xy(cell.rect.xy())
                        .color(WHITE)
                        .font_size(12);
            }
        }

        // TODO check, if led device is ready
        let _ = self.device.send_data(led_buffer.clone());
    }

    pub fn move_by(&mut self, offset: Vec2) {
        //   self.main_rect.set_xy(self.main_rect.xy() + offset);
        self.update_cells();
    }

    pub fn resize_by(&mut self, amount: Vec2) {
        let new_size = (self.main_rect.wh() + amount).max(vec2(20.0, 20.0));
        self.main_rect = Rect::from_x_y_w_h(
            self.main_rect.x(),
            self.main_rect.y(),
            new_size.x,
            new_size.y,
        );
        self.update_cells();
    }
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading(&self.device.name);
        ui.add_space(5.0);
        ui.label(format!("IP: {}", &self.device.ip));
        ui.add_space(5.0);
        ui.label(format!("Max Len: {}", &self.device.max_len));
        ui.add_space(5.0);

        let status = if self.device.establish_conn {
            "Device connected"
        } else {
            "Device not connected"
        };

        ui.label(status);
        ui.add_space(5.0);

        if ui.button("Connect").clicked() {
            let _ = &self.device.open_connection();
            changed = true;
        }

        changed
    }

}
