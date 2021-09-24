use std::cmp::Ordering;

use macroquad::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
/// Represents a **live** cell
pub(crate) struct Cell {
    pub(crate) pos: (i64, i64),
}

// impl PartialOrd<Self> for Cell {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.pos.partial_cmp(&other.pos)
//     }
// }

impl Cell {
    pub(crate) fn new(pos: (i64, i64)) -> Self {
        Self { pos }
    }
}
