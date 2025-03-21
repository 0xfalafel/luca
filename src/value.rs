use std::ops::Add;

use num_rational::BigRational;

#[derive(Debug, Clone, PartialEq)]

enum Unit {
    Euros,
    Dollars
}

#[derive(Debug, Clone, PartialEq)]

pub struct Value {
    number: BigRational,
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

pub enum CalculationError {
    IncompatileTypes
}

impl Add<Value> for Value {
    type Output = Result<Value, CalculationError>;

    fn add (self, rhs: Value) -> Self::Output {


        // If the types are compatible, procced with the addition
        Ok(Value {
            number: self.number + rhs.number,
            unit: self.unit
        })
    }
}