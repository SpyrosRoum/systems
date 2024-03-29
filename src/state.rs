use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{systems, System};

pub(crate) struct State<'a> {
    systems: HashMap<&'a str, Box<dyn System>>,
    system: Box<dyn System>,
    is_paused: bool,
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        let mut systems: HashMap<&str, Box<dyn System>> = HashMap::new();

        // Starting system is life so we don't insert it
        let nbody_sim = systems::NBody::new();
        systems.insert(nbody_sim.name(), Box::new(nbody_sim));
        let ant = systems::AntAutomata::new();
        systems.insert(ant.name(), Box::new(ant));

        let mut life = systems::Life::new();
        life.init(false);
        Self {
            systems,
            system: Box::new(life),
            is_paused: true,
        }
    }
}

impl<'a> State<'a> {
    pub(crate) fn set_system(&mut self, new_system: &str) {
        let new_system = self.find_system(new_system);

        if let Some(mut sys) = new_system {
            sys.init(false);
            let old_sys = std::mem::replace(&mut self.system, sys);
            self.systems.insert(old_sys.name(), old_sys);
        } else {
            // Then the system is the current system, so we restart it
            self.get_cur_system_mut().init(true);
        }
    }

    pub(crate) fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    pub(crate) fn find_system(&mut self, system: &str) -> Option<Box<dyn System>> {
        self.systems.remove(system)
    }

    pub(crate) fn get_cur_system(&self) -> &dyn System {
        self.system.as_ref()
    }

    pub(crate) fn get_cur_system_mut(&mut self) -> &mut Box<dyn System> {
        &mut self.system
    }

    pub(crate) fn is_paused(&self) -> bool {
        self.is_paused
    }
}
