# `td-rs`

Experiments integrating Rust into TouchDesigner using `cxx`.

## Status

This is a work in progress, and is not yet ready for production use. The current
implementation is a proof of concept, and is not yet optimized for performance.

## Structure

Using `autocxx` we generate a C++ interface or "bridge" to our Rust library, which is then compiled
into a C++ plugin that can be loaded in TouchDesigner.

## Build

### `cargo-xtask` (WIP)

Run `cargo xtask build` to build the project. This will build the Rust library and
generate the C++ glue code, and then build the C++ plugin. The resulting plugin
will be placed in `./target/` and can be loaded in TouchDesigner.

```shell
cargo xtask build filter-chop
```

Running the project may require some modification to the respective MSVC and Xcode projects.

### Windows

#### Dependencies
- MSVC toolchain (Desktop C++, including Clang from additional components). Tested on VS 2022.
- Rust `x86_64-pc-windows-msvc` target using rustup.

### macOS

#### Dependencies
- Xcode (installable from App Store).

 Run make, which will preduce a `.plugin` file in `./build/` that can be loaded in TD. NB: the project
 is configured for M1 ARM, and modifications to the Xcode project are necessary to build for x86.