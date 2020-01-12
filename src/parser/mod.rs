pub struct Parser {
    l: lexer::Lexer,

    cur_token: tokens::Token,
    peek_token: tokens::Token,
}

pub fn new(l: &lexer::Lexer) -> Parser {
    let mut p = Parser {l: l, ..}
}
