use crate::define_parsable_enum;

define_parsable_enum! {
    pub enum IllegalSymbol {

        // delimiters:
        LeftParentheses => "(",
        RightParentheses => ")",
        LeftBracket => "[",
        RightBracket => "]",
        LeftBrace => "{",
        RightBrace => "}",

        // arithmetic:
        Plus => "+",
        Minus => "-",
        Star => "*",
        ForwardSlash => "/",
        Percent => "%",

        // comparison:
        LessThan => "<",
        MoreThan => ">",
        Equals => "=",

        // Other:
        Caret => "^",
        Hashtag => "#",
        Ampersand => "&",
        Quote => "\"",
        Bang => "!",
        Backtick => "`",
        Tilde => "~",
        Bar => "|",
        Comma => ",",
        Semicolon => ";",
        Colon => ":",
        Dot => ".",
        Backslash => "\\",
        AtSign => "@",
        Question => "?",
    }
}
