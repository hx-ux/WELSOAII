use nannou_egui::egui::{self, RichText};

const HISTORY_SECS: f32 = 5.0;

// ─────────────────────────────────────────────────────────────────────────────

pub struct PerfStats {
    frame_history: egui::util::History<f32>,
}

impl PerfStats {
    pub fn new() -> Self {
        Self {
            frame_history: egui::util::History::new(0..usize::MAX, HISTORY_SECS),
        }
    }

    pub fn on_new_frame(&mut self, now_secs: f64, delta_secs: f32) {
        self.frame_history.add(now_secs, delta_secs);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let fps = self.current_fps();
        ui.label(RichText::new(format!("FPS  {:.1}", fps)).monospace());
    }

    fn current_fps(&self) -> f32 {
        match self.frame_history.mean_time_interval() {
            Some(dt) if dt > 0.0 => 1.0 / dt,
            _ => 0.0,
        }
    }
}
