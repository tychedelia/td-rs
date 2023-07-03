fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src");
    let mut b = autocxx_build::Builder::new("src/cxx.rs", &[&path])
        .auto_allowlist(true);

    if cfg!(windows) {
        b = b.extra_clang_args(&["-std=c++17", "-D_CRT_USE_BUILTIN_OFFSETOF"]);
    }

    let mut b = b.build()?;
    b.flag_if_supported("-std=c++17");

    if !cfg!(windows) {
        b.flag("-Wno-unused-parameter")
            .flag("-Wno-reorder-ctor")
            .flag("-Wno-mismatched-tags")
            .flag("-Wno-unused-private-field");
    }

    b.compile("td-rs-base");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/input.cc");
    println!("cargo:rerun-if-changed=src/input.h");
    Ok(())
}
