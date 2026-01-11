use super::{AnimatedObject, AnimatorSettings, ObjectShape};
use crate::animator::animation_type::{AnimationType, ModeHelper, PulseModes};
use crate::animator::animator_structs::AnimationParam;
use crate::animator::presets_manager::PresetManager;
use crate::utils::ColorHelpers;
use nannou::prelude::*;
use nannou_egui::egui::{self};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PulseBackgroundSettings {
    pub mode: PulseModes,
    pub speed: AnimationParam<f32>,
    pub color: AnimationParam<Rgba>,
    pub limit: AnimationParam<f32>,

    #[serde(skip)]
    pub presets: PresetManager,
}

impl AnimatorSettings for PulseBackgroundSettings {
    fn new(win_rect: &Rect) -> Self {
        Self {
            mode: PulseModes::default(),
            speed: AnimationParam::new(100.0, 1.0, 200.0, "Speed"),
            color: AnimationParam::new_without_range(Rgba::red(), "Color"),
            limit: AnimationParam::new(0.8, 0.1, 1.0, "Limit"),
            presets: PresetManager::new(AnimationType::PulseBackground),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.heading(self.animation_type().as_str());
        ui.add_space(5.0);

        ui.label("Mode:");

        ui.horizontal(|ui| {
            for options in PulseModes::iterator() {
                if ui
                    .radio_value(&mut self.mode, *options, options.as_str())
                    .changed()
                {
                    changed = true;
                };
            }
        });

        ui.label("Speed");
        changed |= self.speed.to_slider(ui);

        ui.add_space(5.0);
        ui.label("Limit");
        changed |= self.limit.to_slider(ui);

        ui.add_space(5.0);
        changed |= self.color.to_color_picker(ui);

        ui.add_space(5.0);
        self.presets.ui(ui);
        changed
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::PulseBackground
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        let mut animated_objects: Vec<Box<dyn AnimatedObject>> = Vec::new();

        animated_objects.push(Box::new(PulseBackground::new(
            self.mode,
            self.color.value,
            self.speed.value,
            self.limit.value,
        )));

        animated_objects
    }

    fn set_dimension(&mut self, window_rect: &Rect) {}
}

pub struct PulseBackground {
    mode: PulseModes,
    color: Rgba,
    speed: f32,
    current_size_w: f32,
    current_size_h: f32,
    time: f32,
    limit: f32,
}

impl PulseBackground {
    fn new(mode: PulseModes, color: Rgba, speed: f32, limit: f32) -> Self {
        Self {
            mode,
            speed,
            color,
            current_size_w: 20.0,
            current_size_h: 20.0,
            time: 0.0,
            limit,
        }
    }
}

impl AnimatedObject for PulseBackground {
    fn update(&mut self, win_rect: &Rect, delta_time: f32) {
        let min_w = 20.0;
        let min_h = 20.0;

        let max_w = win_rect.w();
        let max_h = win_rect.h();

        let max_w_allowed = max_w * self.limit;
        let max_h_allowed = max_h * self.limit;

        self.time += delta_time;

        match self.mode {
            PulseModes::Smooth => {
                self.current_size_w = self.time * self.speed;
                self.current_size_h = self.time * self.speed;

                if self.current_size_w >= max_w_allowed || self.current_size_h >= max_h_allowed {
                    self.time = 0.0;
                    self.current_size_w = min_w;
                    self.current_size_h = min_h;
                }
            }
            PulseModes::Elastic => {
                // Use sine wave for rhythmic pulsing (same speed param as Smooth)
                let normalized = ((self.time * self.speed / 100.0).sin() + 1.0) * 0.5; // 0.0..1.0
                let eased = normalized * normalized; // Quadratic easing for smoother effect

                self.current_size_w = min_w + eased * (max_w_allowed - min_w);
                self.current_size_h = min_h + eased * (max_h_allowed - min_h);
            }
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.rect()
            .x(0.0)
            .y(0.0)
            .width(self.current_size_w)
            .height(self.current_size_h)
            .color(self.color);
    }

    fn shape(&self) -> ObjectShape {
        ObjectShape::Rect(Rect::from_x_y_w_h(
            0.0,
            0.0,
            self.current_size_w,
            self.current_size_h,
        ))
    }

    fn color(&self) -> Rgba {
        self.color
    }
}
