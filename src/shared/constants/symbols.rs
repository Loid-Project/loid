pub enum Symbols {
    Bar,
    LeftParentheses,
    RightParentheses,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    Dot,
    AtSign,          // just write as at?
    DashArrow,       // ->
    EqualArrow,      // =>
    Question,        // ?
    CharDeclaration, // #\
    Backslash,
    Underscore,
    Curry, // |>
}

impl Symbols {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            | "|" => Some(Symbols::Bar),
            | "(" => Some(Symbols::LeftParentheses),
            | ")" => Some(Symbols::RightParentheses),
            | "{" => Some(Symbols::LeftBrace),
            | "}" => Some(Symbols::RightBrace),
            | "[" => Some(Symbols::LeftBracket),
            | "]" => Some(Symbols::RightBracket),
            | "," => Some(Symbols::Comma),
            | ";" => Some(Symbols::Semicolon),
            | ":" => Some(Symbols::Colon),
            | "." => Some(Symbols::Dot),
            | "@" => Some(Symbols::AtSign),
            | "->" => Some(Symbols::DashArrow),
            | "=>" => Some(Symbols::EqualArrow),
            | "?" => Some(Symbols::Question),
            | "#\\" => Some(Symbols::CharDeclaration),
            | "\\" => Some(Symbols::Backslash),
            | "_" => Some(Symbols::Underscore),
            | "|>" => Some(Symbols::Curry),
            | _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            | Symbols::Bar => "|",
            | Symbols::LeftParentheses => "(",
            | Symbols::RightParentheses => ")",
            | Symbols::LeftBrace => "{",
            | Symbols::RightBrace => "}",
            | Symbols::LeftBracket => "[",
            | Symbols::RightBracket => "]",
            | Symbols::Comma => ",",
            | Symbols::Semicolon => ";",
            | Symbols::Colon => ":",
            | Symbols::Dot => ".",
            | Symbols::AtSign => "@",
            | Symbols::DashArrow => "->",
            | Symbols::EqualArrow => "=>",
            | Symbols::Question => "?",
            | Symbols::CharDeclaration => "#\\",
            | Symbols::Backslash => "\\",
            | Symbols::Underscore => "_",
            | Symbols::Curry => "|>",
        }
    }
}
