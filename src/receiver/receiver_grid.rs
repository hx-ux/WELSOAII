extern crate nannou;
use crate::animator::presets_manager::{PresetManager, PresetMode};
use crate::receiver::ReceiverDevice;
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};

use crate::ui::controls::styled_text_edit;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LayoutMode {
    FollowRow = 0,   //  0 1 2 3 / 4 5 6 7
    FollowColum = 1, //  0 4 / 1 5 / 2 6 / 3 7
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionMode {
    Rows,
    Columns,
}

impl Default for SelectionMode {
    fn default() -> Self {
        Self::Rows
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DragTarget {
    Row(usize),
    Column(usize),
}

#[derive(Clone, Serialize, Deserialize)]
struct OffsetSave {
    x: f32,
    y: f32,
}

impl OffsetSave {
    fn from_vec2(offset: Vec2) -> Self {
        Self {
            x: offset.x,
            y: offset.y,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct ReceiverGridSaveData {
    device_name: String,
    device_ip: String,
    grid_center: [f32; 2],
    grid_size: [f32; 2],
    led_count: u32,
    rows: u32,
    cols: u32,
    show_debug_info: bool,
    show_grid: bool,
    layout_mode: LayoutMode,
    selection_mode: SelectionMode,
    row_offsets: Vec<OffsetSave>,
    column_offsets: Vec<OffsetSave>,
}

#[derive(Clone)]
/// the single cell, which interacts with the animator
pub struct GridCell {
    pub pos: u32,
    pub row: u32,
    pub col: u32,
    pub rect: Rect,
    pub display_color: Rgba8,
    pub is_active: bool,
    pos_string: String, // Cached string representation
}

impl GridCell {
    pub fn new_from_rect(rect: Rect, pos: u32, row: u32, col: u32) -> Self {
        GridCell {
            rect,
            is_active: false,
            display_color: Rgba8::new(10, 10, 10, 10),
            pos,
            row,
            col,
            pos_string: pos.to_string(), // Cache the string
        }
    }

    pub fn reset(&mut self) {
        self.is_active = false;
        self.display_color = Rgba8::new(10, 10, 10, 10);
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
        Rgba8::new(10, 10, 10, 10)
    }
}

// #[derive(Serialize, Deserialize, Default)]
#[derive(Clone, Serialize)]
pub struct ReceiverGrid {
    #[serde(skip)]
    main_rect: Rect,
    #[serde(skip)]
    pub cells: Vec<GridCell>,
    pub led_count: u32,
    pub cols: u32,
    pub rows: u32,
    device: ReceiverDevice,
    show_debug_info: bool,
    show_grid: bool,
    #[serde(skip)]
    led_buffer: Vec<u8>, // Pre-allocated buffer for LED data
    #[serde(skip)]
    cell_w: f32,
    #[serde(skip)]
    cell_h: f32,
    // #[serde(skip)]
    layout_mode: LayoutMode,
    #[serde(skip)]
    persistence: PresetManager<ReceiverGridSaveData>,
    selection_mode: SelectionMode,
    #[serde(skip)]
    pub edit_mode: bool,
    #[serde(skip)]
    row_offsets: Vec<Vec2>,
    #[serde(skip)]
    column_offsets: Vec<Vec2>,
    #[serde(skip)]
    drag_target: Option<DragTarget>,
    #[serde(skip)]
    drag_anchor_mouse: Point2,
    #[serde(skip)]
    drag_anchor_offset: Vec2,
}

impl ReceiverGrid {
    pub fn new(
        main_rect: Rect,
        cols: u32,
        rows: u32,
        debug: bool,
        layout_mode: LayoutMode,
    ) -> Self {
        let led_count = (rows * cols).max(1);
        let cols = ReceiverGrid::required_cols(led_count, rows);
        let cell_count = led_count as usize;

        // Pre-allocate LED buffer (3 bytes per cell: RGB)
        let led_buffer = vec![0u8; cell_count * 3];

        let mut grid = ReceiverGrid {
            main_rect,
            cells: Vec::new(),
            led_count,
            cols,
            rows,
            device: ReceiverDevice::default(),
            show_debug_info: debug,
            led_buffer,
            cell_w: 0.0,
            cell_h: 0.0,
            layout_mode,
            persistence: PresetManager::new_grid(PresetMode::Grid, "Leds 1".to_string()),
            selection_mode: SelectionMode::Rows,
            show_grid: false,
            edit_mode: false,
            row_offsets: vec![vec2(0.0, 0.0); rows as usize],
            column_offsets: vec![vec2(0.0, 0.0); cols as usize],
            drag_target: None,
            drag_anchor_mouse: pt2(0.0, 0.0),
            drag_anchor_offset: vec2(0.0, 0.0),
        };

        grid.update_cells();
        grid
    }

    pub fn update_cells(&mut self) {
        self.rows = self.rows.max(1);
        self.led_count = self.led_count.max(1);
        self.cols = ReceiverGrid::required_cols(self.led_count, self.rows);
        self.ensure_offset_counts();

        self.cells = ReceiverGrid::create_grid(
            &self.main_rect,
            self.rows,
            self.cols,
            self.led_count,
            self.layout_mode.clone(),
            &self.row_offsets,
            &self.column_offsets,
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

        if self.has_offsets() {
            return (
                0,
                self.cols.saturating_sub(1),
                0,
                self.rows.saturating_sub(1),
            );
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
        led_count: u32,
        layout_mode: LayoutMode,
        row_offsets: &[Vec2],
        column_offsets: &[Vec2],
    ) -> Vec<GridCell> {
        let mut grid: Vec<GridCell> = Vec::new();
        if cols == 0 || rows == 0 || led_count == 0 {
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
                        if _pos >= led_count {
                            return grid;
                        }
                        let row_offset = row_offsets
                            .get(r as usize)
                            .copied()
                            .unwrap_or_else(|| vec2(0.0, 0.0));
                        let column_offset = column_offsets
                            .get(c as usize)
                            .copied()
                            .unwrap_or_else(|| vec2(0.0, 0.0));
                        let x = start_x + c as f32 * cell_w;
                        let y = start_y - r as f32 * cell_h;
                        let total_offset = row_offset + column_offset;
                        let cell_rect = Rect::from_x_y_w_h(
                            x + total_offset.x,
                            y + total_offset.y,
                            cell_w,
                            cell_h,
                        );
                        grid.push(GridCell::new_from_rect(cell_rect, _pos, r, c));
                        _pos += 1;
                    }
                }
            }
            LayoutMode::FollowColum => {
                let mut _pos = 0;
                for c in 0..cols {
                    for r in 0..rows {
                        if _pos >= led_count {
                            return grid;
                        }
                        let row_offset = row_offsets
                            .get(r as usize)
                            .copied()
                            .unwrap_or_else(|| vec2(0.0, 0.0));
                        let column_offset = column_offsets
                            .get(c as usize)
                            .copied()
                            .unwrap_or_else(|| vec2(0.0, 0.0));
                        let x = start_x + c as f32 * cell_w;
                        let y = start_y - r as f32 * cell_h;
                        let total_offset = row_offset + column_offset;
                        let cell_rect = Rect::from_x_y_w_h(
                            x + total_offset.x,
                            y + total_offset.y,
                            cell_w,
                            cell_h,
                        );
                        grid.push(GridCell::new_from_rect(cell_rect, _pos, r, c));
                        _pos += 1;
                    }
                }
            }
        }

        grid
    }

    fn required_cols(led_count: u32, rows: u32) -> u32 {
        let safe_led_count = led_count.max(1);
        let safe_rows = rows.max(1);
        ((safe_led_count as f32) / (safe_rows as f32)).ceil() as u32
    }

    fn ensure_offset_counts(&mut self) {
        let required_rows = self.rows as usize;
        if self.row_offsets.len() < required_rows {
            self.row_offsets.resize(required_rows, vec2(0.0, 0.0));
        } else if self.row_offsets.len() > required_rows {
            self.row_offsets.truncate(required_rows);
        }

        let required_cols = self.cols as usize;
        if self.column_offsets.len() < required_cols {
            self.column_offsets.resize(required_cols, vec2(0.0, 0.0));
        } else if self.column_offsets.len() > required_cols {
            self.column_offsets.truncate(required_cols);
        }

        if let Some(target) = self.drag_target {
            match target {
                DragTarget::Row(row) if row >= required_rows => self.drag_target = None,
                DragTarget::Column(col) if col >= required_cols => self.drag_target = None,
                _ => {}
            }
        }
    }

    fn has_offsets(&self) -> bool {
        self.row_offsets
            .iter()
            .any(|offset| offset.length_squared() > f32::EPSILON)
            || self
                .column_offsets
                .iter()
                .any(|offset| offset.length_squared() > f32::EPSILON)
    }

    pub fn draw(&self, draw: &Draw) {
        if !self.show_grid {
            return;
        }

        for cell in &self.cells {
            draw.rect()
                .xy(cell.rect.xy())
                .wh(cell.rect.wh())
                .stroke_color(SNOW)
                .stroke_weight(1.0)
                .color(cell.get_display_color());

            if self.show_debug_info {
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

    pub fn mouse_pressed(&mut self, mouse_pos: Point2, button: MouseButton) {
        if !self.edit_mode || button != MouseButton::Left {
            return;
        }

        self.drag_target = match self.selection_mode {
            SelectionMode::Rows => self.row_at_point(mouse_pos).map(DragTarget::Row),
            SelectionMode::Columns => self.col_at_point(mouse_pos).map(DragTarget::Column),
        };

        if let Some(target) = self.drag_target {
            self.drag_anchor_mouse = mouse_pos;
            self.drag_anchor_offset = match target {
                DragTarget::Row(row_idx) => self
                    .row_offsets
                    .get(row_idx)
                    .copied()
                    .unwrap_or_else(|| vec2(0.0, 0.0)),
                DragTarget::Column(col_idx) => self
                    .column_offsets
                    .get(col_idx)
                    .copied()
                    .unwrap_or_else(|| vec2(0.0, 0.0)),
            };
        }
    }

    pub fn mouse_moved(&mut self, mouse_pos: Point2) {
        if !self.edit_mode {
            return;
        }

        let Some(target) = self.drag_target else {
            return;
        };

        let drag_delta = mouse_pos - self.drag_anchor_mouse;
        let new_offset = self.drag_anchor_offset + drag_delta;

        let mut updated = false;
        match target {
            DragTarget::Row(row_idx) => {
                if let Some(row_offset) = self.row_offsets.get_mut(row_idx) {
                    *row_offset = new_offset;
                    updated = true;
                }
            }
            DragTarget::Column(col_idx) => {
                if let Some(col_offset) = self.column_offsets.get_mut(col_idx) {
                    *col_offset = new_offset;
                    updated = true;
                }
            }
        }

        if updated {
            self.update_cells();
        }
    }

    pub fn mouse_released(&mut self, button: MouseButton) {
        if button == MouseButton::Left {
            self.drag_target = None;
        }
    }

    fn row_at_point(&self, point: Point2) -> Option<usize> {
        self.cells
            .iter()
            .find(|cell| cell.rect.contains(point))
            .map(|cell| cell.row as usize)
    }

    fn col_at_point(&self, point: Point2) -> Option<usize> {
        self.cells
            .iter()
            .find(|cell| cell.rect.contains(point))
            .map(|cell| cell.col as usize)
    }

    fn reset_offsets(&mut self) {
        for row_offset in self.row_offsets.iter_mut() {
            *row_offset = vec2(0.0, 0.0);
        }
        for col_offset in self.column_offsets.iter_mut() {
            *col_offset = vec2(0.0, 0.0);
        }
        self.update_cells();
    }

    fn to_save_data(&self) -> ReceiverGridSaveData {
        ReceiverGridSaveData {
            device_name: self.device.name.clone(),
            device_ip: self.device.ip.clone(),
            grid_center: [self.main_rect.x(), self.main_rect.y()],
            grid_size: [self.main_rect.w(), self.main_rect.h()],
            led_count: self.led_count,
            rows: self.rows,
            cols: self.cols,
            show_debug_info: self.show_debug_info,
            show_grid: self.show_grid,
            layout_mode: self.layout_mode.clone(),
            selection_mode: self.selection_mode,
            row_offsets: self
                .row_offsets
                .iter()
                .copied()
                .map(OffsetSave::from_vec2)
                .collect(),
            column_offsets: self
                .column_offsets
                .iter()
                .copied()
                .map(OffsetSave::from_vec2)
                .collect(),
        }
    }

    fn save_settings(&self) -> Result<bool, anyhow::Error> {
        let save_data = self.to_save_data();
        let filename = if self.device.name.trim().is_empty() {
            "device".to_string()
        } else {
            self.device.name.trim().to_string()
        };
        self.persistence.save_to_file(&save_data, Some(filename))
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
            // TODO validate IP
            self.device.ip = ip;
            changed = true;
        }

        if ui.checkbox(&mut self.edit_mode, "Edit").clicked() {
            changed = true;
        }

        ui.horizontal(|ui| {
            ui.label("Selection:");
            changed |= ui
                .radio_value(&mut self.selection_mode, SelectionMode::Rows, "Rows")
                .changed();
            changed |= ui
                .radio_value(&mut self.selection_mode, SelectionMode::Columns, "Columns")
                .changed();
        });

        ui.horizontal(|ui| {
            ui.label("LED count:");
            if ui
                .add(
                    egui::DragValue::new(&mut self.led_count)
                        .clamp_range(1..=10_000)
                        .speed(1),
                )
                .changed()
            {
                self.update_cells();
                changed = true;
            }
        });

        ui.horizontal(|ui| {
            ui.label("Rows:");
            if ui
                .add(
                    egui::DragValue::new(&mut self.rows)
                        .clamp_range(1..=1_000)
                        .speed(1),
                )
                .changed()
            {
                self.update_cells();
                changed = true;
            }
        });

        ui.label(format!(
            "Grid: {} rows x {} cols ({} LEDs)",
            self.rows,
            self.cols,
            self.cells.len()
        ));

        if ui.button("Reset row/column offsets").clicked() {
            self.reset_offsets();
            changed = true;
        }

        if self.edit_mode {
            match self.selection_mode {
                SelectionMode::Rows => {
                    ui.label("Tip: drag a row with left mouse button.");
                }
                SelectionMode::Columns => {
                    ui.label("Tip: drag a column with left mouse button.");
                }
            }
        }

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
            match self.save_settings() {
                Ok(saved) => changed |= saved,
                Err(err) => eprintln!("Failed to save receiver grid settings: {}", err),
            }
        }

        changed
    }
}
