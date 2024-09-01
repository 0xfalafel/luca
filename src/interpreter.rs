


#[derive(Debug)]
enum Error {
    ParseError,
    InvalidTokenType
}

#[derive(PartialEq)]
#[derive(Debug)]
enum TokenType {
    INTEGER,
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAR,
    RPAR,
    EOF,
}

#[derive(Debug)]
struct Token {
    kind: TokenType,
    value: i64
}

struct Lexer {
    text: String,
    pos: usize
}

/// Split the input text in multiple tokens
impl Lexer {
    pub fn new(text: String) -> Lexer {

        Lexer {
            text: text,
            pos: 0
        }
    }

    fn advance(&mut self) {
        self.pos += 1
    }

    fn get_char(&self) -> Option<char> {
        self.text.chars().nth(self.pos)
    }

    pub fn get_next_token(&mut self) -> Result<Token, Error> {

        let char = loop {

            let char = self.get_char();
            if char == None {
                return Ok(Token {kind: TokenType::EOF, value: 0});
            }
            
            let char = char.unwrap();
            
            if char.is_whitespace(){
                self.skip_whitespace();
            } else {
                break char;
            }
        };
            
        if char.is_ascii_digit() {
            return Ok(Token {kind: TokenType::INTEGER, value: self.integer()});
        }

        if char == '+' {
            self.advance();
            return Ok(Token { kind: TokenType::PLUS, value: 0 });
        }

        if char == '-' {
            self.advance();
            return Ok(Token { kind: TokenType::MINUS, value: 0 });
        }

        if char == '*' {
            self.advance();
            return Ok(Token { kind: TokenType::MUL, value: 0 });
        }

        if char == '/' {
            self.advance();
            return Ok(Token { kind: TokenType::DIV, value: 0 });
        }

        if char == '(' {
            self.advance();
            return Ok(Token { kind: TokenType::LPAR, value: 0 });
        }

        if char == ')' {
            self.advance();
            return Ok(Token { kind: TokenType::RPAR, value: 0 });
        }

        Err(Error::ParseError)
    }

    /// advance `self.pos` until the next non-whitespace character
    fn skip_whitespace(&mut self) {

        for char in self.text[self.pos..].chars() {
            if char.is_whitespace() || char.is_ascii_alphabetic() {
                self.pos += 1;
            } else {
                break;
            }
        }
   }

    fn integer(&mut self) -> i64{
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
        return i64::from_str_radix(&self.text[int_start..self.pos], 10).unwrap();
    }
}


/// Interpret the text, and calculate the result of the operation.
struct Interpreter {
    lexer: Lexer,
    current_token: Token
}

impl Interpreter {
    /*
     * Grammar:
     *  expr:   term ((MUL|DIV) term)*
     *  term:   factor ((ADD|SUB) factor) *
     *  factor: integer | LPAR expr RPAR
     */
    pub fn new(mut lexer: Lexer) -> Result<Interpreter, Error> {
        let token = lexer.get_next_token()?;

        Ok(Interpreter {
            lexer: lexer,
            current_token: token
        })
    }

    fn eat(&mut self, token_type: TokenType) -> Result<(), Error> {
        if self.current_token.kind == token_type {
            self.current_token = self.lexer.get_next_token()?;
            Ok(())
        } else {
            Err(Error::InvalidTokenType)
        }
    }

    fn integer(&mut self) -> Result<i64, Error> {
        let value = self.current_token.value;
        self.eat(TokenType::INTEGER)?;
        Ok(value)
    }

    // expr:   term ((ADD|SUB) term)*
    pub fn expr(&mut self) -> Result<i64, Error> {
        let mut result = self.term()?;

        while self.current_token.kind != TokenType::EOF {

            if self.current_token.kind == TokenType::PLUS {
                self.eat(TokenType::PLUS)?;
                result = result + self.term()?;
            }

            else if self.current_token.kind == TokenType::MINUS {
                self.eat(TokenType::MINUS)?;
                result = result - self.term()?;
            }

            else {
                return Ok(result);
            }
        }

        return Ok(result);
    }

    // term:   factor ((MUL|DIV) factor)*
    fn term(&mut self) -> Result<i64, Error> {
        let mut result = self.factor()?;

        while self.current_token.kind != TokenType::EOF {

            if self.current_token.kind == TokenType::MUL {
                self.eat(TokenType::MUL)?;
                result = result * self.factor()?;
            }

            else if self.current_token.kind == TokenType::DIV {
                self.eat(TokenType::DIV)?;
                result = result / self.factor()?;
            }

            else {
                return Ok(result);
            }
        }

        return Ok(result);
    }

    fn factor(&mut self) -> Result<i64, Error> {
        let result: i64;

        if self.current_token.kind == TokenType::LPAR {
            self.eat(TokenType::LPAR)?;
            result = self.expr()?;
            self.eat(TokenType::RPAR)?;
        }

        else {
            result = self.integer()?;
        }

        Ok(result)
    }
}

pub fn solve(input: String) -> Result<String, String>{
    let lexer = Lexer::new(input);
    if let Ok(mut interpreter) = Interpreter::new(lexer){
        if let Ok(result) = interpreter.expr() {
            // println!("{}", result);
            return Ok(format!("{}", result));
        } else {
            return Err("Invalid syntax".to_string());
        }
    } else {
        return Err("Invalid syntax".to_string());
    }
}