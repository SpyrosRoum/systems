use std::fmt::{self, Display, Formatter};

use crate::NBody;

#[derive(Debug, Clone)]
pub(crate) enum System {
    NBody(NBody),
}

impl Display for System {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            System::NBody(_) => writeln!(f, "n-body Simulation")?,
        };
        Ok(())
    }
}
