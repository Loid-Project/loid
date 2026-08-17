use crate::define_parsable_enum;

define_parsable_enum! {
    pub enum IllegalSymbol {
        LeftParentheses => "(",
        RightParentheses => ")",
        LeftBracket => "[",
        RightBracket => "]",
        LeftBrace => "{",
        RightBrace => "}",
        Bar => "|",
        Comma => ",",
        Semicolon => ";",
        Colon => ":",
        Dot => ".",
        Backslash => "\\",
        AtSign => "@",
        Question => "?",
        Bang => "!",
        Plus => "+",
        Minus => "-",
        Star => "*",
        Caret => "^",
        Hashtag => "#",
        Ampersand => "&",
        Equals => "=",
        Quote => "\"",
        ForwardSlash => "/",
        LessThan => "<",
        MoreThan => ">",
        Backtick => "`",
        Tilde => "~",
    }
}
