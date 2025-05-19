use std::collections::HashMap;
use std::fmt;
use std::ops::Mul;
use crate::units::unit::Unit;

use super::unit::ConversionFactor;

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


    // pub fn convert_to_unit(&self, unit: &ComposedUnit) -> Result<(f64, ComposedUnit), ComposedUnitError>{
    //     // If one unit is empty, take the type of the other
    //     if self.is_empty() {
    //         return Ok((1.0, unit.clone()))
    //     } else if unit.is_empty() {
    //         return Ok((1.0, self.clone()))
    //     }

    //     let conversion_factor = self.conversion_factor(unit)?;
    //     Ok((conversion_factor, self.clone()))
    // }
}

fn unit_in_vector(vec: &Vec<Unit>, unit: &Unit) -> Option<(ConversionFactor, Unit)> {
    for elem in vec.iter() {
        if let Some(factor) = elem.conversion_factor(unit) {
            return Some((factor, elem.clone()));
        }
    }

    None
}

impl Mul<ComposedUnit> for ComposedUnit {
    type Output = (ConversionFactor, ComposedUnit);

    fn mul(self, rhs: ComposedUnit) -> Self::Output {
        let mut conversion_factor = 1.0;
        let mut new_numerator = self.numerator.clone();

        for unit in rhs.numerator.iter() {
            if let Some((factor, u)) = unit_in_vector(&self.numerator, unit) {
                conversion_factor = conversion_factor * factor;
                new_numerator.push(u);
            } else {
                new_numerator.push(unit.clone());
            }
        }

        (conversion_factor, ComposedUnit {
            numerator: new_numerator,
            denominator: self.denominator.clone(),
        })
    }
}

fn regroup_unit_with_power(units: &Vec<Unit>) -> HashMap<Unit, u64> {
    let mut counts = HashMap::new();
    
    for unit in units {
        *counts.entry(unit.clone()).or_insert(0) += 1;
    }

    counts
}

/// Return a String containing the `nb` as an exponent
/// 24 -> ²⁴
fn pretty_exponent(nb: &u64) -> String {
    if *nb == 0 | 1 {
        return String::from("")
    }

    let nb_as_string = nb.to_string();
    let mut exponent = String::new();

    for char in nb_as_string.chars() {
        let c = match char {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            _ => unreachable!("A u64 should only have number when printed"),
        };
        exponent.push(c);
    }
    exponent
}

impl fmt::Display for ComposedUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        let grouped_unit = regroup_unit_with_power(&self.numerator);

        for (unit, power) in grouped_unit.iter() {
            output.push_str(&unit.symbol);
            output.push_str(&pretty_exponent(power));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composed_uint_m_to_cm() {
        let m: ComposedUnit = ComposedUnit::new_with_unit(Unit::meter());
        assert_eq!(Ok(100.0), m.conversion_factor(&ComposedUnit::new_with_unit(Unit::centimeter())));
    }

    #[test]
    fn composed_uint_mm_to_km() {
        let mm: ComposedUnit = ComposedUnit::new_with_unit(Unit::millimeter());
        assert_eq!(Ok(1e-6), mm.conversion_factor(&ComposedUnit::new_with_unit(Unit::kilometer())));
    }

    #[test]
    fn mul_m_cm() {
        let m: ComposedUnit = ComposedUnit::new_with_unit(Unit::meter());
        let cm: ComposedUnit = ComposedUnit::new_with_unit(Unit::centimeter());

        let res = m * cm;
        assert_eq!(res, (100.0, ComposedUnit {
            numerator: vec![Unit::meter(), Unit::meter()],
            denominator: vec![],
        }));
    }
}