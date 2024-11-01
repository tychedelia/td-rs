# `td-rs` 🎨👩‍💻 ![example workflow](https://github.com/tychedelia/td-rs/actions/workflows/build.yaml/badge.svg)

Experiments integrating Rust into TouchDesigner's plugin framework.

## Version compatibility 

The library is currently intended to be used with TouchDesigner version `2023.12000`.

## Getting started

Fork and clone this repository. Plugins are built using the build system described below. New plugins
can be added by creating a new directory in `plugins/` and adding it to root `Cargo.toml` as a workspace
member.

A plugin's `Cargo.toml` should have the following properties:
- `name` - The name of the plugin. This should be unique across all plugins.
- `lib` - The type of crate. This should be set to `staticlib`. The name of the lib should be the same as the
  `package.name` property, but with underscores instead of hyphens.
- `package.metadata.td-rs` - The `type` should be set to the operator type, e.g. `top`, `chop`, `sop`, `dat`.
  This is used by the build system to generate the correct C++ code.
- A dependency on the parent chop crate, e.g. `td-rs-chop = { path = "../../../td-rs-chop" }`. 

All plugins must call their plugin constructor macro in their `lib.rs` file. For example, a `chop` plugin
would call `chop_plugin!(PluginName)`. This macro will generate the necessary FFI code to register the plugin
with TouchDesigner.

See example plugins for reference. A good starting point can be just to copy an existing plugin.

### Features

The following features are available for all parent operator dependencies:
- `python` - Enable Python support. This can be used in combination with `td-rs-derive-py` to generate 
  Python bindings for the plugin.
- `tracing` - Enable tracing support using the [`tracing`](https://crates.io/crates/tracing) crate. This
  can be used to log messages to the TouchDesigner console.
- `tokio` - Enable Tokio support. This can be used to spawn asynchronous tasks from the plugin from the shared
  Tokio runtime exported as `RUNTIME`.

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