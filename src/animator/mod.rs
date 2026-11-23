use crate::{
    animator::animation_type::{AnimationType, LayerBlendMode, LayerPolygonMode},
    modulator::Modulator,
    parameters::ModulatedParam,
    timecode::TimeCode,
};
use anyhow::Result;
use nannou::prelude::*;

pub mod animation_type;
#[allow(clippy::module_inception)]
pub mod animator;
mod animators;

pub use animator::Animator;

pub enum ObjectShape {
    Circle(Vec2, f32),
    Ring {
        center: Vec2,
        radius: f32,
        thickness: f32,
    },
    Rect(Rect),
    RotatedRect {
        center: Vec2,
        size: Vec2,
        angle: f32,
    },
    Polyline {
        points: Vec<Point2>,
        thickness: f32,
    },
}

impl ObjectShape {
    /// Returns axis-aligned bounding box `(left, right, bottom, top)` in world coordinates.
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        match self {
            ObjectShape::Circle(pos, radius) => (
                pos.x - radius,
                pos.x + radius,
                pos.y - radius,
                pos.y + radius,
            ),
            ObjectShape::Ring {
                center,
                radius,
                thickness,
            } => {
                let r = radius + thickness * 0.5;
                (center.x - r, center.x + r, center.y - r, center.y + r)
            }
            ObjectShape::Rect(r) => (r.left(), r.right(), r.bottom(), r.top()),
            ObjectShape::RotatedRect {
                center,
                size,
                angle,
            } => {
                let (sin_a, cos_a) = angle.sin_cos();
                let hx = size.x * 0.5;
                let hy = size.y * 0.5;
                let ext_x = hx * cos_a.abs() + hy * sin_a.abs();
                let ext_y = hx * sin_a.abs() + hy * cos_a.abs();
                (
                    center.x - ext_x,
                    center.x + ext_x,
                    center.y - ext_y,
                    center.y + ext_y,
                )
            }
            ObjectShape::Polyline { points, thickness } => {
                if points.is_empty() {
                    return (0.0, 0.0, 0.0, 0.0);
                }
                let mut min_x = f32::INFINITY;
                let mut max_x = f32::NEG_INFINITY;
                let mut min_y = f32::INFINITY;
                let mut max_y = f32::NEG_INFINITY;
                for p in points {
                    min_x = min_x.min(p.x);
                    max_x = max_x.max(p.x);
                    min_y = min_y.min(p.y);
                    max_y = max_y.max(p.y);
                }
                let pad = thickness * 0.5;
                (min_x - pad, max_x + pad, min_y - pad, max_y + pad)
            }
        }
    }

    /// Tests whether this shape intersects a `ReceiverGrid` cell, respecting wireframe mode.
    pub fn intersects_cell(&self, cell_rect: &Rect, wireframe: bool) -> bool {
        let cell_center = cell_rect.xy();
        let cell_w = cell_rect.w();
        let cell_h = cell_rect.h();
        let cell_min_dim = cell_w.min(cell_h);

        match self {
            ObjectShape::Circle(pos, radius) => {
                if *radius <= 0.0 {
                    return false;
                }
                if !wireframe {
                    let closest_x = pos.x.clamp(cell_rect.left(), cell_rect.right());
                    let closest_y = pos.y.clamp(cell_rect.bottom(), cell_rect.top());
                    ((pos.x - closest_x).powi(2) + (pos.y - closest_y).powi(2)) < (radius * radius)
                } else {
                    let dist = cell_center.distance(*pos);
                    let half_band = (cell_min_dim * 0.65).max(2.0);
                    (dist - *radius).abs() <= half_band
                }
            }
            ObjectShape::Ring {
                center,
                radius,
                thickness,
            } => {
                if *radius <= 0.0 {
                    return false;
                }
                let dist = cell_center.distance(*center);
                let half_band = (thickness * 0.5).max(cell_min_dim * 0.55);
                (dist - *radius).abs() <= half_band
            }
            ObjectShape::Rect(r) => {
                if r.w() <= 0.0 || r.h() <= 0.0 {
                    return false;
                }
                let inside = r.left() < cell_rect.right()
                    && r.right() > cell_rect.left()
                    && r.top() > cell_rect.bottom()
                    && r.bottom() < cell_rect.top();
                if !inside || !wireframe {
                    return inside;
                }
                let band_x = cell_w.max(2.0);
                let band_y = cell_h.max(2.0);
                (cell_center.x - r.left()).abs() <= band_x
                    || (r.right() - cell_center.x).abs() <= band_x
                    || (cell_center.y - r.bottom()).abs() <= band_y
                    || (r.top() - cell_center.y).abs() <= band_y
            }
            ObjectShape::RotatedRect {
                center,
                size,
                angle,
            } => {
                if size.x <= 0.0 || size.y <= 0.0 {
                    return false;
                }
                let delta = cell_center - *center;
                let (sin_a, cos_a) = (-*angle).sin_cos();
                let local_x = (delta.x * cos_a - delta.y * sin_a).abs();
                let local_y = (delta.x * sin_a + delta.y * cos_a).abs();
                let hx = size.x * 0.5 + cell_w * 0.35;
                let hy = size.y * 0.5 + cell_h * 0.35;
                if local_x > hx || local_y > hy {
                    return false;
                }
                if !wireframe {
                    true
                } else {
                    let band_x = cell_w.max(2.0);
                    let band_y = cell_h.max(2.0);
                    local_x >= (hx - band_x).max(0.0) || local_y >= (hy - band_y).max(0.0)
                }
            }
            ObjectShape::Polyline { points, thickness } => {
                if points.len() < 2 {
                    return false;
                }
                let threshold = (thickness * 0.5) + cell_min_dim * 0.55;
                let threshold_sq = threshold * threshold;
                for seg in points.windows(2) {
                    if point_segment_dist_sq(cell_center, seg[0], seg[1]) <= threshold_sq {
                        return true;
                    }
                }
                false
            }
        }
    }
}

fn point_segment_dist_sq(p: Vec2, a: Point2, b: Point2) -> f32 {
    let ab = b - a;
    let len_sq = ab.length_squared();
    if len_sq <= 1e-6 {
        return p.distance_squared(a);
    }
    let t = ((p - a).dot(ab) / len_sq).clamp(0.0, 1.0);
    let proj = a + ab * t;
    p.distance_squared(proj)
}

pub trait AnimatedObject {
    fn update(&mut self, win_rect: &Rect, clock: &TimeCode);
    fn draw(&self, draw: &Draw);
    fn shape(&self) -> ObjectShape;
    fn color(&self) -> Srgba;
}

pub trait AnimatorSettings: Send + Sync {
    fn control_ui(&mut self, ui: &mut egui::Ui, mods: &[Box<dyn Modulator>]);
    fn animation_type(&self) -> AnimationType;
    fn init(&mut self);
    fn set_dimension(&mut self, _window_rect: &Rect) {}
    fn hot_update(&mut self);
    fn draw(&self, draw: &Draw);
    fn update(&mut self, win_rect: &Rect, timecode: &TimeCode);
    fn get_objects(&self) -> Vec<&dyn AnimatedObject>;
    #[allow(dead_code)]
    fn get_objects_mut(&mut self) -> Vec<&mut dyn AnimatedObject>;
    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam>;

    fn update_modulations(&mut self, beat_pos: f32, modulators: &[Box<dyn Modulator>]) {
        let mut any_modulated = false;
        for param in self.modulated_params_mut() {
            param.modulate(beat_pos, modulators);
            if param.modulation_active {
                any_modulated = true;
            }
        }
        if any_modulated {
            self.hot_update();
        }
    }

    fn reset_modulations(&mut self) {
        for param in self.modulated_params_mut() {
            param.ghost_value = None;
        }
    }

    #[allow(dead_code)]
    fn snapshot(&mut self) -> Result<()> {
        Ok(())
    }

    fn set_visibility(&mut self, state: bool);
    fn get_visibility(&self) -> bool;

    fn get_blend_mode(&self) -> LayerBlendMode {
        LayerBlendMode::Normal
    }

    fn set_blend_mode(&mut self, _mode: LayerBlendMode) {}

    fn get_polygon_mode(&self) -> LayerPolygonMode {
        LayerPolygonMode::Fill
    }

    fn set_polygon_mode(&mut self, _mode: LayerPolygonMode) {}
}
