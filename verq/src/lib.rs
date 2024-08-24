#[inline(never)]
pub fn without_first_char(text: &str) -> &str {
    match text.len() {
        0 => text,
        1 => &text[1..],
        2.. if text.is_char_boundary(1) => &text[1..],
        2 => &text[2..],
        3.. if text.is_char_boundary(2) => &text[2..],
        3 => &text[3..],
        4.. if text.is_char_boundary(3) => &text[3..],
        4.. => &text[4..],
    }
}
