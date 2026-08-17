#[macro_export]
macro_rules! define_parsable_enum {
    {
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident => $str:literal),* $(,)?
        }
    } => {
        $(#[$meta])*
        $vis enum $name {
            $($variant,)*
        }

        impl $name {
            $vis fn from_str(s: &str) -> Option<Self> {
                match s {
                    $($str => Some(Self::$variant),)*
                    _ => None,
                }
            }

            $vis fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $str,)*
                }
            }
        }
    }
}
