use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::ops::Neg;
use duplicate::duplicate_item;

use super::math_utils::{try_to_f64, PrecisonLossError};

// Percentage
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Percentage {
    pub value: f64
}

impl Percentage {
    pub fn new(value: f64) -> Percentage {
        Percentage { value: value}
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", self.value)
    }
}

// Implement operations for Percent

macro_rules! impl_arithmetic_op_for_Percentage {
    ($trait_name:ident $fn_name:ident $op:tt) => {
        
        impl $trait_name<Percentage> for Percentage {
            type Output = Percentage;
        
            fn $fn_name(self, rhs: Percentage) -> Self::Output {
                Percentage::new(self.value $op rhs.value)
            }
        }                
    };
}

/*
impl Add<Percentage> for Percentage {
    type Output = Percentage;

    fn add(self, rhs: Percentage) -> Self::Output {
        Percentage::new(self.value + rhs.value)
    }
}
*/
impl_arithmetic_op_for_Percentage!(Add add +);
impl_arithmetic_op_for_Percentage!(Sub sub -);
impl_arithmetic_op_for_Percentage!(Mul mul *);
impl_arithmetic_op_for_Percentage!(Div div /);


impl Add<f64> for Percentage {
    type Output = f64;

    fn add(self, rhs: f64) -> Self::Output {
        rhs + (rhs * self.value / 100.0)
    }
}

impl Sub<f64> for Percentage {
    type Output = f64;

    fn sub(self, rhs: f64) -> Self::Output {
        rhs - (rhs * self.value / 100.0)
    }
}

impl Mul<f64> for Percentage {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self.value / 100.0
    }
}

impl Div<f64> for Percentage {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        rhs * 100.0 / self.value
    }
}

macro_rules! arithmetic_op_percentage_for_f64 {
    ($trait_name:ident $operation:ident) => {
        //   Add, Sub, ...
        impl $trait_name<Percentage> for f64 {
            type Output = f64;
        
            // add, sub
            fn $operation(self, rhs: Percentage) -> Self::Output {
                rhs.$operation(self)
            }
        }        
    }
}
arithmetic_op_percentage_for_f64!(Add add);
arithmetic_op_percentage_for_f64!(Sub sub);
arithmetic_op_percentage_for_f64!(Mul mul);
arithmetic_op_percentage_for_f64!(Div div);

/*
    Arithmetic operations for i128
*/

impl Add<i128> for Percentage {
    type Output = Result<f64, PrecisonLossError>;

    fn add(self, rhs: i128) -> Self::Output {
        // TODO, check if we can do thing while still being an i128
        let rhs: f64 = try_to_f64(rhs)?;
        let res = rhs + (rhs * self.value / 100.0);
        Ok(res)
    }
}

impl Sub<i128> for Percentage {
    type Output = Result<f64, PrecisonLossError>;

    fn sub(self, rhs: i128) -> Self::Output {
        // TODO, check if we can do thing while still being an i128
        let rhs: f64 = try_to_f64(rhs)?;
        let res = rhs - (rhs * self.value / 100.0);
        Ok(res)
    }
}

impl Mul<i128> for Percentage {
    type Output = Result<f64, PrecisonLossError>;

    fn mul(self, rhs: i128) -> Self::Output {
        // TODO, check if we can do thing while still being an i128
        let rhs: f64 = try_to_f64(rhs)?;
        let res = rhs * self.value / 100.0;
        Ok(res)
    }
}

impl Div<i128> for Percentage {
    type Output = Result<f64, PrecisonLossError>;

    fn div(self, rhs: i128) -> Self::Output {
        // TODO, check if we can do thing while still being an i128
        let rhs: f64 = try_to_f64(rhs)?;
        let res = rhs * 100.0 / self.value;
        Ok(res)
    }
}

// Implement all the symetric operation: Add<Percentage> for i128

macro_rules! arithmetic_op_percentage_for_i128 {
    ($trait_name:ident $operation:ident) => {

        impl $trait_name<Percentage> for i128 {
            type Output = Result<f64, PrecisonLossError>;

            fn $operation(self, rhs: Percentage) -> Self::Output {
                rhs.$operation(self)
            }
        }
    }
}
arithmetic_op_percentage_for_i128!(Add add);
arithmetic_op_percentage_for_i128!(Sub sub);
arithmetic_op_percentage_for_i128!(Mul mul);
arithmetic_op_percentage_for_i128!(Div div);

impl Neg for Percentage {
    type Output = Percentage;
    fn neg(self) -> Percentage {
        Percentage {value: -self.value}
    }
}

#[duplicate_item(Type; [f64]; [i128]; [i32];)]
impl Into<Type> for Percentage {
    fn into(self) -> Type {
        self.value as Type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Percentage::new(15.0) + Percentage::new(22.0), Percentage { value: 37.0});
    }

    #[test]
    fn sub() {
        assert_eq!(Percentage::new(15.0) - Percentage::new(22.0), Percentage { value: -7.0});
    }

    #[test]
    fn mul() { assert_eq!(Percentage::new(-7.0) * Percentage::new(-2.0), Percentage { value: 14.0})}

    #[test]
    fn div() { assert_eq!(Percentage::new(13.0) / Percentage::new(2.0), Percentage { value: 6.5})}

    // #[test]
    // fn div_zero() { assert_eq!(Percentage::new(13.0) / Percentage::new(0.0), Percentage { value: 0.0})}
}