use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;
use duplicate::duplicate_item;

use crate::units::math_utils::{try_to_f64, PrecisonLossError};
use crate::units::percentage::Percentage;

// Currency Type
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Currency {
    Euros,
    Dollars
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Currency::Euros => '€',
            Currency::Dollars => '$',
        };
        write!(f, "{}", symbol)
    }
}

// Parse Currency from Str
#[derive(Debug, Eq, PartialEq)]
pub struct ParseCurrencyError;

impl FromStr for Currency {
    type Err = ParseCurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let currency = match s {
            "€" => Currency::Euros,
            "$" => Currency::Dollars,
            _ => return Err(ParseCurrencyError)
        };

        Ok(currency)
    }
}


// Money Type
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Money {
    pub amount: f64,
    pub currency: Currency
}

impl Money {
    pub fn new(amount: f64, currency: Currency) -> Money {
        Money {amount: amount, currency: currency}
    }

    fn conversion(self, new_currency: Currency) -> Money {
        if self.currency == new_currency {
            return self;
        }

        if self.currency == Currency::Euros && new_currency == Currency::Dollars {
            Money::new(self.amount * 1.05, new_currency)
            
        } else if self.currency == Currency::Dollars && new_currency == Currency::Euros {
            Money::new(self.amount / 1.05, new_currency)
            
        } else {
            unreachable!("Fuck, we need to thing about conversion now :(")
        }
    }
}

// Parse Currency from Str
#[derive(Debug, Eq, PartialEq)]
pub struct ParseMoneyError;

impl FromStr for Money {
    type Err = ParseMoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let first_char = s.chars().next().ok_or(ParseMoneyError)?;
        let last_char = s.chars().last().ok_or(ParseMoneyError)?;

        let mut s: String = s.to_owned();

        // Currency symbol is at the start: $47.0
        if !first_char.is_ascii_digit() {
            let currency = Currency::from_str(&first_char.to_string()).map_err(|_| ParseMoneyError)?;

            s.remove(0); // remove currency symbol / first char
            let s = s.trim(); // remove whitespace

            let amount: f64 = f64::from_str(s).map_err(|_| ParseMoneyError)?;

            Ok(Money::new(amount, currency))


        // Currency symbol is at the end: 12.0€
        } else if !last_char.is_ascii_digit() {
            let currency = Currency::from_str(&last_char.to_string()).map_err(|_| ParseMoneyError)?;

            s.pop(); // remove currency symbol / last char
            let s: &str = s.trim(); // remove whitespace
            
            let amount: f64 = f64::from_str(s).map_err(|_| ParseMoneyError)?;

            Ok(Money::new(amount, currency))

        } else {
            Err(ParseMoneyError)
        }
    }
}


/*
    Implement Arithmetic Operation for Money
    with other types

    i.e 42€ + 10.0
*/

// implement Add for f64, i32
#[duplicate_item(Type; [f64]; [i32];)]

impl Add<Type> for Money {
    type Output = Money;

    fn add(self, rhs: Type) -> Self::Output {
        Money::new(self.amount + f64::from(rhs), self.currency)
    }
}

// implement Sub for f64, i32
#[duplicate_item(Type; [f64]; [i32];)]

impl Sub<Type> for Money {
    type Output = Money;

    fn sub(self, rhs: Type) -> Self::Output {
        Money::new(self.amount - f64::from(rhs), self.currency)
    }
}

// implement Mul for f64, i32
#[duplicate_item(Type; [f64]; [i32];)]

impl Mul<Type> for Money {
    type Output = Money;

    fn mul(self, rhs: Type) -> Self::Output {
        Money::new(self.amount * f64::from(rhs), self.currency)
    }
}

// implement Div for f64, i32
#[duplicate_item(Type; [f64]; [i32];)]

impl Div<Type> for Money {
    type Output = Money;

    fn div(self, rhs: Type) -> Self::Output {
        Money::new(self.amount / f64::from(rhs), self.currency)
    }
}

// Arithmetic operations for i128

macro_rules! impl_i128_arithmetic_op_for_Money {
    ($trait_name:ident $fn_name:ident $op:tt) => {

        impl $trait_name<i128> for Money {
            type Output = Result<Money, PrecisonLossError>;

            fn $fn_name(self, rhs: i128) -> Self::Output {
                let other: f64 = match try_to_f64(rhs) { 
                    Ok(val) => val,
                    Err(_) => return Err(PrecisonLossError)
                };
                let money = Money::new(self.amount $op other, self.currency);
                Ok(money)
            }
        }
    }
}
impl_i128_arithmetic_op_for_Money!(Add add +);
impl_i128_arithmetic_op_for_Money!(Sub sub -);
impl_i128_arithmetic_op_for_Money!(Mul mul *);
impl_i128_arithmetic_op_for_Money!(Div div /);


/*
    Implement operation for Money with money

impl Add<Money> for Money {
    type Output = Money;

    fn add(self, other: Money) -> Self::Output {
        let other = other.conversion(self.currency);
        Money::new(self.amount + other.amount, self.currency)
    }
}
*/

macro_rules! impl_arithmetic_op_for_Money {
    ($trait_name:ident $fn_name:ident $op:tt) => {
        impl $trait_name<Money> for Money {
            type Output = Money;
        
            fn $fn_name(self, other: Money) -> Self::Output {
                let other = other.conversion(self.currency);
                Money::new(self.amount $op other.amount, self.currency)
            }
        }        
    };
}

impl_arithmetic_op_for_Money!(Add add +);
impl_arithmetic_op_for_Money!(Sub sub -);
impl_arithmetic_op_for_Money!(Mul mul *);
impl_arithmetic_op_for_Money!(Div div /);

// Implement Percentage operations

impl Add<Percentage> for Money {
    type Output = Money;

    fn add(self, rhs: Percentage) -> Self::Output {
        Money {
            amount: self.amount + (self.amount * rhs.value / 100.0),
            currency: self.currency
        }
    }
}

impl Sub<Percentage> for Money {
    type Output = Money;

    fn sub(self, rhs: Percentage) -> Self::Output {
        Money {
            amount: self.amount - (self.amount * rhs.value / 100.0),
            currency: self.currency
        }
    }
}

impl Mul<Percentage> for Money {
    type Output = Money;

    fn mul(self, rhs: Percentage) -> Self::Output {
        Money {
            amount: self.amount * rhs.value / 100.0,
            currency: self.currency
        }
    }
}

impl Neg for Money {
    type Output =  Money;

    fn neg(self) -> Self::Output {
        Money {
            amount: -self.amount,
            currency: self.currency
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.currency {
            Currency::Dollars => write!(f, "${:.2}", self.amount),
            Currency::Euros => write!(f, "{:.2}€", self.amount)         
        }
    }
}

#[duplicate_item(Type; [f64]; [i128]; [i32];)]
impl Into<Type> for Money {
    fn into(self) -> Type {
        self.amount as Type
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currency_from_str() {
        assert_eq!(Currency::from_str("€"), Ok(Currency::Euros));
        assert_eq!(Currency::from_str("$"), Ok(Currency::Dollars));
        assert_eq!(Currency::from_str("Nothing to see here"), Err(ParseCurrencyError));
    }

    #[test]
    fn money_from_str() {
        assert_eq!(Money::from_str("42.0€"), Ok(Money{amount: 42.0, currency: Currency::Euros}));
        assert_eq!(Money::from_str("42€"), Ok(Money{amount: 42.0, currency: Currency::Euros}));
        assert_eq!(Money::from_str("$ 13"), Ok(Money{amount: 13.0, currency: Currency::Dollars}));
        assert_eq!(Money::from_str("$ 13 $"), Err(ParseMoneyError));
    }


    #[test]
    fn add_money() {
        let bob = Money::new(1337.0, Currency::Dollars);
        let alice = Money::new(42.0, Currency::Euros);
        assert_eq!(bob + alice, Money{amount: 1381.1, currency: Currency::Dollars});
    }

    #[test]
    fn arithmetic_operations() {
        assert_eq!(Money::from_str("100€").unwrap() + Money::from_str("500€").unwrap(), Money::from_str("600€").unwrap());
        assert_eq!(Money::from_str("1000€").unwrap() - Money::from_str("500€").unwrap(), Money::from_str("500€").unwrap());
        assert_eq!(Money::from_str("300€").unwrap() * Money::from_str("2€").unwrap(), Money::from_str("600€").unwrap());
        assert_eq!(Money::from_str("100€").unwrap() / Money::from_str("2€").unwrap(), Money::from_str("50€").unwrap());
    }

    #[test]
    fn add_i32() {
        assert_eq!(Money::new(42.0, Currency::Euros) + 12, Money {amount: 54.0, currency: Currency::Euros});
    }
    
    #[test]
    fn mul_i32() {
        assert_eq!(Money::new(42.0, Currency::Euros) * 12, Money {amount: 504.0, currency: Currency::Euros});
    }

    #[test]
    fn add_f64() {
        assert_eq!(Money::new(7.0, Currency::Euros) + f64::from(6.0), Money {amount: 13.0, currency: Currency::Euros});
    }

    #[test]
    fn mul_f64() {
        assert_eq!(Money::new(7.0, Currency::Euros) * 6.0, Money {amount: 42.0, currency: Currency::Euros});
    }

    #[test]
    fn add_percent() {
        assert_eq!(Money::new(42.0, Currency::Euros) + Percentage::new(12.0), Money {amount: 47.04, currency: Currency::Euros});
    }

    #[test]
    fn sub_percent() {
        assert_eq!(Money::from_str("100€").unwrap() - Percentage::new(12.0), Money::from_str("88€").unwrap());
    }

    #[test]
    fn percentage_of() {
        assert_eq!(Money::new(42.0, Currency::Euros) * Percentage::new(12.0), Money {amount: 5.04, currency: Currency::Euros});
    }
}