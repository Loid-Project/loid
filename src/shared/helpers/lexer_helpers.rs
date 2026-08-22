use crate::shared::helpers::look_up_aliased_token::lookup_token_alias;
use crate::shared::types::syntax::keyword::Keyword;

pub fn is_letter(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

pub fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

pub fn is_white_space(c: char) -> bool {
    matches!(c, ' ' | '\n' | '\t' | '\r')
}

pub fn is_operator_start(c: char) -> bool {
    matches!(
        c,
        '+' | '-' | '*' | '/' | '%' | '&' | '=' | '!' | '<' | '>' | '|' | '#' | '@' | '\'
    )
}

pub fn is_keyword(s: &str) -> bool {
    Keyword::from_str(s).is_some()
}

pub fn is_alias_declaration(s: &str) -> bool {
    lookup_token_alias(s).is_some()
}
