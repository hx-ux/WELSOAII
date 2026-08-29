// External crate imports
extern crate nannou;
use bevy_egui::egui;
use nannou::prelude::bevy_render::view::window;
use nannou::prelude::*;

// Module imports
mod animator;
mod color;
mod modulator;
mod parameters;
mod presets;
mod receiver;
mod timecode;
mod ui;
mod utils;

// Re-exports for public API
pub use utils::AppMode;

// Core component imports
use crate::animator::Animator;
use crate::animator::animation_type::UpdateBehaviour;
use crate::receiver::{LayoutMode, ReceiverGrid};
use crate::utils::GlobalSettings;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    animator: Animator,
    global_settings: GlobalSettings,
    device_modal_open: bool,
    settings_modal_open: bool,
    window: Entity,
}

fn model(app: &App) -> Model {
    let global_settings = GlobalSettings::load_or_default();

    let window = app
        .new_window()
        .title(GlobalSettings::APP_NAME)
        .size(
            global_settings.view_window_size.0,
            global_settings.view_window_size.1,
        )
        .view(view)
        // .event(event)
        // .raw_event(settings_window_event)
        .build();

    let win_rect = app.window_rect();

    let receiver_grid = ReceiverGrid::new(
        Rect::from_x_y_w_h(0.0, 0.0, 400.0, 300.0),
        20,
        20,
        false,
        LayoutMode::FollowColum,
    );

    let mut animator = Animator::new(&win_rect, receiver_grid);
    animator.reset(&win_rect);

    Model {
        animator,
        global_settings,
        device_modal_open: false,
        settings_modal_open: false,
        window,
    }
}

fn update(_app: &App, _model: &mut Model) {
    let win_rect = _app.window_rect();

    let Model {
        window,
        global_settings,
        animator,
        ..
    } = *model;

    // let egui = &mut _model.egui;
    // egui.set_elapsed_time(_update.since_start);

    let ctx = _app.draw_for_window(_model.window);
    // crate::ui::style_injector::apply_custom_style(
    //     &ctx,
    //     _model.global_settings.control_windows_opacity.value as u8,
    // );

    // egui:::top("MENU").show(&ctx, |ui| {
    //     egui::menu::bar(ui, |ui| {
    //         ui.menu_button("Settings", |ui| {
    //             if ui.button("Device").clicked() {
    //                 _model.device_modal_open = true;
    //                 ui.close_menu();
    //             }
    //             if ui.button("Settings").clicked() {
    //                 _model.settings_modal_open = true;
    //                 ui.close_menu();
    //             }
    //         });
    //         ui.separator();
    //         _model.animator.timecode.ui(ui);
    //     });
    // });

    // egui::TopBottomPanel::bottom("ANIMATOR")
    //     .exact_height(200.00)
    //     .show(&ctx, |ui| {
    //         // Top divider line
    //         ui.add_space(1.0);

    //         ui.columns(3, |cols| {
    //             cols[0].set_width(120.0);
    //             _model.animator.animator_layer_ui(&mut cols[0], &win_rect);

    //             egui::ScrollArea::vertical().show(&mut cols[1], |ui| {
    //                 match _model.animator.control_ui(ui) {
    //                     UpdateBehaviour::NeedsReset => _model.animator.reset(&win_rect),
    //                     UpdateBehaviour::HotUpdate => _model.animator.behaviour_hot_update(),
    //                     UpdateBehaviour::LoadPreset => {}
    //                     UpdateBehaviour::SavePresets => {}
    //                     UpdateBehaviour::None => {}
    //                 }
    //             });

    //             egui::ScrollArea::vertical().show(&mut cols[2], |ui| {
    //                 if let Some(index) = _model.animator.current_ani_index {
    //                     if let Some(animator) = _model.animator.active_animations.get_mut(index) {
    //                         ui.label(egui::RichText::new("COLOR"));
    //                         ui.add(egui::Separator::default().spacing(4.0));
    //                         animator.color_ui(ui);
    //                     }
    //                 }
    //             });
    //         });
    //     });

    // egui::TopBottomPanel::bottom("MODULATOR")
    //     .exact_height(130.0)
    //     .show(&ctx, |ui| {
    //         ui.add_space(1.0);
    //         ui.label(egui::RichText::new("MODULATOR"));
    //         ui.add(egui::Separator::default().spacing(4.0));
    //         egui::ScrollArea::vertical()
    //             .id_source("mod_scroll")
    //             .show(ui, |ui| {
    //                 _model.animator.mod_matrix.ui(ui);
    //             });
    //     });

    // egui::Window::new("GLOBAL SETTINGS")
    //     .resizable(true)
    //     .default_open(true)
    //     .open(&mut _model.settings_modal_open)
    //     .show(&ctx, |ui| {
    //         _model.global_settings.ui(ui);
    //     });

    // egui::Window::new("DEVICE")
    //     .resizable(true)
    //     .default_open(true)
    //     .open(&mut _model.device_modal_open)
    //     .show(&ctx, |ui| {
    //         _model.animator.grid.ui(ui);
    //     });

    _model.animator.behaviour_hot_update();

    _model.animator.update(&win_rect, 1.00);
}

// fn settings_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
//     model.egui.handle_raw_event(event);
// }

// fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
//     let receiver = &mut _model.animator.grid;
//     let win_rect = _app.window_rect();

//     match event {
//         KeyPressed(_key) => match _key {
//             Key::Up => {
//                 if receiver.edit_mode {
//                     receiver.move_by(vec2(0.0, 10.0))
//                 }
//             }
//             Key::Down => {
//                 if receiver.edit_mode {
//                     receiver.move_by(vec2(0.0, -10.0))
//                 }
//             }
//             Key::Right => {
//                 if receiver.edit_mode {
//                     receiver.move_by(vec2(10.0, 0.0));
//                 } else {
//                 }
//             }
//             Key::Left => {
//                 if receiver.edit_mode {
//                     receiver.move_by(vec2(-10.0, 0.0));
//                 } else {
//                 }
//             }

//             Key::Equals | Key::Plus => receiver.resize_by(vec2(10.0, 10.0)),
//             Key::Minus => receiver.resize_by(vec2(-10.0, -10.0)),
//             Key::P => _model.global_settings.app_mode = AppMode::Presentation,
//             Key::E => _model.global_settings.app_mode = AppMode::Edit,
//             _ => (),
//         },
//         MousePressed(_button) => {}
//         MouseReleased(_button) => {}

//         _other => {}
//     }
// }

fn view(_app: &App, _model: &Model) {
    let draw = _app.draw();
    draw.background().color(BLACK);

    _model.animator.draw_animator(&draw);
    _model.animator.draw_grid(&draw);

    match _model.global_settings.app_mode {
        AppMode::Presentation => {}
        AppMode::Edit => {}
    }
}
