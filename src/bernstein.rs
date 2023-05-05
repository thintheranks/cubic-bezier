use std::ops::Mul;

use num::Float;
use vector2d::Vector2D;

use crate::handle::{Continuity, Direction, Handle, PartValidity};

pub struct Bezier<F: Float> {
    handles: Vec<Handle<F>>,
    detail: usize,
    points: Vec<Vector2D<F>>,
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
        let mut result : Vec<Vector2D<F>> = vec![];
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
