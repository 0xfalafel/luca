use num_rational::BigRational;
enum Unit {
    Euros,
    Dollars
}

pub struct Value {
    number: BigRational,
    unit: Vec<Unit>
}

impl Value {
    pub fn new(number: BigRational) -> Value {
        Value {
            number: number,
            unit: vec![]
        }
    }
}