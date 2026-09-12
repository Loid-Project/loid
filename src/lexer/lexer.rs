use crate::lexer::types::token::Token;
use crate::shared::helpers::lexer_helper::{is_digit, is_white_space};

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

pub fn _lex(_s: &str) -> Vec<Token> {
    /*
    let mut cur_pos: i32 = 0;
    let mut cur_lin: i32 = 0;
    */

    let tokens: Vec<Token> = Vec::new();

    /*
    while cur_pos < s.len() as i32 {
        let cur_char: Option<char> = s.chars().nth((cur_pos as i32).try_into().unwrap());

        if is_white_space(cur_char) {
            if cur_char == '\n' {
                cur_line += 1;
            }

            cur_pos += 1;
            continue;
        }

        if cur_char == "\\".chars().next().unwrap()
            && cur_pos + 1 < s.len()
            && s[cur_pos + 1] == "\\"
        {
            while cur_pos < s.len() && s[cur_pos] != "\n" {
                cur_pos += 1;
            }
            continue;
        }

        if is_digit(cur_char) {}
    }
    */

    tokens
}
