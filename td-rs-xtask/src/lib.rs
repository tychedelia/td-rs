mod config;
mod link;
#[cfg(target_os = "macos")]
mod macos;
mod metadata;
mod util;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
use crate::macos::{build_plugin, install_plugin};
#[cfg(target_os = "windows")]
use crate::windows::{build_plugin, install_plugin};
use anyhow::Context;
use std::env;

use std::process::Command;

pub use anyhow::Result;

const PLUGIN_HOME: &str = "target/plugin";

/// A plugin crate that lives outside this repository (`--path <dir>`).
///
/// td-rs is a framework; a plugin belongs with the project it serves, not
/// in this workspace. An external crate is a normal staticlib plugin with
/// its own `[workspace]` and td-rs as a path dependency. It is built with
/// `--manifest-path` into THIS repo's `target/`, so the Xcode and MSBuild
/// templates find the staticlib, td-rs-chop's and the cxx bridge exactly
/// where a workspace plugin would have put them.
static EXTERNAL: std::sync::OnceLock<Option<std::path::PathBuf>> = std::sync::OnceLock::new();

pub(crate) fn external_manifest_dir() -> Option<&'static std::path::Path> {
    EXTERNAL.get().and_then(|p| p.as_deref())
}

/// `cargo` arguments that point a command at the plugin's workspace: none
/// for a workspace member, manifest + target dir for an external crate.
pub(crate) fn cargo_workspace_args() -> Vec<String> {
    match external_manifest_dir() {
        Some(dir) => vec![
            "--manifest-path".into(),
            dir.join("Cargo.toml").display().to_string(),
            "--target-dir".into(),
            "target".into(),
        ],
        None => vec![],
    }
}

pub fn build(packages: &[&str], args: &[&str]) -> Result<()> {
    let package_args = packages.iter().flat_map(|package| ["-p", package]);
    let mut cmd = Command::new("cargo")
        .arg("build")
        .args(cargo_workspace_args())
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

    // `--path <dir>` anywhere after the command: the plugin crate lives there
    let argv: Vec<String> = env::args().collect();
    let external = argv
        .iter()
        .position(|a| a == "--path")
        .and_then(|i| argv.get(i + 1))
        .map(|p| {
            std::path::Path::new(p)
                .canonicalize()
                .unwrap_or_else(|e| panic!("--path {p}: {e}"))
        });
    EXTERNAL.set(external).ok();

    let config = config::read_config();

    match cmd.as_str() {
        "build" | "install" => {
            let plugin = env::args()
                .nth(2)
                .with_context(|| "must provide plugin as second argument")?;
            let plugin_type = metadata::plugin_type(&plugin);

            match cmd.as_str() {
                "build" => build_plugin(&config, &plugin, plugin_type)?,
                "install" => install_plugin(&config, &plugin, plugin_type)?,
                _ => {}
            }
        }
        "list-plugins" => {
            let plugins = metadata::list_plugins()?;
            println!("Available Plugins:");
            for plugin in plugins {
                println!(" - {}", plugin);
            }
        }
        _ => {
            return Err(anyhow::anyhow!("command must be 'build'"));
        }
    }

    Ok(())
}
