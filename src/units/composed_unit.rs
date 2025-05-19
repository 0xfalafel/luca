use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::ops::Mul;
use crate::units::unit::Unit;

use super::unit::{self, ConversionFactor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposedUnitError {
    ConversionError
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComposedUnit {
    units: HashMap<Unit, i64>,
}

impl ComposedUnit {
    pub fn new() -> ComposedUnit {
        ComposedUnit {
            units: HashMap::new(),
        }
    }

    pub fn new_with_unit(unit: Unit) -> ComposedUnit {
        let mut units = HashMap::new();
        units.insert(unit, 1);

        ComposedUnit {
            units: units,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    pub fn conversion_factor(&self, rhs: &ComposedUnit) -> Result<f64, ComposedUnitError>{
        let mut conversion_factor = 1.0;

        for (key, _power) in rhs.units.iter() {
            for unit in self.units.keys().into_iter() {
                if let Some(f) = unit.conversion_factor(key) {
                    conversion_factor *= f;
                } else {
                    return Err(ComposedUnitError::ConversionError);
                }
            }
        }
        Ok(conversion_factor)
    }

    fn has_denominator(&self) -> bool {
        self.units.values().any(|power| power.is_negative())
    }
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
fn pretty_exponent(nb: &i64) -> String {
    if *nb < 1 {
        unreachable!("This value should never be under 1");
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
            _ => unreachable!("A u128 should only have number when printed"),
        };
        exponent.push(c);
    }
    exponent
}

impl fmt::Display for ComposedUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let mut output = String::new();

        // numerator (units above 0)
        let numerator: String = self.units
            .iter()
            .filter(|&(_u, power)| *power > 0)
            .map(|(unit, power)| format!("{}{}", unit.symbol, pretty_exponent(power)))
            .collect();

        if !self.has_denominator() {
            write!(f, "{}", numerator)
        
        // We have a denominator
        } else {
            let denominator: String = self.units
                .iter()
                .filter(|&(_u, power)| *power < 0)
                .map(|(unit, power)| format!("{}{}", unit.symbol, pretty_exponent(power)))
                .collect();

                write!(f, "{}/{}", numerator, denominator)
        }
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