mod state;
mod systems;
mod utils;

use macroquad::{experimental::camera::mouse::Camera, prelude::*};

use state::State;

pub(crate) const BG: Color = BLACK;
pub(crate) const FG: Color = WHITE;

pub(crate) trait System {
    fn name(&self) -> &'static str;
    fn clear(&mut self);
    fn init(&mut self, restart: bool);
    fn handle_input(&mut self, mouse_pos: Vec2);
    fn step(&mut self);
    fn draw(&self);
}

#[macroquad::main("Systems")]
async fn main() {
    let mut state = State::default();
    let mut cam = Camera::new(Vec2::ZERO, 0.1);

    loop {
        clear_background(BG);

        utils::update_camera(&mut cam);
        if utils::handle_common_input(&mut state) {
            break;
        }

        let cam2d: Camera2D = cam.clone().into();
        let pos = Vec2::from(mouse_position());
        // The position is still not exact but it's lose enough for now
        let mouse_pos = pos - Vec2::ONE;
        state
            .get_cur_system_mut()
            .handle_input(cam2d.screen_to_world(mouse_pos));

        if !state.is_paused() {
            state.get_cur_system_mut().step();
        }
        // We draw even if it's paused, otherwise as soon as we pause the screen will get cleared
        state.get_cur_system().draw();

        utils::draw_ui(&mut state);
        next_frame().await;
    }
}
