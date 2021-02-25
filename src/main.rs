mod state;
mod system;

mod game_of_life;
mod n_body_sim;

mod utils;

use ggez::{conf::WindowMode, event, ContextBuilder};

use {game_of_life::Life, n_body_sim::NBody, state::State, system::System};

fn main() {
    let systems = vec![System::NBody(NBody::new()), System::Life(Life::new())];

    let (mut ctx, event_loop) = ContextBuilder::new("Simulations & More", "Spyros")
        .window_mode(WindowMode::default().resizable(true))
        .build()
        .expect("Could not create ggez context!");

    let state = State::new(&mut ctx, systems);

    event::run(ctx, event_loop, state);
}
