use macroquad::prelude::*;

use crate::System;

pub(crate) struct NBodySim {
    is_initialised: bool,
}

impl NBodySim {
    pub(crate) fn new() -> Self {
        Self {
            is_initialised: false,
        }
    }
}

impl System for NBodySim {
    fn name(&self) -> &'static str {
        "N-Body Simulation"
    }

    fn clear(&mut self) {
        todo!()
    }

    fn init(&mut self, restart: bool) {
        todo!()
    }

    fn handle_input(&mut self, mouse_pos: Vec2) {
        todo!()
    }

    fn step(&mut self) {
        todo!()
    }

    fn draw(&self) {
        todo!()
    }
}
