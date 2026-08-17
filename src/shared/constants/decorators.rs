pub enum Decorator {
    LAZY,
    CACHED,
    PURE,
}

impl Decorator {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            | "lazy" => Some(Decorator::LAZY),
            | "cached" => Some(Decorator::CACHED),
            | "pure" => Some(Decorator::PURE),
            | _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            | Decorator::LAZY => "lazy",
            | Decorator::CACHED => "cached",
            | Decorator::PURE => "pure",
        }
    }
}
