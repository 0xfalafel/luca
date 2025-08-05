use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub type Variables = Rc<RefCell<HashMap<String, Value>>>;

fn get_variable(variables: &Variables, key: &str) -> Option<Value> {
    let var_list = variables.borrow();

    if let Some(val) = var_list.get(key) {
        Some(val.clone())
    } else if let Some(singular) = key.strip_suffix('s') {
        var_list.get(singular).cloned()
    } else {
        None
    }
}

use crate::units::number::Number;
use crate::units::unit::Unit;
use crate::value::Value;
use crate::units::composed_unit::ComposedUnit;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidSyntax,
    UndefinedVariable,
    DivisonByZero,
    CalculationError,
    NumberParsingFailed,
    FailedConversion,
}

/*
Our grammar is the following:

statement   : expr | assignement
assignment  : VAR ASSIGN expr
expr        : term   ((PLUS | MINUS) term)*
term        : factor ((MUL  | DIV) factor)*
factor      : NUMBER | LPAREN expr RPAREN | VAR

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
pub enum Token {
    NUMBER(Number),
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
    LARGE(i64),
    TITLE,
    NONE, // used for words that have no meaning
    COMMENT, // Similar to EOF
    LABEL, // Bold, and not interpreted
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
    SquareMeters,
    SquareKilometers,
    SquareDecimeters,
    SquareCentimeters,
    SquareMillimeters,
    Second,
    Minute,
    Hour,
    Millisecond,
}

impl UnitSymbol {
    fn to_unit(&self) -> ComposedUnit {
        match self {
            UnitSymbol::Meter => ComposedUnit::meter(),
            UnitSymbol::Kilometer => ComposedUnit::kilometer(),
            UnitSymbol::Decimeter => ComposedUnit::decimeter(),
            UnitSymbol::Centimeter => ComposedUnit::centimeter(),
            UnitSymbol::Millimeter => ComposedUnit::millimeter(),
            UnitSymbol::Second => ComposedUnit::second(),
            UnitSymbol::Minute => ComposedUnit::minute(),
            UnitSymbol::Hour => ComposedUnit::hour(),
            UnitSymbol::Millisecond => ComposedUnit::millisecond(),
            UnitSymbol::SquareMeters => ComposedUnit::square_meters(),
            UnitSymbol::SquareKilometers => ComposedUnit::square_kilometers(),
            UnitSymbol::SquareDecimeters => ComposedUnit::square_decimeters(),
            UnitSymbol::SquareCentimeters => ComposedUnit::square_centimeters(),
            UnitSymbol::SquareMillimeters => ComposedUnit::square_millimeters(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Lexer {
    text: String,
    pos: usize,
    variables: Variables,
}

/// The Lexer is in charge of spliting the input in a bunch of tokens.
impl Lexer {
    pub fn new(text: String, variables: Variables) -> Lexer {
        Lexer {
            text: text,
            pos: 0,
            variables: variables,
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

        while self.pos < self.text.len() && self.text.chars().nth(self.pos).unwrap_or_default().is_whitespace() {
            self.pos += 1;
        }
    }

    /// Return a (multidigit) Token::NUMBER consumed from the input.
    fn number(&mut self) -> Result<Token, Error> {
        let mut ascii_number = String::from("");

        // dumb code is smart code
        while let Some (char) = self.get_char() {
                if char.is_ascii_digit() {
                    self.advance();
                    ascii_number.push(char);
                } else if char == '.' || char == ',' {
                    self.advance();
                    ascii_number.push('.');
                } else if char.is_whitespace() || char == '_' {
                    self.advance();
                } else {
                    break;
                }
        }

        match Number::from_str(&ascii_number) {
            Ok(val) => Ok(Token::NUMBER(val)),
            Err(_) => Err(Error::NumberParsingFailed)
        }
    }

    fn is_keyword(&self, var: &str) -> Option<Token> {
        match var {
            "per" | "par" => Some(Token::DIV), // $ per km 
            "of" | "de" => Some(Token::OF), // Percentage
            
            "en" | "as" if matches!(
                self.peek_next_token(),
                Some(Token::UNIT(_)) | Some(Token::MONEY(_))
            ) => Some(Token::AS), // Conversion

            "m" | "meter" | "metre" => Some(Token::UNIT(UnitSymbol::Meter)),
            "km" | "kilometer" | "kilometre" => Some(Token::UNIT(UnitSymbol::Kilometer)),
            "dm" | "decimeter" | "decimetre" => Some(Token::UNIT(UnitSymbol::Decimeter)),
            "cm" | "centimeter" | "centimetre" => Some(Token::UNIT(UnitSymbol::Centimeter)),
            "mm" | "millimeter" | "millimetre" => Some(Token::UNIT(UnitSymbol::Millimeter)),

            "m²"  | "m2"  => Some(Token::UNIT(UnitSymbol::SquareMeters)),
            "km²" | "km2" => Some(Token::UNIT(UnitSymbol::SquareKilometers)),
            "dm²" | "dm2" => Some(Token::UNIT(UnitSymbol::SquareDecimeters)),
            "cm²" | "cm2" => Some(Token::UNIT(UnitSymbol::SquareCentimeters)),
            "mm²" | "mm2" => Some(Token::UNIT(UnitSymbol::SquareMillimeters)),

            "s" | "second" | "seconde" => Some(Token::UNIT(UnitSymbol::Second)),
            "min" | "minute" => Some(Token::UNIT(UnitSymbol::Minute)),
            "h" | "hour" | "heure" => Some(Token::UNIT(UnitSymbol::Hour)),
            "ms" | "millisecond" | "milliseconde" => Some(Token::UNIT(UnitSymbol::Millisecond)),
            
            "k" => Some(Token::LARGE(1000)),
            _ => None,
        }
    }

    fn keyword_or_variable(&mut self) -> Result<Token, Error> {
        let var = self.variable();

        if let Some(token) = self.is_keyword(var.as_str()) {
            Ok(token)
        } else if self.peek_next_token() == Some(Token::ASSIGN) {
            Ok(Token::VAR(var))
        } else if get_variable(&self.variables, &var).is_some() {
            Ok(Token::VAR(var))
        } else {
            Ok(Token::NONE)
        }
    }

    fn variable(&mut self) -> String {
        let str_start = self.pos;
        let input_chars: Vec<char> = self.text.chars().skip(self.pos).collect();
    
        let end_of_variable = input_chars.iter().position(|&c| {
            c == '=' || c == '€' || c == '$' || c == '+' || c == '-' || c == '*' || c == '/'
            || c == ':' || c.is_whitespace() || c == '(' || c == ')'
        });
    
        let end = end_of_variable.unwrap_or(input_chars.len());
        self.pos = str_start + end;
    
        // Collect the characters up to the end index
        let new_var: String = input_chars.into_iter().take(end).collect();
        new_var
    }
    
    /// Lexical analyser (also known as scanner or tokenizer).
    ///    
    /// This method is responsible for breaking a sentence
    /// appart into tokens. One token at the time.
    pub fn get_next_token(&mut self) -> Result<Token, Error> {
        let mut token = self.next_token()?;

        while token == Token::NONE {
            token = self.next_token()?;
        }
        Ok(token)
    }

    fn next_token(&mut self) -> Result<Token, Error> {
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
            '+' | '﹢' | '＋' => {
                self.advance();
                Ok(Token::PLUS)
            },
            '-' | '−' => {
                self.advance();
                Ok(Token::MINUS)
            },
            '*' | '×' => {
                self.advance();
                Ok(Token::MUL,)
            },
            '/' if self.peek(1) == Some('/') => {
                self.advance();
                self.advance();
                Ok(Token::COMMENT)
            },
            '/' | '÷' => {
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
            '#' => {
                Ok(Token::TITLE)
            },
            ':' => {
                self.advance();
                Ok(Token::LABEL)
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
//   Syntax analysis
//#############################################################

#[allow(unused)]
/// Returns (Token, start, end) for each token in the input text
pub fn syntax_analysis(input: &str, variables: Variables) -> Vec<(Token, usize, usize)> {
    let mut lexer = Lexer::new(input.to_string(), variables);
    let mut res = vec![];

    let mut start = lexer.pos;
    while let Ok(token) = lexer.next_token() {
        let end = lexer.pos;

        res.push((token.clone(), start, end));
        start = end;

        // otherwise the loop never exit
        if matches!(token, Token::EOF | Token::TITLE | Token::COMMENT)  {
            return res
        }
    }

    res
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
        matches!(self.children[0].token, Token::EOF | Token::COMMENT | Token::TITLE)
    }

    /// Pretty-print the AST tree for debugging.
    #[allow(unused)]
    pub fn print_tree(&self, indent: usize) {
        let indent_str = "  ".repeat(indent);
        println!("{}{:?}", indent_str, self.token);
        for child in &self.children {
            child.print_tree(indent + 1);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Parser, Error> {
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

    /// number : NUMBER
    fn number(&mut self) -> Result<AST, Error> {
        let token = self.current_token.clone();
        self.eat(&token)?;
        let node = AST::new(token, vec![]);
        Ok(node)
    }

    /// value : (MONEY) number | UNIT | number (LARGE) (MONEY | PERCENTAGE | UNIT)
    /// TODO: we should fusion UNIT and MONEY to have (and maybe even UNIT and PERCENTAGE)
    /// value : UNIT (number) | number (PERCENTAGE | UNIT)
    fn value(&mut self) -> Result<AST, Error> {
        let token = self.current_token.clone();

        match token {
            // MONEY number
            Token::MONEY(currency) => {
                self.eat(&Token::MONEY(currency))?;
                let node: AST = AST::new(Token::MONEY(currency), vec![self.number()?]);
                Ok(node)
            },

            // UNIT
            Token::UNIT(symbol) => {
                self.eat(&Token::UNIT(symbol))?;
                let node: AST = AST::new(Token::UNIT(symbol), vec![]);
                Ok(node)
            },

            // NUMBER
            Token::NUMBER(_) => {
                let mut node = self.number()?;

                // LARGE: the k in 10k €
                if let Token::LARGE(prop) = self.current_token {
                    self.eat(&Token::LARGE(prop))?;
                    let number = Token::NUMBER(Number::from_i64(prop).unwrap());
                    node = AST::new(Token::MUL, vec![node, AST::new(number, vec![])]);
                }

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
            _ => Err(Error::InvalidSyntax)
        }
    }

    /// factor : (PLUS | MINUS) factor | (number | UNIT | MONEY) | LPAREN expr RPAREN | VAR
    fn factor(&mut self) -> Result<AST, Error> {
        let token = self.current_token.clone();
        
        match token {
            Token::MONEY(_) | Token::UNIT(_) | Token::NUMBER(_) => {
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

    /// expr    : term   ((PLUS | MINUS | AS) term)*
    fn expr(&mut self) -> Result<AST, Error> {
        let mut node = self.term()?;

        while matches!(self.current_token, Token::PLUS | Token::MINUS | Token::AS) {

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
                Token::AS => {
                    self.eat(&Token::AS)?;
                    let children: Vec<AST> = vec![node, self.term()?];
                    node = AST::new(Token::AS, children);
                }
                _ => panic!("Incorrect token in expr()")
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
        // For debug pupropses
        // let mut lexer_clone = self.lexer.clone();
        // while let Ok(token) = lexer_clone.get_next_token() {
        //     println!("tok: {:?}", token);
        //     if matches!(token, Token::EOF | Token::COMMENT | Token::TITLE) {
        //         break;
        //     }
        // }
        // println!("----------------------------------------------------");

        if matches!(self.current_token, Token::VAR(_)) && self.lexer.peek_next_token() == Some(Token::ASSIGN) {
            self.assignement()
        } else {
            self.expr()
        }
    }

    // Jump just after the last label if we have one
    fn seek_to_last_label(&mut self) -> Result<(), Error> {
        let mut final_lexer = self.lexer.clone();
        let mut final_token = self.current_token.clone();

        let mut lexer = self.lexer.clone();
        
        while let Ok(token) = lexer.get_next_token() {
            if matches!(token, Token::EOF | Token::COMMENT | Token::TITLE) {
                break;
            }

            if token == Token::LABEL {
                final_lexer = lexer.clone();
                final_token = final_lexer.get_next_token()?;
            }
        }
        self.lexer = final_lexer;
        self.current_token = final_token;

        Ok(())
    }

    fn parse(&mut self) -> Result<AST, Error> {
        self.seek_to_last_label()?;
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
    pub fn new(parser: Parser, variables: Rc<RefCell<HashMap<String, Value>>>) -> Interpreter {
        Interpreter {
            parser: parser,
            variables: variables
        }
    }

    fn visit_num(&self, node: &AST) -> Value {
        match &node.token {
            Token::NUMBER(i) => Value::new(i.clone()),
            _ => panic!("Error: end node is not an number")
        }
    }

    fn visit_variable(&self, node: &AST) -> Result<Value, Error> {
        match &node.token {
            Token::VAR(var_name) => {
                match get_variable(&self.variables, &var_name) {
                    Some(val) => Ok(val),
                    None => Err(Error::UndefinedVariable),
                }
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

                if right_val.number == Number::from_u8(0) {
                    return Err(Error::DivisonByZero)
                }

                // Division has been implemented as a trait for Value
                let res = match left_val / right_val {
                    Ok(val) => val,
                    Err(_) => return Err(Error::CalculationError)
                };  // Todo: use more explicit errors
                Ok(res)
            },
            Token::AS => {
                left_val.convert_to_value(&right_val)
                    .map_err(|_| Error::FailedConversion)
            }
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
                let value = self.visit(&node.children[0])?;
                let unit = unit_symbol.to_unit();

                match value.convert_to_unit(&unit) {
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
            Token::NUMBER(_) => Ok(self.visit_num(node)),
            Token::VAR(_) => Ok(self.visit_variable(node)?),
            Token::ASSIGN => Ok(self.visit_assign(node)?),
            Token::PLUS | Token::MINUS | Token::MUL | Token::DIV | Token::MONEY(_) | Token::PERCENTAGE => {
                match node.children.len() {
                    1 => Ok(self.visit_unaryop(node)?),
                    2 => Ok(self.visit_binop(node)?),
                    _ => panic!("Too many children for an AST node")
                }             
            },
            Token::AS => { // Conversion
                Ok(self.visit_binop(node)?)
            },
            Token::UNIT(symbol) => {
                match node.children.len() {
                    0 => Ok(Value::new_with_units(
                        Number::one(),
                        symbol.to_unit())
                    ),
                    1 => Ok(self.visit_unaryop(node)?),
                    _ => panic!("Too many children for an AST node")
                }             
            }
            _ => panic!("Unkown Token in the AST")
        }
    }

    pub fn interpret(&mut self) -> Result<Value, Error> {
        let tree = self.parser.parse()?;
        // tree.print_tree(0); // Print the AST for debugging
        let result = self.visit(&tree)?;
        // println!("res: {:?}", result);
        Ok(result)
    }
}

pub fn solve(input: String, variables: Variables) -> Result<String, String>{
    let text = String::from(input.trim());
    let lexer = Lexer::new(text, variables.clone());

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
