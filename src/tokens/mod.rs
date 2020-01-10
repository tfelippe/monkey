// #[derive(PartialEq)]
#[derive(Debug, PartialEq)]
pub enum Type {
    Illegal,
    EOF,

    // identifiers and literals
    Identifer,
    Integer,

    // operators
    Assign,
    Plus,

    // delimiters
    Comma,
    Semicolon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // keywords
    Function,
    Let,
}

pub struct Token {
    pub kind: Type,
    pub literal: String,
}
