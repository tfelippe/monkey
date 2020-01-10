mod lexer;
mod tokens;

fn main() {
    let tok = tokens::Token {
        kind: tokens::Type::Identifer,
        literal: String::from("hello"),
    };

    println!("My token: {}", tok.literal)
}
