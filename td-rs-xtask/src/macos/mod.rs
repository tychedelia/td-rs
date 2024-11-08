use crate::config::Config;
use crate::metadata::PluginType;
use crate::util::ToTitleCase;
use crate::{build, PLUGIN_HOME};
use anyhow::Context;
use fs_extra::dir::CopyOptions;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) fn install_plugin(
    config: &Config,
    plugin: &str,
    _plugin_type: PluginType,
) -> anyhow::Result<()> {
    let plugin = &plugin.replace('-', "_");
    let plugin_target_path = plugin_target_path(plugin).join(format!("{plugin}.plugin"));
    let td_plugin_folder = &config.macos.plugin_folder;
    std::fs::create_dir_all(&td_plugin_folder).context("Could not create plugin directory")?;
    println!(
        "Installing plugin {:?} to {}",
        plugin_target_path, td_plugin_folder
    );
    fs_extra::dir::copy(
        &plugin_target_path,
        td_plugin_folder,
        &CopyOptions::new().overwrite(true),
    )
    .context("Could not move plugin to TouchDesigner plugin directory")?;
    Ok(())
}

pub(crate) fn build_plugin(
    config: &Config,
    plugin: &str,
    plugin_type: PluginType,
) -> anyhow::Result<()> {
    let target = if cfg!(target_arch = "x86_64") {
        "x86_64-apple-darwin"
    } else {
        "aarch64-apple-darwin"
    };
    build(
        &[plugin, plugin_type.to_plugin_name()],
        &["--release", &format!("--target={target}")],
    )?;

    let is_python_enabled = crate::metadata::is_python_enabled(plugin, &plugin_type);
    let plugin = &plugin.replace('-', "_");
    let path = pbxproj_path(plugin);

    println!("Writing xcode project to {:?}", path);
    write_xcodeproj(target, plugin, &plugin_type, &path)?;
    println!("Building xcode project");
    build_xcode(config, plugin, is_python_enabled)?;
    println!("Moving plugin to {:?}", PLUGIN_HOME);
    move_plugin(plugin, &path)?;
    Ok(())
}

fn move_plugin(plugin: &str, path: &PathBuf) -> anyhow::Result<()> {
    fs_extra::dir::remove(path.parent().unwrap())
        .context("Could not remove xcode project directory")?;
    let plugin_build_path = format!("build/Release/{plugin}.plugin");
    let plugin_target_path = plugin_target_path(plugin);
    std::fs::create_dir_all(&plugin_target_path).context("Could not create plugin directory")?;
    fs_extra::dir::remove(plugin_target_path.join(format!("{plugin}.plugin")))
        .context("Could not remove plugin directory")?;
    fs_extra::dir::move_dir(plugin_build_path, &plugin_target_path, &CopyOptions::new())
        .context("Could not move plugin to target directory")?;
    Ok(())
}

fn plugin_target_path(plugin: &str) -> PathBuf {
    let plugin_target_path = Path::new(PLUGIN_HOME).join(plugin);
    plugin_target_path
}

fn pbxproj_path(plugin: &str) -> PathBuf {
    let mut path = PathBuf::from(format!("{plugin}.xcodeproj"));
    path.push("project.pbxproj");
    path
}

fn build_xcode(config: &Config, plugin: &str, is_python_enabled: bool) -> anyhow::Result<()> {
    let mut cmd = Command::new("xcodebuild")
        .arg("-project")
        .arg(format!("./{plugin}.xcodeproj"))
        .arg("clean")
        .arg("build")
        .arg(format!(
            "PYTHON_INCLUDE_DIR={}",
            config.macos.python_include_dir
        ))
        .arg(if is_python_enabled {
            "EXTRA_CFLAGS=-DPYTHON_ENABLED"
        } else {
            "FOO=BAR"
        })
        .spawn()
        .expect("ls command failed to start");
    if !cmd.wait()?.success() {
        anyhow::bail!("Could not build xcode project");
    }
    Ok(())
}

fn write_xcodeproj(
    target: &str,
    plugin: &str,
    plugin_type: &PluginType,
    path: &PathBuf,
) -> anyhow::Result<()> {
    std::fs::create_dir_all(path.parent().unwrap())
        .context("Could not create xcode project directory")?;
    let short_title = plugin_type.to_short_name().to_title_case();
    let short_upper = plugin_type.to_short_name().to_uppercase();
    let op_path = plugin_type.to_plugin_name();

    let project = std::fs::read_to_string("td-rs-xtask/xcode/project.pbxproj")
        .expect("Could not read xcode project")
        .replace("{{ LIB_NAME }}", &format!("lib{plugin}.a"))
        .replace(
            "{{ LIB_PATH }}",
            &format!("target/{}/release/lib{plugin}.a", target),
        )
        .replace("{{ PLUGIN_FILE_NAME }}", &format!("{plugin}.plugin"))
        .replace("{{ PLUGIN_NAME }}", &plugin)
        .replace("{{ PLUGIN_PRODUCT_NAME }}", &plugin)
        .replace(
            "{{ TD_OP_H_PATH }}",
            &format!("{}/src/{}_CPlusPlusBase.h", op_path, short_upper),
        )
        .replace(
            "{{ TD_OP_H_NAME }}",
            &format!("{}_CPlusPlusBase.h", short_upper),
        )
        .replace(
            "{{ PLUGIN_CPP_NAME }}",
            &format!("Rust{}Plugin.cpp", short_title),
        )
        .replace(
            "{{ PLUGIN_CPP_PATH }}",
            &format!("{}/src/Rust{}Plugin.cpp", op_path, short_title),
        )
        .replace(
            "{{ PLUGIN_H_NAME }}",
            &format!("Rust{}Plugin.h", short_title),
        )
        .replace(
            "{{ PLUGIN_H_PATH }}",
            &format!("{}/src/Rust{}Plugin.h", op_path, short_title),
        )
        .replace(
            "{{ OP_LIB_NAME }}",
            &format!("lib{}.a", op_path.replace("-", "_")),
        )
        .replace(
            "{{ OP_LIB_PATH }}",
            &format!(
                "target/{}/release/lib{}.a",
                target,
                op_path.replace("-", "_")
            ),
        );
    std::fs::write(path, project)?;
    Ok(())
}
