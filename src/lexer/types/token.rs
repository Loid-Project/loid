use crate::lexer::types::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub line: u32,
    pub id: u64,
    pub value: String,
}
