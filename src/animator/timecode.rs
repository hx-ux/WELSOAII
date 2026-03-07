// use nannou::{ draw::background::new};
use nannou_egui::egui::{self, Rect};
use rusty_link::{AblLink, SessionState};
use serde::{Deserialize, Serialize};
// https://github.com/anzbert/rusty_link/blob/master/examples/link_hut_silent/main.rs

// Currently read only
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

    pub fn commit_app_state(&mut self) {
        self.link.commit_app_session_state(&self.session_state);
    }
}

impl Default for AblLinkState {
    fn default() -> Self {
        Self {
            link: AblLink::new(120.),
            session_state: Default::default(),
            quantum: Default::default(),
        }
    }
}

/// Master Clock for syncing all effects
#[derive(Serialize, Deserialize, Default)]
pub struct TimeCode {
    pub tempo: f32,
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
            tempo: 120.0,
            current_time: 0.0,
            total_beats: 0.0,
            is_running: true,
            // is_linked: false,
            abl_sync_state: AblLinkState::new(),
            sync_active: false,
            prev_time: 0.0,
            delta_time: 0.0,
        }
    }

    pub fn start_link(&mut self) {
        println!("toggle link");
        self.sync_active = true;
        self.abl_sync_state.link.enable(true);
    }
    pub fn stop_link(&mut self) {
        println!("toggle link");
        self.sync_active = false;
        self.abl_sync_state.link.enable_start_stop_sync(false);
    }

    pub fn set_bpm(&mut self, bpm: f32) {
        self.tempo = bpm.max(1.0).min(240.0);
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
                self.total_beats += (self.tempo / 60.0) * delta_time;
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

        // Get the current time
        let time = self.abl_sync_state.link.clock_micros();

        // Get the current beat position from Link's session
        // This will always give a beat position, even with no peers
        let beats = self
            .abl_sync_state
            .session_state
            .beat_at_time(time, self.abl_sync_state.quantum);

        self.total_beats = beats as f32;

        self.tempo = self.abl_sync_state.session_state.tempo() as f32;
        self.current_time = (self.total_beats * 60.0) / self.tempo;

        // let phase = self.ablLsync.session_state.session_state.phase_at_time(time, state.quantum);
        self.abl_sync_state.commit_app_state();
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

    pub fn get_beat_progress(&self) -> f32 {
        self.total_beats.fract()
    }

    // pub fn get_bar_progress(&self) -> u8 {
    //     (self.total_beats / 4.0).fract() as u8
    // }

    pub fn get_formatted_time(&self) -> String {
        let minutes = (self.current_time / 60.0).floor() as i32;
        let seconds = (self.current_time % 60.0).floor() as i32;
        let milliseconds = ((self.current_time % 1.0) * 1000.0).floor() as i32;
        format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds)
    }

    // todo ableton live
    pub fn get_beat_counter(&self) -> (i32, f32) {
        let beat = self.total_beats.floor() as i32 % 4 + 1;
        let progress = self.get_beat_progress();
        (beat, progress)
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.heading("Master Clock");

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

            ui.label("BPM:");
            let mut bpm = self.tempo;
            // todo bpm should always be int

            if ui.add(egui::DragValue::new(&mut bpm).speed(0.5)).changed() {
                bpm = bpm.clamp(60.0, 240.0);
                self.set_bpm(bpm);
            }

            for i in 1..5 {
                let mut col = egui::Color32::from_rgba_premultiplied(0, 0, 0, 255);

                if i == self.get_beat_counter().0 {
                    col = egui::Color32::from_rgba_premultiplied(255, 255, 255, 255);
                }

                let color_preview =
                    ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());

                ui.painter().rect_filled(color_preview.0, 4.0, col);
            }
        });

        ui.horizontal(|ui| {
            ui.label(format!("Time: {}", self.get_formatted_time()));

            let (beat, progress) = self.get_beat_counter();

            ui.label(format!("Beat: {} ({:.2})", beat, progress));
            ui.label(format!("Total Beats: {} ", self.total_beats as usize));
        });

        let enabled = match self.sync_active {
            true => "Disconnect Link",
            false => "Connect Link",
        };

        if ui.button(enabled).clicked() {
            if self.sync_active {
                self.stop_link();
            } else {
                self.start_link();
            }
        }
    }
}
