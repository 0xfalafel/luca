use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

use num_rational::BigRational;
use crate::units::unit::Unit;
use num_traits::{FromPrimitive, ToPrimitive};

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

    pub fn is_percent(&self) -> bool {
        self.unit == vec![Unit::percent()]
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
        // Nothing to worry about here
        if self.number.is_integer() {
            return print_num(&self.number, &self.unit, f)
        }
        
        match self.number.to_f64() {
            Some(float) => print_num(&float, &self.unit, f), // display a float
            None => print_num(&self.number, &self.unit, f), // still display a fraction
        }
    }
}

fn print_num<T: std::fmt::Display>(num: &T, unit: &Vec<Unit>,f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(unit) = unit.first() {
        write!(f, "{}{}", num, unit.symbol)
    } else {
        write!(f, "{}", num)
    }
}

pub enum CalculationError {
    IncompatibleTypes
}

/// Used to determine the final type for Addition and Substraction
fn same_type(left: Vec<Unit>, right: Vec<Unit>) -> Result<Vec<Unit>, CalculationError>{
    match (left, right) {
        // We don't have any unit
        (left, right) if left.is_empty() && right.is_empty() => Ok(vec![]),

        // Only one side has unit, let's convert this to the unit
        // ex: 12€ + 4 = 16€
        (left, right) if left.is_empty()  => Ok(right),
        (left, right) if right.is_empty() => Ok(left),
        
        // We have the same type on both sides. Keep the type
        // ex: 12€ + 4€ = 16€
        (left, right) if left == right => Ok(left),

        // TODO: try to convert between types

        _ => Err(CalculationError::IncompatibleTypes)
    }
}

impl Add<Value> for Value {
    type Output = Result<Value, CalculationError>;

    fn add(self, rhs: Value) -> Self::Output {

        // Handle the special case of percentage addition:
        // 100€ + 10% = 110€
        if self.is_percent() || rhs.is_percent() {
            let (percentage, val, unit) = match self.is_percent() {
                true  => (self.number, rhs.number, rhs.unit),
                false => (rhs.number, self.number, self.unit)
            };

            return Ok(Value { 
                number: val.clone() + val * percentage / BigRational::from_u8(100).unwrap(),
                unit: unit
            })
        }

        // Otherwise, let's do a normal addition
        Ok(Value {
            number: self.number + rhs.number,
            unit: same_type(self.unit, rhs.unit)?
        })
    }
}

impl Sub<Value> for Value {
    type Output = Result<Value, CalculationError>;

    fn sub(self, rhs: Value) -> Self::Output {

        // Handle the special case of percentage substraction:
        // 100€ - 10% = 90€
        if self.is_percent() || rhs.is_percent() {
            let (percentage, val, unit) = match self.is_percent() {
                true  => (self.number, rhs.number, rhs.unit),
                false => (rhs.number, self.number, self.unit)
            };

            return Ok(Value { 
                number: val.clone() - val * percentage / BigRational::from_u8(100).unwrap(),
                unit: unit
            })
        }


        Ok(Value {
            number: self.number - rhs.number,
            unit: same_type(self.unit, rhs.unit)?
        })
    }
}


impl Mul<Value> for Value {
    type Output = Result<Value, CalculationError>;

    fn mul(self, rhs: Value) -> Self::Output {
        // Handle the special case of percentage substraction:
        // 100€ * 10% = 10€
        if self.is_percent() || rhs.is_percent() {
            let (percentage, val, unit) = match self.is_percent() {
                true  => (self.number, rhs.number, rhs.unit),
                false => (rhs.number, self.number, self.unit)
            };

            return Ok(Value { 
                number: val * percentage / BigRational::from_u8(100).unwrap(),
                unit: unit
            })
        }

        // If only one of the operands has a unit, we keep the same unit
        if self.unit.is_empty() || rhs.unit.is_empty() {
            let unit = match self.unit.is_empty() {
                true  => rhs.unit,
                false => self.unit,
            };

            return Ok(Value { 
                number: self.number * rhs.number,
                unit: unit
            })
        }

        // Normal multiplication

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
        // Handle the special case of percentage substraction:
        // 100€ / 10% = 100€ * 10 / 100 = 1000€
        if self.is_percent() || rhs.is_percent() {
            let (percentage, val, unit) = match self.is_percent() {
                true  => (self.number, rhs.number, rhs.unit),
                false => (rhs.number, self.number, self.unit)
            };

            return Ok(Value { 
                number: val * BigRational::from_u8(100).unwrap() / percentage,
                unit: unit
            })
        }

        // If only one of the operands has a unit, we keep the same unit
        if self.unit.is_empty() || rhs.unit.is_empty() {
            let unit = match self.unit.is_empty() {
                true  => rhs.unit,
                false => self.unit,
            };

            return Ok(Value { 
                number: self.number / rhs.number,
                unit: unit
            })
        }
        
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

