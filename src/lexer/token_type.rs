use clap::builder::Str;

use crate::lexer::keyword;
use crate::lexer::query;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Invalid,
    Keyword(keyword::Keyword),

    Float(String),

    Variable,  // start with "\"", can be fn arg or define a new variable
    Procedure, // start with ":" obtain variable val
    Query(query::Query),

    LRrace,    // (
    RBrace,    // )
    LSBracket, // [
    RSBracket, // ]
}
