// use crate::{
//     animator::{AnimatedObject, AnimatorSettings, ObjectShape, animation_type::AnimationType},
//     color::ColorParam,
//     modulator::Modulator,
//     parameters::{ConstantParam, ModulatedParam},
//     timecode::TimeCode,
// };
// use anyhow::Ok;
// use nannou::prelude::*;
// use nannou_egui::egui;
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// pub struct StrobeSettings {
//     pub rate: ConstantParam<u8>,
//     pub duty_cycle: ModulatedParam,
//     color: ColorParam,
//     #[serde(skip)]
//     win_size: Vec2,
//     #[serde(skip)]
//     animator: Vec<Strobe>,
//     visible: bool,
// }

// impl StrobeSettings {
//     pub fn new(win_rect: &Rect) -> Self {
//         Self {
//             rate: ConstantParam::new(1, 1, 4, "Rate (per beat)", "strobe_rate"),
//             duty_cycle: ModulatedParam::new(0.3, 0.01, 0.99, "Duty Cycle", "strobe_duty"),
//             color: ColorParam::default(),
//             win_size: vec2(win_rect.w(), win_rect.h()),
//             animator: Vec::new(),
//             visible: true,
//         }
//     }
// }

// impl AnimatorSettings for StrobeSettings {
//     fn control_ui(&mut self, ui: &mut egui::Ui, mods: &[Box<dyn Modulator>]) {
//         if self.rate.slider_ui(ui) {
//             self.hot_update();
//         }
//         if self.duty_cycle.slider_ui(ui, mods) {
//             self.hot_update();
//         }
//         if self.color.ui(ui, mods) {
//             self.hot_update();
//         }
//     }

//     fn animation_type(&self) -> AnimationType {
//         AnimationType::Strobe
//     }

//     fn init(&mut self) {
//         self.animator.clear();
//         self.animator.push(Strobe::from_settings(self));
//     }

//     fn set_dimension(&mut self, window_rect: &Rect) {
//         self.win_size = vec2(window_rect.w(), window_rect.h());
//     }

//     fn hot_update(&mut self) {
//         let color = self.color.value_mapped(0);
//         let rate = self.rate.value;
//         let duty_cycle = self.duty_cycle.value();

//         for obj in self.animator.iter_mut() {
//             obj.color = color;
//             obj.rate = rate;
//             obj.duty_cycle = duty_cycle;
//             obj.visible = self.visible;
//         }
//     }

//     fn draw(&self, draw: &Draw) {
//         for s in self.animator.iter() {
//             s.draw(draw);
//         }
//     }

//     fn update(&mut self, win_rect: &Rect, timecode: &TimeCode) {
//         for s in self.animator.iter_mut() {
//             s.update(win_rect, timecode);
//         }
//     }

//     fn get_objects(&self) -> Vec<&dyn AnimatedObject> {
//         self.animator
//             .iter()
//             .map(|s| s as &dyn AnimatedObject)
//             .collect()
//     }

//     fn get_objects_mut(&mut self) -> Vec<&mut dyn AnimatedObject> {
//         self.animator
//             .iter_mut()
//             .map(|s| s as &mut dyn AnimatedObject)
//             .collect()
//     }

//     fn modulated_params_mut(&mut self) -> Vec<&mut ModulatedParam> {
//         let mut params = vec![&mut self.duty_cycle];
//         params.extend(self.color.modulated_params_mut());
//         params
//     }

//     fn snapshot(&mut self) -> anyhow::Result<()> {
//         Ok(())
//     }

//     fn set_visibility(&mut self, state: bool) {
//         self.visible = state;
//         self.hot_update();
//     }

//     fn get_visibility(&self) -> bool {
//         self.visible
//     }
// }

// pub struct Strobe {
//     pub color: Rgba8,
//     pub rate: u8,
//     pub duty_cycle: f32,
//     is_on: bool,
//     win_size: Vec2,
//     visible: bool,
// }

// impl Strobe {
//     pub fn from_settings(settings: &StrobeSettings) -> Self {
//         Self {
//             color: settings.color.value_mapped(0),
//             rate: settings.rate.value,
//             duty_cycle: settings.duty_cycle.value(),
//             is_on: false,
//             win_size: settings.win_size,
//             visible: settings.visible,
//         }
//     }
// }

// impl AnimatedObject for Strobe {
//     fn update(&mut self, win_rect: &Rect, clock: &TimeCode) {
//         self.win_size = vec2(win_rect.w(), win_rect.h());

//         // Phase within one strobe cycle — rate flashes per beat
//         let beat_pos = clock.get_beats() * self.rate as f32;
//         let cycle_phase = beat_pos.fract();
//         self.is_on = cycle_phase < self.duty_cycle;
//     }

//     fn draw(&self, draw: &Draw) {
//         if !self.is_on || !self.visible {
//             return;
//         }

//         draw.rect()
//             .x(0.0)
//             .y(0.0)
//             .width(self.win_size.x)
//             .height(self.win_size.y)
//             .color(self.color);
//     }

//     fn shape(&self) -> ObjectShape {
//         if self.is_on && self.visible {
//             ObjectShape::Rect(Rect::from_x_y_w_h(
//                 0.0,
//                 0.0,
//                 self.win_size.x,
//                 self.win_size.y,
//             ))
//         } else {
//             // Zero-size when off so it doesn't activate any LEDs
//             ObjectShape::Rect(Rect::from_x_y_w_h(0.0, 0.0, 0.0, 0.0))
//         }
//     }

//     fn color(&self) -> Rgba8 {
//         self.color
//     }
// }
