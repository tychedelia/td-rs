fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src");
    let base_path = std::path::PathBuf::from("../td-rs-base/src");
    let mut b = autocxx_build::Builder::new("src/cxx.rs", &[&path, &base_path])
        .auto_allowlist(true)
        .build()?;
    b.flag_if_supported("-std=c++17")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-reorder-ctor")
        .flag("-Wno-mismatched-tags")
        .flag("-Wno-unused-private-field")
        .compile("td-rs-top");
    println!("cargo:rerun-if-changed=src/cxx.rs");
    println!("cargo:rerun-if-changed=src/RustTopPlugin.h");
    println!("cargo:rerun-if-changed=src/RustTopPlugin.cpp");
    Ok(())
}
