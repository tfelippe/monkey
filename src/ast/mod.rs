use crate::tokens;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    statements: Box<Vec<Node>>,
}

impl Node for Program {
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return *self.statements[0].token_literal();
        } else {
            return String::from("");
        }
    }
}

pub struct Identifier {
    token: tokens::Token,
    value: String,
}

impl Node for Identifier {
    pub fn token_literal(&self) -> String {
        self.token.literal
    }
}

pub struct LetStatement {
    token: tokens::Token,
    name: &Identifier,
    value: Expression,
}

impl Node for LetStatement {
    pub fn token_literal(&self) -> String {
        self.token.literal
    }
}
