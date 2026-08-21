#[macro_export]
macro_rules! define_aliases {
    (
        $wrapper:ident,
        $( $str:literal => $enum:ident :: $variant:ident ),* $(,)?
    ) => {
        #[allow(dead_code)]
        pub static LANG_ALIASES: phf::Map<&'static str, $wrapper> = phf_map! {
            $(
                $str => $crate::define_aliases!(@wrap $wrapper $enum $variant),
            )*
        };
    };

    (@wrap $wrapper:ident Keyword $variant:ident) => {
        $wrapper::Keyword(Keyword::$variant)
    };
    (@wrap $wrapper:ident Operator $variant:ident) => {
        $wrapper::Operator(Operator::$variant)
    };
        (@wrap $wrapper:ident BuiltinFunction $variant:ident) => {
        $wrapper::BuiltinFunction(BuiltinFunction::$variant)
    };
}
