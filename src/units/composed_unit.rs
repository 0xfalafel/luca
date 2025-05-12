use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::units::unit::Unit;

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