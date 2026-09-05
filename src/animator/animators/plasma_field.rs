use crate::{
    animator::{
        AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour,
        animation_type::AnimationType,
    },
    color::ColorParam,
    modulator::Modulator,
    parameters::{ConstantParam, ModulatedParam},
    timecode::TimeCode,
};
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};

fn default_rect() -> Rect {
    Rect::from_w_h(800.0, 600.0)
}

/// Settings for the Plasma Field effect.
/// A classic interference-wave plasma that tiles the screen into cells,
/// each colored by a sum of sine fields — creating a living, morphing color surface.
#[derive(Serialize, Deserialize)]
pub struct PlasmaFieldSettings {
    pub tile_size: ConstantParam<u32>,
    pub freq_x: ModulatedParam,
    pub freq_y: ModulatedParam,
    pub freq_radial: ModulatedParam,
    pub speed: ModulatedParam,
    pub palette_speed: ModulatedParam,
    pub beat_kick: ModulatedParam,
    color: ColorParam,
    #[serde(skip)]
    #[serde(default = "default_rect")]
    dimension: Rect,
}

impl PlasmaFieldSettings {
    pub fn new(win_rect: &Rect) -> Self {
        Self {
            tile_size: ConstantParam::new(40, 40, 80, "Tile Size", "plasma_tile"),
            freq_x: ModulatedParam::new(0.018, 0.002, 0.1, "Freq X", "plasma_fx"),
            freq_y: ModulatedParam::new(0.022, 0.002, 0.1, "Freq Y", "plasma_fy"),
            freq_radial: ModulatedParam::new(0.012, 0.0, 0.08, "Freq Radial", "plasma_fr"),
            speed: ModulatedParam::new(1.0, 0.0, 6.0, "Speed", "plasma_speed"),
            palette_speed: ModulatedParam::new(0.4, 0.0, 3.0, "Palette Speed", "plasma_palette"),
            beat_kick: ModulatedParam::new(0.5, 0.0, 3.0, "Beat Kick", "plasma_kick"),
            color: ColorParam::default(),
            dimension: *win_rect,
        }
    }
}

impl AnimatorSettings for PlasmaFieldSettings {
    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
        vec![
            &mut self.freq_x,
            &mut self.freq_y,
            &mut self.freq_radial,
            &mut self.speed,
            &mut self.palette_speed,
            &mut self.beat_kick,
        ]
    }

    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut Vec<Box<dyn Modulator>>) -> UpdateBehaviour {
        let mut change = UpdateBehaviour::None;
        ui.heading(format!("{}", self.animation_type()));

        if self.tile_size.to_slider(ui) {
            change = UpdateBehaviour::NeedsReset;
        }
        if self.freq_x.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.freq_y.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.freq_radial.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.speed.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.palette_speed.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.beat_kick.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.color.ui(ui) {
            change = UpdateBehaviour::HotUpdate;
        }
        change
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::Plasma
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        let tile = self.tile_size.value.max(1) as f32;
        let cols = (self.dimension.w() / tile).ceil() as usize + 1;
        let rows = (self.dimension.h() / tile).ceil() as usize + 1;

        (0..cols * rows)
            .map(|idx| {
                let col = idx % cols;
                let row = idx / cols;
                let cx = self.dimension.left() + col as f32 * tile + tile * 0.5;
                let cy = self.dimension.bottom() + row as f32 * tile + tile * 0.5;
                Box::new(PlasmaCell::new(
                    cx,
                    cy,
                    tile,
                    *self.freq_x.value(),
                    *self.freq_y.value(),
                    *self.freq_radial.value(),
                    *self.speed.value(),
                    *self.palette_speed.value(),
                    *self.beat_kick.value(),
                    self.color.clone(),
                )) as Box<dyn AnimatedObject>
            })
            .collect()
    }

    fn set_dimension(&mut self, window_rect: &Rect) {
        self.dimension = *window_rect;
    }

    fn hot_update(&self, objects: &mut Vec<Box<dyn AnimatedObject>>) {
        for obj in objects.iter_mut() {
            if let Some(cell) = obj.as_any_mut().downcast_mut::<PlasmaCell>() {
                cell.freq_x = *self.freq_x.value();
                cell.freq_y = *self.freq_y.value();
                cell.freq_radial = *self.freq_radial.value();
                cell.speed = *self.speed.value();
                cell.palette_speed = *self.palette_speed.value();
                cell.beat_kick = *self.beat_kick.value();
                cell.color_param = self.color.clone();
            }
        }
    }

    fn reset(&mut self) {
        self.freq_x.reset();
        self.freq_y.reset();
        self.freq_radial.reset();
        self.speed.reset();
        self.palette_speed.reset();
        self.beat_kick.reset();
    }

    fn force_update(&self) -> UpdateBehaviour {
        UpdateBehaviour::NeedsReset
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

/// A single tile in the plasma grid.
/// Its color is computed from overlapping sine fields evaluated at its fixed position,
/// shifted by a global time phase so the field appears to flow and breathe.
pub struct PlasmaCell {
    cx: f32,
    cy: f32,
    size: f32,
    pub freq_x: f32,
    pub freq_y: f32,
    pub freq_radial: f32,
    pub speed: f32,
    pub palette_speed: f32,
    pub beat_kick: f32,
    pub color_param: ColorParam,
    time: f32,
    current_color: Rgba8,
}

impl PlasmaCell {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cx: f32,
        cy: f32,
        size: f32,
        freq_x: f32,
        freq_y: f32,
        freq_radial: f32,
        speed: f32,
        palette_speed: f32,
        beat_kick: f32,
        color_param: ColorParam,
    ) -> Self {
        Self {
            cx,
            cy,
            size,
            freq_x,
            freq_y,
            freq_radial,
            speed,
            palette_speed,
            beat_kick,
            color_param,
            time: random_range(0.0_f32, TAU),
            current_color: Rgba8::new(0, 0, 0, 255),
        }
    }

    /// Sample the plasma field at this cell's position.
    /// Returns a value in [0, 1] that drives the palette lookup.
    fn sample_field(&self, beat_kick_amp: f32) -> f32 {
        let t = self.time;
        let cx = self.cx;
        let cy = self.cy;
        let radial = (cx * cx + cy * cy).sqrt();

        let v1 = (cx * self.freq_x + t).sin();
        let v2 = (cy * self.freq_y + t * 1.3 + 0.7).sin();
        let v3 = (radial * self.freq_radial + t * 0.8 + beat_kick_amp).sin();
        // Diagonal interference for extra richness
        let v4 = ((cx + cy) * self.freq_x * 0.7 + t * 0.6).sin();

        // Sum and remap [-4, 4] → [0, 1]
        (v1 + v2 + v3 + v4 + 4.0) / 8.0
    }

    /// Map a [0, 1] sample to a palette color, using smooth linear interpolation
    /// between adjacent palette entries so there are no sharp steps.
    fn palette_color(&self, sample: f32) -> Rgba8 {
        let palette = self.color_param.clone().value_mapped(0);
        // For solid mode, modulate brightness with the plasma sample
        let brightness = (sample * TAU).sin() * 0.5 + 0.5;
        let r = (palette.red as f32 * brightness) as u8;
        let g = (palette.green as f32 * brightness) as u8;
        let b = (palette.blue as f32 * brightness) as u8;

        // In palette mode, we interpolate across palette entries
        let color_entries = {
            use crate::color::ColorParam;
            // Build a synthetic palette using value_mapped across a sweep
            (0..8)
                .map(|i| {
                    let dummy = ColorParam {
                        single_color: Rgba8::new(r, g, b, 255),
                        mode: self.color_param.mode,
                        palette: self.color_param.palette,
                    };
                    dummy.value_mapped(i)
                })
                .collect::<Vec<_>>()
        };

        let count = color_entries.len();
        if count == 0 {
            return Rgba8::new(r, g, b, 255);
        }

        // Drive palette index with plasma sample + slow time offset
        let index_f =
            (sample + self.time * self.palette_speed * 0.05).rem_euclid(1.0) * (count - 1) as f32;
        let lo = index_f.floor() as usize;
        let hi = (lo + 1).min(count - 1);
        let t = index_f.fract();

        let a = color_entries[lo];
        let b = color_entries[hi];
        Rgba8::new(
            lerp_u8(a.red, b.red, t),
            lerp_u8(a.green, b.green, t),
            lerp_u8(a.blue, b.blue, t),
            255,
        )
    }
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).clamp(0.0, 255.0) as u8
}

impl AnimatedObject for PlasmaCell {
    fn update(&mut self, _win_rect: &Rect, clock: &TimeCode) {
        self.time += clock.get_delta_time() * self.speed;

        // Beat kick: on each beat, inject an extra phase burst into the radial field
        let beat_progress = clock.get_beat_progress();
        // Sharp attack, exponential decay within the beat
        let kick_env = if beat_progress < 0.05 {
            1.0
        } else {
            (-beat_progress * 6.0).exp()
        };
        let beat_kick_amp = kick_env * self.beat_kick;

        let sample = self.sample_field(beat_kick_amp);
        self.current_color = self.palette_color(sample);
    }

    fn draw(&self, draw: &Draw) {
        draw.rect()
            .x_y(self.cx, self.cy)
            .w_h(self.size + 0.5, self.size + 0.5) // slight overdraw avoids seams
            .color(self.current_color);
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Rect(Rect::from_x_y_w_h(self.cx, self.cy, self.size, self.size))
    }

    fn color(&self) -> Rgba8 {
        self.current_color
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
