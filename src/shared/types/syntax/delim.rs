use crate::define_parsable_enum;

define_parsable_enum! {
    pub enum IllegalSymbol {
        // delimiters:
        LeftParentheses   => "(",
        RightParentheses  => ")",
        LeftBracket       => "[",
        RightBracket      => "]",
        LeftBrace         => "{",
        RightBrace        => "}",
    }
}
