use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

use num_rational::BigRational;

#[derive(Debug, Clone, PartialEq)]

enum Unit {
    Euros,
    Dollars
}

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
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.number)
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