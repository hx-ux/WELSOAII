use nannou::{color::Rgba, lyon::path::iterator};
use nannou_egui::egui::{self, Ui};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
// use strum::{IntoEnumIterator, AsRefStr};
use std::{default, ops::RangeInclusive};

use crate::animator::animation_type::{AnimationType, ModeHelper};

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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AnimationParam<T> {
    pub value: T,
    #[serde(skip_serializing)]
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
    pub fn new_without_range(default: T, desc: &str) -> Self
    where
        T: Copy,
        T: Default,
    {
        Self {
            value: default,
            range: RangeHolder::default(),
            desc: desc.to_string(),
        }
    }
    pub fn new_options(default: T, desc: &str) -> Self
    where
        T: Copy,
        T: Default,
    {
        Self {
            value: default,
            range: RangeHolder::default(),
            desc: desc.to_string(),
        }
    }

    pub fn to_slider(&mut self, ui: &mut egui::Ui) -> bool
    where
        T: egui::emath::Numeric,
    {
        ui.add(egui::Slider::new(
            &mut self.value,
            self.range.lower..=self.range.upper,
        ))
        .changed()
    }

    // pub fn to_options(&mut self, ui: &mut egui::Ui) -> bool
    // where
    //     T: Copy + PartialEq,
    // {

    //     // let mut changed = false;
    //     // ui.horizontal(|ui| {



    //     //     for variant in T::iter() {
    //     //         if ui
    //     //             .radio_value(&mut self.value, *variant, variant.as_str())
    //     //             .changed()
    //     //         {
    //     //             changed = true;
    //     //         }
    //     //     }
    //     // });
    //     // changed
    // }
}

impl AnimationParam<Rgba> {
    pub fn to_color_picker(&mut self, ui: &mut egui::Ui) -> bool {
        let (r, g, b, a): (f32, f32, f32, f32) = self.value.into();
        let mut col = egui::Rgba::from_rgba_unmultiplied(r, g, b, a);
        let changed = egui::color_picker::color_edit_button_rgba(
            ui,
            &mut col,
            egui::color_picker::Alpha::BlendOrAdditive,
        )
        .changed();
        if changed {
            self.value = Rgba::new(col.r(), col.g(), col.b(), col.a());
        }
        changed
    }
}
