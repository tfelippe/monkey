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
    Minus,
    Bang,
    Asterisk,
    Slash,

    LessThan,
    GreaterThan,

    Equal,
    NotEqual,

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
    True,
    False,
    If,
    Else,
    Return,
}

pub struct Token {
    pub kind: Type,
    pub literal: String,
}

pub fn lookup_identifier(lit: String) -> Type {
    return match lit.as_ref() {
        "fn" => Type::Function,
        "let" => Type::Let,
        "true" => Type::True,
        "false" => Type::False,
        "if" => Type::If,
        "else" => Type::Else,
        "return" => Type::Return,
        _ => Type::Identifier,
    };
}
