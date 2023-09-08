use crate::parser::token::Token;

pub struct Scanner {
    pub source: String,
}

impl Scanner {
    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        // TODO: Determine tokens.
        tokens
    }
}
