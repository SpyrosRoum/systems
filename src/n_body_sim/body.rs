use {
    ggez::{
        graphics::Rect,
        mint::{Point2, Vector2},
    },
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
    pub(crate) fn new_random(rng: &mut ThreadRng, coords: &Rect) -> Self {
        let (low_x, high_x) = (coords.x + 10.0, coords.x + coords.w - 10.0);
        let (low_y, high_y) = (coords.y + 10.0, coords.y + coords.h - 10.0);
        Self {
            pos: [
                rng.gen_range(low_x as f64..high_x as f64),
                rng.gen_range(low_y as f64..high_y as f64),
            ]
            .into(),
            vel: [rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0)].into(),
            acc: [0.0, 0.0].into(),
            mass: 500,
        }
    }
}
