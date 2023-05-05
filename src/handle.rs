use std::ops::Mul;

use num::Float;
use vector2d::Vector2D;

#[derive(PartialEq)]
pub enum PartValidity {
    Uninitialized,
    Invalidated,
    Valid,
}

#[derive(PartialEq)]
pub enum Direction {
    Forward,
    Backward,
    Both,
}

#[derive(PartialEq)]
pub enum Continuity {
    Detached(Direction),
    Broken,
    Aligned,
    Mirrored,
}

pub struct Handle<F: Float> {
    pub before: Vector2D<F>,
    pub position: Vector2D<F>,
    pub after: Vector2D<F>,

    pub continuity: Continuity,
    pub(crate) validity: PartValidity,
}

impl<F: Float> Handle<F> {
    pub fn detached(before: Vector2D<F>, position: Vector2D<F>, direction: Direction) -> Self {
        let after = position.mul(F::from(2.0).unwrap()) - before;
        Handle {
            before,
            position,
            after,
            continuity: Continuity::Detached(direction),
            validity: PartValidity::Uninitialized,
        }
    }
    pub fn new(before: Vector2D<F>, position: Vector2D<F>, after: Vector2D<F>) -> Self {
        Handle {
            before,
            position,
            after,
            continuity: Continuity::Broken,
            validity: PartValidity::Uninitialized,
        }
    }
    pub fn aligned(before: Vector2D<F>, position: Vector2D<F>, after_multiplier: F) -> Self {
        let after = position + (position - before) * after_multiplier;
        Handle {
            before,
            position,
            after,
            continuity: Continuity::Aligned,
            validity: PartValidity::Uninitialized,
        }
    }
    pub fn mirrored(before: Vector2D<F>, position: Vector2D<F>) -> Self {
        let after = position.mul(F::from(2.0).unwrap()) - before;
        Handle {
            before,
            position,
            after,
            continuity: Continuity::Mirrored,
            validity: PartValidity::Uninitialized,
        }
    }
}
