use core::f64;
use std::collections::HashMap;
use std::{i128, io};
use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Add, Sub, Neg, Mul, Div};
use std::fmt;


#[derive(Debug, Eq, PartialEq)]
enum Error {
    InvalidSyntax,
    UndefinedVariable,
    DivisonByZero,
    IncorrectFloat // Could not parse the float
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
    INTEGER(i128),
    FLOAT(f64),
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    ASSIGN,
    VAR(String),
    MONEY(Currency),
    EOF,
}

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

    /// advance `self.pos` until the next non-whitespace character
    fn skip_whitespace(&mut self) {

        while self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap().is_whitespace() {
            self.pos += 1;
        }
    }

    /// Return a (multidigit) Token::INTEGER or TOKEN::FLOAT consumed from the input.
    fn number(&mut self) -> Result<Token, Error> {
        let mut is_float = false;

        let mut ascii_number = String::from("");

        // dumb code is smart code
        while let Some (char) = self.get_char() {
                if char.is_ascii_digit() {
                    self.advance();
                    ascii_number.push(char);
                } else if char == '.' {
                    is_float = true;
                    self.advance();
                    ascii_number.push(char);
                } else {
                    break;
                }
        }

        match is_float {
            false => {
                let val: i128 = i128::from_str_radix(&ascii_number, 10).unwrap();
                Ok(Token::INTEGER(val))
            },
            true => {
                if let Ok(val) = &ascii_number.parse::<f64>() {
                    Ok(Token::FLOAT(*val))
                } else {
                    Err(Error::IncorrectFloat)
                }
            }
        }

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
                Ok(Token::MONEY(Currency::Euro))
            },
            '$' => {
                self.advance();
                Ok(Token::MONEY(Currency::Dollar))
            },
            char if char.is_alphabetic() => {
                Ok(Token::VAR(self.variable()))
            },
            _ => {Err(Error::InvalidSyntax)}
        }
    }
}


//#############################################################
//   Parser / AST
//#############################################################

/// The parser consume the tokens and create an AST tree

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
}

#[derive(Debug, Clone)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token
}

impl Parser {
    fn new(mut lexer: Lexer) -> Result<Parser, Error> {
        let token = lexer.get_next_token()?;

        Ok(Parser {
            lexer: lexer,
            current_token: token
        })
    }

    /// Consume one 'token' if we have the correct 'token type', else send an error
    fn eat(&mut self, token: Token) -> Result<(), Error> {
        if token == self.current_token {
            self.current_token = self.lexer.get_next_token()?;
            Ok(())
        } else {
            Err(Error::InvalidSyntax)
        }
    }

    /// number : INTEGER | FLOAT
    fn number(&mut self) -> Result<AST, Error> {
        let token = self.current_token.clone();

        match token {
            // INTEGER
            Token::INTEGER(i) => {
                self.eat(Token::INTEGER(i))?;
                let node = AST::new(token, vec![]);
                Ok(node)
            },
            // FLOAT
            Token::FLOAT(f) => {
                self.eat(Token::FLOAT(f))?;
                let node = AST::new(token, vec![]);
                Ok(node)
            },
            _ => {Err(Error::InvalidSyntax)}
        }
    }

    /// value : (MONEY) number | number (MONEY)
    fn value(&mut self) -> Result<AST, Error> {
        let token = self.current_token.clone();

        match token {
            // MONEY
            Token::MONEY(currency) => {
                self.eat(Token::MONEY(currency))?;
                let node: AST = AST::new(Token::MONEY(currency), vec![self.number()?]);
                Ok(node)
            },

            // INTEGER
            Token::INTEGER(_) | Token::FLOAT(_) => {
                let node = self.number()?;

                // MONEY: check if our value ends with a currency, like 12€
                match self.current_token {

                    Token::MONEY(currency) => {
                        self.eat(Token::MONEY(currency))?;
                        let node: AST = AST::new(Token::MONEY(currency), vec![node]);
                        Ok(node)
                    },

                    // Otherwise, just return the number 22 -> Int(22)
                    _ => {Ok(node)}
                }
            },
            _ => {Err(Error::InvalidSyntax)}
        }
    }

    /// factor : (PLUS | MINUS) factor | number | LPAREN expr RPAREN | VAR
    fn factor(&mut self) -> Result<AST, Error> {
        let token = self.current_token.clone();
        
        match token {
            Token::MONEY(_) | Token::INTEGER(_) | Token::FLOAT(_) => {
                self.value()
            },
            // (PLUS | MINUS) factor
            Token::PLUS | Token::MINUS=> {
                match token {
                    Token::PLUS => self.eat(Token::PLUS)?,
                    Token::MINUS => self.eat(Token::MINUS)?,
                    _ => {panic!()}
                }
                let children = vec![self.factor()?];
                let node = AST::new(token, children); 
                Ok(node)
            },
            // LPAREN expr RPAREN
            Token::LPAREN => {
                self.eat(Token::LPAREN)?;
                let node = self.expr()?;
                self.eat(Token::RPAREN)?;
                Ok(node)
            },
            Token::VAR(name) => {
                self.eat(Token::VAR(name.clone()))?;
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
    fn term(&mut self) -> Result<AST, Error> {
        let mut node = self.factor()?;

        while matches!(self.current_token, Token::VAR(_)) {
            match self.current_token.clone() {
                Token::VAR(name) => {
                    self.eat(Token::VAR(name.clone()))?;
                    let var_node = AST::new(Token::VAR(name.clone()), vec![]);
                    node = AST::new(Token::MUL, vec![node, var_node]);
                },
                _ => {}
            }                
        }

        while self.current_token == Token::MUL || self.current_token == Token::DIV {
            
            match self.current_token {
                Token::MUL => {
                    self.eat(Token::MUL)?;
                    let children: Vec<AST> = vec![node, self.factor()?];
                    node = AST::new(Token::MUL, children);
                },
                Token::DIV => {
                    self.eat(Token::DIV)?;
                    let children: Vec<AST> = vec![node, self.factor()?];
                    node = AST::new(Token::DIV, children);
                }
                _ => {panic!("Incorrect token in term()")}
            }
        }
        Ok(node)
    }

    /// expr    : term   ((PLUS | MINUS) term)*
    fn expr(&mut self) -> Result<AST, Error> {
        let mut node = self.term()?;

        while self.current_token == Token::PLUS || self.current_token == Token::MINUS {

            match self.current_token {
                Token::PLUS => {
                    self.eat(Token::PLUS)?;
                    let children: Vec<AST> = vec![node, self.term()?];
                    node = AST::new(Token::PLUS, children);
                },
                Token::MINUS => {
                    self.eat(Token::MINUS)?;
                    let children: Vec<AST> = vec![node, self.term()?];
                    node = AST::new(Token::MINUS, children);
                },
                _ => {panic!("Incorrect token in expr()")}
            }
        }

        Ok (node)
    }
    
    /// assignment  : variable ASSIGN expr
    fn assignement(&mut self) -> Result<AST, Error> {
        
        // Make a copy of the variable name
        let var_name = self.current_token.clone();    
        self.eat(var_name.clone())?;
        
        self.eat(Token::ASSIGN)?; // `=`

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
        match self.current_token {
            Token::VAR(_) => {
                let mut lex = self.lexer.clone();
                if lex.get_next_token()? == Token::ASSIGN {
                    self.assignement()
                } else {
                    self.expr()
                }
            },
            _ => {self.expr()}
        }
    }


    fn parse(&mut self) -> Result<AST, Error> {
        //self.expr()
        self.statement()
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

//#############################################################
//   Interpreter
//#############################################################

pub struct Interpreter {
    parser: Parser,
    variables: Rc<RefCell<HashMap<String, ResType>>>
}

impl Interpreter {
    fn new(parser: Parser, variables: Rc<RefCell<HashMap<String, ResType>>>) -> Interpreter {
        Interpreter {
            parser: parser,
            variables: variables
        }
    }

    fn visit_num(&self, node: &AST) -> ResType {
        match node.token {
            Token::INTEGER(i) => ResType::Int(i),
            Token::FLOAT(f) => ResType::Float(f),
            _ => panic!("Error: end node is not an integer")
        }
    }

    fn visit_variable(&self, node: &AST) -> Result<ResType, Error> {
        match &node.token {
            Token::VAR(var_name) => {
                let var_list = self.variables.borrow();

                match var_list.get(var_name) {
                    Some(val) => return Ok(*val),
                    None => {}
                };

                // if variable ends with an 's', we check if the singular is a variable
                if let Some(last_char) = var_name.chars().nth(var_name.len()-1) {
                    
                    if last_char == 's' {
                        let singular_varname: String = var_name.chars().take(var_name.len()-1).collect();

                        match var_list.get(&singular_varname) {
                            Some(val) => return Ok(*val),
                            _ => {}
                        }
                    }
                }
                
                Err(Error::UndefinedVariable)
            },
            _ => panic!("Token is not a variable")
        }
    }

    fn visit_binop(&mut self, node: &AST) -> Result<ResType, Error> {
        let left_val = self.visit(&node.children[0])?;
        let right_val = self.visit(&node.children[1])?;

        match node.token {
            Token::PLUS => {
                Ok(left_val + right_val)
            },
            Token::MINUS => {
                Ok(left_val - right_val)
            },
            Token::MUL => {
                Ok(left_val * right_val)
            },
            Token::DIV => {
                // Let's catch division by zero before the happend
                // because there is no checked_div function for f64.
                
                match right_val {
                    ResType::Int(0) => return Err(Error::DivisonByZero),
                    ResType::Float(val) => {
                        if val == 0.0 {return Err(Error::DivisonByZero)}},
                    _ => {}
                };

                // Division has been implemented as a trait for ResType
                let res = left_val / right_val;
                Ok(res)
            },
            _ => panic!("Unkown BinOp Token in the AST")
        }
    }

    fn visit_unaryop(&mut self, node: &AST) -> Result<ResType, Error> {
        let val = self.visit(&node.children[0])?;

        match &node.token {
            Token::PLUS  => {  Ok(val) },
            Token::MINUS => { Ok(-val) },
            Token::MONEY(currency) => {
                let number = self.visit(&node.children[0])?;

                match number {
                    ResType::Int(val) => {
                        Ok(ResType::Money(val as f64, *currency))
                    },
                    ResType::Float(val) => {
                        Ok(ResType::Money(val, *currency))
                    },
                    _ => panic!("Unknown number type in Money creation")
                }

            }
            _ => {panic!("Invalid token type for an unary node")}
        }
    }

    fn visit_assign(&mut self, node: &AST) -> Result<ResType, Error> {
        let right_val = self.visit(&node.children[1])?;

        match &node.children[0].token {
            Token::VAR(var_name) => {
                let mut var = self.variables.borrow_mut();
                var.insert(var_name.clone(), right_val);
                // self.variables.set(insert(var_name.clone(), right_val));
            },
            _ => panic!("Assignement without a variable")
        }
        Ok(right_val)
    }

    fn visit(&mut self, node: &AST) -> Result<ResType, Error> {
        match node.token {
            Token::INTEGER(_) | Token::FLOAT(_) => {
                Ok(self.visit_num(node))
            },
            Token::VAR(_) => Ok(self.visit_variable(node)?),
            Token::ASSIGN => Ok(self.visit_assign(node)?),
            Token::PLUS | Token::MINUS | Token::MUL | Token::DIV | Token::MONEY(_)=> {
                match node.children.len() {
                    1 => Ok(self.visit_unaryop(node)?),
                    2 => Ok(self.visit_binop(node)?),
                    _ => panic!("Too many children for an AST node")
                }             
            },
            _ => panic!("Unkown Token in the AST")
        }
    }

    fn interpret(&mut self) -> Result<ResType, Error> {
        let tree = self.parser.parse()?;
        let result = self.visit(&tree)?;
        // println!("res: {:?}", result);
        Ok(result)
    }
}

pub fn solve(input: String, variables: Rc<RefCell<HashMap<String, ResType>>>) -> Result<String, String>{
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

#[allow(unused)]
fn main() {
    let variables: Rc<RefCell<HashMap<String, ResType>>> = Rc::new(RefCell::new(HashMap::new()));

    loop {
        // show the interactive prompt
        print!("calc> ");
        let mut input = String::new();
        io::stdout().flush().unwrap();
    
        // read input from user
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.eq("") || input.eq("exit\n") {
            break;
        }

        match solve(input, variables.clone()) {
            Ok(result) => println!("{}", result),
            Err(_) => println!("Invalid syntax")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_interpreter(text: &str, variables: Option<Rc<RefCell<HashMap<String, ResType>>>>) -> Interpreter {
        
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
        assert_eq!(result, Ok(ResType::Int(3)));
    }

    #[test]
    fn test_expression2() {
        let mut interpreter = make_interpreter("2 + 7 * 4", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Int(30)));
    }

    #[test]
    fn test_expression3() {
        let mut interpreter = make_interpreter("7 - 8 / 4", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Int(5)));
    }

    #[test]
    fn test_expression4() {
        let mut interpreter = make_interpreter("14 + 2 * 3 - 6 / 2", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Int(17)));
    }

    #[test]
    fn test_expression5() {
        let mut interpreter = make_interpreter("7 + 3 * (10 / (12 / (3 + 1) - 1))", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Int(22)));
    }

    #[test]
    fn test_expression6() {
        let mut interpreter = make_interpreter(
            "7 + 3 * (10 / (12 / (3 + 1) - 1)) / (2 + 3) - 5 - 3 + (8)", None
        );
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Int(10)));
    }

    #[test]
    fn test_expression7() {
        let mut interpreter = make_interpreter("7 + (((3 + 2)))", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Int(12)));
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
        assert_eq!(result, Ok(ResType::Int(-42)));
    }

    #[test]
    fn test_expression_unary2() {
        let mut interpreter = make_interpreter("-6*-7 - 3", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Int(39)));
    }

    #[test]
    fn test_expression_variable1() {
        let vars : Rc<RefCell<HashMap<String, ResType>>> = Rc::new(RefCell::new(HashMap::new()));

        let mut interpreter = make_interpreter("a=5", Some(vars.clone()));
        _ = interpreter.interpret();
        let mut interpreter = make_interpreter("a", Some(vars));
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Int(5)));
    }

    #[test]
    fn test_expression_variable2() {
        let vars : Rc<RefCell<HashMap<String, ResType>>> = Rc::new(RefCell::new(HashMap::new()));

        let mut interpreter = make_interpreter("bob=(525+83)/4", Some(vars.clone()));
        _ = interpreter.interpret();
        let mut interpreter = make_interpreter("bob + 48", Some(vars));
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Int(200)));
    }

    #[test]
    fn test_expression_variable3() {
        let vars : Rc<RefCell<HashMap<String, ResType>>> = Rc::new(RefCell::new(HashMap::new()));

        let mut interpreter = make_interpreter("a=2", Some(vars.clone()));
        _ = interpreter.interpret();
        let mut interpreter = make_interpreter("b=1", Some(vars.clone()));
        _ = interpreter.interpret();
        let mut interpreter = make_interpreter("b=3", Some(vars.clone()));
        _ = interpreter.interpret();
        let mut interpreter = make_interpreter("a+b", Some(vars));
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Int(5)));
    }

    #[test]
    fn test_float() {
        let mut interpreter = make_interpreter("4.0", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Float(4.0)));
    }

    #[test]
    fn test_negative_float() {
        let mut interpreter = make_interpreter("-16.0 + 4", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Float(-12.0)));
    }

    #[test]
    fn test_division1() {
        let mut interpreter = make_interpreter("20/4", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Int(5)));
    }

    #[test]
    fn test_division2() {
        let mut interpreter = make_interpreter("-5/2", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Float(-2.5)));
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
        assert_eq!(result, Ok(ResType::Money(12.0, Currency::Euro)));
    }

    #[test]
    fn test_money2() {
        let mut interpreter = make_interpreter("$47", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Money(47.0, Currency::Dollar)));
    }

    #[test]
    fn test_money_add() {
        let mut interpreter = make_interpreter("22€ + 8", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Money(30.0, Currency::Euro)));
    }

    #[test]
    fn test_money_sub() {
        let mut interpreter = make_interpreter("500€ - 1000€", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Money(-500.0, Currency::Euro)));
    }

    #[test]
    fn test_money_mul() {
        let mut interpreter = make_interpreter("$33 * -4", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Money(-132.0, Currency::Dollar)));
    }

    #[test]
    fn test_money_div() {
        let mut interpreter = make_interpreter("25€ / 4", None);
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Money(6.25, Currency::Euro)));
    }

    #[test]
    fn test_handling_spaces() {
        let mut interpreter = make_interpreter("4€ b", None);
        let _ = interpreter.interpret();
    }

    #[test]
    fn implicit_multiplication() {
        let vars : Rc<RefCell<HashMap<String, ResType>>> = Rc::new(RefCell::new(HashMap::new()));

        let mut interpreter = make_interpreter("a=2", Some(vars.clone()));
        _ = interpreter.interpret();
        let mut interpreter = make_interpreter("4a", Some(vars));
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Int(8)));
    }

    #[test]
    #[ignore]
    fn implicit_multiplication2() {
        let vars : Rc<RefCell<HashMap<String, ResType>>> = Rc::new(RefCell::new(HashMap::new()));

        let mut interpreter = make_interpreter("a=2", Some(vars.clone()));
        _ = interpreter.interpret();
        let mut interpreter = make_interpreter("b=-3", Some(vars.clone()));
        _ = interpreter.interpret();
        let mut interpreter = make_interpreter("4ab", Some(vars));
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Int(-24)));
    }

    #[test]
    #[ignore]
    fn implicit_multiplication3() {
        let vars : Rc<RefCell<HashMap<String, ResType>>> = Rc::new(RefCell::new(HashMap::new()));

        let mut interpreter = make_interpreter("a=2", Some(vars.clone()));
        _ = interpreter.interpret();
        let mut interpreter = make_interpreter("b=3", Some(vars.clone()));
        _ = interpreter.interpret();
        let mut interpreter = make_interpreter("4ab + 2 ab", Some(vars));
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Int(-24)));
    }

    #[test]
    fn scenario_cinema() {
        let vars : Rc<RefCell<HashMap<String, ResType>>> = Rc::new(RefCell::new(HashMap::new()));

        let mut interpreter = make_interpreter("enfant=4€", Some(vars.clone()));
        _ = interpreter.interpret();
        let mut interpreter = make_interpreter("adulte=12€", Some(vars.clone()));
        _ = interpreter.interpret();
        let mut interpreter = make_interpreter("2adultes+3 enfants", Some(vars));
        let result = interpreter.interpret();
        assert_eq!(result, Ok(ResType::Money(36.0, Currency::Euro)));
    }
}