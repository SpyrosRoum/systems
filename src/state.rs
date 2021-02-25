use ggez::{
    event::EventHandler,
    graphics::{self, Color, Text},
    input::keyboard::{KeyCode, KeyMods},
    Context, GameResult,
};

use crate::{utils, System};

pub(crate) struct State {
    /// The current system and its index in the list
    system: Option<(System, usize)>,
    paused: bool,
    systems: Vec<System>,
}

impl State {
    pub(crate) fn new(_ctx: &mut Context, systems: Vec<System>) -> Self {
        Self {
            system: None,
            systems,
            paused: false,
        }
    }

    pub(crate) fn set_system(&mut self, ctx: &Context, system_index: usize) {
        if system_index >= self.systems.len() {
            return;
        }

        if self.system.is_none() {
            let system = self.systems[system_index].to_owned();
            self.system = Some((system, system_index));
        } else {
            if system_index == self.system.as_ref().unwrap().1 {
                return;
            }
            // We clone the system we want to use, meaning we also clone its state
            let new = self.systems[system_index].to_owned();
            // We put the new system in place of the old, getting back the old and where it should go in the vec
            let (mut old, old_i) = self.system.replace((new, system_index)).unwrap();
            // We put the old system in the vec in its place "saving" its new state and deleting its old
            std::mem::swap(&mut self.systems[old_i], &mut old);
        }

        // Finally initialise the system. `system.initialise` should check if it has already been initialised or not
        let coords = graphics::screen_coordinates(ctx);
        match &mut self.system.as_mut().unwrap().0 {
            System::NBody(nbody) => nbody.initialise(5, false, &coords),
            System::Life(life) => life.initialise(false, &coords),
        }
    }

    pub(crate) fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub(crate) fn restart(&mut self, ctx: &Context) {
        if let Some((sys, _)) = &mut self.system {
            let coords = graphics::screen_coordinates(ctx);
            match sys {
                System::NBody(nbody) => nbody.initialise(5, true, &coords),
                System::Life(life) => life.initialise(true, &coords),
            }
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if let Some((sys, _)) = &mut self.system {
            match sys {
                System::NBody(nbody) => nbody.update(self.paused),
                System::Life(life) => life.update(self.paused),
            };
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        if let Some((sys, _)) = &mut self.system {
            utils::draw_fps(ctx, Color::WHITE)?;
            utils::draw_paused(ctx, self.paused, Color::WHITE)?;
            match sys {
                System::NBody(nbody) => nbody.draw(ctx)?,
                System::Life(life) => life.draw(ctx)?,
            };
        } else {
            let coords = graphics::screen_coordinates(ctx);
            let x = coords.x + 20.0;
            let mut y = coords.y + coords.h / 4.0;
            for (i, sys) in self.systems.iter().enumerate() {
                let sis_display = Text::new(format!("{}. {}", i + 1, sys));
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
        if character.is_ascii_digit() {
            let d = character.to_digit(10).unwrap() as usize;
            if d <= self.systems.len() && d != 0 {
                self.set_system(ctx, d - 1);
                let sys = &self.system.as_ref().unwrap().0;
                graphics::set_window_title(ctx, format!("{}", sys).as_str());
            }
        }
    }
}
