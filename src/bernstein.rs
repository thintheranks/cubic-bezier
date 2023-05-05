use std::ops::Mul;

use num::{Float, Num, NumCast};
use vector2d::Vector2D;

use crate::handle::{Continuity, Direction, Handle, PartValidity};

pub struct Bezier<F: Float> {
    handles: Vec<Handle<F>>,
    detail: usize,
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
        }
    }

    pub fn handles_mut(&mut self) -> &mut Vec<Handle<F>> {
        &mut self.handles
    }
    pub fn handles(&self) -> &Vec<Handle<F>> {
        &self.handles
    }

    ///Inserts a handle without changing the appearance of the curve.
    ///### Parameters
    /// * Time: decimal value in which the value before the decimal
    /// is the part index, and the value after is the progress
    /// through that part.
    ///### Example
    /// ```
    ///
    /// ```
    pub fn knot_insert(&mut self, mut time: F) {
        let part_index = <usize as NumCast>::from(time).unwrap();
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
    pub fn all_part_point_dbg(&self) -> Vec<Vector2D<F>> {
        let mut result: Vec<Vector2D<F>> = vec![];
        for part_index in 0..self.handles().len() - 1 {
            result.push(self.part_points(part_index)[0].to_owned());
            result.push(self.part_points(part_index)[1].to_owned());
            result.push(self.part_points(part_index)[2].to_owned());
            result.push(self.part_points(part_index)[3].to_owned());
        }
        result
    }
    fn part_is_empty(&self, part_index: usize) -> bool {
        if part_index >= self.handles.len() - 1 {
            panic!("part_index exceeds part length in part_is_empty");
        }
        self.handles[part_index].continuity == Continuity::Detached(Direction::Forward)
            || self.handles[part_index].continuity == Continuity::Detached(Direction::Both)
            || self.handles[part_index + 1].continuity == Continuity::Detached(Direction::Backward)
            || self.handles[part_index + 1].continuity == Continuity::Detached(Direction::Both)
    }

    fn calculate_part(&mut self, part_index: usize, n: usize) -> usize {
        if part_index >= self.handles.len() - 1 {
            panic!("part_index exceeds part length in calculate_part");
        }
        let validity = &self.handles[part_index].validity;
        if self.part_is_empty(part_index) {
            return n + 1;
        }
        if validity == &PartValidity::Valid {
            return n;
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

        let start_index = (self.detail + 1) * (part_index - n);
        if validity == &PartValidity::Uninitialized {
            self.points.splice(
                start_index..start_index,
                vec![Vector2D::new(F::zero(), F::zero()); self.detail + 1],
            );
        }

        for point_index in 0..=self.detail {
            let t = F::from(point_index).unwrap() / F::from(self.detail).unwrap();
            self.points[start_index + point_index] = coefficients.0
                + coefficients.1.mul(t)
                + coefficients.2.mul(t.powi(2))
                + coefficients.3.mul(t.powi(3));
        }
        self.handles[part_index].validity = PartValidity::Valid;
        n
    }

    pub fn calculate(&mut self) -> &Vec<Vector2D<F>> {
        let mut n = 0;
        for part_index in 0..self.handles.len() - 1 {
            n = self.calculate_part(part_index, n);
        }
        &self.points
    }
}
