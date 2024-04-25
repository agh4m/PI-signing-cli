fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("lib/library.cpp")
        .std("c++20")
        .compile("lib");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=.env");
    println!("cargo:rerun-if-changed=lib/library.cpp");
    println!("cargo:rerun-if-changed=lib/library.h");
    println!("cargo:rustc-link-lib=pteidlib");
}
