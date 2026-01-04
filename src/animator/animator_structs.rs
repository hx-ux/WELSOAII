use nannou_egui::egui::{self, Ui};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use std::ops::RangeInclusive;

/// Defines a simple range between two values
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RangeHolder<T> {
    pub lower: T,
    pub upper: T,
}

impl<T> RangeHolder<T> {
    pub fn new(lower: T, upper: T) -> Self {
        Self { lower, upper }
    }
    pub fn as_range(&self) -> RangeInclusive<T>
    where
        T: Copy,
    {
        RangeInclusive::new(self.lower, self.upper)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimationParam<T> {
    pub value: T,
    pub range: RangeHolder<T>,
    pub desc: String,
}

impl<T> AnimationParam<T> {
    pub fn new(default: T, lower: T, upper: T, desc: &str) -> Self
    where
        T: Copy,
    {
        Self {
            value: default,
            range: RangeHolder { lower, upper },
            desc: desc.to_string(),
        }
    }
    pub fn new_with_range(default: T, range: RangeHolder<T>, desc: &str) -> Self
    where
        T: Copy,
    {
        Self {
            value: default,
            range,
            desc: desc.to_string(),
        }
    }
    pub fn to_slider(&mut self, ui: &mut egui::Ui) -> bool
    where
        T: egui::emath::Numeric,
    {
        ui.add(
            egui::Slider::new(&mut self.value, self.range.lower..=self.range.upper)
                .text(self.desc.to_string()),
        )
        .changed()
    }
}
