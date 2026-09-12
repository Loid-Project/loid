use crate::lexer::types::token::Token;
use crate::lexer::types::token_type::TokenType;

use crate::shared::types::syntax::operator::Operator;
use crate::shared::types::syntax::string_escapes::StringEscape;

use crate::shared::helpers::lexer_helper::{is_digit, is_operator};
use crate::shared::helpers::uid_generator::next_id;

// This module is for functions for lexing each part

pub struct LexParams<'a> {
    pub s: &'a str,
    pub cur_pos: usize,
    pub cur_char: char,
    pub cur_line: u32,
}

impl<'a> LexParams<'a> {
    pub fn peek(&self) -> Option<char> {
        self.s[self.cur_pos..].chars().next()
    }

    pub fn advance(&mut self) -> Option<char> {
        let c: char = self.peek()?;
        self.cur_pos += 1;
        self.cur_char = c;
        Some(c)
    }
}

pub fn lex_int(l: &mut LexParams) -> Token {
    //TODO: change name from int to like any number thing

    let start_pos: usize = l.cur_pos.clone();

    while is_digit(l.cur_char) {
        // TODO: make sure it doesn't go out of bounds, and add negatives
        // decimal

        l.advance();

        if l.cur_char == '.' {
            lex_float(l, start_pos);
        }
    }
    Token {
        id: next_id(),
        line: l.cur_line,
        token_type: TokenType::IntLiteral,
        value: l.s.get(start_pos..l.cur_pos).unwrap().to_string(), // [start_pos, cur_pos)
    }
}

pub fn lex_float(l: &mut LexParams, start_pos: usize) -> Token {
    if let Some(var) = l.peek() {
        if !is_digit(var) {
            //error
        }
    }
    while is_digit(l.cur_char) {
        l.advance();
    }

    return Token {
        id: next_id(),
        line: l.cur_line,
        token_type: TokenType::FloatLiteral,
        value: l.s.get(start_pos..l.cur_pos).unwrap().to_string(),
    };
}

pub fn lex_operator(l: &mut LexParams) -> Token {
    let start_pos: usize = l.cur_pos;

    if l.cur_char == '-' {
        return Token {
            id: next_id(),
            line: l.cur_line,
            token_type: TokenType::Neg,
            value: "-".to_string(),
        };
    }

    while is_operator(l.cur_char) {
        l.advance();
    }

    let a: String = l.s.get(start_pos..l.cur_pos).unwrap().to_string();

    if let Some(var) = Operator::from_str(&a) {
        //actually return the token
        Token {
            id: next_id(),
            line: l.cur_line,
            token_type: TokenType::Operator,
            value: var.as_str().to_string(),
        }
    } else {
        // error
        Token {
            id: 6969,
            line: 6767,
            token_type: TokenType::Decorator,
            value: "Mr. Poopybutthole".to_string(),
        }
    }
}

pub fn lex_single_line_string(l: &mut LexParams) -> Token {
    l.advance(); // Look here... may have to remove this depending on how we call it. (i.e when we call do we begin with " or like the actual string)
    // TODO: handle out of bounds?

    let mut cur_char: char = l.s.chars().nth(l.cur_pos).unwrap();
    let mut buffer: String = String::new();

    while cur_char != '"' {
        if cur_char == '\\' {
            // escape sequence start
            let escape: String = format!("{}{}", cur_char, l.peek().unwrap());
            // TODO: ummm what if the escape is like fake
            // TODO: handle F strings
            // ToDo: :3 uWu owo T_T UwU -_- :o) ( -_•)ᡕᠵデᡁ᠊╾━💥 ▄︻デ══━一💥▄︻デ══━一💥 ඞාඞාඞා ( ˘ ³˘(◡‿◡˶)
            buffer.push('😿' /* escape raw so \" -> " */); // :3
            l.cur_pos += 2;

            cur_char = l.s.chars().nth(l.cur_pos).unwrap();

            continue;

            // ඞ
            // ඞ
            // ඞ
            // ඞ
            // ඞ
            /*
                			        ⠀⣠⣤⣤⣤⣤⣤⣶⣦⣤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀
            ⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⡿⠛⠉⠙⠛⠛⠛⠛⠻⢿⣿⣷⣤⡀⠀⠀⠀⠀⠀
            ⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⠋⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⠈⢻⣿⣿⡄⠀⠀⠀⠀
            ⠀⠀⠀⠀⠀⠀⠀⣸⣿⡏⠀⠀⠀⣠⣶⣾⣿⣿⣿⠿⠿⠿⢿⣿⣿⣿⣄⠀⠀⠀
            ⠀⠀⠀⠀⠀⠀⠀⣿⣿⠁⠀⠀⢰⣿⣿⣯⠁⠀⠀⠀⠀⠀⠀⠀⠈⠙⢿⣷⡄⠀
            ⠀⠀⣀⣤⣴⣶⣶⣿⡟⠀⠀⠀⢸⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣷⠀  we aren't done yet...
            ⠀⢰⣿⡟⠋⠉⣹⣿⡇⠀⠀⠀⠘⣿⣿⣿⣿⣷⣦⣤⣤⣤⣶⣶⣶⣶⣿⣿⣿⠀
            ⠀⢸⣿⡇⠀⠀⣿⣿⡇⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠃⠀
            ⠀⣸⣿⡇⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠉⠻⠿⣿⣿⣿⣿⡿⠿⠿⠛⢻⣿⡇⠀⠀
            ⠀⣿⣿⠁⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣧⠀⠀
            ⠀⣿⣿⠀⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⠀⠀
            ⠀⣿⣿⠀⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⠀⠀
            ⠀⢿⣿⡆⠀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⡇⠀⠀
            ⠀⠸⣿⣧⡀⠀⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠃⠀⠀
            ⠀⠀⠛⢿⣿⣿⣿⣿⣇⠀⠀⠀⠀⠀⣰⣿⣿⣷⣶⣶⣶⣶⠶⠀⢠⣿⣿⠀⠀⠀
            ⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⣿⣿⡇⠀⣽⣿⡏⠁⠀⠀⢸⣿⡇⠀⠀⠀
            ⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⣿⣿⡇⠀⢹⣿⡆⠀⠀⠀⣸⣿⠇⠀⠀⠀
            ⠀⠀⠀⠀⠀⠀⠀⢿⣿⣦⣄⣀⣠⣴⣿⣿⠁⠀⠈⠻⣿⣿⣿⣿⡿⠏⠀⠀⠀⠀
            ⠀⠀⠀⠀⠀⠀⠀⠈⠛⠻⠿⠿⠿⠿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
             */
        }

        buffer.push(cur_char);
        l.cur_pos += 1;
        cur_char = l.s.chars().nth(l.cur_pos).unwrap();
    }

    l.advance();

    Token {
        id: next_id(),
        line: l.cur_line,
        token_type: TokenType::StringLiteral,
        value: buffer,
    }
}

pub fn lex_bool(l: LexParams) -> Token {
    Token {
        id: next_id(),
        line: l.cur_line,
        token_type: TokenType::BoolLiteral,
        value: if l.s[l.cur_pos..].starts_with("true ") {
            true.to_string()
        } else {
            false.to_string()
        },
    }
}
