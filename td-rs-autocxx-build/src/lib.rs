pub fn build(output: &str, include_base: bool) -> miette::Result<()> {
    let python_enabled = std::env::var("CARGO_FEATURE_PYTHON").is_ok();

    let path = std::path::PathBuf::from("src");
    let mut incs = vec![path];
    let mut clang_args = vec![];

    if include_base {
        let base_path = std::path::PathBuf::from("../td-rs-base/src");
        incs.push(base_path);
    }

    if python_enabled {
        let python_path = if cfg!(windows) {
            panic!()
        } else {
            std::path::PathBuf::from(
                "/Applications/TouchDesigner.app/Contents/Frameworks/Python.framework/Headers",
            )
        };
        incs.push(python_path);
        clang_args.push("-DPYTHON_ENABLED");
    }

    if cfg!(windows) {
        clang_args.push("-std=c++17");
        clang_args.push("-D_CRT_USE_BUILTIN_OFFSETOF");
    }

    let mut b = autocxx_build::Builder::new("src/cxx.rs", &incs)
        .extra_clang_args(&clang_args)
        .auto_allowlist(true);

    let mut b = b.build()?;
    b.flag_if_supported("-std=c++17");

    if !cfg!(windows) {
        b.flag("-Wno-unused-parameter")
            .flag("-Wno-reorder-ctor")
            .flag("-Wno-mismatched-tags")
            .flag("-Wno-unused-private-field");
    }

    b.compile(output);
    println!("cargo:rerun-if-changed=src/cxx.rs");
    Ok(())
}
