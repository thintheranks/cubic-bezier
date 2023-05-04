use std::ops;

use num::Float;

pub struct Point<F : Float> {
    x : F,
    y : F
}

impl<F : Float> Point<F> {
    fn new(x : F, y : F) -> Self {
        Point { x, y }
    }
}

impl<F : Float> ops::Add<Point<F>> for Point<F> {
    type Output = Point<F>;

    fn add(self, _rhs: Point<F>) -> Point<F> {
        Point::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

impl<F : Float> ops::Sub<Point<F>> for Point<F> {
    type Output = Point<F>;

    fn sub(self, _rhs: Point<F>) -> Point<F> {
        Point::new(self.x - _rhs.x, self.y - _rhs.y)
    }
}

impl<F : Float> ops::Mul<Point<F>> for Point<F> {
    type Output = Point<F>;

    fn mul(self, _rhs: Point<F>) -> Point<F> {
        Point::new(self.x * _rhs.x, self.y * _rhs.y)
    }
}

impl<F : Float> ops::Mul<F> for Point<F> {
    type Output = Point<F>;

    fn mul(self, _rhs: F) -> Point<F> {
        Point::new(self.x * _rhs, self.y * _rhs)
    }
}

impl<F : Float> ops::Div<Point<F>> for Point<F> {
    type Output = Point<F>;

    fn div(self, _rhs: Point<F>) -> Point<F> {
        Point::new(self.x * _rhs.x, self.y * _rhs.y)
    }
}

impl<F : Float> ops::Div<F> for Point<F> {
    type Output = Point<F>;

    fn div(self, _rhs: F) -> Point<F> {
        Point::new(self.x / _rhs, self.y / _rhs)
    }
}