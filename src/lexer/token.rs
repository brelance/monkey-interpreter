
type TokenType = String;


#[derive(PartialEq, Eq, Debug)]
pub enum KeyWords {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    ASSIGN,
    BANG,

    LT,
    GT,

    EQ,
    NOT_EQ,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}


#[derive(PartialEq, Eq, Debug)]
pub struct Token {
    pub token_type: KeyWords,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: KeyWords, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }

    pub fn lookup_token(literal: &String) -> KeyWords {
        match literal.as_str() {
            "fn" => KeyWords::FUNCTION,
            "let" => KeyWords::LET,
            "true" => KeyWords::TRUE,
            "false" => KeyWords::FALSE,
            "if" => KeyWords::IF,
            "else" => KeyWords::ELSE,
            "return" => KeyWords::RETURN,
            _ => KeyWords::IDENT,
        }
    }
}