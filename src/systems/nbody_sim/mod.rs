mod body;
mod sim;

pub(crate) use sim::NBody;

const G: f32 = 10.0;
const DT: f32 = 0.015;
const SOFTENING: f32 = 0.01;
