use super::token::Token;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}
