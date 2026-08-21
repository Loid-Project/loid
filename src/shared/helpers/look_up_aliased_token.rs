use crate::shared::constants::lang::aliases::{self, AliasableTokenKind, LANG_ALIASES};

pub fn lookup_token_alias(s: &str) -> Option<&AliasableTokenKind> {
    LANG_ALIASES.get(s)
}
