use crate::{
    parameters::ConstantParam,
    receiver::cell::GridCell,
    sender::WLEDSender,
    ui::controls::monospace_text_edit,
    utils::{AppLogger, Severity},
};
use nannou::prelude::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, EnumIter, Display)]
pub enum LayoutMode {
    Row,
    Colum,
}

#[derive(Clone, Serialize)]
pub struct ReceiverGrid {
    #[serde(skip)]
    main_rect: Rect,
    #[serde(skip)]
    pub cells: Vec<GridCell>,
    pub cols: u32,
    pub rows: u32,
    pub device: WLEDSender,
    #[serde(skip)]
    led_buffer: Vec<u8>,
    #[serde(skip)]
    cell_w: f32,
    #[serde(skip)]
    cell_h: f32,
    pub layout_mode: LayoutMode,
    #[serde(skip)]
    pub edit_mode: ConstantParam<bool>,
    #[serde(skip)]
    pub needs_clear: bool,
    #[serde(skip)]
    pub logger: AppLogger,
}

impl ReceiverGrid {
    pub fn new(
        main_rect: Rect,
        cols: u32,
        rows: u32,
        layout_mode: LayoutMode,
        logger: AppLogger,
    ) -> Self {
        let cell_count = (rows * cols) as usize;
        let led_buffer = vec![0u8; cell_count * 3];
        let sender = WLEDSender::default();
        let mut grid = ReceiverGrid {
            main_rect,
            cells: Vec::new(),
            cols,
            rows,
            device: sender,
            led_buffer,
            cell_w: 0.0,
            cell_h: 0.0,
            layout_mode,
            edit_mode: ConstantParam::new(false, false, true, "Show grid", "show_grid"),
            needs_clear: false,
            logger,
        };

        grid.update_cells();
        grid
    }

    pub fn take_needs_clear(&mut self) -> bool {
        std::mem::take(&mut self.needs_clear)
    }

    pub fn update_cells(&mut self) {
        self.cells =
            ReceiverGrid::create_grid(&self.main_rect, self.rows, self.cols, self.layout_mode);

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
        let mut grid: Vec<GridCell> = Vec::with_capacity((rows * cols) as usize);
        if cols == 0 || rows == 0 {
            return grid;
        }

        let cell_w = dimension.w() / cols as f32;
        let cell_h = dimension.h() / rows as f32;
        let start_x = dimension.left() + cell_w / 2.0;
        let start_y = dimension.top() - cell_h / 2.0;

        match layout_mode {
            LayoutMode::Row => {
                let mut pos = 0;
                for r in 0..rows {
                    for c in 0..cols {
                        let x = start_x + c as f32 * cell_w;
                        let y = start_y - r as f32 * cell_h;
                        let cell_rect = Rect::from_x_y_w_h(x, y, cell_w, cell_h);
                        grid.push(GridCell::new_from_rect(cell_rect, pos));
                        pos += 1;
                    }
                }
            }
            LayoutMode::Colum => {
                let mut pos = 0;
                for c in 0..cols {
                    for r in 0..rows {
                        let x = start_x + c as f32 * cell_w;
                        let y = start_y - r as f32 * cell_h;
                        let cell_rect = Rect::from_x_y_w_h(x, y, cell_w, cell_h);
                        grid.push(GridCell::new_from_rect(cell_rect, pos));
                        pos += 1;
                    }
                }
            }
        }

        grid
    }

    pub fn draw(&self, draw: &Draw) {
        if !self.edit_mode.value {
            return;
        }

        for cell in &self.cells {
            if cell.is_active {
                draw.rect()
                    .xy(cell.rect.xy())
                    .wh(cell.rect.wh())
                    .color(cell.get_display_color());
            }

            let last_total_pos = (self.cols * self.rows) - 1;
            let last_row_pos = (self.cols * self.rows) - self.rows;
            let last_col_pos = self.cols - 1;

            if cell.pos == 0
                || cell.pos == last_total_pos
                || cell.pos == last_row_pos
                || cell.pos == last_col_pos
            {
                draw.text(&cell.pos.to_string())
                    .xy(cell.rect.xy())
                    .color(WHITE)
                    .font_size(10);
            }
        }

        let left = self.main_rect.left();
        let right = self.main_rect.right();
        let top = self.main_rect.top();
        let bottom = self.main_rect.bottom();

        for c in 0..=self.cols {
            let x = left + c as f32 * self.cell_w;
            draw.line()
                .start(pt2(x, top))
                .end(pt2(x, bottom))
                .stroke_weight(1.0)
                .color(WHITE);
        }

        for r in 0..=self.rows {
            let y = top - r as f32 * self.cell_h;
            draw.line()
                .start(pt2(left, y))
                .end(pt2(right, y))
                .stroke_weight(1.0)
                .color(WHITE);
        }
    }

    pub fn update_led_buffer_and_send(&mut self) {
        if self.device.establish_conn {
            let buffer_len = self.cells.len() * 3;
            if self.led_buffer.len() != buffer_len {
                self.led_buffer.resize(buffer_len, 0);
            }

            for (idx, cell) in self.cells.iter().enumerate() {
                let [r, g, b, _] = cell.get_send_color().to_u8_array();
                let base_idx = idx * 3;
                self.led_buffer[base_idx] = r;
                self.led_buffer[base_idx + 1] = g;
                self.led_buffer[base_idx + 2] = b;
            }

            let _ = self.device.send_data(&self.led_buffer);
        }
    }

    pub fn move_by(&mut self, offset: Vec2) {
        if self.edit_mode.value {
            let new_center = self.main_rect.xy() + offset;
            self.main_rect = Rect::from_x_y_w_h(
                new_center.x,
                new_center.y,
                self.main_rect.w(),
                self.main_rect.h(),
            );
            self.update_cells();
            self.needs_clear = true;
        }
    }

    pub fn resize_by(&mut self, amount: Vec2) {
        if self.edit_mode.value {
            let new_size = (self.main_rect.wh() + amount).max(vec2(20.0, 20.0));
            self.main_rect = Rect::from_x_y_w_h(
                self.main_rect.x(),
                self.main_rect.y(),
                new_size.x,
                new_size.y,
            );
            self.update_cells();
            self.needs_clear = true;
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
            if self.device.establish_conn {
                self.device.close_connection();
                self.logger
                    .log("Disconnected from WLED device", Severity::Info);
            } else {
                match self.device.open_connection() {
                    Ok(_) => self.logger.log(
                        format!("Connected to WLED device at {}", self.device.ip),
                        Severity::Info,
                    ),
                    Err(e) => self.logger.log(format!("{e}"), Severity::Error),
                }
            }
            changed = true;
        }

        self.edit_mode.checkbox_ui(ui);

        let mut layout_changed = false;
        egui::ComboBox::from_label("")
            .selected_text(self.layout_mode.to_string())
            .show_ui(ui, |ui| {
                for option in LayoutMode::iter() {
                    layout_changed |= ui
                        .selectable_value(&mut self.layout_mode, option, option.to_string())
                        .changed();
                }
            });

        if layout_changed {
            self.update_cells();
            changed = true;
        }

        changed
    }
}
