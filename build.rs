fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/squared.cpp")
        .std("c++14")
        .compile("squared");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/squared.cpp");
    println!("cargo:rerun-if-changed=include/squared.h");
}
