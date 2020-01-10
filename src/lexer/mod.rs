use crate::tokens;

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn next_token(&mut self) -> tokens::Token {
        let token = match self.ch {
            '=' => make_token(tokens::Type::Assign, self.ch),
            ';' => make_token(tokens::Type::Semicolon, self.ch),
            '(' => make_token(tokens::Type::LeftParen, self.ch),
            ')' => make_token(tokens::Type::RightParen, self.ch),
            ',' => make_token(tokens::Type::Comma, self.ch),
            '+' => make_token(tokens::Type::Plus, self.ch),
            '{' => make_token(tokens::Type::LeftBrace, self.ch),
            '}' => make_token(tokens::Type::RightBrace, self.ch),
            _ => make_token(tokens::Type::EOF, '\0'),
        };

        self.read_char();
        return token;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}

fn new(input: String) -> Lexer {
    let mut l = Lexer {
        input: input,
        position: 0,
        read_position: 0,
        ch: '\0',
    };

    l.read_char();
    return l;
}

fn make_token(typ: tokens::Type, lit: char) -> tokens::Token {
    return tokens::Token {
        kind: typ,
        literal: lit.to_string(),
    };
}

#[cfg(test)]
mod lexer {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let mut l = new(input.to_string());

        let tests = [
            (tokens::Type::Assign, "="),
            (tokens::Type::Plus, "+"),
            (tokens::Type::LeftParen, "("),
            (tokens::Type::RightParen, ")"),
            (tokens::Type::LeftBrace, "{"),
            (tokens::Type::RightBrace, "}"),
            (tokens::Type::Comma, ","),
            (tokens::Type::Semicolon, ";"),
            (tokens::Type::EOF, "\0"),
        ];

        for (expected_type, expected_literal) in &tests {
            let tok = l.next_token();

            assert_eq!(tok.kind, *expected_type);
            assert_eq!(tok.literal, *expected_literal);
        }
    }
}
