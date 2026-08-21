use crate::define_parsable_enum;

/*  * Style Guide:
 * All enum values are CamelCase
 * Structural Types start with an extra S
 * Built in types start with an extra B
*/

define_parsable_enum! {
    pub enum BuiltinType {
        // primitives:
        Atom    => "atom",
        Int     => "int",
        Type    => "type",
        Char    => "char",
        BigInt  => "bigint",
        Float   => "float",
        Double  => "double",
        Bool    => "bool",
        Void    => "void",
        Qbit    => "qbit",

        // structural:
        STuple          => "tuple",
        SEnum           => "enum",
        SStruct         => "struct",
        SArray          => "array",
        SChildren       => "children",
        SUniqType       => "uniq_type",
        SProxyClass     => "proxy_class",
        SAbstractClass  => "abstract_class",
        SConcreteClass  => "concrete_class",
        SNamespace      => "namespace",

        // these will be in std.types
        // but they will be automatically resolved into every file
        // unless stated otherwise in the config
        // builtin:
        BResult  => "result",
        BOption  => "option",
        BEither  => "either",
    }
}
