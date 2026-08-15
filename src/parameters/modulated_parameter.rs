use crate::{
    modulator::{ModRoute, ModTarget, Modulator},
    ui::controls::styled_dual_slider,
};
use nannou_egui::egui::{self, Label};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ModulatedParam {
    pub value: f32,
    #[serde(skip_serializing)]
    pub modulated_value: f32,
    #[serde(skip_serializing)]
    pub default_value: f32,
    pub range: (f32, f32),
    #[serde(skip_serializing)]
    pub display_text: String,
    #[serde(skip_serializing)]
    pub modulation_active: bool,
    #[serde(skip_serializing)]
    pub ghost_value: Option<f32>,
    #[serde(skip_serializing)]
    pub mod_target: ModTarget,
    // unique id for routing and persistence
    pub identifier: String,
    #[serde(skip_serializing)]
    pub mod_amount: f32,
}

impl ModulatedParam {
    const SPACE: f32 = 5.0;
    pub fn new(
        default_value: f32,
        lower: f32,
        upper: f32,
        display_text: &str,
        identifier: &str,
    ) -> Self {
        Self {
            value: default_value,
            modulated_value: default_value,
            default_value,
            range: (lower, upper),
            display_text: display_text.to_string(),
            ghost_value: None,
            modulation_active: false,
            mod_target: ModTarget(identifier.clone().to_string()),
            mod_amount: 1.0,
            identifier: identifier.clone().to_string(),
        }
    }

    pub fn reset(&mut self) {
        self.value = self.default_value;
    }

    pub fn connect_modulation(&self, mods: &mut Modulator) {
        if !self.mod_target.is_none() {
            mods.routes.push(ModRoute::new(self.mod_target.clone()));
        }
    }

    pub fn modulate(&mut self, beat_pos: f32, mod_matrix: &Modulator) {
        if self.modulation_active {
            let speed = self.value
                * mod_matrix.calc_modulation(beat_pos, &self.mod_target)
                * self.mod_amount;
            self.ghost_value = Some(speed);
            self.modulated_value = speed;
        }
    }

    pub fn value(&self) -> &f32 {
        if self.modulation_active {
            if let Some(_ghost_val) = self.ghost_value {
                &self.modulated_value
            } else {
                &self.value
            }
        } else {
            &self.value
        }
    }

    pub fn to_slider_modulate(&mut self, ui: &mut egui::Ui, mods: &mut Modulator) -> bool {
        ui.add_space(Self::SPACE);
        let mut changed = false;
        ui.add(Label::new(self.display_text.to_string()));

        let mut mod_desc = "U";

        if self.modulation_active {
            mod_desc = "M";
        }

        ui.horizontal(|ui| {
            changed |= ui
                .add(styled_dual_slider(
                    &mut self.value,
                    self.ghost_value,
                    self.range.0..=self.range.1,
                    "",
                ))
                .changed();

            if ui.button("↻").clicked() {
                changed = true;
                self.reset();
            }
            if ui.button(mod_desc).clicked() {
                self.modulation_active = !self.modulation_active;
                mods.set_enables(self.modulation_active, &self.mod_target);
                changed = true;
            }

            // TODO add clamp range
            if self.ghost_value.is_some() {
                ui.add(
                    egui::DragValue::new(&mut self.mod_amount)
                        .speed(0.1)
                        .clamp_range(0.000..=1.000),
                );
            }
        });
        changed
    }
}
