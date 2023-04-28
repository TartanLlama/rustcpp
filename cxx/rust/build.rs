fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/cats.cpp")
        .compile("cats");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/cats.cpp");
    println!("cargo:rerun-if-changed=include/cats.hpp");
}