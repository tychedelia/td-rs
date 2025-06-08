# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**td-rs** is an experimental Rust framework for creating TouchDesigner plugins. It provides Rust bindings for TouchDesigner's C++ plugin API, enabling development of CHOPs, TOPs, SOPs, and DATs in Rust instead of C++.

## Architecture

### Workspace Structure
- **`td-rs-base/`** - Core traits, types, and shared functionality for all plugin types
- **`td-rs-chop/`**, **`td-rs-dat/`**, **`td-rs-sop/`**, **`td-rs-top/`** - Operator-specific frameworks
- **`td-rs-derive/`** - Procedural macros for parameter generation  
- **`td-rs-xtask/`** - Build system and plugin management tools
- **`plugins/`** - Example plugins organized by operator type

### Plugin Development Pattern
Each plugin follows this structure:
1. Define parameter struct with `#[derive(Params)]`
2. Implement required traits: `OpNew`, `OpInfo`, `Op`, and operator-specific trait (`Chop`, `Top`, etc.)
3. Use `chop_plugin!()`, `top_plugin!()`, etc. macro to register
4. Configure `Cargo.toml` with `crate-type = ["staticlib"]` and `package.metadata.td-rs.type`

### Key Traits
- **`OpInfo`** - Plugin metadata (name, version, inputs/outputs)
- **`OpNew`** - Constructor 
- **`Op`** - Base functionality (parameters, pulse handling)
- **`Chop`/`Top`/`Sop`/`Dat`** - Operator-specific execution logic

## Development Commands

Use `just` (justfile) for all build operations:

```bash
# Build a specific plugin
just build <plugin-name>

# Install plugin to TouchDesigner plugins directory
just install <plugin-name>

# Watch mode development (requires bacon)
just dev <plugin-name>

# List all available plugins
just list-plugins
```

### Plugin locations:
- **Windows**: `$HOME\OneDrive\Documents\Derivative\Plugins\`
- **macOS**: `$HOME/Library/Application Support/Derivative/TouchDesigner099/Plugins`

## Testing

No centralized test suite - each plugin can have individual tests. Use standard `cargo test` in plugin directories.

## Platform Requirements

### Windows
- MSVC toolchain with Clang support
- May require setting `LIBCLANG_PATH` environment variable
- Target: `x86_64-pc-windows-msvc`

### macOS  
- Xcode with command line tools
- Target: `aarch64-apple-darwin` (Apple Silicon)

## Current Development Status - Bevy TOP Plugin

### 🚧 Active Issues (December 2024)

**PRIMARY FOCUS**: Debugging bevy-top plugin CUDA-Vulkan-Bevy interop pipeline

**CRITICAL FIXES COMPLETED** ✅:
1. **Format Pipeline Overhaul**: Fixed sRGB vs linear format mismatches
   - `BGRA8Fixed` → `Bgra8Unorm` (linear) for base textures  
   - `Bgra8UnormSrgb` views for Bevy camera compatibility
   - Dynamic format detection instead of hardcoded `Bgra8UnormSrgb`

2. **API Call Ordering**: Fixed "input before output" crash
   - Get input CUDA arrays BEFORE `beginCUDAOperations()` (matches C++ sample)
   - Proper CUDA array validation timing

3. **Bytes-per-pixel Calculations**: Fixed hardcoded assumptions
   - `get_bytes_per_pixel()` function for all TouchDesigner formats
   - Dynamic width calculations instead of `width * 4`

**CURRENT DETECTIVE WORK** 🕵️‍♀️:
4. **Row Pitch Alignment Issues**: `cudaErrorInvalidPitchValue`
   - **Problem**: Vulkan external memory row pitch ≠ CUDA alignment requirements  
   - **Root Cause**: GPU drivers align texture rows (256/512-byte boundaries)
   - **Solution**: Query actual Vulkan row pitch via `get_image_subresource_layout()`
   - **Status**: Mega debug logging added, testing alignment fixes

5. **Pending Investigation**: Segfault when input connected before output
   - May be related to row pitch/external memory lifecycle issues

### 🧬 Technical Deep Dive - CUDA-Vulkan Pipeline

**Architecture**: TouchDesigner → CUDA → Vulkan External Memory → Bevy → Back to CUDA → TouchDesigner

**The Problem**: Hardware-level memory layout assumptions
- **TouchDesigner**: Provides CUDA arrays with unknown pitch
- **Vulkan**: Creates external memory with driver-aligned row pitch  
- **CUDA**: Strict alignment requirements for `cudaMemcpy2D` operations
- **Mismatch**: `width * bytes_per_pixel` ≠ actual driver row pitch

**Detective Evidence**:
```rust
// What we calculate: 512 * 4 = 2048 bytes
// What Vulkan actually uses: 2304 bytes (512-byte aligned)
// What CUDA needs: Aligned pitch values
```

**Current Fix Strategy**:
- Query real Vulkan row pitch via `vk::get_image_subresource_layout()`
- Align pitch to CUDA requirements (512-byte boundaries)
- Use actual pitch in `cudaMemcpy2DFromArray`/`cudaMemcpy2DToArray`

## Important Implementation Details

1. **FFI Safety**: Uses autocxx for C++ bridge generation and careful Pin<> usage
2. **Parameter System**: Derive macros generate TouchDesigner-compatible parameters automatically
3. **Memory Management**: Rust structs are managed via opaque pointers in C++ layer
4. **Optional Features**: `python` (PyO3 integration), `tracing` (logging), `tokio` (async support)
5. **Alpha Status**: Experimental - expect breaking changes and potential instability

## Working with Plugins

When creating new plugins:
1. Copy existing plugin structure from `/plugins/` examples
2. Update `Cargo.toml` metadata for operator type
3. Implement required traits following established patterns
4. Use parameter derive macros for TouchDesigner integration
5. Test with `just dev <plugin-name>` for rapid iteration