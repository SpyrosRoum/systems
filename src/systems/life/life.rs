use std::{
    collections::{BTreeMap, BTreeSet},
    time::{Duration, Instant},
};

use macroquad::prelude::*;

use {
    super::{Cell, LENGTH},
    crate::System,
};

pub(crate) struct Life {
    /// A set with **only alive** cells
    cells: BTreeSet<Cell>,
    is_initialised: bool,
    last_tick: Instant,
}

impl Life {
    pub(crate) fn new() -> Self {
        Self {
            cells: BTreeSet::new(),
            is_initialised: false,
            last_tick: Instant::now(),
        }
    }
}

impl System for Life {
    fn name(&self) -> &'static str {
        "Game of Life"
    }

    fn clear(&mut self) {
        self.cells.clear();
        self.is_initialised = true;
        self.last_tick = Instant::now();
    }

    fn init(&mut self, restart: bool) {
        if !restart && self.is_initialised {
            return;
        }

        self.cells.clear();

        // Glider \:D
        self.cells.insert(Cell::new((0, 0)));
        self.cells.insert(Cell::new((LENGTH * 2, 0)));
        self.cells.insert(Cell::new((LENGTH, LENGTH)));
        self.cells.insert(Cell::new((LENGTH, LENGTH * 2)));
        self.cells.insert(Cell::new((LENGTH * 2, LENGTH)));

        self.is_initialised = true;
        self.last_tick = Instant::now();
    }

    fn handle_input(&mut self, mouse_pos: Vec2) {
        if is_mouse_button_down(MouseButton::Left) {
            self.cells
                .insert(Cell::new((mouse_pos.x as i64, mouse_pos.y as i64)));
        }
    }

    fn step(&mut self) {
        // We only keep alive cells in a BTreeSet so it *should* be fine to search in that based on location that neighbours should be
        // This should allow for a more "infinite" grid but I should implement a limit for how far out cells can get and probably just prune them
        // Also use a map to check the dead cells around the live once and determine if we need to revive them
        // {
        //     some_dead: number of live neighbours that will be calculated as I iterate the live ones
        // }
        // ToDo: Limit how far they can get

        if Instant::now() - self.last_tick < Duration::from_millis(250) {
            // Update a certain amount of times per second so it's not too much or too little
            return;
        }
        self.last_tick = Instant::now();

        let mut new_cells = BTreeSet::new();
        let mut dead = BTreeMap::new();

        for cell in self.cells.iter() {
            let (x, y) = (cell.pos.0, cell.pos.1);
            let mut alive_neighbours = 0;
            let neighbours = [
                Cell::new((x + LENGTH, y)),          // Right
                Cell::new((x - LENGTH, y)),          // Left
                Cell::new((x, y - LENGTH)),          // Above
                Cell::new((x, y + LENGTH)),          // Under
                Cell::new((x + LENGTH, y + LENGTH)), // Under & Right
                Cell::new((x - LENGTH, y + LENGTH)), // Under & Left
                Cell::new((x + LENGTH, y - LENGTH)), // Above & Right
                Cell::new((x - LENGTH, y - LENGTH)), // Above & Left
            ];
            for neighbour in neighbours.iter().copied() {
                if self.cells.contains(&neighbour) {
                    alive_neighbours += 1;
                } else {
                    *dead.entry(neighbour).or_insert(0) += 1;
                }
            }

            if alive_neighbours == 2 || alive_neighbours == 3 {
                // Congrats, you survived
                new_cells.insert(cell.to_owned());
            }
        }

        for (cell, _) in dead.iter().filter(|(_c, n_count)| **n_count == 3) {
            new_cells.insert(cell.to_owned());
        }
        std::mem::swap(&mut self.cells, &mut new_cells);
    }

    fn draw(&self, visible_space: Rect) {
        for cell in self.cells.iter() {
            if !crate::utils::contains(&visible_space, vec2(cell.pos.0 as f32, cell.pos.1 as f32)) {
                continue;
            }
            draw_rectangle(
                cell.pos.0 as f32,
                cell.pos.1 as f32,
                LENGTH as f32,
                LENGTH as f32,
                crate::FG,
            );
            draw_rectangle_lines(
                cell.pos.0 as f32,
                cell.pos.1 as f32,
                LENGTH as f32,
                LENGTH as f32,
                0.1,
                DARKGRAY,
            );
        }
    }
}
