use std::{
    collections::BTreeSet,
    time::{Duration, Instant},
};

use macroquad::prelude::*;

use crate::System;

struct Ant {
    pos: Vec2,
    dir: Direction,
}

impl Ant {
    fn new(pos: Vec2, dir: Direction) -> Self {
        Self { pos, dir }
    }

    fn turn_clockwise(&mut self) {
        self.dir.turn_clockwise();
    }

    fn turn_anticlockwise(&mut self) {
        self.dir.turn_anticlockwise();
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_clockwise(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        };
    }

    fn turn_anticlockwise(&mut self) {
        *self = match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        };
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Block {
    pos: (i32, i32),
}

impl From<&Vec2> for Block {
    fn from(v: &Vec2) -> Self {
        Self {
            pos: (v.x as i32, v.y as i32),
        }
    }
}

pub(crate) struct AntAutomata {
    is_initialised: bool,
    ants: Vec<Ant>,
    white_blocks: BTreeSet<Block>,
    last_tick: Instant,
    tick_per: Duration,
}

impl AntAutomata {
    pub(crate) fn new() -> Self {
        Self {
            is_initialised: false,
            ants: Vec::new(),
            white_blocks: BTreeSet::new(),
            last_tick: Instant::now(),
            tick_per: Duration::from_millis(150),
        }
    }
}

impl System for AntAutomata {
    fn name(&self) -> &'static str {
        "Langton's Ant"
    }

    fn clear(&mut self) {
        self.ants.clear();
        self.white_blocks.clear();
        self.is_initialised = true;
        self.last_tick = Instant::now();
        self.tick_per = Duration::from_millis(150);
    }

    fn init(&mut self, restart: bool) {
        if !restart && self.is_initialised {
            return;
        }

        self.white_blocks.clear();
        self.ants.clear();

        self.ants.push(Ant::new(Vec2::ZERO, Direction::Up));

        self.is_initialised = true;
        self.last_tick = Instant::now();
    }

    fn handle_input(&mut self, mouse_pos: Vec2) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.ants.push(Ant::new(mouse_pos, Direction::Up));
        }

        if is_key_pressed(KeyCode::Right) && self.tick_per > Duration::from_millis(20) {
            self.tick_per -= Duration::from_millis(20);
        } else if is_key_pressed(KeyCode::Left) {
            self.tick_per += Duration::from_millis(20);
        }
    }

    fn step(&mut self) {
        if self.last_tick.elapsed() < self.tick_per {
            // Update a certain amount of times per second so it's not too much or too little
            return;
        }

        for ant in self.ants.iter_mut() {
            if self.white_blocks.remove(&Block::from(&ant.pos)) {
                ant.turn_clockwise();
            } else {
                self.white_blocks.insert(Block::from(&ant.pos));
                ant.turn_anticlockwise();
            }

            ant.pos += match ant.dir {
                Direction::Up => vec2(0., -1.),
                Direction::Down => vec2(0., 1.),
                Direction::Left => vec2(-1., 0.),
                Direction::Right => vec2(1., 0.),
            };
        }

        self.last_tick = Instant::now();
    }

    fn draw(&self, visible_space: Rect) {
        for cell in self.white_blocks.iter() {
            if !crate::utils::contains(&visible_space, vec2(cell.pos.0 as f32, cell.pos.1 as f32)) {
                continue;
            }
            draw_rectangle(cell.pos.0 as f32, cell.pos.1 as f32, 1., 1., WHITE);
            draw_rectangle_lines(cell.pos.0 as f32, cell.pos.1 as f32, 1., 1., 0.1, DARKGRAY);
        }
        for ant in self.ants.iter() {
            if !crate::utils::contains(&visible_space, ant.pos) {
                continue;
            }
            draw_rectangle(ant.pos.x, ant.pos.y, 1., 1., RED);
        }
    }
}
