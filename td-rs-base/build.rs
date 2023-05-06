fn main() {
    // build the bridge
    cxx_build::bridge("src/cxx.rs")
        .include("./src")
        .flag_if_supported("-std=c++17")
        .compile("td-rs-base");

    println!("cargo:rerun-if-changed=src/parameter_manager/ParameterManager.h");
    println!("cargo:rerun-if-changed=src/parameter_manager/ParameterManager.cpp");
}
