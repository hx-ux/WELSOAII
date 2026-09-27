use crate::receiver::ReceiverDevice;
use nannou::prelude::*;
use nannou_egui::egui::{self};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::ui::controls::monospace_text_edit;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum LayoutMode {
    Row,
    Colum,
}

#[derive(Clone)]
pub struct GridCell {
    pub rect: Rect,
    pub display_color: Rgba8,
    pub is_active: bool,
    pos_string: String,
}

impl GridCell {
    pub fn new_from_rect(rect: Rect, pos: u32) -> Self {
        GridCell {
            rect,
            is_active: false,
            display_color: Rgba8::new(0, 0, 0, 0),
            pos_string: pos.to_string(), // Cache the string
        }
    }

    pub fn reset(&mut self) {
        self.is_active = false;
        self.display_color = Rgba8::new(0, 0, 0, 0);
    }

    pub fn get_send_color(&self) -> Rgba8 {
        if self.is_active {
            return self.display_color;
        }
        Rgba8::new(0, 0, 0, 0)
    }

    pub fn get_display_color(&self) -> Rgba8 {
        if self.is_active {
            return self.display_color;
        }
        Rgba8::new(0, 0, 0, 0)
    }
}

#[derive(Clone, Serialize)]
pub struct ReceiverGrid {
    #[serde(skip)]
    main_rect: Rect,
    #[serde(skip)]
    pub cells: Vec<GridCell>,
    pub cols: u32,
    pub rows: u32,
    device: ReceiverDevice,
    #[serde(skip)]
    led_buffer: Vec<u8>, // Pre-allocated buffer for LED data
    #[serde(skip)]
    cell_w: f32,
    #[serde(skip)]
    cell_h: f32,
    layout_mode: LayoutMode,
    #[serde(skip)]
    pub edit_mode: bool,
}

impl ReceiverGrid {
    pub fn new(main_rect: Rect, cols: u32, rows: u32, layout_mode: LayoutMode) -> Self {
        let cell_count = (rows * cols) as usize;

        // Pre-allocate LED buffer (3 bytes per cell: RGB)
        let led_buffer = vec![0u8; cell_count * 3];

        let mut grid = ReceiverGrid {
            main_rect,
            cells: Vec::new(),
            cols,
            rows,
            device: ReceiverDevice::default(),
            led_buffer,
            cell_w: 0.0,
            cell_h: 0.0,
            layout_mode,
            edit_mode: false,
        };

        grid.update_cells();
        grid
    }

    pub fn update_cells(&mut self) {
        self.cells = ReceiverGrid::create_grid(
            &self.main_rect,
            self.rows,
            self.cols,
            self.layout_mode.clone(),
        );

        if self.cols == 0 || self.rows == 0 {
            self.cell_w = 0.0;
            self.cell_h = 0.0;
        } else {
            self.cell_w = self.main_rect.w() / self.cols as f32;
            self.cell_h = self.main_rect.h() / self.rows as f32;
        }

        let buffer_len = self.cells.len() * 3;
        if self.led_buffer.len() != buffer_len {
            self.led_buffer.resize(buffer_len, 0);
        }
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

        let cell_w = self.cell_w;
        let cell_h = self.cell_h;
        if cell_w == 0.0 || cell_h == 0.0 {
            return (0, 0, 0, 0);
        }

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
            LayoutMode::Row => {
                // Row-major: index = row * cols + col
                (row * self.cols + col) as usize
            }
            LayoutMode::Colum => {
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
            LayoutMode::Row => {
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
            LayoutMode::Colum => {
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
        if !self.edit_mode {
            return;
        }

        for cell in &self.cells {
            draw.rect()
                .xy(cell.rect.xy())
                .wh(cell.rect.wh())
                .stroke_color(SNOW)
                .stroke_weight(1.0)
                .color(cell.get_display_color());
            let mut size = 12;
            if self.cells.len() >= 100 {
                size = 10;
            }

            draw.text(&cell.pos_string)
                .xy(cell.rect.xy())
                .color(WHITE)
                .font_size(size);
        }
    }

    pub fn update_led_buffer_and_send(&mut self) {
        if self.device.establish_conn {
            let buffer_len = self.cells.len() * 3;
            if self.led_buffer.len() != buffer_len {
                self.led_buffer.resize(buffer_len, 0);
            }

            for (idx, cell) in self.cells.iter().enumerate() {
                let cell_send_col = cell.get_send_color();
                let base_idx = idx * 3;
                self.led_buffer[base_idx] = cell_send_col.red;
                self.led_buffer[base_idx + 1] = cell_send_col.green;
                self.led_buffer[base_idx + 2] = cell_send_col.blue;
            }

            let _ = self.device.send_data(&self.led_buffer);
        }
    }

    pub fn move_by(&mut self, offset: Vec2) {
        if self.edit_mode {
            let new_center = self.main_rect.xy() + offset;
            self.main_rect = Rect::from_x_y_w_h(
                new_center.x,
                new_center.y,
                self.main_rect.w(),
                self.main_rect.h(),
            );
            self.update_cells();
        }
    }

    pub fn resize_by(&mut self, amount: Vec2) {
        if self.edit_mode {
            let new_size = (self.main_rect.wh() + amount).max(vec2(20.0, 20.0));
            self.main_rect = Rect::from_x_y_w_h(
                self.main_rect.x(),
                self.main_rect.y(),
                new_size.x,
                new_size.y,
            );
            self.update_cells();
        }
    }
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        let mut name = self.device.name.clone();
        if ui
            .add(monospace_text_edit(&mut name, "Device Name"))
            .changed()
        {
            self.device.name = name;
            changed = true;
        }

        ui.add_space(5.0);

        let mut ip = self.device.ip.clone();
        if ui.add(monospace_text_edit(&mut ip, "IP Address")).changed() {
            // TODO validate IP
            self.device.ip = ip;
            changed = true;
        }

        ui.add_space(5.0);

        let status = if self.device.establish_conn {
            "Disconnect"
        } else {
            "Connect"
        };

        if ui.button(status).clicked() {
            let _ = &self.device.open_connection();
            changed = true;
        }

        if ui.checkbox(&mut self.edit_mode, "Edit").clicked() {
            changed = true;
        }
        egui::ComboBox::from_label("")
            .selected_text(format!("{}", self.layout_mode).to_string())
            .show_ui(ui, |ui| {
                LayoutMode::iter().for_each(|option| {
                    changed |= ui
                        .selectable_value(
                            &mut self.layout_mode,
                            option,
                            format!("{}", option.clone()),
                        )
                        .changed();
                });
            });

        changed
    }
}
