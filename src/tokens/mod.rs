#[derive(Debug, PartialEq)]
pub enum Type {
    Illegal,
    EOF,

    // identifiers and literals
    Identifier,
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

pub fn lookup_identifier(lit: String) -> Type {
    return match lit.as_ref() {
        "fn" => Type::Function,
        "let" => Type::Let,
        _ => Type::Identifier,
    };
}
