use std::fmt::{self, Display, Formatter};

use crate::{Life, NBody};

#[derive(Debug, Clone)]
pub(crate) enum System {
    NBody(NBody),
    Life(Life),
}

impl Display for System {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            System::NBody(_) => writeln!(f, "n-body Simulation")?,
            System::Life(_) => writeln!(f, "Game of Life")?,
        };
        Ok(())
    }
}
