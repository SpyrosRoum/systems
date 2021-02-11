mod body;
mod sim;

pub(crate) use body::Body;
pub(crate) use sim::NBody;

pub(crate) const G: f64 = 10.0;
pub(crate) const SOFTENING: f64 = 0.01;
