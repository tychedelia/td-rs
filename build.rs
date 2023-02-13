fn main() {
    // build the bridge
    cxx_build::bridge("src/lib.rs")
        .file("./cpp/BoxDynChop.cpp")
        .flag_if_supported("-std=c++11")
        .compile("td-rs");
}
