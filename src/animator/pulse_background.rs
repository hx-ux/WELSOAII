use super::{AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour};
use crate::animator::animation_type::{AnimationType, ModeHelper, PulseModes};
use crate::animator::animator_structs::AnimationParam;
use crate::animator::presets_manager::PresetManager;
use crate::utils::{ColorParam};
use nannou::prelude::*;
use nannou_egui::egui::{self};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PulseBackgroundSettings {
    pub mode: PulseModes,
    pub speed: AnimationParam<f32>,
    pub color: ColorParam,
    pub limit: AnimationParam<f32>,
    #[serde(skip)]
    pub presets: PresetManager<PulseBackgroundSettings>,
}

impl AnimatorSettings for PulseBackgroundSettings {
    fn new(win_rect: &Rect) -> Self {
        Self {
            mode: PulseModes::default(),
            speed: AnimationParam::new(100.0, 1.0, 200.0, "speed"),
            color: ColorParam::default(),
            limit: AnimationParam::new(0.8, 0.1, 1.0, "limit"),
            presets: PresetManager::new_animator(AnimationType::PulseBackground),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> UpdateBehaviour {
        let mut change_type = UpdateBehaviour::None;

        ui.heading(self.animation_type().as_str());

        ui.add_space(5.0);
        ui.label("Mode:");

        ui.horizontal(|ui| {
            for options in PulseModes::iterator() {
                if ui
                    .radio_value(&mut self.mode, *options, options.as_str())
                    .changed()
                {
                    change_type = UpdateBehaviour::NeedsReset;
                };
            }
        });

        if self.speed.to_slider(ui) {
            change_type = UpdateBehaviour::HotUpdate;
        }
        if self.limit.to_slider(ui) {
            change_type = UpdateBehaviour::HotUpdate;
        }
        if self.color.ui(ui) {
            change_type = UpdateBehaviour::HotUpdate;
        }

        let (preset_changed, preset_behaviour) = self.presets.ui(ui);
        if preset_changed {
            change_type = preset_behaviour;
        }

        change_type
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::PulseBackground
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        let mut animated_objects: Vec<Box<dyn AnimatedObject>> = Vec::new();

        animated_objects.push(Box::new(PulseBackground::new(
            self.mode,
            self.color.clone().value_mapped(0),
            self.speed.value,
            self.limit.value,
            0
        )));

        animated_objects
    }

    fn set_dimension(&mut self, window_rect: &Rect) {}

    fn update_behaviour(&self, objects: &mut Vec<Box<dyn AnimatedObject>>) {
        for obj in objects.iter_mut() {
            if let Some(pulse_bg) = obj.as_any_mut().downcast_mut::<PulseBackground>() {
                pulse_bg.color = self.color.clone().value_mapped(pulse_bg.index);
                pulse_bg.speed = self.speed.value;
                pulse_bg.mode = self.mode;
                pulse_bg.limit = self.limit.value;
            }
        }
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        self.presets.save_to_file(*&self, None)?;
        Ok(())
    }

    fn reset(&mut self) {
        todo!()
    }
}

pub struct PulseBackground {
    mode: PulseModes,
    color: Rgba8,
    speed: f32,
    current_size_w: f32,
    current_size_h: f32,
    time: f32,
    limit: f32,
    index:usize,
}

impl PulseBackground {
    fn new(mode: PulseModes, color: Rgba8, speed: f32, limit: f32,index: usize) -> Self {
        Self {
            mode,
            speed,
            color,
            current_size_w: 20.0,
            current_size_h: 20.0,
            time: 0.0,
            limit,
            index
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

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    
    fn color(&self) -> Rgba8 {
       self.color
    }
}
