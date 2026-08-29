use crate::ui::controls::single_slider_styled;
use bevy_egui::egui::{self};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, PartialOrd)]
pub struct ConstantParam<T> {
    pub value: T,
    pub default: T,
    pub lower: T,
    pub upper: T,
    #[serde(skip_serializing)]
    pub display_text: String,
    pub identifier: String,
}

impl<T> ConstantParam<T> {
    pub fn new(default: T, lower: T, upper: T, display_text: &str, identifier: &str) -> Self
    where
        T: Clone,
    {
        Self {
            value: default.clone(),
            default: default,
            lower,
            upper,
            display_text: display_text.to_string(),
            identifier: identifier.to_string(),
        }
    }

    pub fn to_drag(&mut self, ui: &mut egui::Ui) -> bool
    where
        T: egui::emath::Numeric + Clone,
    {
        let mut changed = false;
        ui.horizontal(|ui| {
            changed |= ui
                .add(
                    egui::DragValue::new(&mut self.value)
                        .speed(1)
                        .range(self.lower..=self.upper),
                )
                .changed();
            if ui.button("↻").clicked() {
                changed = true;
                self.reset();
            }
            ui.label(self.display_text.to_string());
        })
        .inner;

        changed
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
                    self.lower..=self.upper,
                ))
                .changed();
            if ui.button("↻").clicked() {
                changed = true;
                self.reset();
            }
            ui.label(self.display_text.to_string());
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
