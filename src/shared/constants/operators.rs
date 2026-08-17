pub enum Operator {
    PLUS,
    PLUS_PLUS,
    MINUS,
    MINUS_MINUS,
    STAR,
    SLASH,
    POWER,
    PERCENT,
    OR,
    AND,
    OR_MATH,  // same as or,  but with the unicode symbol
    AND_MATH, // same as and, but with the unicode symbol
    XOR_MATH,
    EQUALS,
    NOT_EQUALS,
    EQUAL_EQUAL,
    LESS,
    GREATER,
    LESS_EQUAL,
    GREATER_EQUAL,
    PLUS_EQUAL,
    MINUS_EQUAL,
    STAR_EQUAL,
    SLASH_EQUAL,
    PERCENT_EQUAL,
}

impl Operator {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            | "+" => Some(Operator::PLUS),
            | "++" => Some(Operator::PLUS_PLUS),
            | "-" => Some(Operator::MINUS),
            | "--" => Some(Operator::MINUS_MINUS),
            | "*" => Some(Operator::STAR),
            | "/" => Some(Operator::SLASH),
            | "**" => Some(Operator::POWER),
            | "%" => Some(Operator::PERCENT),
            | "||" => Some(Operator::OR),  // or "or"
            | "&&" => Some(Operator::AND), // or "and"
            | "∨" => Some(Operator::OR_MATH),
            | "∧" => Some(Operator::AND_MATH),
            | "⊕" => Some(Operator::XOR_MATH),
            | "=" => Some(Operator::EQUALS),
            | "==" => Some(Operator::EQUAL_EQUAL),
            | "!=" => Some(Operator::NOT_EQUALS),
            | "<" => Some(Operator::LESS),
            | ">" => Some(Operator::GREATER),
            | "<=" => Some(Operator::LESS_EQUAL),
            | ">=" => Some(Operator::GREATER_EQUAL),
            | "+=" => Some(Operator::PLUS_EQUAL),
            | "-=" => Some(Operator::MINUS_EQUAL),
            | "*=" => Some(Operator::STAR_EQUAL),
            | "/=" => Some(Operator::SLASH_EQUAL),
            | "%=" => Some(Operator::PERCENT_EQUAL),
            | _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            | Operator::PLUS => "+",
            | Operator::PLUS_PLUS => "++",
            | Operator::MINUS => "-",
            | Operator::MINUS_MINUS => "--",
            | Operator::STAR => "*",
            | Operator::SLASH => "/",
            | Operator::POWER => "**",
            | Operator::PERCENT => "%",
            | Operator::OR => "||",
            | Operator::AND => "&&",
            | Operator::OR_MATH => "∨",
            | Operator::AND_MATH => "∧",
            | Operator::XOR_MATH => "⊕",
            | Operator::EQUALS => "=",
            | Operator::EQUAL_EQUAL => "==",
            | Operator::NOT_EQUALS => "!=",
            | Operator::LESS => "<",
            | Operator::GREATER => ">",
            | Operator::LESS_EQUAL => "<=",
            | Operator::GREATER_EQUAL => ">=",
            | Operator::PLUS_EQUAL => "+=",
            | Operator::MINUS_EQUAL => "-=",
            | Operator::STAR_EQUAL => "*=",
            | Operator::SLASH_EQUAL => "/=",
            | Operator::PERCENT_EQUAL => "%=",
        }
    }
}
