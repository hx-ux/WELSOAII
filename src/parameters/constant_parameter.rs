use crate::ui::controls::single_slider_styled;
use nannou_egui::egui::{self};
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
    /// Optional step size used in sliders and drag widgets.
    #[serde(skip_serializing)]
    pub step: Option<f32>,
}

impl<T> ConstantParam<T> {
    pub fn new(default: T, lower: T, upper: T, display_text: &str, identifier: &str) -> Self
    where
        T: Clone,
    {
        Self {
            value: default.clone(),
            default,
            lower,
            upper,
            display_text: display_text.to_string(),
            identifier: identifier.to_string(),
            step: None,
        }
    }

    /// Set a step size for this parameter (builder pattern).
    pub fn with_step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn to_drag(&mut self, ui: &mut egui::Ui) -> bool
    where
        T: egui::emath::Numeric + Clone,
    {
        let mut changed = false;
        let speed = self.step.unwrap_or(1.0);

        ui.horizontal(|ui| {
            changed |= ui
                .add(
                    egui::DragValue::new(&mut self.value)
                        .speed(speed)
                        .clamp_range(self.lower..=self.upper),
                )
                .changed();
            if ui.button("↻").clicked() {
                changed = true;
                self.reset();
            }
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
            let mut slider = single_slider_styled(&mut self.value, self.lower..=self.upper, "");
            if let Some(step) = self.step {
                slider = slider.step_by(2.0);
            }
            changed |= ui.add(slider).changed();

            if ui.button("↻").clicked() {
                changed = true;
                self.reset();
            }
            ui.label(&self.display_text);
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
