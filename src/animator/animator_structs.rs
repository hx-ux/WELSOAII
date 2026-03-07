use crate::{
    modulator::{ModMatrix, ModRoute, ModTarget},
    ui::controls::styled_slider,
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
        {
            self.lower = self.default.0.clone();
            self.upper = self.default.1.clone();
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AnimationParam<T> {
    pub value: T,
    #[serde(skip_serializing)]
    pub default: T,
    // #[serde(skip_serializing)]
    pub range: RangeHolder<T>,
    #[serde(skip_serializing)]
    pub display_text: String,
    #[serde(skip_serializing)]
    pub modulator_active: bool,
    #[serde(skip_serializing)]
    pub ghost_value: Option<T>,
    pub mod_target: ModTarget,
}

impl<T> AnimationParam<T> {
    const SPACE: f32 = 5.0;

    pub fn new(default: T, lower: T, upper: T, desc: &str) -> Self
    where
        T: Clone,
    {
        Self {
            value: default.clone(),
            default,
            range: RangeHolder::new(lower, upper),
            display_text: desc.to_string(),
            ghost_value: None,
            modulator_active: false,
            mod_target: ModTarget::None,
            // modulator:None
        }
    }

    pub fn new_modulate(default: T, lower: T, upper: T, desc: &str, mod_target: ModTarget) -> Self
    where
        T: Clone,
    {
        Self {
            value: default.clone(),
            default,
            range: RangeHolder::new(lower, upper),
            display_text: desc.to_string(),
            ghost_value: None,
            modulator_active: false,
            mod_target,
        }
    }

    pub fn reset(&mut self)
    where
        T: Clone,
    {
        self.value = self.default.clone();
    }

    pub fn connect_modulation(&self, mods: &mut ModMatrix) {
        mods.routes.push(ModRoute::new(self.mod_target));
    }

    fn to_slider_core(&mut self, ui: &mut egui::Ui) -> bool
    where
        T: egui::emath::Numeric + Clone,
    {
        let mut changed = false;
        ui.horizontal(|ui| {
            changed |= ui
                .add(styled_slider(
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

    pub fn to_slider_modulate(&mut self, ui: &mut egui::Ui, mods: &mut ModMatrix) -> bool
    where
        T: egui::emath::Numeric + Clone,
    {
        ui.add_space(Self::SPACE);
        let mut changed = false;
        ui.add(Label::new(self.display_text.to_string()));

        if self.mod_target != ModTarget::None {
            let color = if self.modulator_active {
                egui::Color32::GREEN
            } else {
                egui::Color32::RED
            };

            let button = egui::Button::new("M1").fill(color);

            if ui.add(button).clicked() {
                self.modulator_active = !self.modulator_active;
                mods.set_enables(self.modulator_active, self.mod_target);
                changed = true;
            }
        }

        if self.mod_target != ModTarget::None {
            if let Some(mut ghost) = self.ghost_value {
                ui.add_enabled(
                    false,
                    egui::Slider::new(&mut ghost, self.range.lower..=self.range.upper)
                        .show_value(false),
                );
            }
        }

        changed |= self.to_slider_core(ui);
        changed
    }

    pub fn to_slider(&mut self, ui: &mut egui::Ui) -> bool
    where
        T: egui::emath::Numeric + Clone,
    {
        ui.add_space(Self::SPACE);
        let mut changed = false;
        ui.add(Label::new(self.display_text.to_string()));

        changed |= self.to_slider_core(ui);
        changed
    }
}
