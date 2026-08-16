pub enum Type {
    INT,
    TYPE,
    CHAR,
    ATOM,
    BIGINT,
    FLOAT,
    DOUBLE,
    UNTIL,
    BOOL,
    VOID,
    TUPLE,
    ENUM,
    STRUCT,
    QBIT,
    LAMBDA,
    ARRAY,
    CHILDREN,
    UniqType,
    ProxyClass,
}

impl Type {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            | "int" => Some(Type::INT),
            | "type" => Some(Type::TYPE),
            | "char" => Some(Type::CHAR),
            | "atom" => Some(Type::ATOM),
            | "bigint" => Some(Type::BIGINT),
            | "float" => Some(Type::FLOAT),
            | "double" => Some(Type::DOUBLE),
            | "until" => Some(Type::UNTIL),
            | "bool" => Some(Type::BOOL),
            | "void" => Some(Type::VOID),
            | "tuple" => Some(Type::TUPLE),
            | "enum" => Some(Type::ENUM),
            | "struct" => Some(Type::STRUCT),
            | "qbit" => Some(Type::QBIT),
            | "lambda" => Some(Type::LAMBDA),
            | "array" => Some(Type::ARRAY),
            | "children" => Some(Type::CHILDREN),
            | "uniq_type" => Some(Type::UniqType),
            | "proxy_class" => Some(Type::ProxyClass),
            | _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            | Type::INT => "int",
            | Type::TYPE => "type",
            | Type::CHAR => "char",
            | Type::ATOM => "atom",
            | Type::BIGINT => "bigint",
            | Type::FLOAT => "float",
            | Type::DOUBLE => "double",
            | Type::UNTIL => "until",
            | Type::BOOL => "bool",
            | Type::VOID => "void",
            | Type::TUPLE => "tuple",
            | Type::ENUM => "enum",
            | Type::STRUCT => "struct",
            | Type::QBIT => "qbit",
            | Type::LAMBDA => "lambda",
            | Type::ARRAY => "array",
            | Type::CHILDREN => "children",
            | Type::UniqType => "uniq_type",
            | Type::ProxyClass => "proxy_class",
        }
    }
}
