use lexer::Tokenizer;
use lexer::token::{Token, TokenType};

macro_rules! token {
    ($tokenizer:expr, $token_type:ident, $accum:expr) => {{
        token!($tokenizer , TokenType::$token_type, $accum)
    }};
    ($tokenizer:expr, $token_type:expr, $accum:expr) => {{
        let tokenizer = $tokenizer as &$crate::lexer::Tokenizer;
        let token_type = $token_type as $crate::lexer::token::TokenType;
        Some(Token::new(token_type, tokenizer.last_position(), $accum))
    }};
}

pub trait Matcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token>;
}