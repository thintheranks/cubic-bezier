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
    /// - `before`: The point before the handle.
    /// - `position`: The position of the handle.
    /// - `after`: The point after the handle.
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

    /// Create a handle that is broken and ensures G0 continuity for this part. This means that endpoints
    /// are ensured to meet without discontinuity in position.
    /// ### Parameters
    /// - `before`: The point before the handle.
    /// - `position`: The position of the handle.
    /// - `after`: The point after the handle.
    /// ### Example
    /// ```
    /// use cubic_bezier::{Handle, Bezier};
    /// 
    /// let mut bezier = Bezier::new(50, 1);
    /// bezier.push(Handle::new(point!(-1.0, 0.0), point!(0.0, 0.0), point!(1.0, 0.0)));
    /// ```
    pub fn new(before: Vector2D<F>, position: Vector2D<F>, after: Vector2D<F>) -> Self {
        Handle::empty(before, position, after, Continuity::Broken)
    }

    /// Create a handle that is aligned and ensures G1 continuity for this part. This means that endpoints
    /// are ensured to meet without discontinuity in both position and tangent.
    /// ### Parameters
    /// - `before`: The point before the handle.
    /// - `position`: The position of the handle.
    /// - `after_multiplier`: The multiplier to determine the position of the point after the handle.
    /// ### Example
    /// ```
    /// use cubic_bezier::{Handle, Bezier};
    /// 
    /// let mut bezier = Bezier::new(50, 1);
    /// bezier.push(Handle::aligned(point!(-1.0, 0.0), point!(0.0, 0.0), 2.0));
    /// ```
    pub fn aligned(before: Vector2D<F>, position: Vector2D<F>, after_multiplier: F) -> Self {
        let after = position + (position - before) * after_multiplier;
        Handle::empty(before, position, after, Continuity::Aligned)
    }

    /// Create a handle that is mirrored and ensures G2 continuity for this part. This means that endpoints
    /// are ensured to meet without discontinuity in position, tangent, and curvature.
    /// ### Parameters
    /// - `before`: The point before the handle.
    /// - `position`: The position of the handle.
    /// ### Example
    /// ```
    /// use cubic_bezier::{Handle, Bezier};
    /// 
    /// let mut bezier = Bezier::new(50, 1);
    /// bezier.push(Handle::mirrored(point!(-1.0, 0.0), point!(0.0, 0.0)));
    /// ```
    pub fn mirrored(before: Vector2D<F>, position: Vector2D<F>) -> Self {
        let after = position.mul(F::from(2.0).unwrap()) - before;
        Handle::empty(before, position, after, Continuity::Mirrored)
    }
}
