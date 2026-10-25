use nannou_egui::egui;
use serde::{Deserialize, Serialize};

use crate::ui::icons::ICON_REFRESH;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, PartialOrd)]
pub struct ConstantParam<T> {
    pub value: T,
    pub default: T,
    pub lower: T,
    pub upper: T,
    display_text: String,
    pub identifier: String,
    pub steps: Option<f32>,
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
            steps: None,
        }
    }

    #[allow(dead_code)]
    pub fn steps(mut self, steps: f32) -> Self {
        self.steps = Some(steps);
        self
    }

    pub fn drag_ui(&mut self, ui: &mut egui::Ui) -> bool
    where
        T: egui::emath::Numeric + Clone,
    {
        let mut changed = false;
        ui.vertical(|ui| {
            if !self.display_text.is_empty() {
                ui.label(&self.display_text);
            }
            ui.horizontal(|ui| {
                changed |= ui
                    .add(
                        egui::DragValue::new(&mut self.value)
                            .speed(1)
                            .clamp_range(self.lower..=self.upper),
                    )
                    .changed();
                if ui.button(ICON_REFRESH).clicked() {
                    changed = true;
                    self.reset();
                }
            });
        });

        changed
    }

    pub fn slider_ui(&mut self, ui: &mut egui::Ui) -> bool
    where
        T: egui::emath::Numeric + Clone,
    {
        let mut changed = false;
        ui.vertical(|ui| {
            if !self.display_text.is_empty() {
                ui.label(&self.display_text);
            }
            ui.horizontal(|ui| {
                let mut slider = egui::Slider::new(&mut self.value, self.lower..=self.upper);

                if let Some(steps) = self.steps {
                    slider = slider.step_by(steps as f64);
                }

                changed |= ui.add(slider).changed();

                if ui.button(ICON_REFRESH).clicked() {
                    changed = true;
                    self.reset();
                }
            });
        });

        changed
    }

    pub fn reset(&mut self)
    where
        T: Clone,
    {
        self.value = self.default.clone();
    }
}

impl ConstantParam<bool> {
    pub fn checkbox_ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.horizontal(|ui| {
            changed |= ui.checkbox(&mut self.value, &self.display_text).changed();
        });

        changed
    }
}
