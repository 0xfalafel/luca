use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::units::unit::Unit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposedUnitError {
    ConversionError
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComposedUnit {
    numerator: Vec<Unit>,
    denominator: Vec<Unit>,
}

impl ComposedUnit {
    pub fn new() -> ComposedUnit {
        ComposedUnit {
            numerator: vec![],
            denominator: vec![],
        }
    }

    pub fn new_with_unit(unit: Unit) -> ComposedUnit {
        ComposedUnit {
            numerator: vec![unit],
            denominator: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.numerator.is_empty() && self.denominator.is_empty()
    }

    pub fn conversion_factor(&self, unit: &ComposedUnit) -> Result<f64, ComposedUnitError>{
        if self.numerator.len() != unit.numerator.len() && self.denominator.len() != unit.denominator.len() {
            return Err(ComposedUnitError::ConversionError)
        }

        let mut conversion_factor = 1.0;

        for units in self.numerator.iter().zip(unit.numerator.iter()) {
            let (self_numerator, other_denominator) = units;

            match self_numerator.conversion_factor(other_denominator) {
                Some(factor) => conversion_factor *= factor,
                None => return Err(ComposedUnitError::ConversionError),
            }
        }

        for units in self.denominator.iter().zip(unit.denominator.iter()) {
            let (self_denominator, other_denominator) = units;

            match self_denominator.conversion_factor(other_denominator) {
                Some(factor) => conversion_factor /= factor,
                None => return Err(ComposedUnitError::ConversionError),
            }
        }

        Ok(conversion_factor)
    }

    pub fn convert_to_unit(&self, unit: &ComposedUnit) -> Result<(f64, ComposedUnit), ComposedUnitError>{
        // If one unit is empty, take the type of the other
        if self.is_empty() {
            return Ok((1.0, unit.clone()))
        } else if unit.is_empty() {
            return Ok((1.0, self.clone()))
        }

        let conversion_factor = self.conversion_factor(unit)?;
        Ok((conversion_factor, self.clone()))
    }
}

impl fmt::Display for ComposedUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for unit in self.numerator.iter() {
            output.push_str(&unit.symbol);
        }

        if !self.denominator.is_empty() {
            output.push('/');

            for unit in self.denominator.iter() {
                output.push_str(&unit.symbol);
            }
        }
        write!(f, "{}", output)
    }
}