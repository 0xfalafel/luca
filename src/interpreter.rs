use std::io;
use std::io::Write;

#[derive(Debug, Eq, PartialEq)]
enum Error {
    ParseError,
    InvalidTokenType,
    InvalidSyntax
}


/**************************************************************
*   Tokens / Lexer
**************************************************************/

// Token types
//
// EOF (end-of-file) is  used to indicate that there is no more input left


/// Token are used to represent the differents elements given as an input.
/// The input is separated in a bunch of tokens.
#[derive(Debug, Clone, Eq, PartialEq)]
enum Token {
    INTEGER(i128),
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    EOF,
}

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

        for char in self.text[self.pos..].chars() {
            if char.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    /// Return a (multidigit) integer consumed from the input.
    fn integer(&mut self) -> i128 {
        let int_start = self.pos;

        loop {
            if let Some (char) = self.get_char() {
                if char.is_ascii_digit() {
                    self.advance()
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        return i128::from_str_radix(&self.text[int_start..self.pos], 10).unwrap();
    }

    /// Lexical analyser (also known as scanner or tokenizer).
    ///    
    /// This method is responsible for breaking a sentence
    /// appart into tokens. One token at the time.
    pub fn get_next_token(&mut self) -> Result<Token, Error> {

        let char = loop {

            let char = self.get_char();
            if char == None {
                return Ok(Token::EOF);
            }
            
            let char = char.unwrap();
            
            if char.is_whitespace(){
                self.skip_whitespace();
            } else {
                break char;
            }
        };

        match char {
            char if char.is_ascii_digit() => {
                Ok(Token::INTEGER(self.integer()))
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
            _ => {Err(Error::ParseError)}
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
            Err(Error::InvalidTokenType)
        }
    }

    /// factor : (PLUS | MINUS) factor | INTEGER | LPAREN expr RPAREN
    fn factor(&mut self) -> Result<AST, Error> {
        let token = self.current_token.clone();
        
        match token {
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
            // INTEGER
            Token::INTEGER(i) => {
                self.eat(Token::INTEGER(i))?;
                let node = AST::new(token, vec![]);
                Ok(node)
            },
            // LPAREN expr RPAREN
            Token::LPAREN => {
                self.eat(Token::LPAREN)?;
                let node = self.expr()?;
                self.eat(Token::RPAREN)?;
                Ok(node)
            },
            _ => {
                Err(Error::InvalidSyntax)
            }
        }
    }

    /// term : factor ((MUL | DIV) factor)*
    fn term(&mut self) -> Result<AST, Error> {
        let mut node = self.factor()?;

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
    /// term    : factor ((MUL  | DIV) factor)*
    /// factor  : INTEGER | LPAREN expr RPAREN
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

    fn parse(&mut self) -> Result<AST, Error> {
        self.expr()
    }
}


//#############################################################
//   Interpreter
//#############################################################

pub struct Interpreter {
    parser: Parser
}

impl Interpreter {
    fn new(parser: Parser) -> Interpreter {
        Interpreter { parser: parser }
    }

    fn visit_num(&self, node: &AST) -> i128 {
        match node.token {
            Token::INTEGER(i) => i,
            _ => panic!("Error: end node is not an integer")
        }
    }

    fn visit_binop(&self, node: &AST) -> i128 {
        let left_val = self.visit(&node.children[0]);
        let right_val = self.visit(&node.children[1]);

        match node.token {
            Token::PLUS => {
                return left_val + right_val
            },
            Token::MINUS => {
                return left_val - right_val
            },
            Token::MUL => {
                return left_val * right_val
            },
            Token::DIV => {
                return left_val / right_val
            },
            _ => panic!("Unkown BinOp Token in the AST")
        }
    }

    fn visit_unaryop(&self, node: &AST) -> i128 {
        let val = self.visit(&node.children[0]);

        match node.token {
            Token::PLUS  => {  val },
            Token::MINUS => { -val }
            _ => {panic!("Invalid token type for an unary node")}
        }
    }

    fn visit(&self, node: &AST) -> i128 {
        match node.token {
            Token::INTEGER(_i) => {
                return self.visit_num(node);
            },
            Token::PLUS | Token::MINUS | Token::MUL | Token::DIV => {
                match node.children.len() {
                    1 => return self.visit_unaryop(node),
                    2 => return self.visit_binop(node),
                    _ => panic!("Too many children for an AST node")
                }             
            },
            _ => panic!("Unkown Token in the AST")
        }
    }

    fn interpret(&mut self) -> Result<i128, Error> {
        let tree = self.parser.parse()?;
        let result = self.visit(&tree);
        Ok(result)
    }
}

#[allow(unused)]
fn main() {

    loop {
        // show the interactive prompt
        print!("calc> ");
        io::stdout().flush().unwrap();
    
        // read input from user
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.eq("") || input.eq("exit\n") {
            break;
        }

        let text = String::from(input.trim());
        let lexer = Lexer::new(text);

        match Parser::new(lexer) {
            Ok(parser) => {
                let mut interpreter = Interpreter::new(parser);
                match interpreter.interpret() {
                    Ok(result) => println!("{}", result),
                    Err(_) => println!("Invalid syntax")
                }
            },
            Err(_) => println!("Invalid syntax")
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_interpreter(text: &str) -> Interpreter {
        let lexer = Lexer::new(String::from(text));
        let parser = Parser::new(lexer).expect("Could not parse");
        let interpreter = Interpreter::new(parser);

        interpreter
    }

    #[test]
    fn test_expression1() {
        let mut interpreter = make_interpreter("3");
        let result = interpreter.interpret();
        assert_eq!(result, Ok(3));
    }

    #[test]
    fn test_expression2() {
        let mut interpreter = make_interpreter("2 + 7 * 4");
        let result = interpreter.interpret();
        assert_eq!(result, Ok(30));
    }

    #[test]
    fn test_expression3() {
        let mut interpreter = make_interpreter("7 - 8 / 4");
        let result = interpreter.interpret();
        assert_eq!(result, Ok(5));
    }

    #[test]
    fn test_expression4() {
        let mut interpreter = make_interpreter("14 + 2 * 3 - 6 / 2");
        let result = interpreter.interpret();
        assert_eq!(result, Ok(17));
    }

    #[test]
    fn test_expression5() {
        let mut interpreter = make_interpreter("7 + 3 * (10 / (12 / (3 + 1) - 1))");
        let result = interpreter.interpret();
        assert_eq!(result, Ok(22));
    }

    #[test]
    fn test_expression6() {
        let mut interpreter = make_interpreter(
            "7 + 3 * (10 / (12 / (3 + 1) - 1)) / (2 + 3) - 5 - 3 + (8)"
        );
        let result = interpreter.interpret();
        assert_eq!(result, Ok(10));
    }

    #[test]
    fn test_expression7() {
        let mut interpreter = make_interpreter("7 + (((3 + 2)))");
        let result = interpreter.interpret();
        assert_eq!(result, Ok(12));
    }

    #[test]
    fn test_expression_invalid_syntax() {
        let mut interpreter = make_interpreter("10 *");
        let result = interpreter.interpret();
        assert_eq!(result, Err(Error::InvalidSyntax));
    }
    
    #[test]
    fn test_expression_unary() {
        let mut interpreter = make_interpreter("---42");
        let result = interpreter.interpret();
        assert_eq!(result, Ok(-42));
    }

    #[test]
    fn test_expression_unary2() {
        let mut interpreter = make_interpreter("-6*-7 - 3");
        let result = interpreter.interpret();
        assert_eq!(result, Ok(39));
    }
}

pub fn solve(input: String) -> Result<String, String>{
    let text = String::from(input.trim());
    let lexer = Lexer::new(text);

    match Parser::new(lexer) {
        Ok(parser) => {
            let mut interpreter = Interpreter::new(parser);
            match interpreter.interpret() {
                Ok(result) => Ok(format!("{}", result)),
                Err(_) => Err("Invalid syntax".to_string())
            }
        },
        Err(_) => Err("Invalid syntax".to_string())
    }
}