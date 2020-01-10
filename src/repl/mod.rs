use crate::lexer;
use crate::tokens;
use std::io::{stdin, stdout, Write};

const PROMPT: &str = ">>";

pub fn start() {
    loop {
        println!("Hello! This is the Monkey programming language.");
        print!("{} ", PROMPT);

        let mut s = String::new();
        let _ = stdout().flush();

        stdin().read_line(&mut s).expect("Error: Input invalid");

        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }

        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        let mut l = lexer::new(s);
        loop {
            let tok = l.next_token();
            match tok.kind {
                tokens::Type::EOF => {
                    break;
                }
                _ => {
                    println!("[{:?}:\"{}\"]", tok.kind, tok.literal);
                }
            }
        }
    }
}
