use std::ops::Mul;

use num::Float;
use vector2d::Vector2D;

#[derive(PartialEq,Clone)]
pub enum Validity {
    Uninitialized,
    Invalidated,
    Valid,
}

#[derive(PartialEq,Clone)]
pub enum Direction {
    Forward,
    Backward,
    Both,
}

#[derive(PartialEq,Clone)]
pub enum Continuity {
    Detached(Direction),
    Broken,
    Aligned,
    Mirrored,
}

#[derive(Clone)]
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

    /// Create a handle that is not attached to the rest of the bezier
    /// on one or both sides. This function does not ensure continuity.
    /// ### Parameters
    /// A handle consists of three points in the order
    /// 1. before,
    /// 2. position,
    /// 3. after.
    /// ### Example
    /// ```
    /// use cubic_bezier::{Handle,Bezier};
    /// 
    /// let mut bezier = Bezier::new(50,1);
    /// bezier.push(Handle::detached(point!(-1.0,0.0),point!(0.0,0.0),point!(1.0,0.0)));
    /// ```
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
