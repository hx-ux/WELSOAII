use crate::{
    modulator::{ModRoute, ModTarget, Modulator},
    ui::controls::styled_dual_slider,
};
use nannou_egui::egui::{self, Label};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ModulatedParam {
    value: f32,
    #[serde(skip_serializing)]
    pub modulated_value: f32,
    #[serde(skip_serializing)]
    pub default: f32,
    pub range: (f32, f32),
    #[serde(skip_serializing)]
    pub display_text: String,
    #[serde(skip_serializing)]
    pub modulation_active: bool,
    #[serde(skip_serializing)]
    pub ghost_value: Option<f32>,
    pub mod_target: ModTarget,
    #[serde(skip_serializing)]
    pub mod_amount: f32,
}

impl ModulatedParam {
    const SPACE: f32 = 5.0;
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
            modulated_value: default,
            default,
            range: (lower, upper),
            display_text: desc.to_string(),
            ghost_value: None,
            modulation_active: false,
            mod_target: target,
            mod_amount: 1.0,
        }
    }

    pub fn reset(&mut self) {
        self.value = self.default;
    }

    pub fn connect_modulation(&self, mods: &mut Modulator) {
        if self.mod_target != ModTarget::None {
            mods.routes.push(ModRoute::new(self.mod_target));
        }
    }

    pub fn modulate(&mut self, beat_pos: f32, mod_matrix: &Modulator) {
        if self.modulation_active {
            let speed = self.value
                * mod_matrix.calc_modulation(beat_pos, self.mod_target)
                * self.mod_amount;
            self.ghost_value = Some(speed);
            self.modulated_value = speed;
        }
    }

    pub fn value(&self) -> &f32 {
        //&self.value
        if self.modulation_active {
            if let Some(ghost_val) = self.ghost_value {
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

        let mut modDesc = "U";

        if self.modulation_active {
            modDesc = "M";
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
            if ui.button(modDesc).clicked() {
                self.modulation_active = !self.modulation_active;
                mods.set_enables(self.modulation_active, self.mod_target);
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
        })
        .inner;
        changed
    }
}
