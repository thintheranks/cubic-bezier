use num::{NumCast,Float};

use crate::point::{Point};

fn bernstein_polynomial<F : Float>(x : F) -> (F,F,F,F) {
    let three : F = NumCast::from(3.0).unwrap();
    (   
        (F::one() - x).powi(3),
        three * x * (F::one() - x).powi(2),
        three * x.powi(2) * (F::one() - x),
        x.powi(3),
    )
}

fn cubic_lerp<F : Float>(a : Point<F>, b : Point<F>, c : Point<F>, d : Point<F>, t : F) -> Point<F> {
    let weights = bernstein_polynomial(t);
    a * weights.0 + b * weights.1 + c * weights.2 + d * weights.3
}