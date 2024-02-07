mod tests;
pub mod control;

use std::ops::{Mul, Range};

use num::{Float, NumCast};
use vector2d::Vector2D;

pub use control::{Continuity, Direction, Handle, Validity};

#[macro_export]
macro_rules! point {
    ($x : expr,$y : expr) => {
        Vector2D::new($x, $y)
    };
}

/// A cubic bezier type controlled by `Handles`.
/// ```
/// use cubic_bezier::{point, Bezier, Handle};
///
/// let mut bezier = Bezier::new(10,2);
/// bezier.push(Handle::mirrored(point!(-1.0,1.0),point!(0.0,0.0)));
/// bezier.push(Handle::mirrored(point!(1.0,1.0),point!(2.0,0.0)));
/// 
/// let points = bezier.calculate();
/// ```
/// Creating a new bezier is done with the `new` method. The
/// first parameter determines how many point each segment should
/// have. The second parameter is an estimation of how many
/// handles will be added to the bezier. This is used for optimization,
/// since it will cause the entire `points` vec to be allocated at
/// once, avoiding unneccecary vector allocations. This value does
/// not need to be accurate and you can set it to `0` to allocate no
/// space to begin with.
/// 
/// The calculated points are stored internally. You can get a reference
/// to the points by calling `calculate()`. Calling this function multiple
/// times (without changing the curve) does not result in additional
/// calculations. 
/// If only one segment of the curve changed, only that segment will be
/// recalculated.
pub struct Bezier<F: Float> {
    handles: Vec<Handle<F>>,
    detail: usize,

    num_ignored: usize,
    points: Vec<Vector2D<F>>,
}

fn interpolate<T: Float>(a: &Vector2D<T>, b: &Vector2D<T>, t: T) -> Vector2D<T> {
    a.mul(T::one() - t) + b.mul(t)
}

impl<F: Float> Bezier<F> {
    pub fn new(detail: usize, expected_handle_num: usize) -> Self {
        Bezier {
            handles: Vec::with_capacity(expected_handle_num),
            detail,
            points: Vec::with_capacity(expected_handle_num * detail),
            num_ignored: 0,
        }
    }

    fn invalidate_handle(&mut self, handle_index: usize) {
        if handle_index >= self.handles.len() {
            return;
        }
        self.handles[handle_index as usize].validity = Validity::Invalidated;
    }

    pub fn push(&mut self, handle: Handle<F>) {
        self.handles.push(handle);
    }
    pub fn insert(&mut self, index: usize, handle: Handle<F>) {
        if index != 0 {
            self.invalidate_handle(index - 1);
        }
        self.handles.insert(index, handle);
    }
    pub fn splice(&mut self, range: Range<usize>, handles: Vec<Handle<F>>) {
        if range.start != 0 {
            self.invalidate_handle(range.start - 1);
        }
        self.handles.splice(range, handles);
    }

    pub fn remove(&mut self, index: usize) {
        if index != 0 {        
            self.invalidate_handle(index - 1);
        }            
        self.handles.remove(index);
    }
    pub fn drain(&mut self, range: Range<usize>) {
        if range.start != 0 {
            self.invalidate_handle(range.start - 1);
        }
        self.handles.drain(range);
    }

    pub fn get_handle(&self, index: usize) -> &Handle<F> {
        &self.handles[index]
    }
    pub fn get_handle_mut(&mut self, index: usize) -> &mut Handle<F> {
        self.invalidate_handle(index);
        &mut self.handles[index]
    }

    ///Inserts a handle without changing the appearance of the curve.
    ///### Parameters
    /// * Time: decimal value in which the value before the decimal
    /// is the part index, and the value after is the progress
    /// through that part.
    ///### Example
    /// ```
    /// use cubic_bezier::{point, Bezier, Handle};
    ///
    /// let mut bezier = Bezier::new(10,3);
    /// bezier.splice(0..0,vec![
    ///     Handle::mirrored(point!(0.0,0.0),point!(1.0,1.0)),
    ///     Handle::mirrored(point!(4.0,1.0),point!(5.0,0.0)),
    /// ]);
    /// let points_before = bezier.calculate().to_owned();
    /// 
    /// bezier.knot_insert(0.5);
    /// let points_after = bezier.calculate().to_owned();
    /// 
    /// assert_eq!(points_before,points_after);
    /// ```
    pub fn knot_insert(&mut self, mut time: F) {
        let part_index = <usize as NumCast>::from(time).unwrap();
        if part_index >= self.handles.len() - 1 {
            panic!("Knot insertion failed because time was out of bounds.")
        }
        let points = self.part_points(part_index);
        time = time - time.floor();

        let center_point = interpolate(points[1], points[2], time);

        let prev_forward = interpolate(points[0], points[1], time);
        let next_backward = interpolate(points[2], points[3], time);
        let new_backward = interpolate(&prev_forward, &center_point, time);
        let new_forward = interpolate(&center_point, &next_backward, time);
        let new_position = interpolate(&new_backward, &new_forward, time);

        self.handles[part_index].after = prev_forward;
        self.handles[part_index + 1].before = next_backward;

        let handle = Handle::new(new_backward, new_position, new_forward);
        self.handles.insert(part_index + 1, handle);
    }

    /// Return the 4 points of a part.
    fn part_points(&self, part_index: usize) -> [&Vector2D<F>; 4] {
        if part_index >= self.handles.len() - 1 {
            panic!("part_index exceeds part length in part_points");
        }
        [
            &self.handles[part_index].position,
            &self.handles[part_index].after,
            &self.handles[part_index + 1].before,
            &self.handles[part_index + 1].position,
        ]
    }
    /// Helper function to get a vector of all control points.
    pub fn all_part_point(&self) -> Vec<Vector2D<F>> {
        let mut result: Vec<Vector2D<F>> = vec![];
        for part_index in 0..self.handles.len() - 1 {
            result.push(self.part_points(part_index)[0].to_owned());
            result.push(self.part_points(part_index)[1].to_owned());
            result.push(self.part_points(part_index)[2].to_owned());
            result.push(self.part_points(part_index)[3].to_owned());
        }
        result
    }
    /// Returns wether a part is empty. This could happen due to a handle being detached.
    fn part_is_empty(&self, part_index: usize) -> bool {
        if part_index >= self.handles.len() - 1 {
            panic!("part_index exceeds part length in part_is_empty");
        }
        self.handles[part_index].continuity == Continuity::Detached(Direction::Forward)
            || self.handles[part_index].continuity == Continuity::Detached(Direction::Both)
            || self.handles[part_index + 1].continuity == Continuity::Detached(Direction::Backward)
            || self.handles[part_index + 1].continuity == Continuity::Detached(Direction::Both)
    }
    /// Calculate a single part.
    /// ### Parameters
    /// * Part_index: index of the handle at the start of a part.
    fn calculate_part(&mut self, part_index: usize) {
        if part_index >= self.handles.len() - 1 {
            panic!("part_index exceeds part length in calculate_part");
        }
        let validity = &self.handles[part_index].validity;
        if self.part_is_empty(part_index) {
            self.num_ignored += 1;
            return;
        }
        if validity == &Validity::Valid {
            return;
        }

        let controls = self.part_points(part_index);

        let f_three = F::from(3.0).unwrap();
        let f_six = F::from(6.0).unwrap();
        let coefficients = (
            *controls[0],
            controls[0].mul(-f_three) + controls[1].mul(f_three),
            controls[0].mul(f_three) + controls[1].mul(-f_six) + controls[2].mul(f_three),
            controls[0].mul(-F::one())
                + controls[1].mul(f_three)
                + controls[2].mul(-f_three)
                + *controls[3],
        );

        let start_index = (self.detail) * (part_index - self.num_ignored);
        if validity == &Validity::Uninitialized {
            self.points.splice(
                start_index..start_index,
                vec![Vector2D::new(F::zero(), F::zero()); self.detail],
            );
        }

        for point_index in 0..self.detail {
            let t = F::from(point_index).unwrap() / F::from(self.detail).unwrap();
            let tpow2 = t * t;
            let tpow3 = t * tpow2;
            self.points[start_index + point_index] = coefficients.0
                + coefficients.1.mul(t)
                + coefficients.2.mul(tpow2)
                + coefficients.3.mul(tpow3);
        }
        self.handles[part_index].validity = Validity::Valid;
    }

    /// Calculate all points of the curve. Does not recalculate
    /// parts that were unchanged. Returns a vector with coordinates.
    /// of length *detail * num_handles*.
    /// ```
    /// use cubic_bezier::{Handle,Bezier};
    ///
    /// let bezier = Bezier::new(50,2);
    /// bezier.push(Handle::mirrored(point!(0.0,0.0),point!(1.0,0.0)));
    /// bezier.push(Handle::mirrored(point!(2.0,2.0),point!(3.0,3.0)));
    ///
    /// let points = bezier.calculate();
    /// assert_eq!(points.len(),50 * 2);
    /// ```
    pub fn calculate(&mut self) -> &Vec<Vector2D<F>> {
        self.num_ignored = 0;
        for part_index in 0..self.handles.len() - 1 {
            self.calculate_part(part_index);
        }
        &self.points
    }
}
