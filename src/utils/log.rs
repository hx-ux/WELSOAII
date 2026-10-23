use chrono::Local;
use nannou_egui::egui;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

const MAX_LOG_MESSAGES: usize = 200;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, Default)]
pub struct AppLogger {
    logs: Arc<Mutex<VecDeque<LogMessage>>>,
}

impl AppLogger {
    pub fn log(&self, message: impl Into<String>, severity: Severity) {
        if let Ok(mut logs) = self.logs.lock() {
            if logs.len() >= MAX_LOG_MESSAGES {
                logs.pop_front();
            }
            logs.push_back(LogMessage {
                timestamp: Local::now().format("%H:%M:%S").to_string(),
                message: message.into(),
                severity,
            });
        }
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        if ui.button("Clear").clicked()
            && let Ok(mut logs) = self.logs.lock()
        {
            logs.clear();
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

