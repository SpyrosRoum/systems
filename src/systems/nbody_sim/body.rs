use macroquad::prelude::*;

const DEFAULT_MASS: u32 = 500;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Body {
    /// Position
    pub(crate) pos: Vec2,
    /// Velocity
    pub(crate) vel: Vec2,
    /// Acceleration
    pub(crate) acc: Vec2,
    pub(crate) mass: u32,
}

impl Body {
    pub(crate) fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: vec2(rand::gen_range(-10.0, 10.0), rand::gen_range(-10.0, 10.0)),
            acc: Vec2::ZERO,
            mass: DEFAULT_MASS,
        }
    }

    /// `coords` is the view area
    pub(crate) fn new_random(coords: &Rect) -> Self {
        let (low_x, high_x) = (coords.x + 10.0, coords.x + coords.w - 10.0);
        let (low_y, high_y) = (coords.y + 10.0, coords.y + coords.h - 10.0);
        Self {
            pos: vec2(
                rand::gen_range(low_x, high_x),
                rand::gen_range(low_y, high_y),
            ),
            vel: vec2(rand::gen_range(-10.0, 10.0), rand::gen_range(-10.0, 10.0)),
            acc: Vec2::ZERO,
            mass: DEFAULT_MASS,
        }
    }
}
