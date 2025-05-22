use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

use crate::units::unit::Unit;
use crate::units::number::Number;
use crate::units::composed_unit::{ComposedUnit, ComposedUnitError};

#[derive(Debug, Clone, PartialEq)]
pub enum ValueError {
    InvalidConversion,
    FailedToParseConversionFactor,
    FailedToParseNumber,
    FailedToConvertionNumberFloat,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub number: Number,
    unit: ComposedUnit,
}

impl Value {
    pub fn new(number: Number) -> Value {
        Value {
            number: number,
            unit: ComposedUnit::new(),
        }
    }

    pub fn new_with_unit(number: Number, unit: &Unit) -> Value {
        Value { number: number, unit: ComposedUnit::new_with_unit(unit.clone()) }
    }

    #[allow(unused)]
    pub fn new_with_units(number: Number, units: ComposedUnit) -> Value {
        Value { number: number, unit: units }
    }

    pub fn set_unit(self, unit: Unit) -> Value {
        Value { number: self.number, unit: ComposedUnit::new_with_unit(unit) }
    }

    pub fn convert_to_unit(&self, unit: &ComposedUnit) -> Result <Value, ValueError> {
        
        let factor = match self.unit.conversion_factor(unit) {
            Ok(conversion_factor) => conversion_factor,
            Err(ComposedUnitError::ConversionError) => {
                return Err(ValueError::InvalidConversion)
            }
        };

        let factor = match Number::from_float(factor) {
            Some(factor) => factor,
            None => return Err(ValueError::FailedToParseConversionFactor),
        };

        Ok( Value {
            number: self.number.clone() * factor,
            unit: unit.clone(),
        })
    }

    pub fn convert_to_value(&self, rhs: &Value) -> Result <Value, ValueError> {
        let mut conversion_factor = 1.0;
        let mut units = self.unit.clone();

        // For each unit in `self`, we look if it can be converted to a unit of `rhs`
        for (unit, _) in &rhs.unit.units {
            for (key, power) in &self.unit.units {
                if let Some(factor) = key.conversion_factor(unit) {
                    conversion_factor *= factor.powi(*power as i32);

                    // We replace the previous unit, with the corresopnding unit
                    // of `rhs`
                    let tmp_power = units.units.remove(&key).unwrap();
                    units.units.insert(unit.clone(), tmp_power);
                }
            }
        }
        
        let conversion_factor = Number::from_float(conversion_factor)
            .ok_or(ValueError::FailedToParseConversionFactor)?; 

        Ok(Value {
            number: self.number.clone() * conversion_factor,
            unit: units,
        })
    }

    pub fn is_percent(&self) -> bool {
        self.unit == ComposedUnit::new_with_unit(Unit::percent())
    }

    pub fn has_no_unit(&self) -> bool {
        self.unit.is_empty()
    }

    pub fn pow(&self, rhs: Value) -> Result<Value, ValueError> {
        if !rhs.has_no_unit() {
            eprintln!("exponent should have no units");
        }

        match self.number.pow(rhs.number) {
            Ok(num) => {
                Ok(Value {
                    number: num,
                    unit: self.unit.clone(),
                })                
            },
            Err(e) => Err(e),
        }
    }
}

// Helper function to create Value with Unit in tests
impl Value {
    #[allow(unused)]
    pub fn from_f64_with_unit(number: f64, unit: Unit) -> Value {
        Value {
            number: Number::from_float(number).unwrap(),
            unit: ComposedUnit::new_with_unit(unit),
        }
    }

    #[allow(unused)]
    pub fn from_int(number: i64) -> Value {
        Value {
            number: Number::from_i64(number).unwrap(),
            unit: ComposedUnit::new(),
        }
    }
    
    #[allow(unused)]
    pub fn from_float(number: f64) -> Value {
        Value {
            number: Number::from_float(number).unwrap(),
            unit: ComposedUnit::new(),
        }
    }
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.number, self.unit)
    }
}

#[derive(Debug)]
pub enum CalculationError {
    IncompatibleTypes
}

/// Todo: Create a composed unit, and implement the function under as traits

/// Used to determine the final type for Addition and Substraction
fn same_unit(left: &ComposedUnit, right: &ComposedUnit) -> Option<ComposedUnit> {
    match (left, right) {
        // We don't have any unit
        (left, right) if left.is_empty() && right.is_empty() => Some(ComposedUnit::new()),

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
                number: val.clone() + val * percentage / Number::from_u8(100),
                unit: unit
            })
        }

        // If the have the same type, let's do a normal addition
        if let Some(unit) = same_unit(&self.unit, &rhs.unit) {
            Ok(Value {
                number: self.number + rhs.number,
                unit: unit
            })

        // We can convert between the unit, do a conversion for `rhs`.
        // We keep the unit of `self`.
        } else if let Ok(conversion_factor) = &self.unit.conversion_factor(&rhs.unit) {
            Ok(Value {
                number: self.number + (rhs.number / Number::from_float(*conversion_factor).unwrap()),
                unit: self.unit
            })

        // We can't add different types
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
                number: val.clone() - val * percentage / Number::from_u8(100),
                unit: unit
            })
        }

        // If the have the same type, let's do a normal addition
        if let Some(unit) = same_unit(&self.unit, &rhs.unit) {
            Ok(Value {
                number: self.number - rhs.number,
                unit: unit
            })

        // We can convert between the unit, do a conversion for `rhs`.
        // We keep the unit of `self`.
        } else if let Ok(conversion_factor) = &self.unit.conversion_factor(&rhs.unit) {
            Ok(Value {
                number: self.number - (rhs.number / Number::from_float(*conversion_factor).unwrap()),
                unit: self.unit
            })

        // We can't add different types
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
                number: val * percentage / Number::from_u8(100),
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

        let (factor, new_unit) = self.unit * rhs.unit;

        Ok(Value {
            number: self.number * (rhs.number / factor)
                // TODO: funsion CalculationError and ValueError
                .map_err(|_| CalculationError::IncompatibleTypes)?,
            unit: new_unit,
        })
    }
}

impl Div<Value> for Value {
    type Output = Result<Value, CalculationError>;

    fn div(self, rhs: Value) -> Self::Output {
        // Handle the special case of percentage substraction:
        // 100€ / 10% = 100€ * 10 / 100 = 1000€
        if rhs.is_percent() {
            let (percentage, val, unit) = match self.is_percent() {
                true  => (self.number, rhs.number, rhs.unit),
                false => (rhs.number, self.number, self.unit)
            };

            return Ok(Value { 
                number: val * Number::from_u8(100) / percentage,
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
        

        let (factor, new_unit) = self.unit / rhs.unit;

        Ok(Value {
            number: self.number * (rhs.number * factor)
                // TODO: funsion CalculationError and ValueError
                .map_err(|_| CalculationError::IncompatibleTypes)?,
            unit: new_unit,
        })
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        Value { number: -self.number, unit: self.unit }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "library bug"]
    fn add_m_to_cm() {
        // 12cm + 1 m
        let cm = Value::new_with_unit(Number::from_float(12.0).unwrap(), &Unit::centimeter());
        let m = Value::new_with_unit(Number::from_u8(1), &Unit::meter());
        let res = Value::new_with_unit(Number::from_float(112.0).unwrap(), &Unit::centimeter());
        assert_eq!(res, (cm + m).unwrap());
    }

}
