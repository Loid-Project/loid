// module for string manipulation

pub fn words_with_newlines(s: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();
    let mut start: Option<usize> = None;

    for (i, chr) in s.char_indices() {
        if chr == '\n' || chr == '\r' || chr.is_whitespace() {
            if let Some(si) = start {
                tokens.push(&s[si..i]);
                start = None;
            }

            if chr == '\n' {
                tokens.push("\n");
            }
        } else if start.is_none() {
            start = Some(i);
        }
    }

    if let Some(si) = start {
        tokens.push(&s[si..]);
    }

    tokens
}

pub fn words(s: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();
    let mut start: Option<usize> = None;

    for (i, chr) in s.char_indices() {
        if chr == '\n' || chr == '\r' || chr.is_whitespace() {
            if let Some(si) = start {
                tokens.push(&s[si..i]);
                start = None;
            }
        } else if start.is_none() {
            start = Some(i);
        }
    }

    if let Some(si) = start {
        tokens.push(&s[si..]);
    }

    tokens
}
