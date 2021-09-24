use macroquad::prelude::*;

use {
    super::{DT, G, SOFTENING},
    crate::{systems::nbody_sim::body::Body, System},
};

const STARTING_BODIES: i32 = 6;

pub(crate) struct NBody {
    bodies: Vec<Body>,
    is_initialised: bool,
}

impl NBody {
    pub(crate) fn new() -> Self {
        Self {
            bodies: vec![],
            is_initialised: false,
        }
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

                body.acc.x = G * (dx * inv) * old_body.mass as f32;
                body.acc.y = G * (dy * inv) * old_body.mass as f32;
            }
        }
    }
}

impl System for NBody {
    fn name(&self) -> &'static str {
        "N-Body Simulation"
    }

    fn clear(&mut self) {
        self.bodies.clear();
        self.is_initialised = true;
    }

    fn init(&mut self, restart: bool) {
        if !restart && self.is_initialised {
            return;
        }

        self.bodies.clear();

        // ToDO: Get the actual Rect
        (0..STARTING_BODIES).for_each(|_| {
            self.bodies
                .push(Body::new_random(&Rect::new(0., 0., 100., 100.)))
        });

        // At first all bodies start with acceleration of 0 so we want to update it
        self.update_acc();
        self.is_initialised = true;
    }

    fn handle_input(&mut self, mouse_pos: Vec2) {
        // ToDo Allow creating bodies with different mass when holding click
        if is_mouse_button_pressed(MouseButton::Left) {
            self.bodies.push(Body::new(mouse_pos));
        }
    }

    fn step(&mut self) {
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

    fn draw(&self) {
        for body in self.bodies.iter() {
            draw_circle(body.pos.x, body.pos.y, 1., crate::FG);
        }
    }
}
