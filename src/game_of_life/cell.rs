use std::cmp::Ordering;

use ggez::mint::Point2;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Represents a **live** cell
pub(crate) struct Cell {
    // In ggez coordinates are f64 but only the integer part is being used.
    // Using Point2<i64> here makes things easier because Eq is implemented for i64
    pub(crate) pos: Point2<i64>,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.pos.partial_cmp(&other.pos)
    }
}

impl Cell {
    pub(crate) fn new<T: Into<i64>>(x: T, y: T) -> Self {
        Self {
            pos: Point2::from([x.into(), y.into()]),
        }
    }
}
