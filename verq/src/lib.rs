#[inline(never)]
pub extern "C" fn without_first_char(text: &str) -> &str {
    const LUT: [u8; 16] = [
        1, 1, 1, 1, 1, 1, 1, 1, // 0x0*** => 1
        0, 0, 0, 0, // 0x10**, invalid(continuation byte)
        2, 2, // 0x110* => 2
        3, // 0x1110
        4, // 0x1111
    ];

    match text.len() {
        0 => text,
        1.. => {
            let lower = text.as_bytes()[0] >> 4;
            let start = LUT[lower as usize] as usize;
            unsafe { text.get_unchecked(start..) }
        }
    }
}
