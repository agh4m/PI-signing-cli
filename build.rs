fn main() {
    cxx_build::bridge("src/main.rs")
        .file("sig_lib/library.cpp")
        .std("c++20")
        .compile("sig_lib")
}
