pub fn build(output: &str, include_base: bool) -> miette::Result<()> {
    let python_enabled = std::env::var("CARGO_FEATURE_PYTHON").is_ok();

    let path = std::path::PathBuf::from("src");
    let mut incs = vec![path];
    let mut clang_args = vec![];

    if include_base {
        let base_path = std::path::PathBuf::from("../td-rs-base/src");
        incs.push(base_path);
    }

    println!("python_enabled: {}", python_enabled);

    if python_enabled {
        if cfg!(windows) {
            incs.push(std::path::PathBuf::from(
                "C:\\Program Files\\Derivative\\TouchDesigner\\Samples\\CPlusPlus\\3rdParty\\Python\\Include"
            ));
            incs.push(std::path::PathBuf::from(
                "C:\\Program Files\\Derivative\\TouchDesigner\\Samples\\CPlusPlus\\3rdParty\\Python\\Include\\PC"
            ));
        } else {
            incs.push(std::path::PathBuf::from(
                "/Applications/TouchDesigner.app/Contents/Frameworks/Python.framework/Headers",
            ));
        };
        clang_args.push("-DPYTHON_ENABLED");
    }

    if cfg!(windows) {
        clang_args.push("-std=c++17");
        clang_args.push("-D_CRT_USE_BUILTIN_OFFSETOF");
    }

    let b = autocxx_build::Builder::new("src/cxx.rs", &incs)
        .extra_clang_args(&clang_args)
        .auto_allowlist(true);

    let mut b = b.build()?;
    if python_enabled {
        b.define("PYTHON_ENABLED", None);
    }

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
