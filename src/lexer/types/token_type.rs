/*
This Enum must have the following values:
Kw[each keyword]

-one by one, consider making a macro specifically for this by passing in the string of all keywords by making a module for the list of these in the constants, maybe even construct the list with a macro.

    macros save lives people.
    for example: KwEnum

Ident

    identifiers

[type of literal]

    Say IntLiteral, FloatLiteral, StringLiteral, CharLiteral

[delim]

    delimiters like: LParen, RParen, LBrace, RBrace, Comma, Period, Semicolon, etc...

Op[operator name]

    self explanatory

Special:

    OEF, Illegal

Please use macros where possible to limit the length of the code.
A lot of this is Macroable, especially if one makes lists of stuff in the constants.
 */

pub enum TokenType {
    Keyword,
    Identifier,
    Decorator,

    IntLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BoolLiteral,

    Operator,
    Delim,
    Newline,
    Special,
    Neg,
}

/*
use crate::define_parsable_enum;

define_parsable_enum! {
    pub enum Decorator {
        Lazy    => "@lazy",
        Cached  => "@cached",
        Pure    => "@pure",
        Total   => "@total",
    }
}

*/
