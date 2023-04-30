fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src");
    let mut b = autocxx_build::Builder::new("src/cxx.rs", &[&path])
        .auto_allowlist(true)
        .build()?;
    b.flag_if_supported("-std=c++17")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-reorder-ctor")
        .flag("-Wno-mismatched-tags")
        .flag("-Wno-unused-private-field")
        .compile("td-rs-base");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/input.cc");
    println!("cargo:rerun-if-changed=src/input.h");
    Ok(())
}
