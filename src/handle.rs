use std::{borrow::BorrowMut, ops::Mul};

use num::Float;
use vector2d::Vector2D;

#[macro_export]
macro_rules! point {
    ($x : expr,$y : expr) => {
        Vector2D::new($x, $y)
    };
}

#[derive(PartialEq)]
pub enum Validity {
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
    pub(crate) validity: Validity,
}

impl<F: Float> Handle<F> {
    fn empty(
        before: Vector2D<F>,
        position: Vector2D<F>,
        after: Vector2D<F>,
        continuity: Continuity,
    ) -> Self {
        Handle {
            before,
            position,
            after,
            continuity,
            validity: Validity::Uninitialized,
        }
    }

    pub fn detached(before: Vector2D<F>, position: Vector2D<F>, direction: Direction) -> Self {
        let after = position.mul(F::from(2.0).unwrap()) - before;
        Handle::empty(before, position, after, Continuity::Detached(direction))
    }
    pub fn new(before: Vector2D<F>, position: Vector2D<F>, after: Vector2D<F>) -> Self {
        Handle::empty(before, position, after, Continuity::Broken)
    }
    pub fn aligned(before: Vector2D<F>, position: Vector2D<F>, after_multiplier: F) -> Self {
        let after = position + (position - before) * after_multiplier;
        Handle::empty(before, position, after, Continuity::Aligned)
    }
    pub fn mirrored(before: Vector2D<F>, position: Vector2D<F>) -> Self {
        let after = position.mul(F::from(2.0).unwrap()) - before;
        Handle::empty(before, position, after, Continuity::Mirrored)
    }
}
