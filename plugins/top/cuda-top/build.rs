extern crate cc;

use std::{env, path::PathBuf, process::Command};

fn main() {
    if cfg!(windows) {
        let mut cuda_target = cc::Build::new();

        cuda_target
            .cuda(true)
            .cudart("shared");

        cuda_target
            .flag("-arch=native");

        cuda_target
            .flag("-I../../../td-rs-base/src/");

        if env::var_os("CXX").is_none() {
            if let Ok(cuda_path) = env::var("CUDA_PATH") {
                // Look for the g++ that CUDA wants.
                let compiler = std::path::PathBuf::from(cuda_path).join("bin/g++");
                if compiler.exists() {
                    println!("cargo:warning=Setting $CXX to {}", compiler.display());
                    env::set_var("CXX", compiler.into_os_string());
                }
            }
        }

        cuda_target.define(
            // The DEBUG env. variable is set by cargo. If running "cargo build
            // --release", DEBUG is "false", otherwise "true". C/C++/CUDA like
            // the compile option "NDEBUG" to be defined when using assert.h, so
            // if appropriate, define that here. We also define "DEBUG" so that
            // can be used.
            match env::var("DEBUG").as_deref() {
                Ok("false") => "NDEBUG",
                _ => "DEBUG",
            },
            None,
        );

        // If we're told to, use single-precision floats. The default in the GPU
        // code is to use double-precision.
        #[cfg(feature = "gpu-single")]
        cuda_target.define("SINGLE", None);

        // Break in case of emergency.
        // cuda_target.debug(true);

        cuda_target.file("src/kernels/kernel.cu");
        cuda_target.compile("kernel");
    }
}