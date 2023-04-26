use std::fmt;

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Debug, Clone, Eq, Hash)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT, // add, foobar, x, y, ...
    INT, // 1343456
    // STRING, // "hello world"

    ASSIGN, // =
    PLUS, // +
    MINUS, // -
    ASTERISK, // *
    SLASH, // /
    // BANG, // !

    LT, // <
    GT, // >
    EQ, // ==
    NOTEQ, // !=
    LTEQ, // <=
    GTEQ, // >=

    // COMMA, // ,
    SEMICOLON, // ;
    // COLON, // :
    // DOT, // .

    LPAREN, // (
    RPAREN, // )
    LBRACE, // {
    RBRACE, // }
    // LBRACKET, // [
    // RBRACKET, // ]

    // FUNCTION, // fn
    // LET,     // let
    // TRUE,   // true
    // FALSE, // false
    IF, // if
    ELSE, // else
    RETURN, // return
    WHILE, // while
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "token_type: {:?}, literal: {}",
            self.token_type, self.literal
        )
    }
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        // "fn" => TokenType::FUNCTION,
        // "let" => TokenType::LET,
        // "true" => TokenType::TRUE,
        // "false" => TokenType::FALSE,
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "return" => TokenType::RETURN,
        "while" => TokenType::WHILE,
        _ => TokenType::IDENT,
    }
}

pub fn new_token(token_type: TokenType, literal: String) -> Token {
    return Token {
        token_type,
        literal: literal,
    };
}