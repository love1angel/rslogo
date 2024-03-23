use crate::lexer::TokenType;

#[derive(Debug)]
pub struct Token {
    pub souce: String,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(souce: String, token_type: TokenType) -> Token {
        Token { souce, token_type }
    }
}
