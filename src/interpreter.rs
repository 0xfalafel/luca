use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::str::FromStr;

use num_rational::BigRational;
use num_bigint::BigInt;
use num_traits::FromPrimitive;

// use crate::units::money::{Money, Currency};

use crate::units::unit::Unit;
use crate::value::Value;

#[derive(Debug, Eq, PartialEq)]
enum Error {
    InvalidSyntax,
    UndefinedVariable,
    DivisonByZero,
    CalculationError,
    IntParsingFailed,
    FloatParsingFailed,
    FailedConversion,
}

/*
Our grammar is the following:

statement   : expr | assignement
assignment  : VAR ASSIGN expr
expr        : term   ((PLUS | MINUS) term)*
term        : factor ((MUL  | DIV) factor)*
factor      : INTEGER | LPAREN expr RPAREN | VAR

*/



/**************************************************************
*   Tokens / Lexer
**************************************************************/

// Token types
//
// EOF (end-of-file) is  used to indicate that there is no more input left

/// Token are used to represent the differents elements given as an input.
/// The input is separated in a bunch of tokens.
#[derive(Debug, Clone, PartialEq)]
enum Token {
    INTEGER(BigRational),
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    ASSIGN,
    VAR(String),
    MONEY(Currency),
    PERCENTAGE,
    OF, // Keyword of for percentage
    UNIT(UnitSymbol),
    AS, // Conversion
    EOF,
}

// Currency Type
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Currency {
    Euros,
    Dollars
}

// Currency Type
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum UnitSymbol {
    Meter,
    Kilometer,
    Decimeter,
    Centimeter,
    Millimeter,
}


#[derive(Debug, Clone)]
struct Lexer {
    text: String,
    pos: usize
}

/// The Lexer is in charge of spliting the input in a bunch of tokens.
impl Lexer {
    pub fn new(text: String) -> Lexer {

        Lexer {
            text: text,
            pos: 0
        }
    }

    /// Advance the `pos` pointer and set the `current_char` variable.
    fn advance(&mut self) {
        self.pos += 1
    }

    /// Return the char at the `pos` position
    fn get_char(&self) -> Option<char> {
        self.text.chars().nth(self.pos)
    }

    /// Look at the `n` char after the current `pos`
    #[allow(unused)]
    fn peek(&self, n: usize) -> Option<char> {
        self.text.chars().nth(self.pos+n)
    }

    /// advance `self.pos` until the next non-whitespace character
    fn skip_whitespace(&mut self) {

        while self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap().is_whitespace() {
            self.pos += 1;
        }
    }

    /// Return a (multidigit) Token::INTEGER or TOKEN::FLOAT consumed from the input.
    fn number(&mut self) -> Result<Token, Error> {
        let mut ascii_number = String::from("");
        let mut is_float = false;

        // dumb code is smart code
        while let Some (char) = self.get_char() {
                if char.is_ascii_digit() {
                    self.advance();
                    ascii_number.push(char);
                } else if char == '.' || char == ',' {
                    is_float = true;
                    self.advance();
                    ascii_number.push('.');
                } else {
                    break;
                }
        }

        if !is_float {
            match BigRational::from_str(&ascii_number) {
                Ok(val) => Ok(Token::INTEGER(val)),
                Err(_) => Err(Error::IntParsingFailed)
            }    
        } else { // we parse a float
            let val = match f64::from_str(&ascii_number) {
                Ok(val) => val,
                Err(_) => return Err(Error::FloatParsingFailed)
            };

            match BigRational::from_f64(val) {
                Some(num) => Ok(Token::INTEGER(num)),
                None => Err(Error::FloatParsingFailed)
            }
        }
    }

    fn keyword_or_variable(&mut self) -> Result<Token, Error> {
        let var = self.variable();

        let token = match var.as_str() {
            "of" | "de" => Token::OF, // Percentage
            "en" | "as" => Token::AS, // Conversion
            "m" | "meter" | "metre" => Token::UNIT(UnitSymbol::Meter),
            "km" | "kilometer" | "kilometre" => Token::UNIT(UnitSymbol::Kilometer),
            "dm" | "decimeter" | "decimetre" => Token::UNIT(UnitSymbol::Decimeter),
            "cm" | "centimeter" | "centimetre" => Token::UNIT(UnitSymbol::Centimeter),
            "mm" | "millimeter" | "millimetre" => Token::UNIT(UnitSymbol::Millimeter),
            _ => Token::VAR(var)
        };

        Ok(token)
    }

    /// Retun a string
    fn variable(&mut self) -> String {
        let str_start = self.pos;
        let input_text: String = self.text.chars().skip(self.pos).collect();

        let end_of_variable = input_text
            .find(|c: char| c == '=' || c == '€' || c == '$'
                || c == '+' || c == '-' || c == '*' || c == '/'
                || c.is_whitespace())
            .unwrap_or(input_text.len());

        
        self.pos = str_start + end_of_variable;
        
        let new_var: String = input_text.chars().take(end_of_variable).collect();
        // println!("new_var: {:?}", new_var);
        new_var
    }

    /// Lexical analyser (also known as scanner or tokenizer).
    ///    
    /// This method is responsible for breaking a sentence
    /// appart into tokens. One token at the time.
    pub fn get_next_token(&mut self) -> Result<Token, Error> {

        // get the next non-whitespace char, or EOF
        let char = loop {
            let my_char = self.get_char();
            match my_char {
                None => return Ok(Token::EOF),
                Some(char) if char.is_whitespace() => {
                    self.skip_whitespace()
                },
                Some(char) => break char
            }
        };

        match char {
            char if char.is_ascii_digit() => {
                Ok(self.number()?)
            },
            '+' => {
                self.advance();
                Ok(Token::PLUS)
            },
            '-' => {
                self.advance();
                Ok(Token::MINUS)
            },    
            '*' => {
                self.advance();
                Ok(Token::MUL,)
            },    
            '/' => {
                self.advance();
                Ok(Token::DIV,)
            },    
            '(' => {
                self.advance();
                Ok(Token::LPAREN)
            },    
            ')' => {
                self.advance();
                Ok(Token::RPAREN)
            },
            '=' => {
                self.advance();
                Ok(Token::ASSIGN)
            },
            '€' => {
                self.advance();
                Ok(Token::MONEY(Currency::Euros))
            },
            '$' => {
                self.advance();
                Ok(Token::MONEY(Currency::Dollars))
            },
            '%' => {
                self.advance();
                Ok(Token::PERCENTAGE)
            },
            char if char.is_alphabetic() => {
                self.keyword_or_variable()
            },
            _ => Err(Error::InvalidSyntax)
        }
    }

    /// Take a look at what the next token will be, without consuming it.
    pub fn peek_next_token(&self) -> Option<Token> {
        let mut lex = self.clone();

        match lex.get_next_token() {
            Ok(token) => Some(token),
            Err(_) => None,
        }
    }
}


//#############################################################
//   Parser / AST
//#############################################################

/// The parser consume the tokens and create an AST tree

#[derive(Debug)]
struct AST {
    token: Token,
    children: Vec<AST>
}

impl AST {
    fn new(token: Token, children: Vec<AST>) -> AST {
        AST {
            token: token,
            children: children
        }
    }

    fn has_no_children(&self) -> bool {
        self.children[0].token == Token::EOF
    }
}

#[derive(Debug, Clone)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Result<Parser, Error> {
        let token = lexer.get_next_token()?;

        Ok(Parser {
            lexer: lexer,
            current_token: token,
        })
    }

    /// Consume one 'token' if we have the correct 'token type', else send an error
    fn eat(&mut self, token: &Token) -> Result<(), Error> {
        if *token == self.current_token {
            self.current_token = self.lexer.get_next_token()?;
            Ok(())
        } else {
            Err(Error::InvalidSyntax)
        }
    }

    /// number : INTEGER | FLOAT
    fn number(&mut self) -> Result<AST, Error> {
        let token = self.current_token.clone();

        self.eat(&token)?;
        let node = AST::new(token, vec![]);
        Ok(node)
    }

    /// value : (MONEY) number | number (MONEY | PERCENTAGE | UNIT)
    fn value(&mut self) -> Result<AST, Error> {
        let token = self.current_token.clone();

        match token {
            // MONEY
            Token::MONEY(currency) => {
                self.eat(&Token::MONEY(currency))?;
                let node: AST = AST::new(Token::MONEY(currency), vec![self.number()?]);
                Ok(node)
            },

            // INTEGER
            Token::INTEGER(_) => {
                let node = self.number()?;

                match self.current_token {
                    // MONEY: check if our value ends with a currency, like 12€
                    Token::MONEY(currency) => {
                        self.eat(&Token::MONEY(currency))?;
                        let node: AST = AST::new(Token::MONEY(currency), vec![node]);
                        Ok(node)
                    },
                    // PERCENTAGE: check if our value ends with a percentage, like 25%
                    Token::PERCENTAGE => {
                        self.eat(&Token::PERCENTAGE)?;
                        let node: AST = AST::new(Token::PERCENTAGE, vec![node]);
                        Ok(node)
                    },
                    // UNIT: check if our value ends with a unit, like 15m
                    Token::UNIT(unit) => {
                        self.eat(&Token::UNIT(unit))?;
                        let node: AST = AST::new(Token::UNIT(unit), vec![node]);
                        Ok(node)
                    },

                    // Otherwise, just return the number 22 -> Int(22)
                    _ => Ok(node)
                }
            },
            _ => {Err(Error::InvalidSyntax)}
        }
    }

    /// factor : (PLUS | MINUS) factor | number | LPAREN expr RPAREN | VAR
    fn factor(&mut self) -> Result<AST, Error> {
        let token = self.current_token.clone();
        
        match token {
            Token::MONEY(_) | Token::INTEGER(_) => {
                self.value()
            },
            // (PLUS | MINUS) factor
            Token::PLUS | Token::MINUS=> {
                match token {
                    Token::PLUS => self.eat(&Token::PLUS)?,
                    Token::MINUS => self.eat(&Token::MINUS)?,
                    _ => panic!()
                }
                let children = vec![self.factor()?];
                let node = AST::new(token, children); 
                Ok(node)
            },
            // LPAREN expr RPAREN
            Token::LPAREN => {
                self.eat(&Token::LPAREN)?;
                let node = self.expr()?;
                self.eat(&Token::RPAREN)?;
                Ok(node)
            },
            Token::VAR(name) => {
                self.eat(&Token::VAR(name.clone()))?;
                let node = AST::new(Token::VAR(name), vec![]);
                Ok(node)
            },
            _ => {
                Err(Error::InvalidSyntax)
            }
        }
    }

    /// term : factor (VAR)* ((MUL | DIV) factor)*
    ///      | factor (VAR)*            <-- implicit multiplication of variables. Like 4ab + 12 TODO
    ///      | percentage OF factor
    fn term(&mut self) -> Result<AST, Error> {
        let mut node = self.factor()?;

        while matches!(self.current_token, Token::VAR(_)) {
            match self.current_token.clone() {
                Token::VAR(name) => {
                    self.eat(&Token::VAR(name.clone()))?;
                    let var_node = AST::new(Token::VAR(name.clone()), vec![]);
                    node = AST::new(Token::MUL, vec![node, var_node]);
                },
                _ => {}
            }                
        }

        while self.current_token == Token::MUL || self.current_token == Token::DIV || self.current_token == Token::OF {
            
            match self.current_token {
                Token::MUL => {
                    self.eat(&Token::MUL)?;
                    let children: Vec<AST> = vec![node, self.factor()?];
                    node = AST::new(Token::MUL, children);
                },
                Token::DIV => {
                    self.eat(&Token::DIV)?;
                    let children: Vec<AST> = vec![node, self.factor()?];
                    node = AST::new(Token::DIV, children);
                },
                Token::OF => {
                    self.eat(&Token::OF)?;

                    // We only consider Token::OF if the previous token is a precentage.
                    // I.E: 20% of 120 = 20% * 120
                    //      20 of 120 doesn't mean anything, and we ignore it
                    if node.token == Token::PERCENTAGE {
                        let children: Vec<AST> = vec![node, self.factor()?];
                        node = AST::new(Token::MUL, children);
                    }
                },
                _ => panic!("Incorrect token in term()")
            }
        }
        Ok(node)
    }

    /// expr    : term   ((PLUS | MINUS) term)* (AS Unit)
    fn expr(&mut self) -> Result<AST, Error> {
        let mut node = self.term()?;

        while self.current_token == Token::PLUS || self.current_token == Token::MINUS {

            match self.current_token {
                Token::PLUS => {
                    self.eat(&Token::PLUS)?;
                    let children: Vec<AST> = vec![node, self.term()?];
                    node = AST::new(Token::PLUS, children);
                },
                Token::MINUS => {
                    self.eat(&Token::MINUS)?;
                    let children: Vec<AST> = vec![node, self.term()?];
                    node = AST::new(Token::MINUS, children);
                },
                _ => panic!("Incorrect token in expr()")
            }
        }

        if self.current_token == Token::AS {
            self.eat(&Token::AS)?;

            if let Token::UNIT(unit) = self.current_token {
                self.eat(&Token::UNIT(unit))?;
                let children: Vec<AST> = vec![node];
                node = AST::new(Token::UNIT(unit), children);
            } else {
                return Err(Error::InvalidSyntax)
            }
        }

        Ok (node)
    }
    
    /// assignment  : variable ASSIGN expr
    fn assignement(&mut self) -> Result<AST, Error> {
        
        // Make a copy of the variable name
        let var_name = self.current_token.clone();    
        self.eat(&var_name.clone())?;
        
        self.eat(&Token::ASSIGN)?; // `=`

        let node = AST::new(
            Token::ASSIGN, vec![
                AST::new(var_name, vec![]),
                self.expr()?
            ]
        );

        Ok(node)
    }
    
    /// statement   : expr | assignement
    fn statement(&mut self) -> Result<AST, Error> {
        if matches!(self.current_token, Token::VAR(_)) && self.lexer.peek_next_token() == Some(Token::ASSIGN) {
            self.assignement()
        } else {
            self.expr()
        }
    }


    fn parse(&mut self) -> Result<AST, Error> {
        self.statement()
    }
}

//#############################################################
//   Interpreter
//#############################################################

pub struct Interpreter {
    parser: Parser,
    variables: Rc<RefCell<HashMap<String, Value>>>
}

impl Interpreter {
    fn new(parser: Parser, variables: Rc<RefCell<HashMap<String, Value>>>) -> Interpreter {
        Interpreter {
            parser: parser,
            variables: variables
        }
    }

    fn visit_num(&self, node: &AST) -> Value {
        match &node.token {
            Token::INTEGER(i) => Value::new(i.clone()),
            _ => panic!("Error: end node is not an integer")
        }
    }

    fn visit_variable(&self, node: &AST) -> Result<Value, Error> {
        match &node.token {
            Token::VAR(var_name) => {
                let var_list = self.variables.borrow();

                match var_list.get(var_name) {
                    Some(val) => return Ok(val.clone()),
                    None => {}
                };

                // if variable ends with an 's', we check if the singular is a variable
                if let Some(last_char) = var_name.chars().nth(var_name.len()-1) {
                    
                    if last_char == 's' {
                        let singular_varname: String = var_name.chars().take(var_name.len()-1).collect();

                        match var_list.get(&singular_varname) {
                            Some(val) => return Ok(val.clone()),
                            _ => {}
                        }
                    }
                }
                
                Err(Error::UndefinedVariable)
            },
            _ => panic!("Token is not a variable")
        }
    }

    fn visit_binop(&mut self, node: &AST) -> Result<Value, Error> {
        let left_val = self.visit(&node.children[0])?;
        let right_val = self.visit(&node.children[1])?;

        match node.token {
            Token::PLUS => {
                (left_val + right_val).map_err(|_| Error::CalculationError)
            },
            Token::MINUS => {
                (left_val - right_val).map_err(|_| Error::CalculationError)
            },
            Token::MUL => {
                (left_val * right_val).map_err(|_| Error::CalculationError)
            },
            Token::DIV => {
                // Let's catch division by zero before the happend
                // because there is no checked_div function for f64.

                if right_val.number == BigRational::from(BigInt::from(0)) {
                    return Err(Error::DivisonByZero)
                }

                // Division has been implemented as a trait for Value
                let res = match left_val / right_val {
                    Ok(val) => val,
                    Err(_) => return Err(Error::CalculationError)
                };  // Todo: use more explicit errors
                Ok(res)
            },
            _ => panic!("Unkown BinOp Token in the AST")
        }
    }

    fn visit_unaryop(&mut self, node: &AST) -> Result<Value, Error> {

        // Children is EOF. We can't apply unary operator on nothing
        if node.has_no_children() {
            return Err(Error::InvalidSyntax)
        }

        let val = self.visit(&node.children[0])?;

        match &node.token {
            Token::PLUS  => Ok(val),
            Token::MINUS => Ok(-val),
            Token::PERCENTAGE => Ok(val.set_unit(Unit::percent())),
            Token::MONEY(currency) => {
                let number = self.visit(&node.children[0])?;

                // Maybe this should be matched somewhere else ?
                match currency {
                    Currency::Euros   => Ok(number.set_unit(Unit::euro())),
                    Currency::Dollars => Ok(number.set_unit(Unit::dollar())),
                }
            },
            Token::UNIT(unit_symbol) => {
                let number = self.visit(&node.children[0])?;

                // Maybe this should be matched somewhere else ?
                let unit = match unit_symbol {
                    UnitSymbol::Meter => Unit::meter(),
                    UnitSymbol::Kilometer => Unit::meter().with_prefix("kilo"),
                    UnitSymbol::Decimeter => Unit::meter().with_prefix("deci"),
                    UnitSymbol::Centimeter => Unit::centimeter(),
                    UnitSymbol::Millimeter => Unit::meter().with_prefix("milli"),
                };

                match number.convert_to_unit(&unit) {
                    Ok(val) => Ok(val),
                    Err(_e) => Err(Error::FailedConversion)
                }
            },
            _ => panic!("Invalid token type for an unary node")
        }
    }

    fn visit_assign(&mut self, node: &AST) -> Result<Value, Error> {
        let right_val = self.visit(&node.children[1])?;

        match &node.children[0].token {
            Token::VAR(var_name) => {
                let mut var = self.variables.borrow_mut();
                var.insert(var_name.clone(), right_val.clone());
                // self.variables.set(insert(var_name.clone(), right_val));
            },
            _ => panic!("Assignement without a variable")
        }
        
        Ok(right_val)
    }

    fn visit(&mut self, node: &AST) -> Result<Value, Error> {
        match node.token {
            Token::INTEGER(_) => Ok(self.visit_num(node)),
            Token::VAR(_) => Ok(self.visit_variable(node)?),
            Token::ASSIGN => Ok(self.visit_assign(node)?),
            Token::PLUS | Token::MINUS | Token::MUL | Token::DIV | Token::MONEY(_) | Token::PERCENTAGE | Token::UNIT(_) => {
                match node.children.len() {
                    1 => Ok(self.visit_unaryop(node)?),
                    2 => Ok(self.visit_binop(node)?),
                    _ => panic!("Too many children for an AST node")
                }             
            },
            _ => panic!("Unkown Token in the AST")
        }
    }

    fn interpret(&mut self) -> Result<Value, Error> {
        let tree = self.parser.parse()?;
        let result = self.visit(&tree)?;
        // println!("res: {:?}", result);
        Ok(result)
    }
}

pub fn solve(input: String, variables: Rc<RefCell<HashMap<String, Value>>>) -> Result<String, String>{
    let text = String::from(input.trim());
    let lexer = Lexer::new(text);

    match Parser::new(lexer) {
        Ok(parser) => {
            let mut interpreter = Interpreter::new(parser, variables);
            match interpreter.interpret() {
                Ok(result) => {
                    Ok(format!("{}", result))
                },
                Err(_) => Err("Invalid syntax".to_string())
            }
        },
        Err(_) => Err("Invalid syntax".to_string())
    }
}



#[cfg(test)]
mod tests {
    use crate::units::unit::Unit;

    use super::*;

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

}