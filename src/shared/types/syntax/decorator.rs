use crate::define_parsable_enum;

define_parsable_enum! {
    pub enum Decorator {
        Lazy    => "@lazy",
        Cached  => "@cached",
        Pure    => "@pure",
        Total   => "@total",
    }
}
