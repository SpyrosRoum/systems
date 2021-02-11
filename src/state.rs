use ggez::{
    event::EventHandler,
    graphics::{self, Color, Text},
    input::keyboard::{KeyCode, KeyMods},
    Context, GameResult,
};

use crate::{n_body_sim::NBody, utils, System};

pub(crate) struct State {
    system: Option<System>,
    paused: bool,
    systems: Vec<String>,
}

impl State {
    pub(crate) fn new(_ctx: &mut Context, systems: &[&str]) -> Self {
        Self {
            system: None,
            systems: systems.iter().map(|s| s.to_string()).collect(),
            paused: false,
        }
    }

    pub(crate) fn set_system(&mut self, system: System) {
        self.system = Some(system);
    }

    pub(crate) fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if let Some(sys) = &mut self.system {
            match sys {
                System::NBody(nbody) => nbody.update(self.paused),
            };
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        if let Some(sys) = &mut self.system {
            utils::draw_fps(ctx, Color::WHITE)?;
            utils::draw_paused(ctx, self.paused, Color::WHITE)?;
            match sys {
                System::NBody(nbody) => nbody.draw(ctx)?,
            };
        } else {
            let coords = graphics::screen_coordinates(ctx);
            let x = coords.x + 20.0;
            let mut y = coords.y + coords.h / 4.0;
            for (i, sis) in self.systems.iter().enumerate() {
                let sis_display = Text::new(format!("{}. {}", i + 1, sis));
                graphics::draw(ctx, &sis_display, ([x, y], Color::WHITE))?;
                y += 15.0;
            }
        }

        graphics::present(ctx)
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, _x: f32, _y: f32, dx: f32, dy: f32) {
        if self.system.is_some() {
            utils::move_camera(ctx, dx, dy);
        }
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if self.system.is_some() {
            utils::handle_key_down(self, ctx, &keycode);
        }
    }

    fn text_input_event(&mut self, ctx: &mut Context, character: char) {
        if self.system.is_none() && character.is_ascii_digit() {
            let coords = graphics::screen_coordinates(ctx);

            let d = character.to_digit(10).unwrap() as usize;
            if d <= self.systems.len() && d != 0 {
                let sis = match d {
                    1 => {
                        let sis = NBody::initialise(10, coords.w, coords.h);
                        graphics::set_window_title(ctx, sis.title());
                        System::NBody(sis)
                    }
                    _ => unreachable!(),
                };

                self.set_system(sis);
            }
        }
    }
}
