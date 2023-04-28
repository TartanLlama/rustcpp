extern crate bindgen;

use std::env;
use std::path::PathBuf;
use cmake;



fn main() {
    // Tell cargo to invalidate the built crate whenever the header changes
    println!("cargo:rerun-if-changed=../cpp/cats.hpp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg("-std=c++20")
        .clang_arg("-x")
        .clang_arg("c++")
		.allowlist_type("cat")
        .generate_inline_functions(true)
		.opaque_type("std::.*")
        // The input header we would like to generate
        // bindings for.
        .header("../cpp/cats.hpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Builds the project in the directory located in `../cpp`, installing it
    // into $OUT_DIR
    let dst = cmake::Config::
            new("../cpp")
            .generator("Visual Studio 17 2022")
            .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=cats");
}
