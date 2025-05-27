use std::{fmt, i32};
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::str::FromStr;
use num_rational::BigRational;
use num_traits::float::FloatCore;
use num_traits::{FromPrimitive, One, ToPrimitive};
use pretty_dtoa::{dtoa, FmtFloatConfig};

use crate::value::ValueError;

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

    pub fn from_str(s: &str) -> Result<Number,  ValueError> {
        let float = match f64::from_str(s) {
            Ok(val) => val,
            Err(_) => return Err(ValueError::FailedToParseNumber)
        };

        match BigRational::from_float(float) {
            Some(num)=> Ok(Number(num)),
            None => Err(ValueError::FailedToParseNumber),
        }
    }

    pub fn one() -> Number {
        Number(BigRational::one())
    }

    pub fn pow(&self, rhs: &Number) -> Result<Number, ValueError> {
        // try to convert rhs to integer to use powi
        if let Some(rhs_integer) =  rhs.0.to_i32() {
            Ok(Number(self.0.pow(rhs_integer)))
            // otherwise try to convert self and rhs to f64 to use powf
        } else {
            let self_f64 = match self.0.to_f64() {
                Some(f) => f,
                None => return Err(ValueError::FailedToConvertionNumberFloat),
            };
            let rhs_f64 = match self.0.to_f64() {
                Some(f) => f,
                None => return Err(ValueError::FailedToConvertionNumberFloat),
            };
            let res = self_f64.powf(rhs_f64);
            if let Some(num) = Number::from_float(res) {
                return Ok(num)
            } else {
                return Err(ValueError::FailedToConvertionNumberFloat)
            }
        }
    }

    pub fn powi(&self, rhs: i32) -> Number {
        Number(self.0.pow(rhs))
    }

    pub fn powf(&self, rhs: f64) -> Result<Number, ValueError> {
        let self_f64 = match self.0.to_f64() {
            Some(f) => f,
            None => return Err(ValueError::FailedToConvertionNumberFloat),
        };
        match Number::from_float(self_f64.powf(rhs)) {
            Some(num) => Ok(num),
            None => Err(ValueError::FailedToConvertionNumberFloat),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0.to_f64() {
            Some(float) => {
                let config = FmtFloatConfig::default()
                    .upper_e_break(9)
                    .lower_e_break(-9)
                    .max_decimal_digits(6)
                    .add_point_zero(false)
                    .round()
                    .group_digits(3, ' ');
                write!(f, "{}", dtoa(*float, config))
            },
            None => write!(f, "{}", self.0.to_string())
        }
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
        Number(self.0 - rhs.0)
    }
}

impl Mul<Number> for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Self::Output {
        Number(self.0 * rhs.0)
    }
}

impl Div<Number> for Number {
    type Output = Number;

    fn div(self, rhs: Number) -> Self::Output {
        Number(self.0 / rhs.0)
    }
}

impl <F: FloatCore> Mul<F> for Number {
    type Output = Result<Number, ValueError>;

    fn mul(self, rhs: F) -> Self::Output {
        match BigRational::from_float(rhs) {
            Some(r) => Ok(Number(self.0 * r)),
            None => Err(ValueError::FailedToParseConversionFactor),
        }
    }
}

impl <F: FloatCore> Div<F> for Number {
    type Output = Result<Number, ValueError>;

    fn div(self, rhs: F) -> Self::Output {
        match BigRational::from_float(rhs) {
            Some(r) => Ok(Number(self.0 / r)),
            None => Err(ValueError::FailedToParseConversionFactor),
        }
    }
}

impl TryFrom <Number> for i32 {
    type Error = ValueError;

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        if value.0.is_integer() == false {
            return Err(ValueError::IsNotInteger)
        }

        let big_int = value.0.to_integer();

        match big_int.to_i32() {
            Some(i32) => Ok(i32),
            None => Err(ValueError::IsNotInteger),
        }
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Number(-self.0)
    }
}
