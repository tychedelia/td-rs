# `td-rs`

Experiments integrating Rust into C++ TouchDesigner plugin, using the wonderful
[cxx-bridge](github.com/dtolnay/cxx-bridge) library. The goal is to be able to
implement TouchDesigner plugins using a safe, pure rust interface, requiring no
additional C++ code, beyond the glue bindings generated by CXX and called by our
wrapper plugin class.

## Structure

Using `cxx`, we codegen a ffi interface to our Rust
library. Each of our C repr functions exposed by `cxx` accepts [a wrapper](./src/BoxDynChop.h) 
around a `std::uintptr_t` pointer that contains the location to our Rust dyn trait object
representing the actual plugin. This wrapper manages calls across the ffi boundary,
and provides a "normal" C++ class interface to the methods exposed by our trait. The [C++
plugin class](td-rs-chop/src/RustCHOP.cpp) which is ultimately instantiated by TouchDesigner holds 
a reference to this wrapper.

Plugins can be written by implementing the [`Chop`](./src/chop/mod.rs) trait and overriding
trait methods as needed.

A number of structs are implemented via `cxx` to map TouchDesigner data classes
to structs that can be used by Rust. This currently introduces some performance overhead
at the FFI boundary that could likely be reduced in the future by eliminating copies
in favor of passing references to the underlying structs managed by TouchDesigner where
possible.

## Examples

- [`sin_chop`](./src/sin_chop.rs) - A basic CHOP generator that outputs a sin wave on a single channel.
  ![example of sin chop](./sin.png)
- [`lissa`](./src/lissa.rs) - A fancier version of an LFO that traces a Lissajous curve with
  modifiable parameters.
  ![example of lissa chop](./lissa.png)


## Build

Running the project may require some modification to the respective MSVC and Xcode projects.

### Windows

#### Dependencies
- MSVC toolchain.
- Rust `x86_64-pc-windows-msvc` target using rustup.

Update the project [Makefile](./Makefile) variable `MS_BUILD` to point to the correct `MSBuild.exe` for
your installed version of Visual Studio, or pass it as a variable to Make. This will produce a DLL to 
`.\Release\` that can be loaded in touch desginer.

### macOS

#### Dependencies
- Xcode (installable from App Store).

 Run make, which will preduce a `.plugin` file in `./build/` that can be loaded in TD. NB: the project
 is configured for M1 ARM, and modifications to the Xcode project are necessary to build for x86.
