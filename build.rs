fn main() {
    cxx_build::bridge("src/main.rs")
        .file("sig_lib/library.cpp")
        .std("c++20")
        .compile("sig_lib");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=sig_lib/library.cpp");
    println!("cargo:rerun-if-changed=sig_lib/library.h");
}
