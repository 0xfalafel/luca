use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use num_rational::BigRational;

pub struct Number(BigRational);

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.0.to_string())
    }
}

impl Add<Number> for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}

impl Sub<Number> for Number {
    type Output = Number;

    fn sub(self, rhs: Number) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}

impl Mul<Number> for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}

impl Div<Number> for Number {
    type Output = Number;

    fn div(self, rhs: Number) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}
