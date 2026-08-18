use crate::define_parsable_enum;

define_parsable_enum! {
    pub enum Operator {
        Or => "∨",
        And => "∧",
        Xor => "⊕",
        Lambda => "λ",
    }
}
