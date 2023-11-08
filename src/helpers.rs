pub(crate) fn is_digit(c: char) -> bool {
    '0' <= c && c <= '9'
}

pub(crate) fn is_alpha(c: char) -> bool {
    'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || c == '_'
}