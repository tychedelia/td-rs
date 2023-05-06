fn main() {

    // build the bridge
    cxx_build::bridge("src/lib.rs")
        .file("./cpp/CHOP_CPlusPlusBase.h")
        .file("./cpp/CPlusPlus_Common.h")
        .flag_if_supported("-std=c++11")
        .compile("td-rs");
}