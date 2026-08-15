use crate::{
    animator::{
        AnimatedObject, AnimatorSettings, ObjectShape, UpdateBehaviour,
        animation_type::AnimationType,
    },
    color::ColorParam,
    modulator::Modulator,
    parameters::ModulatedParam,
    timecode::TimeCode,
};
use anyhow::Ok;
use nannou::prelude::*;
use nannou_egui::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StrobeSettings {
    pub rate: ModulatedParam,
    pub duty_cycle: ModulatedParam,
    pub chroma_offset: ModulatedParam,
    color: ColorParam,
}

impl StrobeSettings {
    pub fn new(_win_rect: &Rect) -> Self {
        Self {
            rate: ModulatedParam::new(1.0, 0.25, 8.0, "Rate (per beat)", "strobe_rate"),
            duty_cycle: ModulatedParam::new(0.3, 0.01, 0.99, "Duty Cycle", "strobe_duty"),
            chroma_offset: ModulatedParam::new(0.0, 0.0, 60.0, "Chroma Offset", "strobe_chroma"),
            color: ColorParam::default(),
        }
    }
}

impl AnimatorSettings for StrobeSettings {
    fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
        vec![
            &mut self.rate,
            &mut self.duty_cycle,
            &mut self.chroma_offset,
        ]
    }

    fn ui(&mut self, ui: &mut egui::Ui, mods: &mut Modulator) -> UpdateBehaviour {
        let mut change = UpdateBehaviour::None;

        ui.heading(format!("{}", self.animation_type()));

        if self.rate.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.duty_cycle.to_slider_modulate(ui, mods) {
            change = UpdateBehaviour::HotUpdate;
        }
        if self.chroma_offset.to_slider_modulate(ui, mods) {
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
            *self.rate.value(),
            *self.duty_cycle.value(),
            *self.chroma_offset.value(),
        ))]
    }

    fn set_dimension(&mut self, _window_rect: &Rect) {}

    fn hot_update(&self, objects: &mut Vec<Box<dyn AnimatedObject>>) {
        for obj in objects.iter_mut() {
            if let Some(s) = obj.as_any_mut().downcast_mut::<Strobe>() {
                s.color = self.color.clone().value_mapped(0);
                s.rate = *self.rate.value();
                s.duty_cycle = *self.duty_cycle.value();
                s.chroma_offset = *self.chroma_offset.value();
            }
        }
    }

    fn reset(&mut self) {
        self.rate.reset();
        self.duty_cycle.reset();
        self.chroma_offset.reset();
    }

    fn save_preset(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

pub struct Strobe {
    pub color: Rgba8,
    pub rate: f32,
    pub duty_cycle: f32,
    pub chroma_offset: f32,
    is_on: bool,
    win_size: Vec2,
}

impl Strobe {
    pub fn new(color: Rgba8, rate: f32, duty_cycle: f32, chroma_offset: f32) -> Self {
        Self {
            color,
            rate,
            duty_cycle,
            chroma_offset,
            is_on: false,
            win_size: vec2(800.0, 600.0),
        }
    }
}

impl AnimatedObject for Strobe {
    fn update(&mut self, win_rect: &Rect, _delta_time: f32, clock: &TimeCode) {
        self.win_size = vec2(win_rect.w(), win_rect.h());

        // Phase within one strobe cycle — rate flashes per beat
        let beat_pos = clock.get_beats() * self.rate;
        let cycle_phase = beat_pos.fract();
        self.is_on = cycle_phase < self.duty_cycle;
    }

    fn draw(&self, draw: &Draw) {
        if !self.is_on {
            return;
        }

        let w = self.win_size.x;
        let h = self.win_size.y;

        if self.chroma_offset > 0.5 {
            // Chroma split: draw three offset rectangles (R, G, B channels)
            let off = self.chroma_offset;
            let r = rgba8(self.color.red, 0, 0, self.color.alpha);
            let g = rgba8(0, self.color.green, 0, self.color.alpha);
            let b = rgba8(0, 0, self.color.blue, self.color.alpha);

            draw.rect().x(-off).y(0.0).width(w).height(h).color(r);
            draw.rect().x(0.0).y(0.0).width(w).height(h).color(g);
            draw.rect().x(off).y(0.0).width(w).height(h).color(b);
        } else {
            draw.rect().x(0.0).y(0.0).width(w).height(h).color(self.color);
        }
    }

    fn shape(&self) -> ObjectShape {
        if self.is_on {
            ObjectShape::Rect(Rect::from_x_y_w_h(0.0, 0.0, self.win_size.x, self.win_size.y))
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
