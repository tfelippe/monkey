mod lexer;
mod tokens;

fn main() {
    let tok = tokens::Token {
        kind: tokens::Type::Identifier,
        literal: String::from("hello"),
    };

    println!("My token: {}", tok.literal)
}
