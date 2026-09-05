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
use anyhow::Ok;
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StrobeSettings {
    pub rate: ConstantParam<u8>,
    pub duty_cycle: ModulatedParam,
    color: ColorParam,
}

impl StrobeSettings {
    pub fn new(_win_rect: &Rect) -> Self {
        Self {
            rate: ConstantParam::new(1, 1, 4, "Rate (per beat)", "strobe_rate"),
            duty_cycle: ModulatedParam::new(0.3, 0.01, 0.99, "Duty Cycle", "strobe_duty"),
            color: ColorParam::default(),
        }
    }
}

impl AnimatorSettings for StrobeSettings {
    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
        vec![&mut self.duty_cycle]
    }

    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut Vec<Box<dyn Modulator>>) -> UpdateBehaviour {
        let mut change = UpdateBehaviour::None;

        if self.rate.to_slider(ui) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.duty_cycle.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }

        if self.color.ui(ui) {
            change = UpdateBehaviour::HotUpdate;
        }

        change
    }

    fn animation_type(&self) -> AnimationType {
        AnimationType::Strobe
    }

    fn create(&self) -> Vec<Box<dyn AnimatedObject>> {
        vec![Box::new(Strobe::new(
            self.color.clone().value_mapped(0),
            self.rate.value,
            *self.duty_cycle.value(),
        ))]
    }

    fn set_dimension(&mut self, _window_rect: &Rect) {}

    fn hot_update(&self, objects: &mut Vec<Box<dyn AnimatedObject>>) {
        for obj in objects.iter_mut() {
            if let Some(s) = obj.as_any_mut().downcast_mut::<Strobe>() {
                s.color = self.color.clone().value_mapped(0);
                s.duty_cycle = *self.duty_cycle.value();
            }
        }
    }

    fn reset(&mut self) {
        self.rate.reset();
        self.duty_cycle.reset();
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

pub struct Strobe {
    pub color: Rgba8,
    pub rate: u8,
    pub duty_cycle: f32,
    is_on: bool,
    win_size: Vec2,
}

impl Strobe {
    pub fn new(color: Rgba8, rate: u8, duty_cycle: f32) -> Self {
        Self {
            color,
            rate,
            duty_cycle,
            is_on: false,
            win_size: vec2(800.0, 600.0),
        }
    }
}

impl AnimatedObject for Strobe {
    fn update(&mut self, win_rect: &Rect, clock: &TimeCode) {
        self.win_size = vec2(win_rect.w(), win_rect.h());

        // Phase within one strobe cycle — rate flashes per beat
        let beat_pos = clock.get_beats() * self.rate as f32;
        let cycle_phase = beat_pos.fract();
        self.is_on = cycle_phase < self.duty_cycle;
    }

    fn draw(&self, draw: &Draw) {
        if !self.is_on {
            return;
        }

        let w = self.win_size.x;
        let h = self.win_size.y;

        draw.rect()
            .x(0.0)
            .y(0.0)
            .width(w)
            .height(h)
            .color(self.color);
    }

    fn shape(&self) -> ObjectShape {
        if self.is_on {
            ObjectShape::Rect(Rect::from_x_y_w_h(
                0.0,
                0.0,
                self.win_size.x,
                self.win_size.y,
            ))
        } else {
            // Zero-size when off so it doesn't activate any LEDs
            ObjectShape::Rect(Rect::from_x_y_w_h(0.0, 0.0, 0.0, 0.0))
        }
    }

    fn color(&self) -> Rgba8 {
        self.color
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
