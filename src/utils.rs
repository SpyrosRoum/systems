use {
    egui_macroquad::egui,
    macroquad::{experimental::camera::mouse::Camera, prelude::*},
};

use crate::{systems, State, System};

/// Handles common input and returns if the program should exit
pub(crate) fn handle_common_input(state: &mut State) -> bool {
    if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
        return true;
    }
    if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Space) {
        state.toggle_pause();
    }
    if is_key_pressed(KeyCode::R) {
        state.get_cur_system_mut().init(true);
    }
    if is_key_pressed(KeyCode::C) {
        state.get_cur_system_mut().clear();
    }

    false
}

pub(crate) fn update_camera(cam: &mut Camera) {
    cam.scale_wheel(Vec2::ZERO, -mouse_wheel().1, 0.5);
    let should_offset = is_mouse_button_down(MouseButton::Middle);
    cam.update(mouse_position_local(), should_offset);

    let cam2d: Camera2D = cam.clone().into();
    set_camera(&cam2d);
}

pub(crate) fn draw_ui(state: &mut State) {
    egui_macroquad::ui(|ctx| {
        egui::Window::new("Info").show(ctx, |ui| {
            ui.label(format!("Current System: {}", state.get_cur_system().name()));
            ui.label(if state.is_paused() {
                "Paused"
            } else {
                "Running"
            });
            ui.label(format!("FPS: {}", get_fps()));
        });

        egui::Window::new("Key Bindings").show(ctx, |ui| {
            ui.label("P or Space to toggle pause.");
            ui.label("C to clear the running system");
            ui.label("R to restart the running system");
        });

        // Note: calling SomeSystem.new() doesn't cost anything since we don't init the system
        egui::Window::new("Systems").show(ctx, |ui| {
            if ui.button("Game of Life").clicked() {
                state.set_system(systems::Life::new().name());
            }
            if ui.button("N-Body Simulation").clicked() {
                state.set_system(systems::NBody::new().name());
            }
            if ui.button("Langton's Ant").clicked() {
                state.set_system(systems::AntAutomata::new().name());
            }
        });
    });

    egui_macroquad::draw();
}

/// A custom impl of Rect::contains
pub(crate) fn contains(rect: &Rect, point: Vec2) -> bool {
    rect.x <= point.x && point.x <= rect.w && point.y <= rect.h && point.y >= rect.y
}
