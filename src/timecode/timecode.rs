use nannou_egui::egui;
use rusty_link::{AblLink, SessionState};
use serde::{Deserialize, Serialize};

use crate::parameters::ConstantParam;

/// Currently read only
pub struct AblLinkState {
    pub link: AblLink,
    pub session_state: SessionState,
    pub quantum: f64,
}

impl AblLinkState {
    pub fn new() -> Self {
        Self {
            link: AblLink::new(120.),
            session_state: SessionState::new(),
            quantum: 4.,
        }
    }

    pub fn capture_app_state(&mut self) {
        self.link.capture_app_session_state(&mut self.session_state);
    }
}

impl Default for AblLinkState {
    fn default() -> Self {
        Self::new()
    }
}

/// Master Clock for syncing all effects
#[derive(Serialize, Deserialize, Default)]
pub struct TimeCode {
    pub tempo: ConstantParam<f32>,
    #[serde(skip)]
    pub current_time: f32,
    #[serde(skip)]
    pub total_beats: f32,
    #[serde(skip)]
    pub is_running: bool,
    #[serde(skip)]
    abl_sync_state: AblLinkState,
    sync_active: bool,
    #[serde(skip)]
    prev_time: f32,
    #[serde(skip)]
    delta_time: f32,
}

impl TimeCode {
    pub fn new() -> Self {
        Self {
            tempo: ConstantParam {
                value: 120.0,
                default: 120.0,
                lower: 40.0,
                upper: 240.0,
                display_text: "".to_string(),
                identifier: "tempo".to_string(),
            },
            current_time: 0.0,
            total_beats: 0.0,
            is_running: true,
            abl_sync_state: AblLinkState::new(),
            sync_active: false,
            prev_time: 0.0,
            delta_time: 0.0,
        }
    }

    pub fn start_link(&mut self) {
        self.sync_active = true;
        self.abl_sync_state.link.enable(true);
    }

    pub fn stop_link(&mut self) {
        self.sync_active = false;
        self.abl_sync_state.link.enable(false);
    }

    pub fn start(&mut self) {
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn reset(&mut self) {
        self.current_time = 0.0;
        self.total_beats = 0.0;
        self.prev_time = 0.0;
        self.delta_time = 0.0;
    }

    pub fn update(&mut self, delta_time: f32) -> f32 {
        if self.is_running {
            if self.sync_active {
                self.update_from_link();
            } else {
                self.current_time += delta_time;
                self.total_beats += (self.tempo.value / 60.0) * delta_time;
            }
            self.delta_time = (self.current_time - self.prev_time).max(0.0);
            self.prev_time = self.current_time;
        } else {
            self.delta_time = 0.0;
        }
        self.delta_time
    }

    fn update_from_link(&mut self) {
        self.abl_sync_state.capture_app_state();

        let time = self.abl_sync_state.link.clock_micros();
        let beats = self
            .abl_sync_state
            .session_state
            .beat_at_time(time, self.abl_sync_state.quantum);

        self.total_beats = beats as f32;
        self.tempo.value = self.abl_sync_state.session_state.tempo() as f32;
        self.current_time = (self.total_beats * 60.0) / self.tempo.value;
    }

    pub fn get_time(&self) -> f32 {
        self.current_time
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn get_beats(&self) -> f32 {
        self.total_beats
    }

    pub fn get_beat_fract(&self) -> f32 {
        self.total_beats.fract()
    }

    pub fn get_formatted_time(&self) -> String {
        let minutes = (self.current_time / 60.0).floor() as i32;
        let seconds = (self.current_time % 60.0).floor() as i32;
        let milliseconds = ((self.current_time % 1.0) * 1000.0).floor() as i32;
        format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds)
    }

    pub fn get_beat_counter(&self) -> (i32, f32) {
        let beat = (self.total_beats.floor() as i32).wrapping_rem_euclid(4) + 1;
        let progress = self.get_beat_fract();
        (beat, progress)
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button(if self.is_running { "⏸" } else { "▶" }).clicked() {
                if self.is_running {
                    self.stop();
                } else {
                    self.start();
                }
            }

            if ui.button("⏹").clicked() {
                self.stop();
                self.reset();
            }

            ui.add_space(4.0);

            if self.sync_active {
                ui.label(format!("{:.1}", self.tempo.value));
            } else {
                self.tempo.to_drag(ui);
            }

            for i in 1..=4 {
                let mut col = egui::Color32::from_gray(40);
                if i == self.get_beat_counter().0 {
                    let brightness = ((1.0 - self.get_beat_fract()) * 255.0) as u8;
                    col = egui::Color32::from_rgb(brightness, 200, 80);
                }
                let rect = ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
                ui.painter().rect_filled(rect.0, 4.5, col);
            }

            let mut link_text = egui::RichText::new("LINK");

            if self.sync_active {
                let peers = self.abl_sync_state.link.num_peers();
                link_text = egui::RichText::new(format!("LINK: {}", peers));
                link_text = link_text.color(egui::Color32::BLUE);
            }

            if ui.button(link_text).clicked() {
                if self.sync_active {
                    self.stop_link();
                } else {
                    self.start_link();
                }
            }
        });

        self.abl_sync_state.capture_app_state();
    }
}
