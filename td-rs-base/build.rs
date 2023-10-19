fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src");
    let python_path = if cfg!(windows) {
        panic!()
    } else {
        std::path::PathBuf::from("/Applications/TouchDesigner.app/Contents/Frameworks/Python.framework/Headers")
    };
    let mut b = autocxx_build::Builder::new("src/cxx.rs", &[&path, &python_path]).auto_allowlist(true);

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
