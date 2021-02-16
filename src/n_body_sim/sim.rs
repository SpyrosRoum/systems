use ggez::{
    graphics::{self, Color, DrawMode, Rect},
    Context, GameResult,
};

use super::{Body, G, SOFTENING, DT};

#[derive(Debug, Clone)]
pub(crate) struct NBody {
    pub(crate) bodies: Vec<Body>,
    initialised: bool,
}

impl NBody {
    pub(crate) fn new() -> Self {
        Self {
            bodies: vec![],
            initialised: false,
        }
    }

    pub(crate) fn initialise(&mut self, bodies: usize, reinitialise: bool, coords: &Rect) {
        if !reinitialise && self.initialised {
            return;
        }
        let mut rng = rand::thread_rng();

        self.bodies.clear();
        (0..bodies).for_each(|_| self.bodies.push(Body::new_random(&mut rng, coords)));

        // At first all bodies start with acceleration of 0 so we want to update that
        self.update_acc();
        self.initialised = true;
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
        if paused {
            return;
        }

        for body in self.bodies.iter_mut() {
            body.vel.x += body.acc.x * DT / 2.0;
            body.vel.y += body.acc.y * DT / 2.0;

            body.pos.x += body.vel.x * DT;
            body.pos.y += body.vel.y * DT;
        }

        self.update_acc();

        for body in self.bodies.iter_mut() {
            body.vel.x += body.acc.x * DT / 2.0;
            body.vel.y += body.acc.y * DT / 2.0;
        }
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
