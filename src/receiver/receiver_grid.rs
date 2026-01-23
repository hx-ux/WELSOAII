extern crate nannou;
use crate::animator::AnimatedObject;
use crate::animator::presets_manager::{PresetManager, PresetMode};
use crate::receiver::ReceiverDevice;
use crate::utils::ColorHelpers;
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

use crate::ui::controls::styled_text_edit;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LayoutMode {
    FollowRow = 0,   //  0 1 2 3 / 4 5 6 7
    FollowColum = 1, //  0 4 / 1 5 / 2 6 / 3 7
}

#[derive(Clone)]
/// the single cell, which interacts with the animator
pub struct GridCell {
    pub pos: u32,
    pub rect: Rect,
    pub display_color: Rgba,
    pub is_active: bool,
    pos_string: String, // Cached string representation
}

impl GridCell {
    pub fn new_from_rect(rect: Rect, pos: u32) -> Self {
        GridCell {
            rect,
            is_active: false,
            display_color: Rgba::almost_transparent(),
            pos,
            pos_string: pos.to_string(), // Cache the string
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

// #[derive(Serialize, Deserialize, Default)]
#[derive(Clone, Serialize)]
pub struct ReceiverGrid {
    #[serde(skip)]
    main_rect: Rect,
    #[serde(skip)]
    pub cells: Vec<GridCell>,
    pub cols: u32,
    pub rows: u32,
    // #[serde(skip)]
    device: ReceiverDevice,
    show_debug_info: bool,
    show_grid: bool,
    #[serde(skip)]
    led_buffer: RefCell<Vec<u8>>, // Pre-allocated buffer for LED data
    #[serde(skip)]
    layout_mode: LayoutMode,
    #[serde(skip)]
    persistence: PresetManager<ReceiverGrid>,
}

impl ReceiverGrid {
    pub fn new(
        main_rect: Rect,
        cols: u32,
        rows: u32,
        debug: bool,
        layout_mode: LayoutMode,
    ) -> Self {
        let cell_count = (rows * cols) as usize;

        // Pre-allocate LED buffer (3 bytes per cell: RGB)
        let led_buffer = RefCell::new(vec![0u8; cell_count * 3]);

        let mut grid = ReceiverGrid {
            main_rect,
            cells: ReceiverGrid::create_grid(&main_rect, rows, cols, layout_mode.clone()),
            cols,
            rows,
            device: ReceiverDevice::default(),
            show_debug_info: debug,
            led_buffer,
            layout_mode,
            persistence: PresetManager::new_grid(PresetMode::Grid, "Leds 1".to_string()),
            show_grid: false,
        };

        grid.update_cells();
        grid
    }

    pub fn update_cells(&mut self) {
        // self.cells.clear();
        // self.create_grid();
    }

    /// Returns the range of cells (col_min, col_max, row_min, row_max) that could intersect

    pub fn get_cell_range(
        &self,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
    ) -> (u32, u32, u32, u32) {
        if self.cols == 0 || self.rows == 0 {
            return (0, 0, 0, 0);
        }

        let cell_w = self.main_rect.w() / self.cols as f32;
        let cell_h = self.main_rect.h() / self.rows as f32;

        // Convert world coordinates to grid indices
        let min_col = ((left - self.main_rect.left()) / cell_w).floor().max(0.0) as u32;
        let max_col = ((right - self.main_rect.left()) / cell_w)
            .ceil()
            .min(self.cols as f32) as u32;
        let min_row = ((self.main_rect.top() - top) / cell_h).floor().max(0.0) as u32;
        let max_row = ((self.main_rect.top() - bottom) / cell_h)
            .ceil()
            .min(self.rows as f32) as u32;

        (
            min_col.min(self.cols - 1),
            max_col.min(self.cols - 1),
            min_row.min(self.rows - 1),
            max_row.min(self.rows - 1),
        )
    }

    /// This accounts for the current layout mode
    pub fn get_cell_index(&self, row: u32, col: u32) -> usize {
        match self.layout_mode {
            LayoutMode::FollowRow => {
                // Row-major: index = row * cols + col
                (row * self.cols + col) as usize
            }
            LayoutMode::FollowColum => {
                // Column-major: index = col * rows + row
                (col * self.rows + row) as usize
            }
        }
    }

    fn create_grid(
        dimension: &Rect,
        rows: u32,
        cols: u32,
        layout_mode: LayoutMode,
    ) -> Vec<GridCell> {
        let mut grid: Vec<GridCell> = Vec::new();
        if cols == 0 || rows == 0 {
            return grid;
        }

        let cell_w = dimension.w() / cols as f32;
        let cell_h = dimension.h() / rows as f32;
        let start_x = dimension.left() + cell_w / 2.0;
        let start_y = dimension.top() - cell_h / 2.0;

        match layout_mode {
            LayoutMode::FollowRow => {
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
            }
            LayoutMode::FollowColum => {
                let mut _pos = 0;
                for c in 0..cols {
                    for r in 0..rows {
                        let x = start_x + c as f32 * cell_w;
                        let y = start_y - r as f32 * cell_h;
                        let cell_rect = Rect::from_x_y_w_h(x, y, cell_w, cell_h);
                        grid.push(GridCell::new_from_rect(cell_rect, _pos));
                        _pos += 1;
                    }
                }
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

            if self.show_grid {
                draw.rect()
                    .xy(cell.rect.xy())
                    .wh(cell.rect.wh())
                    .stroke_color(SNOW)
                    .stroke_weight(1.0)
                    .color(cell.get_display_color());
            }
            // Draw filled cell

            if self.show_grid  && self.show_debug_info {
                let mut size = 12;
                if self.cells.len() >= 100 {
                    size = 10;
                }
                // Draw the position number on each cell
                if self.show_debug_info {
                    draw.text(&cell.pos_string)
                        .xy(cell.rect.xy())
                        .color(WHITE)
                        .font_size(size);
                }
            }
        }

        // TODO check, if led device is ready
        let _ = self.device.send_data(&led_buffer);
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

        let mut name = self.device.name.clone();
        if ui.add(styled_text_edit(&mut name, "Device Name")).changed() {
            self.device.name = name;
            changed = true;
        }

        ui.add_space(5.0);

        let mut ip = self.device.ip.clone();
        if ui.add(styled_text_edit(&mut ip, "IP Address")).changed() {
            // todo validate IP
            self.device.ip = ip;
            changed = true;
        }
        // ui.add_space(5.0);
        // ui.label(format!("Max Len: {}", &self.device.max_len));

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

        ui.checkbox(&mut self.show_debug_info, "Show debug info");
        ui.checkbox(&mut self.show_grid, "Show grid");


        if ui.button("Save Settings").clicked() {
            let _ = self
                .persistence
                .save_to_file(self, Some(self.device.name.clone()));
            changed = true;
        }

        changed
    }
}
