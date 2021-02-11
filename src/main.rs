mod state;

mod n_body_sim;

mod utils;

use ggez::{conf::WindowMode, event, ContextBuilder};

use {n_body_sim::NBody, state::State};

const SYSTEMS: [&str; 1] = ["n-body Simulation"];

enum System {
    NBody(NBody),
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Simulations & More", "Spyros")
        .window_mode(WindowMode::default().resizable(true))
        .build()
        .expect("Could not create ggez context!");

    let state = State::new(&mut ctx, &SYSTEMS);

    event::run(ctx, event_loop, state);
}
