use std::{
    collections::{BTreeMap, BTreeSet},
    time::{Duration, Instant},
};

use ggez::{
    graphics::{self, Color, DrawMode, Rect},
    Context, GameResult,
};

use super::{Cell, LENGTH};

#[derive(Debug, Clone)]
pub(crate) struct Life {
    /// A set with **only** cells that are alive
    cells: BTreeSet<Cell>,
    initialised: bool,
    last_tick: Instant,
}

impl Life {
    pub(crate) fn new() -> Self {
        Self {
            cells: BTreeSet::new(),
            initialised: false,
            last_tick: Instant::now(),
        }
    }

    pub(crate) fn initialise(&mut self, reinitialise: bool, coords: &Rect) {
        if !reinitialise && self.initialised {
            return;
        }

        let mx = ((coords.x + coords.x + coords.w) / 2.0) as i64;
        let my = ((coords.y + coords.y + coords.h) / 2.0) as i64;

        self.cells.clear();

        // Add a glider :D
        self.cells.insert(Cell::new(mx, my));
        self.cells.insert(Cell::new(mx + LENGTH * 2, my));
        self.cells.insert(Cell::new(mx + LENGTH, my + LENGTH));
        self.cells.insert(Cell::new(mx + LENGTH, my + LENGTH * 2));
        self.cells.insert(Cell::new(mx + LENGTH * 2, my + LENGTH));

        self.initialised = true;
        self.last_tick = Instant::now();
    }

    pub(crate) fn update(&mut self, paused: bool) {
        // We only keep alive cells in a BTreeSet so it *should* be fine to search in that based on location that neighbours should be
        // This should allow for a more "infinite" grid but I should implement a limit for how far out cells can get and probably just prune them
        // Also use a map to check the dead cells around the live once and determine if we need to revive them
        // {
        //     some_dead: number of live neighbours that will be calculated as I iterate the live ones
        // }
        // ToDo: Limit how far they can get
        if paused {
            return;
        }
        if Instant::now() - self.last_tick < Duration::from_millis(250) {
            // Update a certain amount of times per second so it's not too much or too little
            return;
        }
        self.last_tick = Instant::now();

        let mut new_cells = BTreeSet::new();
        let mut dead = BTreeMap::new();

        for cell in self.cells.iter() {
            let (x, y) = (cell.pos.x, cell.pos.y);
            let mut alive_neighbours = 0;
            let neighbours = [
                Cell::new(x + LENGTH, y),          // Right
                Cell::new(x - LENGTH, y),          // Left
                Cell::new(x, y - LENGTH),          // Above
                Cell::new(x, y + LENGTH),          // Under
                Cell::new(x + LENGTH, y + LENGTH), // Under & Right
                Cell::new(x - LENGTH, y + LENGTH), // Under & Left
                Cell::new(x + LENGTH, y - LENGTH), // Above & Right
                Cell::new(x - LENGTH, y - LENGTH), // Above & Left
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

    pub(crate) fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for cell in self.cells.iter() {
            let p = graphics::Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(
                    cell.pos.x as f32,
                    cell.pos.y as f32,
                    LENGTH as f32,
                    LENGTH as f32,
                ),
                Color::WHITE,
            )?;
            graphics::draw(ctx, &p, ([0.0, 0.0],))?;
        }

        Ok(())
    }
}
