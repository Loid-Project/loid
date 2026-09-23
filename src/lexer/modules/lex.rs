use crate::lexer::types::token::Token;
use crate::lexer::types::token_type::TokenType;

use crate::shared::constants::lang::aliases::AliasableTokenKind;
use crate::shared::types::syntax::boolean_literal::BooleanLiteral;
use crate::shared::types::syntax::decorator::Decorator;
use crate::shared::types::syntax::keyword::Keyword;
use crate::shared::types::syntax::operator::Operator;

use crate::shared::helpers::lexer_helper::{is_digit, is_letter};
use crate::shared::helpers::uid_generator::next_id;

// This module is for functions for lexing each part

pub struct LexParams<'a> {
    pub src: &'a str,
    pub chars: Vec<char>,
    pub pos: usize,
    pub line: u32,
    pub col: u32,
}

impl<'a> LexParams<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            chars: src.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn cur_char(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    pub fn peek2(&self) -> Option<char> {
        self.chars.get(self.pos + 2).copied()
    }

    pub fn advance(&mut self) -> Option<char> {
        let c: char = self.cur_char()?;

        self.pos += 1;

        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }

        Some(c)
    }

    pub fn bump_if(&mut self, c: char) -> bool {
        if self.cur_char() == Some(c) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn slice(&self, start: usize, end: usize) -> String {
        self.chars[start..end].iter().collect()
    }

    pub fn starts_with(&self, text: &str) -> bool {
        let n: usize = text.chars().count();

        if self.pos + n > self.chars.len() {
            return false;
        }

        self.chars[self.pos..self.pos + n]
            .iter()
            .copied()
            .eq(text.chars())
    }

    pub fn at_line_start(&self) -> bool {
        let mut i: usize = self.pos;

        while i > 0 {
            i -= 1;

            match self.chars[i] {
                | '\n' => return true,
                | c if c == ' ' || c == '\t' || c == '\r' => continue,
                | _ => return false,
            }
        }

        true
    }
}

fn make_token(token_type: TokenType, value: String, line: u32) -> Token {
    Token {
        id: next_id(),
        line,
        token_type,
        value,
    }
}

fn error_token(msg: impl Into<String>, line: u32) -> Token {
    make_token(TokenType::Special, msg.into(), line)
}

pub fn lex_number(l: &mut LexParams) -> Token {
    let start_pos: usize = l.pos;
    let start_line: u32 = l.line;

    let mut is_float: bool = false;

    while l.cur_char().is_some_and(is_digit) {
        l.advance();
    }

    if l.cur_char() == Some('.') && l.peek().is_some_and(is_digit) {
        is_float = true;
        l.advance();

        while l.cur_char().is_some_and(is_digit) {
            l.advance();
        }
    }

    make_token(
        if is_float {
            TokenType::FloatLiteral
        } else {
            TokenType::IntLiteral
        },
        l.slice(start_pos, l.pos),
        start_line,
    )
}

// lex_identifier handles both keyword and booleans because its easier that way i think
// please argue w me if it isn't
// make an issue on github larry or logan
// or some random guy i've never seen before
pub fn lex_identifier(l: &mut LexParams) -> Token {
    let start_pos: usize = l.pos;
    let start_line: u32 = l.line;

    while l
        .cur_char()
        .is_some_and(|c: char| is_letter(c) || is_digit(c))
    {
        l.advance();
    }

    let text: String = l.slice(start_pos, l.pos);

    if let Some(kword) = Keyword::from_str(&text) {
        return make_token(
            TokenType::Keyword,
            kword.as_str().to_string(),
            start_line,
        );
    }
    if let Some(boo) = BooleanLiteral::from_str(&text) {
        return make_token(
            TokenType::BoolLiteral,
            boo.as_str().to_string(),
            start_line,
        );
    }

    make_token(TokenType::Identifier, text, start_line)
}

pub fn lex_operator(l: &mut LexParams) -> Token {
    let start_line: u32 = l.line;

    for len in (1..=3).rev() {
        if l.pos + len > l.chars.len() {
            continue;
        }

        let candidate: String = l.chars[l.pos..l.pos + len].iter().collect();
        if let Some(op) = Operator::from_str(&candidate) {
            for _ in 0..len {
                l.advance();
            }
            return make_token(
                TokenType::Operator,
                op.as_str().to_string(),
                start_line,
            );
        }
    }

    let c: char = l.advance().unwrap_or('\0');
    error_token(format! {"unknown operator char: {c}"}, start_line)
}

pub fn lex_alias(l: &mut LexParams, alias: &AliasableTokenKind) -> Token {
    l.advance(); // all r one char long so far

    let (tt, value): (TokenType, String) = match alias {
        | AliasableTokenKind::Keyword(kword) => (TokenType::Keyword, kword.as_str().to_string()),
        | AliasableTokenKind::Operator(operator) => {
            (TokenType::Operator, operator.as_str().to_string())
        },
        | AliasableTokenKind::BuiltinFunction(builtin_f) => {
            (TokenType::Keyword, builtin_f.as_str().to_string())
        },
    };

    make_token(tt, value, l.line)
}

pub fn lex_single_line_string(l: &mut LexParams) -> Token {
    // Look here... may have to remove this depending on how we call it. (i.e when we call do we begin with " or like the actual string)
    // TODO: handle out of bounds?
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

    /*
    ⣿⣿⣿⣿⠟⠋⠄⠄⠄⠄⠄⠄⠄⢁⠈⢻⢿⣿⣿⣿⣿⣿⣿⣿
    ⣿⣿⣿⣿⠃⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠈⡀⠭⢿⣿⣿⣿⣿
    ⣿⣿⣿⡟⠄⢀⣾⣿⣿⣿⣷⣶⣿⣷⣶⣶⡆⠄⠄⠄⣿⣿⣿⣿
    ⣿⣿⣿⡇⢀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠄⠄⢸⣿⣿⣿⣿
    ⣿⣿⣿⣇⣼⣿⣿⠿⠶⠙⣿⡟⠡⣴⣿⣽⣿⣧⠄⢸⣿⣿⣿⣿
    ⣿⣿⣿⣿⣾⣿⣿⣟⣭⣾⣿⣷⣶⣶⣴⣶⣿⣿⢄⣿⣿⣿⣿⣿
    ⣿⣿⣿⣿⣿⣿⣿⡟⣩⣿⣿⣿⡏⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
    ⣿⣿⣿⣿⣿⣹⡋⠘⠷⣦⣀⣠⡶⠁⠈⠁⠄⣿⣿⣿⣿⣿⣿⣿
    ⣿⣿⣿⣿⣿⣍⠃⣴⣶⡔⠒⠄⣠⢀⠄⠄⠄⡨⣿⣿⣿⣿⣿⣿
    ⣿⣿⣿⣿⣿⣿⣦⡘⠿⣷⣿⠿⠟⠃⠄⠄⣠⡇⠈⠻⣿⣿⣿⣿
    ⣿⣿⣿⡿⠟⠋⢁⣷⣠⠄⠄⠄⠄⣀⣠⣾⡟⠄⠄⠄⠄⠉⠙⠻
    ⠟⠋⠁⠄⠄⠄⢸⣿⣿⡯⢓⣴⣾⣿⣿⡟⠄⠄⠄⠄⠄⠄⠄⠄
    ⠄⠄⠄⠄⠄⠄⣿⡟⣷⠄⠹⣿⣿⣿⡿⠁⠄⠄⠄⠄⠄⠄⠄⠄
     */

    let start_line: u32 = l.line;
    l.advance(); // "

    let mut buffer: String = String::new();

    loop {
        match l.cur_char() {
            | None => {
                return error_token(
                    format!("unterminated string: {buffer:?}"),
                    start_line,
                );
            },

            | Some('"') => {
                l.advance();
                return make_token(TokenType::StringLiteral, buffer, start_line);
            },

            | Some('\\') => {
                l.advance();
                match l.cur_char() {
                    | Some('n') => {
                        buffer.push('\n');
                        l.advance();
                    },
                    | Some('t') => {
                        buffer.push('\t');
                        l.advance();
                    },
                    | Some('"') => {
                        buffer.push('"');
                        l.advance();
                    },
                    | Some('\\') => {
                        buffer.push('\\');
                        l.advance();
                    },
                    | Some('(') => {
                        // parser can find `\(...\)`.
                        buffer.push('\\');
                        buffer.push('(');
                        l.advance();
                    },
                    | Some(c) => {
                        buffer.push('\\');
                        buffer.push(c);
                        l.advance();
                    },
                    | None => {
                        return error_token(
                            format!("unterminated string after backslash: {buffer:?}"),
                            start_line,
                        );
                    },
                }
            },

            | Some(c) => {
                buffer.push(c);
                l.advance();
            },
        }
    }
}

pub fn lex_fstring(l: &mut LexParams) -> Token {
    l.advance();

    lex_single_line_string(l)
}

pub fn lex_multiline_string(l: &mut LexParams) -> Token {
    let start_line: u32 = l.line;
    let mut buffer: String = String::new();
    let mut first_line: bool = true;

    loop {
        debug_assert_eq!(l.cur_char(), Some('>'));
        l.advance(); // consume '>'

        if !first_line {
            buffer.push('\n');
        }
        first_line = false;

        loop {
            match l.cur_char() {
                | None => {
                    return make_token(TokenType::StringLiteral, buffer, start_line);
                },

                // treat \r\n as \n because windows puts \r\n apparently??
                // might want to double check this i'm not sure
                | Some('\r') if l.peek() == Some('\n') => {
                    l.advance();
                },

                | Some('\n') => {
                    l.advance();
                    break;
                },

                | Some('\\') => {
                    l.advance();
                    match l.cur_char() {
                        | Some('n') => {
                            buffer.push('\n');
                            l.advance();
                        },
                        | Some('t') => {
                            buffer.push('\t');
                            l.advance();
                        },
                        | Some('"') => {
                            buffer.push('"');
                            l.advance();
                        },
                        | Some('\\') => {
                            buffer.push('\\');
                            l.advance();
                        },
                        | Some('(') => {
                            // default fstring
                            buffer.push('\\');
                            buffer.push('(');
                            l.advance();
                        },
                        | Some(c) => {
                            buffer.push('\\');
                            buffer.push(c);
                            l.advance();
                        },
                        | None => {
                            return make_token(TokenType::StringLiteral, buffer, start_line);
                        },
                    }
                },

                | Some(c) => {
                    buffer.push(c);
                    l.advance();
                },
            }
        }

        // doesn't consume trailing whitespace
        let mut look: usize = l.pos;
        while look < l.chars.len() {
            match l.chars[look] {
                | ' ' | '\t' | '\r' => look += 1,
                | _ => break,
            }
        }

        let continues: bool = look < l.chars.len() && l.chars[look] == '>';

        if !continues {
            break;
        }

        while l.pos < look {
            l.advance();
        }
    }

    make_token(TokenType::StringLiteral, buffer, start_line)
}

pub fn lex_char(l: &mut LexParams) -> Token {
    let start_line: u32 = l.line;

    l.advance();
    l.advance();

    match l.cur_char() {
        | Some(c) => {
            l.advance();
            make_token(TokenType::CharLiteral, c.to_string(), start_line)
        },
        | None => error_token("unterminated char literal", start_line),
    }
}

pub fn lex_decorator(l: &mut LexParams) -> Token {
    let start_pos: usize = l.pos;
    let start_line: u32 = l.line;

    l.advance();

    while l.cur_char().is_some_and(is_letter) {
        l.advance();
    }

    let text: String = l.slice(start_pos, l.pos);

    if let Some(dec) = Decorator::from_str(&text) {
        return make_token(
            TokenType::Decorator,
            dec.as_str().to_string(),
            start_line,
        );
    }

    error_token(format!("unknown decorator: {text}"), start_line)
}

pub fn lex_delim(l: &mut LexParams) -> Token {
    let start_line = l.line;

    make_token(
        TokenType::Delim,
        l.advance().unwrap_or('\0').to_string(),
        start_line,
    )
}

pub fn lex_newline(l: &mut LexParams) -> Token {
    let start_line: u32 = l.line;

    l.advance();

    make_token(TokenType::Newline, "\n".to_string(), start_line)
}

pub fn skip_line_comment(l: &mut LexParams) {
    while let Some(c) = l.cur_char() {
        if c == '\n' {
            break;
        }
        l.advance();
    }
}

pub fn skip_block_comment(l: &mut LexParams) {
    l.advance();
    l.advance();

    while let Some(c) = l.cur_char() {
        if c == '*' && l.peek() == Some('/') {
            l.advance();
            l.advance();
            return;
        }
        l.advance();
    }
}

pub fn lex_illegal(l: &mut LexParams) -> Token {
    let start_line: u32 = l.line;
    let c: char = l.advance().unwrap_or('\0');

    error_token(format!("illegal character: {c}"), start_line)
}
