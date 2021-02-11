use std::time::Instant;

use ggez::{
    graphics::{self, Color, DrawMode},
    Context, GameResult,
};

use super::{Body, G, SOFTENING};

#[derive(Debug)]
pub(crate) struct NBody {
    pub(crate) bodies: Vec<Body>,
    last_step: Instant,
}

impl NBody {
    pub(crate) fn title(&self) -> &'static str {
        "n-body Simulation"
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            last_step: Instant::now(),
            bodies: Vec::with_capacity(capacity),
        }
    }

    pub(crate) fn initialise(bodies: usize, width: f32, height: f32) -> Self {
        let mut rng = rand::thread_rng();

        let mut system = NBody::with_capacity(bodies);

        (0..bodies).for_each(|_| {
            system
                .bodies
                .push(Body::new_random(&mut rng, width, height))
        });

        // At first all bodies start with acceleration of 0 so we want to update that
        system.update_acc();
        system
    }

    /// Update the acceleration for all bodies
    fn update_acc(&mut self) {
        let old_bodies = self.bodies.clone();

        for body in self.bodies.iter_mut() {
            for old_body in old_bodies.iter() {
                if body == old_body {
                    continue;
                }
                let (dx, dy) = (old_body.pos.x - body.pos.x, old_body.pos.y - body.pos.y);

                let inv = 1.0 / (dx.powi(2) + dy.powi(2) + SOFTENING);

                body.acc.x = G * (dx * inv) * old_body.mass as f64;
                body.acc.y = G * (dy * inv) * old_body.mass as f64;
            }
        }
    }

    pub(crate) fn update(&mut self, paused: bool) {
        let now = Instant::now();
        if paused {
            // Update the last step so there is not a big jump when we resume
            self.last_step = now;
            return;
        }
        let dt = (now - self.last_step).as_secs_f64();

        for body in self.bodies.iter_mut() {
            body.vel.x += body.acc.x * dt / 2.0;
            body.vel.y += body.acc.y * dt / 2.0;

            body.pos.x += body.vel.x * dt;
            body.pos.y += body.vel.y * dt;
        }

        self.update_acc();

        for body in self.bodies.iter_mut() {
            body.vel.x += body.acc.x * dt / 2.0;
            body.vel.y += body.acc.y * dt / 2.0;
        }

        self.last_step = now;
    }

    pub(crate) fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for body in self.bodies.iter() {
            let pos = [body.pos.x as f32, body.pos.y as f32];
            let circle =
                graphics::Mesh::new_circle(ctx, DrawMode::fill(), pos, 10.0, 0.25, Color::WHITE)
                    .unwrap();
            graphics::draw(ctx, &circle, ([0.0, 0.0],)).unwrap();
        }

        Ok(())
    }
}
