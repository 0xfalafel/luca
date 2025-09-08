use std::collections::HashMap;
use std::fmt;
use std::ops::{Mul, Div};
use crate::units::unit::Unit;

use super::unit::ConversionFactor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposedUnitError {
    ConversionError
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComposedUnit {
    pub units: HashMap<Unit, i64>,
}

impl ComposedUnit {
    pub fn new() -> ComposedUnit {
        ComposedUnit {
            units: HashMap::new(),
        }
    }

    #[allow(unused)]
    pub fn new_with_unit(unit: Unit) -> ComposedUnit {
        let mut units: HashMap<Unit, i64> = HashMap::new();
        units.insert(unit, 1);

        ComposedUnit {
            units: units,
        }
    }

    #[cfg(test)]
    #[allow(unused)]
    pub fn new_with_hashmap(hasmap: HashMap<Unit, i64>) -> ComposedUnit {
        ComposedUnit {
            units: hasmap
        }
    }

    #[allow(unused)]
    pub fn set_unit(&mut self, unit: Unit, value: i64) {
        self.units.insert(unit, value);
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

    /// Easly create power for units
    /// meter -> square meters
    pub fn power(&self, power: i64) -> ComposedUnit {
        let mut unit = self.clone();
        unit.units.iter_mut().for_each(|(_, pow)| *pow = power);
        unit
    }
}

#[allow(unused)]
impl ComposedUnit {
    pub fn percent() -> ComposedUnit { Self::new_with_unit(Unit::percent()) }
    pub fn meter() -> ComposedUnit { Self::new_with_unit(Unit::meter()) }
    pub fn kilometer() -> ComposedUnit { Self::new_with_unit(Unit::kilometer()) }
    pub fn decimeter() -> ComposedUnit { Self::new_with_unit(Unit::decimeter()) }
    pub fn centimeter() -> ComposedUnit { Self::new_with_unit(Unit::centimeter()) }
    pub fn millimeter() -> ComposedUnit { Self::new_with_unit(Unit::millimeter()) }
    pub fn euro() -> ComposedUnit { Self::new_with_unit(Unit::euro()) }
    pub fn dollar() -> ComposedUnit { Self::new_with_unit(Unit::dollar()) }
    pub fn second() -> ComposedUnit { Self::new_with_unit(Unit::second()) }
    pub fn millisecond() -> ComposedUnit { Self::new_with_unit(Unit::millisecond()) }
    pub fn minute() -> ComposedUnit { Self::new_with_unit(Unit::minute()) }
    pub fn hour() -> ComposedUnit { Self::new_with_unit(Unit::hour()) }
    pub fn square_meters() -> ComposedUnit { Self::meter().power(2) }
    pub fn square_kilometers() -> ComposedUnit { Self::kilometer().power(2) }
    pub fn square_decimeters() -> ComposedUnit { Self::decimeter().power(2) }
    pub fn square_centimeters() -> ComposedUnit { Self::centimeter().power(2) }
    pub fn square_millimeters() -> ComposedUnit { Self::millimeter().power(2) }
    pub fn inch() -> ComposedUnit { Self::new_with_unit(Unit::inch()) }
    pub fn feet() -> ComposedUnit { Self::new_with_unit(Unit::feet()) }
}

impl Mul<ComposedUnit> for ComposedUnit {
    type Output = (ConversionFactor, ComposedUnit);

    fn mul(self, rhs: ComposedUnit) -> Self::Output {
        let mut conversion_factor = 1.0;
        let mut new_hashmap = self.units.clone();
        
        for (unit, power) in rhs.units {
            // Call conversion_factor for each unit

            if self.units.keys().any(|u| u.can_be_converted_to(&unit)) {
                for key in self.units.keys() {
                    if let Some(factor) = key.conversion_factor(&unit) {
                        conversion_factor *= factor.powi(power as i32);
    
                        new_hashmap
                            .entry(key.clone())
                            .and_modify(|val| *val += power)
                            .or_insert(power);
                    }
                }

            } else {
                new_hashmap
                    .insert(unit, power);
            }
        }

        // Remove any power that is equal to 0
        new_hashmap.retain(|_, power| *power != 0);

        (conversion_factor, ComposedUnit {units: new_hashmap})
    }
}

impl Div<ComposedUnit> for ComposedUnit {
    type Output = (ConversionFactor, ComposedUnit);

    fn div(self, rhs: ComposedUnit) -> Self::Output {
        let mut conversion_factor = 1.0;
        let mut new_hashmap = self.units.clone();
        
        for (unit, power) in rhs.units {
            // Call conversion_factor for each unit

            if self.units.keys().any(|u| u.can_be_converted_to(&unit)) {
                for key in self.units.keys() {
                    if let Some(factor) = key.conversion_factor(&unit) {
                        conversion_factor *= factor.powi(power as i32);
    
                        new_hashmap
                            .entry(key.clone())
                            .and_modify(|val| *val -= power)
                            .or_insert(power);
                    }
                }

            } else {
                new_hashmap
                    .insert(unit, -power);
            }
        }

        // Remove any power that is equal to 0
        new_hashmap.retain(|_, power| *power != 0);

        (conversion_factor, ComposedUnit {units: new_hashmap})
    }
}


/// Return a String containing the `nb` as an exponent
/// 24 -> ²⁴
fn pretty_exponent(nb: &i64) -> String {
    if *nb < 1 {
        unreachable!("This value should never be under 1");
    }

    if *nb == 1 {
        return String::from("");
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
                .map(|(unit, power)| format!("{}{}", unit.symbol, pretty_exponent(&-power)))
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

        let mut assert_unit = HashMap::new();
        assert_unit.insert(Unit::meter(), 2);
        assert_eq!(res, (100.0, ComposedUnit {units: assert_unit}));
    }
}