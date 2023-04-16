#![feature(fs_try_exists)]

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
use crate::macos::build_plugin;
#[cfg(target_os = "windows")]
use crate::windows::build_plugin;
use anyhow::Context;
use std::env;
use std::process::Command;

pub use anyhow::Result;
pub fn build(packages: &[String], args: &[String]) -> Result<()> {
    let package_args = packages.iter().flat_map(|package| ["-p", package]);

    let mut cmd = Command::new("cargo")
        .arg("build")
        .args(package_args)
        .args(args)
        .spawn()
        .with_context(|| format!("Could not call cargo to build {}", packages.join(", ")))?;
    let status = cmd.wait()?;
    if !status.success() {
        anyhow::bail!("Could not build {}", packages.join(", "));
    } else {
        Ok(())
    }
}

pub fn main() -> anyhow::Result<()> {
    let cmd = env::args()
        .nth(1)
        .with_context(|| "must provide command as first argument")?;
    let plugin = env::args()
        .nth(2)
        .with_context(|| "must provide plugin as second argument")?;
    if cmd != "build" {
        return Err(anyhow::anyhow!("command must be 'build'"));
    }
    build_plugin(&plugin)?;
    Ok(())
}
