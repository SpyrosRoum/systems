#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
/// Represents a **live** cell
pub(crate) struct Cell {
    pub(crate) pos: (i64, i64),
}

impl Cell {
    pub(crate) fn new(pos: (i64, i64)) -> Self {
        Self { pos }
    }
}
