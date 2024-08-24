#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CppString {
    ptr: *const u8,
    len: usize,
}

impl CppString {
    unsafe fn from_str(text: &str) -> CppString {
        CppString {
            ptr: text.as_ptr(),
            len: text.len(),
        }
    }
    unsafe fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.ptr, self.len)) }
    }
}

extern "C" {
    fn without_first_char_basalt(data: CppString) -> CppString;
}

fn main() {
    let mut c = criterion::Criterion::default();

    for (name, input) in [
        ("ASCII-only", "abcde"),
        ("2 byte first char", "Ãbcde"),
        ("3 byte first char", "คbcde"),
        ("4 byte first char", "𒆣bcde"),
    ] {
        let mut group = c.benchmark_group(format!("without first char: {name}"));
        println!("in: {input}");
        group.bench_with_input("Rust", input, |b, input| {
            b.iter(|| verq::without_first_char(input))
        });
        println!("out: {}", verq::without_first_char(input));
        group.bench_with_input(
            "C++, Basalt",
            &unsafe { CppString::from_str(input) },
            |b, &input| b.iter(|| unsafe { without_first_char_basalt(input) }),
        );
        println!("out: {}", unsafe {
            without_first_char_basalt(CppString::from_str(input)).as_str()
        });
        group.finish()
    }
}
