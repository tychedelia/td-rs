fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src");
    let base_path = std::path::PathBuf::from("../td-rs-base-autocxx/src");
    let mut b = autocxx_build::Builder::new("src/cxx.rs", &[&path, &base_path])
        .auto_allowlist(true)
        .build()?;
    b.flag_if_supported("-std=c++17")
        // .file("src/input.cc")
        .compile("td-rs-chop");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/input.cc");
    println!("cargo:rerun-if-changed=src/input.h");
    Ok(())
}
