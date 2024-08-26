use std::time::Duration;

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
    fn without_first_char_mnem(data: CppString) -> CppString;
    fn without_first_char_vxppy(data: CppString) -> CppString;
}

fn main() {
    let mut c = criterion::Criterion::default()
        .warm_up_time(Duration::from_millis(50))
        .measurement_time(Duration::from_millis(1000));

    let mut group = c.benchmark_group("without first char");
    for (name, input) in [
        ("ASCII-only", "abcde"),
        ("2 byte first char", "Ãbcde"),
        ("3 byte first char", "คbcde"),
        ("4 byte first char", "𒆣bcde"),
    ] {
        println!("in: {input}");
        group.bench_with_input(format!("{name}, Rust"), input, |b, input| {
            b.iter(|| verq::without_first_char(input))
        });
        println!("out: {}", verq::without_first_char(input));
        group.bench_with_input(
            format!("{name}, C++, Basalt"),
            &unsafe { CppString::from_str(input) },
            |b, &input| b.iter(|| unsafe { without_first_char_basalt(input) }),
        );
        println!("out: {}", unsafe {
            without_first_char_basalt(CppString::from_str(input)).as_str()
        });
        group.bench_with_input(
            format!("{name}, C++, mnem"),
            &unsafe { CppString::from_str(input) },
            |b, &input| b.iter(|| unsafe { without_first_char_mnem(input) }),
        );
        println!("out: {}", unsafe {
            without_first_char_mnem(CppString::from_str(input)).as_str()
        });
        group.bench_with_input(
            format!("{name}, C++, vxppy"),
            &unsafe { CppString::from_str(input) },
            |b, &input| b.iter(|| unsafe { without_first_char_vxppy(input) }),
        );
        println!("out: {}", unsafe {
            without_first_char_vxppy(CppString::from_str(input)).as_str()
        });
    }
    group.finish()
}
