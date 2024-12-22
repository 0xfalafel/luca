use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::units::percentage::Percentage;
use crate::units::money::Money;

#[derive(Debug, Clone, Copy)]
pub enum ResType {
    Int(i128),
    Float(f64),
    Money(Money),
    Percent(Percentage)
}

#[derive(Debug, PartialEq, Eq)]
pub enum CalculationError {
    PrecisonLoss,
    InvalidOperation
}

impl ResType {
    fn is_money(self) -> bool {
        matches!(self, ResType::Money(_))
    }

    fn money(self) -> Option<Money> {
        if let ResType::Money(money) = self {
            return Some(money)
        }
        None
    }

    fn is_float(self) -> bool {
        matches!(self, ResType::Float(_))
    }

    fn is_int(self) -> bool {
        matches!(self, ResType::Int(_))
    }

    fn is_percentage(self) -> bool {
        matches!(self, ResType::Percent(_))
    }
}

impl Into<f64> for ResType {
    fn into(self) -> f64 {
        match self {
            ResType::Float(f) => f,
            ResType::Int(i) => i as f64,
            ResType::Money(m) => m.into(),
            ResType::Percent(p) => p.into()
        }
    }
}

impl Into<i128> for ResType {
    fn into(self) -> i128 {
        match self {
            ResType::Float(f) => f as i128,
            ResType::Int(i) => i,
            ResType::Money(m) => m.into(),
            ResType::Percent(p) => p.into()
        }
    }
}

impl Neg for ResType{
    type Output = ResType;
    fn neg(self) -> ResType {
        match self {
            ResType::Float(f) => ResType::Float(-f),
            ResType::Int(i) => ResType::Int(-i),
            ResType::Money(m) => ResType::Money(-m),
            ResType::Percent(p) => ResType::Percent(-p)
        }
    }
}

impl fmt::Display for ResType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResType::Int(i) => write!(f, "{}", i),
            ResType::Float(float) => write!(f, "{}", float),
            ResType::Money(m) =>  write!(f, "{}", m),
            ResType::Percent(p) =>  write!(f, "{}", p),
        }
    }
}

macro_rules! impl_arithmetic_op_for_ResType {
    ($trait_name:ident $fn_name:ident $op:tt) => {

        impl $trait_name <ResType> for ResType {
            type Output = Result<ResType, CalculationError>;

            fn $fn_name (self, rhs: ResType) -> Self::Output {
                
                // We have some money, the result will be money
                if self.is_money() || rhs.is_money() {
                    let (money, other) = match (self, rhs) {
                        (a, other) if a.is_money() => (a, other),
                        (a, other) if other.is_money() => (other, a),
                        (_, _) => unreachable!(),
                    };

                    let x: Money = money.money().unwrap();

                    match other {
                        ResType::Percent(p) => {
                            let money = (x $op p); 
                            return Ok(ResType::Money(money))
                        },
                        ResType::Float(f) => {
                            return Ok(ResType::Money(x $op f));
                        },
                        ResType::Money(m) => {
                            return Ok(ResType::Money(x $op m));
                        },
                        ResType::Int(i) => {
                            let money = match x $op i {
                                Ok(val) => val,
                                Err(_)  => return Err(CalculationError::PrecisonLoss)
                            };
                            return Ok(ResType::Money(money));
                        },
                    }
                }
                

                // We have some float, the result will be a float
                if self.is_float() || rhs.is_float() {
                    let (float, other) = match (self, rhs) {
                        (a, other) if a.is_float() => (a, other),
                        (a, other) if other.is_float() => (other, a),
                        (_, _) => unreachable!(),
                    };

                    let x: f64 = float.into();

                    match other {
                        ResType::Percent(p) => {
                            let res = x $op p;
                            return Ok(ResType::Float(res))
                        },
                        ResType::Float(f) => {
                            return Ok(ResType::Float(x $op f));
                        },
                        ResType::Int(i) => {
                            let f = match try_to_f64(i) {
                                Ok(val) => val,
                                Err(_) => return Err(CalculationError::PrecisonLoss)
                            };
                            return Ok(ResType::Float(x $op f));
                        },
                        ResType::Money(_) => {
                            unreachable!("Money should have been catch by the previous code.")
                        },
                    }
                }
                
                // Same thing for the int type
                if self.is_int() || rhs.is_int() {
                    let (int, other) = match (self, rhs) {
                        (a, other) if a.is_int() => (a, other),
                        (a, other) if other.is_int() => (other, a),
                        (_, _) => unreachable!(),
                    };

                    let x: i128 = int.into();

                    match other {
                        ResType::Percent(p) => {
                            let res = match x $op p {
                                Ok(val) => val,
                                Err(_)  => return Err(CalculationError::PrecisonLoss)
                            };
                            return Ok(ResType::Float(res))
                        },
                        ResType::Int(i) => {
                            return Ok(ResType::Int(x $op i));
                        },
                        _ => unreachable!("Other types should have been catch by the previously."),
                    }
                }
                

                // Finally our percent type
                if self.is_percentage() && rhs.is_percentage() {

                    if let (
                        ResType::Percent(p1),
                        ResType::Percent(p2)
                    ) = (self, rhs) {
                        return Ok(ResType::Percent(p1 $op p2))
                    }
                    
                    unreachable!("By construction, we should have 2 percentage type.")
                }
                

                unreachable!("All type should have be cratch by now")
            }
        }
    }
}

impl_arithmetic_op_for_ResType!(Add add +);
impl_arithmetic_op_for_ResType!(Sub sub -);
impl_arithmetic_op_for_ResType!(Mul mul *);
impl_arithmetic_op_for_ResType!(Div div /);

struct PrecisonLossError;

fn try_to_f64(v: i128) -> Result<f64, PrecisonLossError> {
    let attempt = v as f64;
    (attempt as i128 == v)
        .then_some(attempt)
        .ok_or(PrecisonLossError)
}
