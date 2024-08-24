fn main() {
    cc::Build::new()
        .file("../cpp/main.cpp")
        .compiler("clang")
        .opt_level(3)
        .static_flag(true)
        .cpp(true)
        .compile("cpplib");
}
