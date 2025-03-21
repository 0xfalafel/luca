use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

use num_rational::BigRational;
use crate::units::unit::Unit;
#[cfg(test)]
use num_traits::FromPrimitive;

#[derive(Debug, Clone, PartialEq)]

pub struct Value {
    pub number: BigRational,
    unit: Vec<Unit>
}

impl Value {
    pub fn new(number: BigRational) -> Value {
        Value {
            number: number,
            unit: vec![]
        }
    }

    pub fn set_unit(self, unit: Unit) -> Value {
        Value { number: self.number, unit: vec![unit] }
    }
}

// Helper function to create Value with Unit in tests
#[cfg(test)]
impl Value {
    pub fn from_f64_with_unit(number: f64, unit: Unit) -> Value {
        Value {
            number: BigRational::from_f64(number).unwrap(),
            unit: vec![unit]
        }
    }

    pub fn from_int(number: i64) -> Value {
        Value {
            number: BigRational::from_i64(number).unwrap(),
            unit: vec![] 
        }
    }
    
    pub fn from_float(number: f64) -> Value {
        Value {
            number: BigRational::from_f64(number).unwrap(),
            unit: vec![] 
        }
    }
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: at the moment, this only displays the first unit
        if let Some(unit) = self.unit.first() {
            write!(f, "{}{}", self.number, unit.symbol)
        } else {
            write!(f, "{}", self.number)
        }
    }
}

pub enum CalculationError {
    IncompatibleTypes
}

impl Add<Value> for Value {
    type Output = Result<Value, CalculationError>;

    fn add(self, rhs: Value) -> Self::Output {
        // TODO: implement a new type contructor
        if self.unit != rhs.unit { 
            return Err(CalculationError::IncompatibleTypes)
        }

        // If the types are compatible, procced with the addition
        Ok(Value {
            number: self.number + rhs.number,
            unit: self.unit
        })
    }
}

impl Sub<Value> for Value {
    type Output = Result<Value, CalculationError>;

    fn sub(self, rhs: Value) -> Self::Output {
        // TODO: implement a new type contructor
        if self.unit != rhs.unit { 
            return Err(CalculationError::IncompatibleTypes)
        }

        // If the types are compatible, procced with the substraction
        Ok(Value {
            number: self.number - rhs.number,
            unit: self.unit
        })
    }
}


impl Mul<Value> for Value {
    type Output = Result<Value, CalculationError>;

    fn mul(self, rhs: Value) -> Self::Output {
        // TODO: implement a new type contructor
        if self.unit != rhs.unit { 
            return Err(CalculationError::IncompatibleTypes)
        }

        Ok(Value {
            number: self.number * rhs.number,
            unit: self.unit
        })
    }
}

impl Div<Value> for Value {
    type Output = Result<Value, CalculationError>;

    fn div(self, rhs: Value) -> Self::Output {
        // TODO: implement a new type contructor
        if self.unit != rhs.unit { 
            return Err(CalculationError::IncompatibleTypes)
        }

        Ok(Value {
            number: self.number / rhs.number,
            unit: self.unit
        })
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        Value { number: -self.number, unit: self.unit }
    }
}