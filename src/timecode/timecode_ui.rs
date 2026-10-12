use crate::timecode::TimeCode;
use crate::ui::icons::{ICON_PAUSE, ICON_PLAY, ICON_STOP};
use crate::ui::style_definitions::custom_colors::ACCENT;
use nannou_egui::egui;

impl TimeCode {
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

        self.tempo.to_drag(ui);

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
            link_text = link_text.color(ACCENT);
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
