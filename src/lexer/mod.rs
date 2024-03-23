pub mod keyword;
mod lexer_impl;
mod literal;
mod procedure;
mod query;
mod token;
pub mod token_type;
pub mod variable;

use std::path::PathBuf;

pub use token::Token;
pub use token_type::TokenType;

use crate::lexer::lexer_impl::LexerImpl;

pub trait Lexer {
    fn next_line_token(&mut self) -> Option<Vec<Token>>;

    fn get_current_line_number(&self) -> usize;
}

pub struct LexerFactory;

impl LexerFactory {
    pub fn create_lexer(source_path: &PathBuf) -> Box<dyn Lexer> {
        Box::new(LexerImpl::new(source_path))
    }
}
