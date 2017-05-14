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

/// A matcher that only matches white-space
pub struct WhitespaceMatcher;

impl Matcher for WhitespaceMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut found = false;
        while !tokenizer.end() && tokenizer.peek().unwrap().is_whitespace() {
            found = true;
            tokenizer.next();
        }
        if found {
            token!(tokenizer, Whitespace, String::new())
        } else {
            None
        }
    }
}

/// A matcher that matches base-10 integer literals
pub struct IntLiteralMatcher {}

impl Matcher for IntLiteralMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut accum = String::new();
        let negative = tokenizer.peek() == Some(&'-');
        if negative {
            tokenizer.advance(1)
        };
        while !tokenizer.end() && tokenizer.peek().unwrap().is_digit(10) {
            accum.push(tokenizer.next().unwrap());
        }
        if !accum.is_empty() {
            let literal: String = if negative {
                match i64::from_str_radix(accum.as_str(), 10) {
                    Ok(result) => format!("-{}", result),
                    Err(error) => panic!("unable to parse int-literal: {}", error)
                }
            } else {
                match u64::from_str_radix(accum.as_str(), 10) {
                    Ok(result) => result.to_string(),
                    Err(error) => panic!("unable to parse int-literal: {}", error)
                }
            };
            token!(tokenizer, IntLiteral, literal)
        } else {
            None
        }
    }
}

/// A matcher that matches float literals
pub struct FloatLiteralMatcher;

impl Matcher for FloatLiteralMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut accum = String::new();
        let curr = tokenizer.next().unwrap();
        if curr.is_digit(10) {
            accum.push(curr)
        } else if curr == '.' {
            accum.push_str("0.")
        } else {
            return None
        }
        while !tokenizer.end() {
            let current = *tokenizer.peek().unwrap();
            if !current.is_whitespace() && current.is_digit(10) || current == '.' {
                if current == '.' && accum.contains('.') {
                    panic!("illegal decimal point")
                }
                accum.push(tokenizer.next().unwrap())
            } else {
                break
            }
        }
        if accum.contains('.') {
            token!(tokenizer, FloatLiteral, accum)
        } else {
            token!(tokenizer, IntLiteral, accum)
        }
    }
}
