mod body;
mod sim;

use body::Body;
pub(crate) use sim::NBody;

const G: f64 = 10.0;
const SOFTENING: f64 = 0.01;
