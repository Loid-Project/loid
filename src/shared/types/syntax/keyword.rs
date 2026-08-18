use crate::define_parsable_enum;

/*  * Style Guide:
 * All enum values are CamelCase
*/

define_parsable_enum! {
    pub enum Keyword {

        Let => "let",
        Var => "var",
        Const => "const",
        Fn => "fn",
        Type => "type",
        Namespace => "namespace",

        // control flow:
        If => "if",
        Else => "else",
        While => "while",
        Until => "until",
        Cond => "cond",
        Match => "match",
        Breakable => "breakable",
        Return => "return",

        // Classes & Interfaces & Things:
        Class => "class",
        Abstract => "abstract",
        Interface => "interface",
        Proxy => "proxy",
        Typestated => "typestated",
        Impl => "impl",
        Inherits => "inherits",
        Derives => "derives",
        Implements => "implements",
        Extends => "extends",
        Override => "override",
        Overwrite => "overwrite",
        // modifiers
        Pub => "pub",
        Priv => "priv",
        Prot => "prot",
        Static => "static",


        Import => "import",
        Use => "use",


        // testing
        Test => "test",
        Stub => "stub",
        Mock => "mock",
        Fails => "fails",

        // Other:
        Lambda => "lambda",
        Inst => "inst",
        Underscore => "_",
        With => "with",
    }
}
