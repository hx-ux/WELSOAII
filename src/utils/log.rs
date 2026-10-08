use chrono::Local;
use nannou_egui::egui;
use std::sync::{Arc, Mutex};
enum Severity {
    Info,
    Error,
}

#[derive(Clone, Debug)]
pub struct LogMessage {
    pub timestamp: String,
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct AppLogger {
    logs: Arc<Mutex<Vec<LogMessage>>>,
}

impl Default for AppLogger {
    fn default() -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl AppLogger {
    pub fn log(&self, message: &str) {
        if let Ok(mut logs) = self.logs.lock() {
            logs.push(LogMessage {
                timestamp: Local::now().format("%H:%M:%S").to_string(),
                message: message.to_string(),
            });
            if logs.len() > 200 {
                logs.remove(0);
            }
        }
    }

    pub fn show_window(&self, ctx: &egui::Context, is_open: &mut bool) {
        egui::Window::new("System Log")
            .open(is_open)
            .resizable(true)
            .default_size([400.0, 250.0])
            .show(ctx, |ui| {
                if ui.button("Clear").clicked() {
                    if let Ok(mut logs) = self.logs.lock() {
                        logs.clear();
                    }
                }
                ui.separator();

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        if let Ok(logs) = self.logs.lock() {
                            for log in logs.iter() {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(&log.timestamp)
                                            .color(egui::Color32::DARK_GRAY),
                                    );
                                    ui.label(&log.message);
                                });
                            }
                        }
                    });
            });
    }
}
