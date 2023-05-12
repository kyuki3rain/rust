use std::fmt;

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq, Debug, Clone, Eq, Hash)]
pub enum TokenType {
    Illegal,
    EoF,

    Ident, // add, foobar, x, y, ...
    Int,   // 1343456
    // STRING, // "hello world"
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
    // BANG, // !
    Lt,    // <
    Gt,    // >
    Eq,    // ==
    NotEq, // !=
    LtEq,  // <=
    GtEq,  // >=

    // COMMA, // ,
    SemiColon, // ;
    // COLON, // :
    // DOT, // .
    LParen, // (
    RPren,  // )
    LBrace, // {
    RBrace, // }
    // LBRACKET, // [
    // RBRACKET, // ]

    // FUNCTION, // fn
    // LET,     // let
    // TRUE,   // true
    // FALSE, // false
    If,     // if
    Else,   // else
    Return, // return
    While,  // while
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
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        "while" => TokenType::While,
        _ => TokenType::Ident,
    }
}

pub fn new_token(token_type: TokenType, literal: String) -> Token {
    Token {
        token_type,
        literal,
    }
}
