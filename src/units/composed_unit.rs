use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

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
}