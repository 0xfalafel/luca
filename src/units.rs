use std::{fmt, ops::{Add, Div, Mul, Neg, Sub}};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Currency {
    Euro,
    Dollar
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Currency::Euro => '€',
            Currency::Dollar => '$',
        };
        write!(f, "{}", symbol)
    }
}


//#############################################################
//   Types used for the interpreter response
//#############################################################

/// Result of parsing the AST
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ResType {
    Int(i128),
    Float(f64),
    Money(f64, Currency)
}

impl ResType {
    fn get_i128(self) -> i128 {
        match self {
            ResType::Int(val) => {val},
            ResType::Float(val) => {val as i128}
            ResType::Money(val, _currency) => {val as i128}
        }
    }
    
    fn get_f64(self) -> f64 {
        match self {
            ResType::Float(val) => {val},
            ResType::Int(val) => {val as f64},
            ResType::Money(val, _currency) => {val},
        }
    }

    fn get_currency(self) -> Option<Currency> {
        match self {
            ResType::Money(_, currency) => {Some(currency)},
            _ => {None}
        }
    }
}

impl Add for ResType {
    type Output = Self; 
    
    fn add(self, other: Self) -> ResType {
        match (self, other) {
            
            // Both numbers are of type Money
            (left, right) if matches!(left, ResType::Money(_, _)) && matches!(right, ResType::Money(_, _)) => {
                let currency_left = left.get_currency().unwrap();
                let currency_right = right.get_currency().unwrap();

                if currency_left != currency_right {
                    panic!("We don't support conversions at the moment");
                }
                
                ResType::Money(left.get_f64() + right.get_f64(), currency_left)
            },
            
            // Left number is of type Money
            (left, right) if matches!(left, ResType::Money(_, _)) => {
                let currency_left = left.get_currency().unwrap();
                ResType::Money(left.get_f64() + right.get_f64(), currency_left)
            }

            // Right number is of type Money
            (left, right) if matches!(right, ResType::Money(_, _)) => {
                let currency_left = right.get_currency().unwrap();
                ResType::Money(left.get_f64() + right.get_f64(), currency_left)
            }

            // One of the types is Float
            (left_value, right_value) if matches!(left_value, ResType::Float(_)) || matches!(right_value, ResType::Float(_)) => {
                ResType::Float(left_value.get_f64() + right_value.get_f64())
            },
            // Both Integers
            _ => {
                ResType::Int(self.get_i128() + other.get_i128())
            }
        }
    }
}

impl Sub for ResType {
    type Output = Self; 
    
    fn sub(self, other: Self) -> ResType {
        match (self, other) {
            
            // Both numbers are of type Money
            (left, right) if matches!(left, ResType::Money(_, _)) && matches!(right, ResType::Money(_, _)) => {
                let currency_left = left.get_currency().unwrap();
                let currency_right = right.get_currency().unwrap();

                if currency_left != currency_right {
                    panic!("We don't support conversions at the moment");
                }
                
                ResType::Money(left.get_f64() - right.get_f64(), currency_left)
            },
            
            // Left number is of type Money
            (left, right) if matches!(left, ResType::Money(_, _)) => {
                let currency_left = left.get_currency().unwrap();
                ResType::Money(left.get_f64() - right.get_f64(), currency_left)
            }

            // Right number is of type Money
            (left, right) if matches!(right, ResType::Money(_, _)) => {
                let currency_left = right.get_currency().unwrap();
                ResType::Money(left.get_f64() - right.get_f64(), currency_left)
            }

            // One of the types is Float
            (left_value, right_value) if matches!(left_value, ResType::Float(_)) || matches!(right_value, ResType::Float(_)) => {
                ResType::Float(left_value.get_f64() - right_value.get_f64())
            },
            // Both Integers
            _ => {
                ResType::Int(self.get_i128() - other.get_i128())
            }
        }
    }
}

impl Mul for ResType {
    type Output = Self; 
    
    fn mul(self, other: Self) -> ResType {
        match (self, other) {
            
            // Both numbers are of type Money
            (left, right) if matches!(left, ResType::Money(_, _)) && matches!(right, ResType::Money(_, _)) => {
                let currency_left = left.get_currency().unwrap();
                let currency_right = right.get_currency().unwrap();

                if currency_left != currency_right {
                    panic!("We don't support conversions at the moment");
                }
                
                ResType::Money(left.get_f64() * right.get_f64(), currency_left)
            },
            
            // Left number is of type Money
            (left, right) if matches!(left, ResType::Money(_, _)) => {
                let currency_left = left.get_currency().unwrap();
                ResType::Money(left.get_f64() * right.get_f64(), currency_left)
            }

            // Right number is of type Money
            (left, right) if matches!(right, ResType::Money(_, _)) => {
                let currency_left = right.get_currency().unwrap();
                ResType::Money(left.get_f64() * right.get_f64(), currency_left)
            }

            // One of the types is Float
            (left_value, right_value) if matches!(left_value, ResType::Float(_)) || matches!(right_value, ResType::Float(_)) => {
                ResType::Float(left_value.get_f64() * right_value.get_f64())
            },
            // Both Integers
            _ => {
                ResType::Int(self.get_i128() * other.get_i128())
            }
        }
    }
}

impl Div for ResType {
    type Output = Self; 
    
    fn div(self, other: Self) -> ResType {
        match (self, other) {
            
            // Both numbers are of type Money
            (left, right) if matches!(left, ResType::Money(_, _)) && matches!(right, ResType::Money(_, _)) => {
                let currency_left = left.get_currency().unwrap();
                let currency_right = right.get_currency().unwrap();

                if currency_left != currency_right {
                    panic!("We don't support conversions at the moment");
                }
                
                ResType::Money(left.get_f64() / right.get_f64(), currency_left)
            },
            
            // Left number is of type Money
            (left, right) if matches!(left, ResType::Money(_, _)) => {
                let currency_left = left.get_currency().unwrap();
                ResType::Money(left.get_f64() / right.get_f64(), currency_left)
            }

            // Right number is of type Money
            (left, right) if matches!(right, ResType::Money(_, _)) => {
                let currency_left = right.get_currency().unwrap();
                ResType::Money(left.get_f64() / right.get_f64(), currency_left)
            }

            // One of the types is Float
            (left_value, right_value) if matches!(left_value, ResType::Float(_)) || matches!(right_value, ResType::Float(_)) => {
                ResType::Float(left_value.get_f64() / right_value.get_f64())
            },

            // Both are Integers
            _ => {
                let left_val = self.get_i128();
                let right_val = other.get_i128();

                // If the divison returns a round value give an Integer
                if left_val % right_val == 0 {
                    ResType::Int(self.get_i128() / other.get_i128())

                // Otherwise, we return a Float
                } else {
                    ResType::Float(self.get_f64() / other.get_f64())
                }
            }
        }
    }
}

impl Neg for ResType {
    type Output = Self; 
    
    fn neg(self) -> Self::Output {
        match self {
            ResType::Int(val) => ResType::Int(-val),
            ResType::Float(val) => ResType::Float(-val),
            ResType::Money(val, currency) => ResType::Money(-val, currency),
        }        
    }
}

impl fmt::Display for ResType {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResType::Int(val)  => {write!(f, "{}", val)},
            ResType::Float(val) => {write!(f, "{:?}", val)},
            ResType::Money(val, currency) => {
                write!(f, "{:.2} {}", val, currency)
            },
        }
    }
}
