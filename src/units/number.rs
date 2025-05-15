use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::str::FromStr;
use num_rational::{BigRational, ParseRatioError};
use num_traits::float::FloatCore;
use num_traits::FromPrimitive;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Number(BigRational);

impl Number {
    pub fn from_u8(n: u8) -> Number {
        Number(BigRational::from_u8(n).unwrap())
    }

    pub fn from_float<T: FloatCore>(f: T) -> Option<Number> {
        match BigRational::from_float(f) {
            Some(num)=> Some(Number(num)),
            None => None,
        }
    }

    pub fn from_i64(n: i64) -> Option<Number> {
        match BigRational::from_i64(n) {
            Some(num)=> Some(Number(num)),
            None => None,
        }
    }

    pub fn from_str(s: &str) -> Result<Number, ParseRatioError> {
        match BigRational::from_str(s) {
            Ok(num)=> Ok(Number(num)),
            Err(e) => Err(e),
        }
    }

}

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

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Number(-self.0)
    }
}
