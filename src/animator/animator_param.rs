use crate::{
    modulator::{ModMatrix, ModRoute, ModTarget},
    ui::controls::{DualSlider, single_slider_styled, styled_dual_slider},
};
use nannou_egui::egui::{self, Label};
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RangeHolder<T> {
    pub lower: T,
    pub upper: T,

    #[serde(skip_serializing)]
    pub default: (T, T),
}

impl<T> RangeHolder<T> {
    pub fn new(lower: T, upper: T) -> Self
    where
        T: Clone,
    {
        Self {
            lower: lower.clone(),
            upper: upper.clone(),
            default: (lower, upper),
        }
    }

    pub fn as_range(&self) -> RangeInclusive<T>
    where
        T: Clone,
    {
        RangeInclusive::new(self.lower.to_owned(), self.upper.to_owned())
    }

    pub fn reset(&mut self)
    where
        T: Clone,
    {
        self.lower = self.default.0.clone();
        self.upper = self.default.1.clone();
    }
}
//#[derive(Debug, Serialize, Deserialize, Clone, Default)]
//pub struct AnimationParam<T> {
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConstantParam<T> {
    pub value: T,
    pub default: T,
    pub range: RangeHolder<T>,
    //  pub lower: T,
    // pub upper: T,
    pub display_text: String,
}

impl<T> ConstantParam<T> {
    pub fn new(default: T, lower: T, upper: T, desc: &str) -> Self
    where
        T: Clone,
    {
        Self {
            value: default.clone(),
            default: default,
            //   lower,
            //     upper,
            range: RangeHolder::new(lower.clone(), upper),
            display_text: desc.to_string(),
        }
    }

    pub fn to_slider(&mut self, ui: &mut egui::Ui) -> bool
    where
        T: egui::emath::Numeric + Clone,
    {
        let mut changed = false;
        ui.horizontal(|ui| {
            changed |= ui
                .add(single_slider_styled(
                    &mut self.value,
                    self.range.lower..=self.range.upper,
                    "",
                ))
                .changed();
            if ui.button("↻").clicked() {
                changed = true;
                self.reset();
            }
        })
        .inner;

        changed
    }
    pub fn reset(&mut self)
    where
        T: Clone,
    {
        self.value = self.default.clone();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AnimationParam {
    pub value: f32,
    #[serde(skip_serializing)]
    pub default: f32,
    // #[serde(skip_serializing)]
    pub range: (f32, f32),
    #[serde(skip_serializing)]
    pub display_text: String,
    #[serde(skip_serializing)]
    pub modulation_active: bool,
    #[serde(skip_serializing)]
    pub ghost_value: Option<f32>,
    pub mod_target: ModTarget,
}

impl AnimationParam {
    const SPACE: f32 = 5.0;

    /// A Slider, which cannot be Modulated
    pub fn new(
        default: f32,
        lower: f32,
        upper: f32,
        desc: &str,
        mod_target: Option<ModTarget>,
    ) -> Self {
        let target = mod_target
            .filter(|t| *t != ModTarget::None)
            .unwrap_or(ModTarget::None);

        Self {
            value: default,
            default,
            range: (lower, upper),
            display_text: desc.to_string(),
            ghost_value: None,
            modulation_active: false,
            mod_target: target,
        }
    }

    pub fn reset(&mut self) {
        self.value = self.default;
    }

    pub fn connect_modulation(&self, mods: &mut ModMatrix) {
        if self.mod_target != ModTarget::None {
            mods.routes.push(ModRoute::new(self.mod_target));
        }
    }

    fn to_slider_core(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.horizontal(|ui| {
            changed |= ui
                .add(single_slider_styled(
                    &mut self.value,
                    self.range.0..=self.range.1,
                    "",
                ))
                .changed();
            if ui.button("↻").clicked() {
                changed = true;
                self.reset();
            }
        })
        .inner;

        changed
    }

    pub fn to_slider_modulate(&mut self, ui: &mut egui::Ui, mods: &mut ModMatrix) -> bool {
        ui.add_space(Self::SPACE);
        let mut changed = false;
        ui.add(Label::new(self.display_text.to_string()));

        if self.mod_target != ModTarget::None {
            let color = if self.modulation_active {
                egui::Color32::GREEN
            } else {
                egui::Color32::RED
            };

            let button = egui::Button::new("M1").fill(color);

            if ui.add(button).clicked() {
                self.modulation_active = !self.modulation_active;
                mods.set_enables(self.modulation_active, self.mod_target);
                changed = true;
            }
        }

        // In to_slider_modulate or ghost render:
        if let Some(ghost) = self.ghost_value {
            ui.add(styled_dual_slider(
                &mut self.value,
                Some(ghost),
                self.range.0..=self.range.1,
                "",
            ));
        } else {
            ui.add(single_slider_styled(
                &mut self.value,
                self.range.1..=self.range.1,
                "",
            ));
        }

        //changed |= self.to_slider_core(ui);
        changed
    }

    pub fn to_slider(&mut self, ui: &mut egui::Ui) -> bool {
        ui.add_space(Self::SPACE);
        let mut changed = false;
        ui.add(Label::new(self.display_text.to_string()));

        changed |= self.to_slider_core(ui);
        changed
    }
}
