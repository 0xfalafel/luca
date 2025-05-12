use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

use num_rational::BigRational;
use crate::units::unit::Unit;
use crate::units::composed_unit::ComposedUnit;
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, PartialEq)]
pub enum ValueError {
    InvalidConversion
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub number: BigRational,
    unit: ComposedUnit,
}

impl Value {
    pub fn new(number: BigRational) -> Value {
        Value {
            number: number,
            unit: ComposedUnit::new(),
        }
    }

    pub fn set_unit(self, unit: Unit) -> Value {
        Value { number: self.number, unit: ComposedUnit::new_with_unit(unit) }
    }

    pub fn convert_to_unit(&self, unit: &Unit) -> Result <Value, ValueError> {
        if self.unit.len() > 1 {
            panic!("We need to fix this when we will create the ComposedUnit type")
        }
        if self.unit.len() == 0 {
            return Ok(self.clone().set_unit(unit.clone()))
        }

        let self_unit = &self.unit[0];

        if let Some(conversion_factor) = self_unit.conversion_factor(unit) {

            return Ok(Value { number: self.number.clone() * BigRational::from_float(conversion_factor).unwrap(), unit: vec![unit.clone()] })
        }
        Err(ValueError::InvalidConversion)
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
        write!(f, "{} {}", num, unit.symbol)
    } else {
        write!(f, "{}", num)
    }
}

pub enum CalculationError {
    IncompatibleTypes
}

/// Todo: Create a composed unit, and implement the function under as traits

/// Used to determine the final type for Addition and Substraction
fn same_unit(left: &Vec<Unit>, right: &Vec<Unit>) -> Option<Vec<Unit>> {
    match (left, right) {
        // We don't have any unit
        (left, right) if left.is_empty() && right.is_empty() => Some(vec![]),

        // Only one side has unit, let's convert this to the unit
        // ex: 12€ + 4 = 16€
        (left, right) if left.is_empty()  => Some(right.clone()),
        (left, right) if right.is_empty() => Some(left.clone()),
        
        // We have the same type on both sides. Keep the type
        // ex: 12€ + 4€ = 16€
        (left, right) if left == right => Some(left.clone()),

        // TODO: try to convert between types

        _ => None
    }
}

/// Return the conversion factor between 2 units
fn conversion_factor(left: &Vec<Unit>, right: &Vec<Unit>) -> Option<f64> {
    if left.len() != 1 || right.len() != 1 {
        return None
    }

    let l = &left[0];
    let r = &right[0];

    l.conversion_factor(r)
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

        // If the have the same type, let's do a normal addition
        if let Some(unit) = same_unit(&self.unit, &rhs.unit) {
            Ok(Value {
                number: self.number + rhs.number,
                unit: unit
            })
        } else if let Some(conversion_factor) = conversion_factor(&self.unit, &rhs.unit) {
            Ok(Value {
                number: self.number + (rhs.number / BigRational::from_float(conversion_factor).unwrap()),
                unit: self.unit
            })
        } else {
            Err(CalculationError::IncompatibleTypes)
        }

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

        // If the have the same type, let's do a normal substraction
        if let Some(unit) = same_unit(&self.unit, &rhs.unit) {
            return Ok(Value {
                number: self.number - rhs.number,
                unit: unit
            })
        } else {
            Err(CalculationError::IncompatibleTypes)
        }
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

