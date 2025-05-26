use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use luca::units::composed_unit::ComposedUnit;
use luca::units::number::Number;
use luca::units::unit::Unit;
use luca::value::Value;
use luca::interpreter::*;

fn make_interpreter(text: &str, variables: Option<Rc<RefCell<HashMap<String, Value>>>>) -> Interpreter {
    
    // Create an empty variables array if none is defined
    let vars = match variables {
        Some(vars) => vars,
        None => Rc::new(RefCell::new(HashMap::new()))
    };

    let lexer = Lexer::new(String::from(text));
    let parser = Parser::new(lexer).expect("Could not parse");
    let interpreter = Interpreter::new(parser, vars);

    interpreter
}

#[test]
fn test_expression1() {
    let mut interpreter = make_interpreter("3", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(3)));
}

#[test]
fn test_expression2() {
    let mut interpreter = make_interpreter("2 + 7 * 4", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(30)));
}

#[test]
fn test_expression3() {
    let mut interpreter = make_interpreter("7 - 8 / 4", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(5)));
}

#[test]
fn test_expression4() {
    let mut interpreter = make_interpreter("14 + 2 * 3 - 6 / 2", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(17)));
}

#[test]
fn test_expression5() {
    let mut interpreter = make_interpreter("7 + 3 * (10 / (12 / (3 + 1) - 1))", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(22)));
}

#[test]
fn test_expression6() {
    let mut interpreter = make_interpreter(
        "7 + 3 * (10 / (12 / (3 + 1) - 1)) / (2 + 3) - 5 - 3 + (8)", None
    );
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(10)));
}

#[test]
fn test_expression7() {
    let mut interpreter = make_interpreter("7 + (((3 + 2)))", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(12)));
}

#[test]
fn test_expression_invalid_syntax() {
    let mut interpreter = make_interpreter("10 *", None);
    let result = interpreter.interpret();
    assert_eq!(result, Err(Error::InvalidSyntax));
}

#[test]
fn test_expression_unary() {
    let mut interpreter = make_interpreter("---42", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(-42)));
}

#[test]
fn test_expression_unary2() {
    let mut interpreter = make_interpreter("-6*-7 - 3", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(39)));
}

#[test]
fn test_expression_variable1() {
    let vars : Rc<RefCell<HashMap<String, Value>>> = Rc::new(RefCell::new(HashMap::new()));

    let mut interpreter = make_interpreter("a=5", Some(vars.clone()));
    _ = interpreter.interpret();
    let mut interpreter = make_interpreter("a", Some(vars));
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(5)));
}

#[test]
fn test_expression_variable2() {
    let vars : Rc<RefCell<HashMap<String, Value>>> = Rc::new(RefCell::new(HashMap::new()));

    let mut interpreter = make_interpreter("bob=(525+83)/4", Some(vars.clone()));
    _ = interpreter.interpret();
    let mut interpreter = make_interpreter("bob + 48", Some(vars));
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(200)));
}

#[test]
fn test_expression_variable3() {
    let vars : Rc<RefCell<HashMap<String, Value>>> = Rc::new(RefCell::new(HashMap::new()));

    let mut interpreter = make_interpreter("a=2", Some(vars.clone()));
    _ = interpreter.interpret();
    let mut interpreter = make_interpreter("b=1", Some(vars.clone()));
    _ = interpreter.interpret();
    let mut interpreter = make_interpreter("b=3", Some(vars.clone()));
    _ = interpreter.interpret();
    let mut interpreter = make_interpreter("a+b", Some(vars));
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(5)));
}

#[test]
fn test_float() {
    let mut interpreter = make_interpreter("4.0", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_float(4.0)));
}

#[test]
fn test_negative_float() {
    let mut interpreter = make_interpreter("-16.0 + 4", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_float(-12.0)));
}

#[test]
fn test_division1() {
    let mut interpreter = make_interpreter("20/4", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(5)));
}

#[test]
fn test_division2() {
    let mut interpreter = make_interpreter("-5/2", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_float(-2.5)));
}

#[test]
fn test_division_zero() {
    let mut interpreter = make_interpreter("120/0", None);
    let result = interpreter.interpret();
    assert_eq!(result, Err(Error::DivisonByZero));
}

#[test]
fn test_money1() {
    let mut interpreter = make_interpreter("12€", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_f64_with_unit(12.0, Unit::euro())))
}

#[test]
fn test_money2() {
    let mut interpreter = make_interpreter("$47", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_f64_with_unit(47.0, Unit::dollar())));
}

#[test]
fn test_money_add() {
    let mut interpreter = make_interpreter("22€ + 8", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_f64_with_unit(30.0, Unit::euro())));
}

#[test]
fn test_money_sub() {
    let mut interpreter = make_interpreter("500€ - 1000€", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_f64_with_unit(-500.0, Unit::euro())));
}

#[test]
fn test_money_mul() {
    let mut interpreter = make_interpreter("$33 * -4", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_f64_with_unit(-132.0, Unit::dollar())));
}

#[test]
fn test_money_div() {
    let mut interpreter = make_interpreter("25€ / 4", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_f64_with_unit(6.25, Unit::euro())));
}

#[test]
fn test_percentage_of() {
    let mut interpreter = make_interpreter("20% of (50+50)", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(20)));
}

#[test]
fn meter() {
    let mut interpreter = make_interpreter("10 m", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_f64_with_unit(10.0, Unit::meter())));        
}

#[test]
fn centimeter() {
    let mut interpreter = make_interpreter("100cm", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_f64_with_unit(100.0, Unit::centimeter())));        
}

#[test]
fn km_to_millimeter() {
    let mut interpreter = make_interpreter("10 km as millimeter", None);
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_f64_with_unit(10000000.0, Unit::meter().with_prefix("milli"))));        
}

#[test]
fn test_handling_spaces() {
    let mut interpreter = make_interpreter("4€ b", None);
    let _ = interpreter.interpret();
}

#[test]
fn implicit_multiplication() {
    let vars : Rc<RefCell<HashMap<String, Value>>> = Rc::new(RefCell::new(HashMap::new()));
    
    let mut interpreter = make_interpreter("a=2", Some(vars.clone()));
    _ = interpreter.interpret();
    let mut interpreter = make_interpreter("4a", Some(vars));
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(8)));
}

#[test]
#[ignore]
fn implicit_multiplication2() {
    let vars : Rc<RefCell<HashMap<String, Value>>> = Rc::new(RefCell::new(HashMap::new()));
    
    let mut interpreter = make_interpreter("a=2", Some(vars.clone()));
    _ = interpreter.interpret();
    let mut interpreter = make_interpreter("b=-3", Some(vars.clone()));
    _ = interpreter.interpret();
    let mut interpreter = make_interpreter("4ab", Some(vars));
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(-24)));
}

#[test]
#[ignore]
fn implicit_multiplication3() {
    let vars : Rc<RefCell<HashMap<String, Value>>> = Rc::new(RefCell::new(HashMap::new()));
    
    let mut interpreter = make_interpreter("a=2", Some(vars.clone()));
    _ = interpreter.interpret();
    let mut interpreter = make_interpreter("b=3", Some(vars.clone()));
    _ = interpreter.interpret();
    let mut interpreter = make_interpreter("4ab + 2 ab", Some(vars));
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_int(-24)));
}

#[test]
fn scenario_cinema() {
    let vars : Rc<RefCell<HashMap<String, Value>>> = Rc::new(RefCell::new(HashMap::new()));
    
    let mut interpreter = make_interpreter("enfant=4€", Some(vars.clone()));
    _ = interpreter.interpret();
    let mut interpreter = make_interpreter("adulte=12€", Some(vars.clone()));
    _ = interpreter.interpret();
    let mut interpreter = make_interpreter("2adultes+3 enfants", Some(vars));
    let result = interpreter.interpret();
    assert_eq!(result, Ok(Value::from_f64_with_unit(36.0, Unit::euro())));
}

#[test]
#[ignore = "bug in a library used"]
fn comma_sub() {
    let mut interpreter = make_interpreter("872,87 - 850", None);
    let result = interpreter.interpret();

    assert_eq!(result, Ok(Value::from_float(22.87)));
}    

#[test]
fn simple_symbol() {
    let mut interpreter = make_interpreter("€", None);
    assert_eq!(interpreter.interpret(), Err(Error::InvalidSyntax));
}

#[test]
fn percent_divided() {
    let result = make_interpreter("30% / 2", None).interpret();
    assert_eq!(result, Ok(Value::from_f64_with_unit(15.0, Unit::percent())));
}

#[test]
fn m_div_by_dm() {
    let result = make_interpreter("120m / 1dm", None).interpret();
    assert_eq!(result, Ok(Value::new(Number::from_i64(1200).unwrap())));
}

#[test]
fn number_with_spaces() {
    let result = make_interpreter("10 000", None).interpret();
    assert_eq!(result, Ok(Value::new(Number::from_i64(10000).unwrap())));
}

#[test]
fn number_with_underscores() {
    let result = make_interpreter("12_345_678", None).interpret();
    assert_eq!(result, Ok(Value::new(Number::from_i64(12345678).unwrap())));
}

#[test]
fn euro_per_km() {
    let result = make_interpreter("10000 € / km", None).interpret();
    let number = Number::from_i64(10000).unwrap();
    let mut units = ComposedUnit::new_with_unit(Unit::euro());
    units.set_unit(Unit::kilometer(), -1);
    assert_eq!(result, Ok(Value::new_with_units(number, units)));
}

#[test]
fn euro_per_km_mul() {
    let result = make_interpreter("$50/km * 10km", None).interpret();
    assert_eq!(result, Ok(Value::new_with_unit(Number::from_i64(500).unwrap(), &Unit::dollar())));
}

#[test]
fn km_per_second_as_meter() {
    let result = make_interpreter("1 km/s as meter", None).interpret();
    let mut units = ComposedUnit::new_with_unit(Unit::meter());
    units.set_unit(Unit::second(), -1);
    assert_eq!(result, Ok(Value::new_with_units(
        Number::from_i64(1000).unwrap(),
        units)
    ));
}

#[test]
#[ignore = "bug in a library used"]
fn m_per_s_to_km_per_h() {
    let result = make_interpreter("100 m/s en km per hour", None).interpret();
    let mut units = ComposedUnit::new_with_unit(Unit::kilometer());
    units.set_unit(Unit::hour(), -1);
    assert_eq!(result, Ok(Value::new_with_units(
        Number::from_i64(360).unwrap(),
        units)
    ));
}

#[test]
fn power() {
    let result = make_interpreter("100 ^ 2", None).interpret();
    assert_eq!(result, Ok(Value::new(Number::from_i64(10000).unwrap())));
}

#[test]
fn power_with_unit() {
    let result = make_interpreter("100 ^ 2 m", None).interpret();
    assert_eq!(result, Ok(
        Value::new_with_unit(
            Number::from_i64(10000).unwrap(),
            &Unit::meter()
        )
    ))
}

#[test]
fn two_vars() {
    let result = make_interpreter("1m² ", None).interpret();
    assert_eq!(result, Ok(Value::new_with_units(
        Number::one(),
        ComposedUnit::square_meters())
    ));
}


#[test]
fn one_square_meter() {
    let result = make_interpreter("1m² ", None).interpret();
    assert_eq!(result, Ok(Value::new_with_units(
        Number::one(),
        ComposedUnit::square_meters())
    ));
}

#[test]
fn square_meter() {
    let result = make_interpreter("1 m² * 2", None).interpret();
    assert_eq!(result, Ok(Value::new_with_units(
        Number::from_i64(2).unwrap(),
        ComposedUnit::square_meters())
    ));
}