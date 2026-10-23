use crate::{
    parameters::ConstantParam,
    timecode::link_state::LinkState,
    ui::icons::{ICON_PAUSE, ICON_PLAY, ICON_STOP},
};
use nannou_egui::egui;
use serde::{Deserialize, Serialize};

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
    pub abl_sync_state: LinkState,
    pub link_active: bool,
    #[serde(skip)]
    prev_time: f32,
    #[serde(skip)]
    delta_time: f32,
}

impl TimeCode {
    pub fn new() -> Self {
        Self {
            tempo: ConstantParam::new(120.0, 40.0, 240.0, "", "tempo"),
            current_time: 0.0,
            total_beats: 0.0,
            is_running: true,
            abl_sync_state: LinkState::new(),
            link_active: false,
            prev_time: 0.0,
            delta_time: 0.0,
        }
    }

    // Start Ableton link
    pub fn start_link(&mut self) {
        self.link_active = true;
        self.abl_sync_state.link.enable(true);
    }

    // Stop Link
    pub fn stop_link(&mut self) {
        self.link_active = false;
        self.abl_sync_state.link.enable(false);
    }

    // Start thw whole timecode
    pub fn start(&mut self) {
        self.is_running = true;
    }

    // stop the whole timecode
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
            if self.link_active {
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn get_formatted_time(&self) -> String {
        let minutes = (self.current_time / 60.0).floor() as i32;
        let seconds = (self.current_time % 60.0).floor() as i32;
        let milliseconds = ((self.current_time % 1.0) * 1000.0).floor() as i32;
        format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds)
    }

    pub fn get_beat_counter(&self) -> (i32, f32) {
        let beat = (self.total_beats.floor() as i32).wrapping_rem_euclid(4) + 1;
        let progress = self.get_beat_progress();
        (beat, progress)
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if ui
            .button(if self.is_running {
                ICON_PAUSE
            } else {
                ICON_PLAY
            })
            .clicked()
        {
            if self.is_running {
                self.stop();
            } else {
                self.start();
            }
        }

        if ui.button(ICON_STOP).clicked() {
            self.stop();
            self.reset();
        }

        ui.add_space(4.0);

        if self.link_active {
            ui.label(format!("{:.1}", self.tempo.value));
        } else {
            self.tempo.drag_ui(ui);
        }

        for i in 1..=4 {
            let mut col = egui::Color32::from_gray(40);
            if i == self.get_beat_counter().0 {
                let brightness = ((1.0 - self.get_beat_progress()) * 255.0) as u8;
                col = egui::Color32::from_rgb(brightness, 200, 80);
            }
            let rect = ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
            ui.painter().rect_filled(rect.0, 4.5, col);
        }

        let mut link_text = egui::RichText::new("LINK");

        if self.link_active {
            let peers = self.abl_sync_state.link.num_peers();
            link_text = egui::RichText::new(format!("LINK: {}", peers));
            link_text = link_text.color(egui::Color32::BLUE);
        }

        if ui.button(link_text).clicked() {
            if self.link_active {
                self.stop_link();
            } else {
                self.start_link();
            }
        }
        self.abl_sync_state.capture_app_state();
    }
}
