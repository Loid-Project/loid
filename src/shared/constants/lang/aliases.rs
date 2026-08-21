use phf;
use phf_macros::phf_map;

use crate::define_aliases;

use crate::shared::types::syntax::{
    builtin_function::BuiltinFunction, keyword::Keyword, operator::Operator,
};

#[allow(dead_code)]
pub enum AliasableTokenKind {
    Keyword(Keyword),
    Operator(Operator),
    BuiltinFunction(BuiltinFunction),
}

// creates LANG_ALIASES
define_aliases!(AliasableTokenKind,
    "λ" => Keyword::Lambda,
    "¬" => BuiltinFunction::Not,
    "∨" => Operator::Or,
    "∧" => Operator::And,
    "⊕" => Operator::Xor,
);
