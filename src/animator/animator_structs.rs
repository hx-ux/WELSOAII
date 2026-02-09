use nannou_egui::egui::{self};
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
        }
    }

    pub fn reset(&mut self)
    where
        T: Clone,
    {
        self.value = self.default.clone();
    }

    pub fn to_slider(&mut self, ui: &mut egui::Ui) -> bool
    where
        T: egui::emath::Numeric + Clone,
    {
        ui.add_space(Self::SPACE);
        ui.label(&self.display_text);

        let mut changed = false;
        ui.horizontal(|ui| {
            changed |= ui
                .add(egui::Slider::new(
                    &mut self.value,
                    self.range.lower..=self.range.upper,
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
}
