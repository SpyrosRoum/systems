use {
    ggez::mint::{Point2, Vector2},
    rand::{rngs::ThreadRng, Rng},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Body {
    /// Position
    pub(crate) pos: Point2<f64>,
    /// Velocity
    pub(crate) vel: Vector2<f64>,
    /// Acceleration
    pub(crate) acc: Vector2<f64>,
    pub(crate) mass: u32,
}

impl Body {
    pub(crate) fn new_random(rng: &mut ThreadRng, width: f32, height: f32) -> Self {
        let (width, height) = (width as f64, height as f64);
        Self {
            pos: [
                rng.gen_range(10.0..(width - 50.0)),
                rng.gen_range(10.0..(height - 50.0)),
            ]
            .into(),
            vel: [rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0)].into(),
            acc: [0.0, 0.0].into(),
            mass: 500,
        }
    }
}
