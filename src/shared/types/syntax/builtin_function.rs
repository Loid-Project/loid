use crate::define_parsable_enum;

define_parsable_enum! {
    pub enum BuiltinFunction {
        RuntimeCheck => "runtime_check",
        Overload     => "overload",
        Not          => "not",
    }
}
