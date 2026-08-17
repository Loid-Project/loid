use crate::define_parsable_enum;

define_parsable_enum! {
    pub enum BooleanLiteral {
        True => "true",
        False => "false",
    }
}
