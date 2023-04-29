fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src");
    let base_path = std::path::PathBuf::from("../td-rs-base/src");
    let mut b = autocxx_build::Builder::new("src/cxx.rs", &[&path, &base_path])
        .auto_allowlist(true)
        .build()?;
    b.flag_if_supported("-std=c++17")
        .compile("td-rs-chop");
    println!("cargo:rerun-if-changed=src/cxx.rs");
    println!("cargo:rerun-if-changed=src/RustChopPlugin.h");
    println!("cargo:rerun-if-changed=src/RustChopPlugin.cpp");
    Ok(())
}
