fn main() {
    // build the bridge
    cxx_build::bridge("src/cxx.rs")
        .include("./src")
        .flag_if_supported("-std=c++11")
        .compile("td-rs-chop");

    println!("cargo:rerun-if-changed=src/cxx.rs");
    println!("cargo:rerun-if-changed=src/RustCHOP.cpp");
}
