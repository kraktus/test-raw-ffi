fn main() {
    cxx_build::bridge("src/lib.rs")  // returns a cc::Build
        .file("src/person.cc")
        .flag_if_supported("-std=c++11")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/demo.cc");
    println!("cargo:rerun-if-changed=include/demo.h");
}