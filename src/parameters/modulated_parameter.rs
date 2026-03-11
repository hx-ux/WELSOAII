use crate::{
    modulator::{ModMatrix, ModRoute, ModTarget},
    ui::controls::{single_slider_styled, styled_dual_slider},
};
use nannou_egui::egui::{self, Label};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ModulatedParam {
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
            default,
            range: (lower, upper),
            display_text: desc.to_string(),
            ghost_value: None,
            modulation_active: false,
            mod_target: target,
            mod_amount: 0.0,
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

    pub fn to_slider_modulate(&mut self, ui: &mut egui::Ui, mods: &mut ModMatrix) -> bool {
        ui.add_space(Self::SPACE);
        let mut changed = false;
        ui.add(Label::new(self.display_text.to_string()));

        let mut modDesc = "U";

        if self.modulation_active {
            modDesc = "M";
        }

        ui.horizontal(|ui| {
            if let Some(ghost) = self.ghost_value {
                changed |= ui
                    .add(styled_dual_slider(
                        &mut self.value,
                        Some(ghost),
                        self.range.0..=self.range.1,
                        "",
                    ))
                    .changed();
            } else {
                changed |= ui
                    .add(single_slider_styled(
                        &mut self.value,
                        self.range.0..=self.range.1,
                        "",
                    ))
                    .changed();
            }

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
                ui.add(egui::DragValue::new(&mut self.mod_amount).speed(0.1));
            }
        })
        .inner;
        changed
    }
}
