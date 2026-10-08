use chrono::Local;
use nannou_egui::egui;
use std::sync::{Arc, Mutex};
#[derive(Clone, Debug)]
pub enum Severity {
    Info,
    Error,
}

#[derive(Clone, Debug)]
pub struct LogMessage {
    pub timestamp: String,
    pub message: String,
    pub severity: Severity,
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
    pub fn log(&self, message: &str, severity: Severity) {
        if let Ok(mut logs) = self.logs.lock() {
            logs.push(LogMessage {
                timestamp: Local::now().format("%H:%M:%S").to_string(),
                message: message.to_string(),
                severity,
            });
            if logs.len() > 200 {
                logs.remove(0);
            }
        }
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
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
                        let text_color = match log.severity {
                            Severity::Info => egui::Color32::GREEN,
                            Severity::Error => egui::Color32::RED,
                        };

                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(&log.timestamp).color(text_color));
                            ui.label(egui::RichText::new(&log.message).color(text_color));
                        });
                    }
                }
            });
    }
}
