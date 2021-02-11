use ggez::{
    event,
    graphics::{self, Color, Rect, Text},
    input::{
        keyboard::KeyCode,
        mouse::{self, MouseButton},
    },
    Context, GameResult,
};

use crate::State;

/// Display current fps on the top left of the screen
pub(crate) fn draw_fps(ctx: &mut Context, color: Color) -> GameResult<()> {
    let coords = graphics::screen_coordinates(ctx);
    let fps = ggez::timer::fps(ctx);
    let fps_display = Text::new(format!("FPS: {:.2}", fps));

    graphics::draw(
        ctx,
        &fps_display,
        ([coords.x + 20.0, coords.y + 10.0], color),
    )
}

/// Display if the system is paused on the bottom left
pub(crate) fn draw_paused(ctx: &mut Context, paused: bool, color: Color) -> GameResult<()> {
    if !paused {
        return Ok(());
    }
    let coords = graphics::screen_coordinates(ctx);
    let paused_display = Text::new("Paused");

    graphics::draw(
        ctx,
        &paused_display,
        ([coords.x + 20.0, coords.y + coords.h - 20.0], color),
    )
}

/// Handle some standard button presses
pub(crate) fn handle_key_down(state: &mut State, ctx: &mut Context, keycode: &KeyCode) {
    match keycode {
        KeyCode::Escape | KeyCode::Q => event::quit(ctx),
        KeyCode::P => state.toggle_pause(),
        _ => {}
    }
}

/// Move the camera when holding down middle click and moving the mouse
pub(crate) fn move_camera(ctx: &mut Context, dx: f32, dy: f32) {
    if mouse::button_pressed(ctx, MouseButton::Middle) {
        let old = graphics::screen_coordinates(ctx);
        let rect = Rect::new(old.x - dx, old.y - dy, old.w, old.h);
        graphics::set_screen_coordinates(ctx, rect).ok();
    }
}
