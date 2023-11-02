# `td-rs` 🎨👩‍💻 ![example workflow](https://github.com/tychedelia/td-rs/actions/workflows/build.yaml/badge.svg)

Experiments integrating Rust into TouchDesigner's plugin framework.

## ⚠️ Status ⚠️

This project should be considered in **alpha** status. It is not yet ready for production use, however
is mostly stable and usable for experimentation. Please file an issue if you encounter any problems,
as it is our goal to make this project viable for production use.

In particular, users may experience any of the following:
- Crashes
- Memory leaks
- Missing APIs
- Performance issues
- Incomplete documentation
- Breaking changes
- Violations of Rust's aliasing rules leading to [scary things](https://predr.ag/blog/falsehoods-programmers-believe-about-undefined-behavior/)

In other words, no warranty is provided, express or implied.

## Structure

Using `autocxx` we generate a C++ interface or "bridge" to our Rust library, which is then compiled
into a C++ plugin that can be loaded in TouchDesigner.

## Build

`cargo-xtask` is used for the build framework. A [`justfile`](./justfile) is also provided.

### `cargo-xtask`

- `cargo xtask build $PLUGIN` - Build the plugin for the current platform.
- `cargo xtask install $PLUGIN` - Install a built plugin to the TouchDesigner plugins directory.
- `cargo xtask list-plugins` - List all available plugins.

### Windows

#### Dependencies
- TouchDesigner, installed in the default location (`C:\Program Files\Derivative\TouchDesigner`).
- MSVC toolchain (Desktop C++, including Clang from additional components). Note: it may be necessary to set the
  `LIBCLANG_PATH` environment variable to the path of the Clang DLL. This can be found in the Visual Studio
    installation directory, e.g. `C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Tools\Llvm\bin`.
- Rust `x86_64-pc-windows-msvc` target.

### macOS

Currently only supports aarch64 (Apple Silicon) builds. Submit a PR if you'd like to add support for x86_64.

#### Dependencies
- TouchDesigner, installed in the default location (`/Applications/TouchDesigner.app`).
- Xcode (installable from App Store).

---
TouchDesigner is a registered trademark of Derivative Inc. This project is not affiliated with or endorsed 
by Derivative Inc. 