use crate::tokens;

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn next_token(&mut self) -> tokens::Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let lit = ch.to_string() + &self.ch.to_string();
                    make_token(tokens::Type::Equal, lit)
                } else {
                    make_token(tokens::Type::Assign, self.ch.to_string())
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let lit = ch.to_string() + &self.ch.to_string();
                    make_token(tokens::Type::NotEqual, lit)
                } else {
                    make_token(tokens::Type::Bang, self.ch.to_string())
                }
            }
            ';' => make_token(tokens::Type::Semicolon, self.ch.to_string()),
            '(' => make_token(tokens::Type::LeftParen, self.ch.to_string()),
            ')' => make_token(tokens::Type::RightParen, self.ch.to_string()),
            ',' => make_token(tokens::Type::Comma, self.ch.to_string()),
            '+' => make_token(tokens::Type::Plus, self.ch.to_string()),
            '-' => make_token(tokens::Type::Minus, self.ch.to_string()),
            '/' => make_token(tokens::Type::Slash, self.ch.to_string()),
            '*' => make_token(tokens::Type::Asterisk, self.ch.to_string()),
            '<' => make_token(tokens::Type::LessThan, self.ch.to_string()),
            '>' => make_token(tokens::Type::GreaterThan, self.ch.to_string()),
            '{' => make_token(tokens::Type::LeftBrace, self.ch.to_string()),
            '}' => make_token(tokens::Type::RightBrace, self.ch.to_string()),
            '\0' => make_token(tokens::Type::EOF, "\0".to_string()),
            _ => {
                if is_letter(self.ch) {
                    let lit = self.read_identifier();
                    let typ = tokens::lookup_identifier(lit.clone());
                    return make_token(typ, lit);
                } else if is_digit(self.ch) {
                    let typ = tokens::Type::Integer;
                    let lit = self.read_number();
                    return make_token(typ, lit);
                } else {
                    return make_token(tokens::Type::Illegal, self.ch.to_string());
                }
            }
        };

        self.read_char();
        return token;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            let ch = self.input.chars().nth(self.read_position).unwrap();
            return ch;
        }
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;

        while is_digit(self.ch) {
            self.read_char();
        }

        return self.input[pos..self.position].to_string();
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;

        while is_letter(self.ch) {
            self.read_char();
        }

        return self.input[pos..self.position].to_string();
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

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
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

fn make_token(typ: tokens::Type, lit: String) -> tokens::Token {
    return tokens::Token {
        kind: typ,
        literal: lit,
    };
}

fn is_letter(ch: char) -> bool {
    return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
}

fn is_digit(ch: char) -> bool {
    return '0' <= ch && ch <= '9';
}

#[cfg(test)]
mod lexer {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
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
            10 != 9;
        "#;

        let mut l = new(input.to_string());
        let tests = vec![
            (tokens::Type::Let, "let"),
            (tokens::Type::Identifier, "five"),
            (tokens::Type::Assign, "="),
            (tokens::Type::Integer, "5"),
            (tokens::Type::Semicolon, ";"),
            (tokens::Type::Let, "let"),
            (tokens::Type::Identifier, "ten"),
            (tokens::Type::Assign, "="),
            (tokens::Type::Integer, "10"),
            (tokens::Type::Semicolon, ";"),
            (tokens::Type::Let, "let"),
            (tokens::Type::Identifier, "add"),
            (tokens::Type::Assign, "="),
            (tokens::Type::Function, "fn"),
            (tokens::Type::LeftParen, "("),
            (tokens::Type::Identifier, "x"),
            (tokens::Type::Comma, ","),
            (tokens::Type::Identifier, "y"),
            (tokens::Type::RightParen, ")"),
            (tokens::Type::LeftBrace, "{"),
            (tokens::Type::Identifier, "x"),
            (tokens::Type::Plus, "+"),
            (tokens::Type::Identifier, "y"),
            (tokens::Type::Semicolon, ";"),
            (tokens::Type::RightBrace, "}"),
            (tokens::Type::Semicolon, ";"),
            (tokens::Type::Let, "let"),
            (tokens::Type::Identifier, "result"),
            (tokens::Type::Assign, "="),
            (tokens::Type::Identifier, "add"),
            (tokens::Type::LeftParen, "("),
            (tokens::Type::Identifier, "five"),
            (tokens::Type::Comma, ","),
            (tokens::Type::Identifier, "ten"),
            (tokens::Type::RightParen, ")"),
            (tokens::Type::Semicolon, ";"),
            (tokens::Type::Bang, "!"),
            (tokens::Type::Minus, "-"),
            (tokens::Type::Slash, "/"),
            (tokens::Type::Asterisk, "*"),
            (tokens::Type::Integer, "5"),
            (tokens::Type::Semicolon, ";"),
            (tokens::Type::Integer, "5"),
            (tokens::Type::LessThan, "<"),
            (tokens::Type::Integer, "10"),
            (tokens::Type::GreaterThan, ">"),
            (tokens::Type::Integer, "5"),
            (tokens::Type::Semicolon, ";"),
            (tokens::Type::If, "if"),
            (tokens::Type::LeftParen, "("),
            (tokens::Type::Integer, "5"),
            (tokens::Type::LessThan, "<"),
            (tokens::Type::Integer, "10"),
            (tokens::Type::RightParen, ")"),
            (tokens::Type::LeftBrace, "{"),
            (tokens::Type::Return, "return"),
            (tokens::Type::True, "true"),
            (tokens::Type::Semicolon, ";"),
            (tokens::Type::RightBrace, "}"),
            (tokens::Type::Else, "else"),
            (tokens::Type::LeftBrace, "{"),
            (tokens::Type::Return, "return"),
            (tokens::Type::False, "false"),
            (tokens::Type::Semicolon, ";"),
            (tokens::Type::RightBrace, "}"),
            (tokens::Type::Integer, "10"),
            (tokens::Type::Equal, "=="),
            (tokens::Type::Integer, "10"),
            (tokens::Type::Semicolon, ";"),
            (tokens::Type::Integer, "10"),
            (tokens::Type::NotEqual, "!="),
            (tokens::Type::Integer, "9"),
            (tokens::Type::Semicolon, ";"),
            (tokens::Type::EOF, "\0"),
        ];

        for (expected_type, expected_literal) in &tests {
            let tok = l.next_token();

            assert_eq!(tok.kind, *expected_type, "{}", expected_literal);
            assert_eq!(tok.literal, *expected_literal);
        }
    }
}
