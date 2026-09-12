use crate::lexer::types::token::Token;

/*

implement a function called lex in the lexer module.

lex: string -> Token[].
the lexer iterates over each rune, maintaining:
- current position,
- line number,
- start position of the current token.
use a state‑machine approach (or a series of switch/if statements) to:
- skip whitespace.
- recognize identifiers/keywords (using isKeyword).
- recognize numbers (integers, floats).
- recognize strings and
- recognize operators (multi‑character operators like ==, !=, <=, >=, &&,
- recognize delimiters.

for each token, create a Token instance: assign the next ID, set the line, fill the type and literal, and set alias if applicable.
*/

pub fn lex(_s: &str) -> Vec<Token> {
    let tokens: Vec<Token> = Vec::new();

    tokens
}
