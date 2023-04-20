fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src");
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path])
        .auto_allowlist(true)
        .build()?;
    b.flag_if_supported("-std=c++17")
        .compile("td-rs-base");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/input.cc");
    println!("cargo:rerun-if-changed=src/input.h");
    Ok(())
}
