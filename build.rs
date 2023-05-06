fn main() {
    // build the bridge
    cxx_build::bridge("src/lib.rs")
        // .file("./src/BoxDynChop.h")
        .flag_if_supported("-std=c++11")
        .compile("td-rs");


    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/RustCHOP.cc");
}
