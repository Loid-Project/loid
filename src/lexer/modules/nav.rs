// this module is for functions for navigating and peeking

fn advance(s: &str, pos: usize) -> Option<char> {
    s.chars().nth(pos)
}

fn peek(s: &str, pos: usize) -> Option<char> {
    s.chars().nth(pos + 1)
}
