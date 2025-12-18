extern crate nannou;

use crate::{
    Utils::*,
    reciver_device::{self, SingleLed},
};
use nannou::prelude::*;
use nannou_egui::egui;
use reciver_device::*;

#[derive(Clone)]
pub struct ReciverCell {
    pub pos: u16,
    pub rect: Rect,
    pub found_color: Rgba,
    pub is_active: bool,
}

impl ReciverCell {
    pub fn new(x: f32, y: f32, w: f32, h: f32, pos: u16) -> Self {
        ReciverCell {
            rect: Rect::from_x_y_w_h(x, y, w, h),
            is_active: false,
            found_color: Rgba::standard(),
            pos,
        }
    }
    pub fn new_from_rect(r: Rect, pos: u16) -> Self {
        ReciverCell {
            rect: r,
            is_active: false,
            found_color: Rgba::standard(),
            pos,
        }
    }

    pub fn reset(&mut self) {
        self.is_active = false;
        self.found_color = Rgba::standard();
    }
    pub fn as_led(&self) -> SingleLed {
        SingleLed::new_rgba(self.found_color, self.pos)
    }
}

#[derive(Clone)]
pub struct ReciverGrid {
    main_rect: Rect,
    pub cells: Vec<ReciverCell>,
    pub cols: i32,
    pub rows: i32,
    device: ReciverDevice,
    pub is_locked: bool,
}

impl ReciverGrid {
    pub fn new(main_rect: Rect, cols: i32, rows: i32) -> Self {
        let mut grid = ReciverGrid {
            main_rect,
            cells: Vec::new(),
            cols,
            rows,
            device: ReciverDevice::new(
                "192.168.178.102".to_string(),
                Vec::new(),
                "name".to_string(),
            ),
            is_locked: true,
        };

        grid.update_cells();
        grid
    }

    pub fn update_cells(&mut self) {
        self.cells.clear();

        if self.cols == 0 || self.rows == 0 {
            return;
        }

        let cell_w = self.main_rect.w() / self.cols as f32;
        let cell_h = self.main_rect.h() / self.rows as f32;
        let start_x = self.main_rect.left() + cell_w / 2.0;
        let start_y = self.main_rect.top() - cell_h / 2.0;

        // TODO Sloppy Implementation
        let g: Vec<SingleLed> = Vec::new();

        let mut _pos = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                let x = start_x + c as f32 * cell_w;
                let y = start_y - r as f32 * cell_h;
                let cell_rect = Rect::from_x_y_w_h(x, y, cell_w, cell_h);
                self.cells.push(ReciverCell::new_from_rect(cell_rect, _pos));
                _pos += 1;
            }
        }

        let mut s: Vec<SingleLed> = Vec::new();

        for f in &self.cells {
            s.push(f.as_led());
        }

        // self.device.send_test_data();
        // replace contents of shared led buffer
        {
            // let mut leds_lock = self.device.leds;
            // *leds_lock = s;
        }

        // send an initial frame immediately, and ensure the background sender is running.
        // self.device.send_test_data();
        // Start background sender (50 ms ~ 20fps). If you want only one thread, guard-start externally.
        self.device.start_sender(50);
    }

    pub fn draw(&self, draw: &Draw) {
        for cell in &self.cells {
            let color = if cell.is_active {
                cell.found_color
            } else {
                Rgba::new(0.1, 0.1, 0.1, 0.1)
            };
            // Cells
            draw.rect()
                .xy(cell.rect.xy())
                .wh(cell.rect.wh())
                .stroke_color(BLACK)
                .stroke_weight(1.0)
                .color(color);
            // Backdrop / Outer Rect
            draw.rect()
                .xy(cell.rect.xy())
                .wh(cell.rect.wh())
                .no_fill()
                .stroke_weight(1.0)
                .stroke(SNOW);
        }
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

        ui.label(&self.device.name);
        ui.label(&self.device.ip);

        if changed {
            self.update_cells();
        }

        changed
    }
}
