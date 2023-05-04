use std::ops::Mul;

use num::Float;
use vector2d::Vector2D;

use crate::handle::Handle;

pub struct Bezier<F: Float> {
    handles: Vec<Handle<F>>,
    detail: usize,
    points: Vec<Vector2D<F>>,
}

impl<F: Float> Bezier<F> {
    pub fn handles_mut(&mut self) -> &mut Vec<Handle<F>> {
        &mut self.handles
    }
    pub fn handles(&self) -> &Vec<Handle<F>> {
        &self.handles
    }
    fn part_points(&self, part_index: usize) -> [&Vector2D<F>; 4] {
        [
            &self.handles[part_index].position,
            &self.handles[part_index].after,
            &self.handles[part_index + 1].before,
            &self.handles[part_index + 1].position,
        ]
    }

    fn calculate_part(&self, part_index: usize) -> &Vec<Vector2D<F>> {
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

        //nice loop? think cuz detached means no part (continous array result return thingy)

        &self.points
    }
}
