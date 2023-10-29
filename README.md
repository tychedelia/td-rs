# `td-rs`

Experiments integrating Rust into TouchDesigner's plugin framework.

## Status

This is a work in progress that is capable of building plugins, but is not fully ready for production use.
The current implementation is not yet optimized for performance or safety and may have missing APIs.

## Structure

Using `autocxx` we generate a C++ interface or "bridge" to our Rust library, which is then compiled
into a C++ plugin that can be loaded in TouchDesigner.

## Build

### `cargo-xtask`

Run `cargo xtask build` to build the project. This will build the Rust library and
generate the C++ glue code, and then build the C++ plugin. The resulting plugin
will be placed in `./target/` and can be loaded in TouchDesigner.


Currently, one command `build` is supported which takes the plugin (cargo package) name as its
first argument. For example, to build the [`filter-chop`](./plugins/chop/filter) plugin:
```shell
cargo xtask build filter-chop
```

### Windows

#### Dependencies
- TouchDesigner, installed in the default location (`C:\Program Files\Derivative\TouchDesigner`).
- MSVC toolchain (Desktop C++, including Clang from additional components). Note: it may be necessary to set the
  `LIBCLANG_PATH` environment variable to the path of the Clang DLL. This can be found in the Visual Studio
    installation directory, e.g. `C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Tools\Llvm\bin`.
- Rust `x86_64-pc-windows-msvc` target.

### macOS

#### Dependencies
- TouchDesigner, installed in the default location (`/Applications/TouchDesigner.app`).
- Xcode (installable from App Store).

---
TouchDesigner is a registered trademark of Derivative Inc. This project is not affiliated with or endorsed 
by Derivative Inc. 