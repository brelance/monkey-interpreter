use std::{char, env};

use super::token::{Token, KeyWords};
struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: 0 as char,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => {
                match self.input.chars().nth(self.read_position).unwrap() {
                    '=' => {
                        let mut ch = self.ch.to_string();
                        self.read_char();
                        ch.push(self.ch);
                        Token::new(KeyWords::EQ, ch)
                    },
                    
                    _ => Token::new(KeyWords::ASSIGN, self.ch.to_string()),
                }
            }

            '+' => Token::new(KeyWords::PLUS, self.ch.to_string()),

            '-' => Token::new(KeyWords::MINUS, self.ch.to_string()),

            '*' => Token::new(KeyWords::ASTERISK, self.ch.to_string()),

            '/' => Token::new(KeyWords::SLASH, self.ch.to_string()),

            '!' => {
                match self.input.chars().nth(self.read_position).unwrap() {
                    '=' => {
                        let mut ch = self.ch.to_string();
                        self.read_char();
                        ch.push(self.ch);
                        Token::new(KeyWords::NOT_EQ, ch)
                    },

                    _ => Token::new(KeyWords::BANG, self.ch.to_string()),
                }
            }

            '<' => Token::new(KeyWords::LT, self.ch.to_string()),
            '>' => Token::new(KeyWords::GT, self.ch.to_string()),

            ',' => Token::new(KeyWords::COMMA, self.ch.to_string()),
            ';' => Token::new(KeyWords::SEMICOLON, self.ch.to_string()),

            '(' => Token::new(KeyWords::LPAREN, self.ch.to_string()),
            ')' => Token::new(KeyWords::RPAREN, self.ch.to_string()),
            '{' => Token::new(KeyWords::LBRACE, self.ch.to_string()),
            '}' => Token::new(KeyWords::RBRACE, self.ch.to_string()),

            '\0' => Token::new(KeyWords::EOF, self.ch.to_string()),

            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = Token::lookup_token(&literal);
                    return Token::new(token_type, literal);
                } else if self.ch.is_ascii_digit() {
                    return Token::new(KeyWords::INT, self.read_number());
                } else {
                    Token::new(KeyWords::ILLEGAL, self.ch.to_string())
                }
            }
        };
        println!("{:?}", token);
        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            if !self.ch.is_whitespace() {
                println!("{}", self.ch);
            } else {
                println!("{}", "空格");
            }
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek(&self) -> char {
        if self.read_position > self.input.len() {
            return 0 as char;
        } else {
            return self.input.chars().nth(self.read_position).unwrap();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        (&self.input[position..self.position]).to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        (&self.input[position..self.position]).to_string()    
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'A' || ch == '_'
}

#[test]
fn lexer_test() {
    // env::set_var("RUST_BACKTRACE", "full");
    let tokens = [
            Token::new(KeyWords::LET, "let".to_string()),
            Token::new(KeyWords::IDENT, "five".to_string()),
            Token::new(KeyWords::ASSIGN, "=".to_string()),
            Token::new(KeyWords::INT, "5".to_string()),
            Token::new(KeyWords::SEMICOLON, ";".to_string()),
            Token::new(KeyWords::LET, "let".to_string()),
            Token::new(KeyWords::IDENT, "ten".to_string()),
            Token::new(KeyWords::ASSIGN, "=".to_string()),
            Token::new(KeyWords::INT, "10".to_string()),
            Token::new(KeyWords::SEMICOLON, ";".to_string()),
            Token::new(KeyWords::LET, "let".to_string()),
            Token::new(KeyWords::IDENT, "add".to_string()),
            Token::new(KeyWords::ASSIGN, "=".to_string()),
            Token::new(KeyWords::FUNCTION, "fn".to_string()),
            Token::new(KeyWords::LPAREN, "(".to_string()),
            Token::new(KeyWords::IDENT, "x".to_string()),
            Token::new(KeyWords::COMMA, ",".to_string()),
            Token::new(KeyWords::IDENT, "y".to_string()),
            Token::new(KeyWords::RPAREN, ")".to_string()),
            Token::new(KeyWords::LBRACE, "{".to_string()),
            Token::new(KeyWords::IDENT, "x".to_string()),
            Token::new(KeyWords::PLUS, "+".to_string()),
            Token::new(KeyWords::IDENT, "y".to_string()),
            Token::new(KeyWords::SEMICOLON, ";".to_string()),
            Token::new(KeyWords::RBRACE, "}".to_string()),
            Token::new(KeyWords::SEMICOLON, ";".to_string()),
            Token::new(KeyWords::LET, "let".to_string()),
            Token::new(KeyWords::IDENT, "result".to_string()),
            Token::new(KeyWords::ASSIGN, "=".to_string()),
            Token::new(KeyWords::IDENT, "add".to_string()),
            Token::new(KeyWords::LPAREN, "(".to_string()),
            Token::new(KeyWords::IDENT, "five".to_string()),
            Token::new(KeyWords::COMMA, ",".to_string()),
            Token::new(KeyWords::IDENT, "ten".to_string()),
            Token::new(KeyWords::RPAREN, ")".to_string()),
            Token::new(KeyWords::SEMICOLON, ";".to_string()),
            Token::new(KeyWords::BANG, "!".to_string()),
            Token::new(KeyWords::MINUS, "-".to_string()),
            Token::new(KeyWords::SLASH, "/".to_string()),
            Token::new(KeyWords::ASTERISK, "*".to_string()),
            Token::new(KeyWords::INT, "5".to_string()),
            Token::new(KeyWords::SEMICOLON, ";".to_string()),
            Token::new(KeyWords::INT, "5".to_string()),
            Token::new(KeyWords::LT, "<".to_string()),
            Token::new(KeyWords::INT, "10".to_string()),
            Token::new(KeyWords::GT, ">".to_string()),
            Token::new(KeyWords::INT, "5".to_string()),
            Token::new(KeyWords::SEMICOLON, ";".to_string()),
            Token::new(KeyWords::IF, "if".to_string()),
            Token::new(KeyWords::LPAREN, "(".to_string()),
            Token::new(KeyWords::INT, "5".to_string()),
            Token::new(KeyWords::LT, "<".to_string()),
            Token::new(KeyWords::INT, "10".to_string()),
            Token::new(KeyWords::RPAREN, ")".to_string()),
            Token::new(KeyWords::LBRACE, "{".to_string()),
            Token::new(KeyWords::RETURN, "return".to_string()),
            Token::new(KeyWords::TRUE, "true".to_string()),
            Token::new(KeyWords::SEMICOLON, ";".to_string()),
            Token::new(KeyWords::RBRACE, "}".to_string()),
            Token::new(KeyWords::ELSE, "else".to_string()),
            Token::new(KeyWords::LBRACE, "{".to_string()),
            Token::new(KeyWords::RETURN, "return".to_string()),
            Token::new(KeyWords::FALSE, "false".to_string()),
            Token::new(KeyWords::SEMICOLON, ";".to_string()),
            Token::new(KeyWords::RBRACE, "}".to_string()),
            Token::new(KeyWords::INT, "10".to_string()),
            Token::new(KeyWords::EQ, "==".to_string()),
            Token::new(KeyWords::INT, "10".to_string()),
            Token::new(KeyWords::SEMICOLON, ";".to_string()),
            Token::new(KeyWords::INT, "10".to_string()),
            Token::new(KeyWords::NOT_EQ, "!=".to_string()),
            Token::new(KeyWords::INT, "9".to_string()),
            Token::new(KeyWords::SEMICOLON, ";".to_string()),
            Token::new(KeyWords::EOF, "\0".to_string())
    ];

    let input = "let five = 5;
    let ten = 10;
    let add = fn (x, y) {
        x + y;
    };
    
    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;
    
    if (5 < 10) {
        return true;
    } else {
        return false;
    }
    
    10 == 10;
    10 != 9;";


    let mut l = Lexer::new(input.to_string());
    l.next_token();
    for token in tokens {
        let test_token = l.next_token();
        assert_eq!(test_token, token);
    }
}