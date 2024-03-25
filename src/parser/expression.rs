use crate::{
    error::fatal_error,
    lexer::{self, Token, TokenType},
};

pub fn is_expression(token: &Token) -> Option<&str> {
    // expression: procudure query literal
    // operator
    match &token.token_type {
        TokenType::Invalid => {
            fatal_error(crate::error::LogoError::UnExpectedToken(
                token.souce.clone(),
            ));
            panic!("not a expression");
        }
        TokenType::Keyword(key) => {
            match key {
                lexer::keyword::Keyword::EQ => Some(&token.souce),
                lexer::keyword::Keyword::NE => Some(&token.souce),
                lexer::keyword::Keyword::GT => Some(&token.souce),
                lexer::keyword::Keyword::LT => Some(&token.souce),
                lexer::keyword::Keyword::AND => Some(&token.souce),
                lexer::keyword::Keyword::OR => Some(&token.souce),
                lexer::keyword::Keyword::Plus => Some(&token.souce),
                lexer::keyword::Keyword::Minus => Some(&token.souce),
                lexer::keyword::Keyword::Multipliy => Some(&token.souce),
                lexer::keyword::Keyword::Divide => Some(&token.souce),
                _ => {
                    // panic!("unknown key");
                    None
                }
            }
        }
        TokenType::Float(f) => Some(f),
        TokenType::Variable => None,
        TokenType::Procedure => Some(&token.souce),
        TokenType::Query(_) => Some(&token.souce),
        TokenType::LRrace => None,
        TokenType::RBrace => None,
        TokenType::LSBracket => None,
        TokenType::RSBracket => None,
    }
}
