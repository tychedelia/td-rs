use crate::build;
use anyhow::Context;
use fs_extra::dir::CopyOptions;
use plist::Value;
use std::fmt::format;
use std::path::{Path, PathBuf};
use std::process::Command;

const PLUGIN_HOME: &'static str = "target/plugin";

pub(crate) fn build_plugin(plugin: &str) -> anyhow::Result<()> {
    let target = "aarch64-apple-darwin";
    let path = pbxproj_path(plugin);
    build(
        &[plugin.to_string(), "td-rs-chop".to_string()],
        &["--release".to_string(), format!("--target={target}")],
    )?;
    println!("Writing xcode project to {:?}", path);

    write_sop_xcodeproj(&target, &plugin, &path)?;


    println!("Building xcode project");
    build_xcode(&plugin)?;
    println!("Moving plugin to {:?}", PLUGIN_HOME);
    move_plugin(plugin, &path)?;
    Ok(())
}

fn move_plugin(plugin: &str, path: &PathBuf) -> anyhow::Result<()> {
    // fs_extra::dir::remove(&path.parent().unwrap())
    //     .context("Could not remove xcode project directory")?;
    let plugin_build_path = format!("build/Release/{plugin}.plugin");
    let plugin_target_path = Path::new(PLUGIN_HOME).join(plugin);
    std::fs::create_dir_all(&plugin_target_path).context("Could not create plugin directory")?;
    fs_extra::dir::remove(&plugin_target_path.join(format!("{plugin}.plugin")))
        .context("Could not remove plugin directory")?;
    fs_extra::dir::move_dir(&plugin_build_path, &plugin_target_path, &CopyOptions::new())
        .context("Could not move plugin to target directory")?;
    Ok(())
}

fn pbxproj_path(plugin: &str) -> PathBuf {
    let mut path = PathBuf::from(format!("{plugin}.xcodeproj"));
    path.push("project.pbxproj");
    path
}

fn build_xcode(plugin: &str) -> anyhow::Result<()> {
    let mut cmd = Command::new("xcodebuild")
        .arg("-project")
        .arg(format!("./{plugin}.xcodeproj"))
        .arg("clean")
        .arg("build")
        .spawn()
        .expect("ls command failed to start");
    cmd.wait().unwrap();
    Ok(())
}

fn write_chop_xcodeproj(target: &str, plugin: &str, path: &PathBuf) -> anyhow::Result<()> {
    const LIB_KEY: &'static str = "9E4ACB8B299AC54200A2B1CE";
    const BUNDLE_KEY: &'static str = "E23329D61DF092AD0002B4FE";
    const BUNDLE_CONFIGURATION_KEY: &'static str = "E23329D51DF092AD0002B4FE";

    std::fs::create_dir_all(&path.parent().unwrap())
        .context("Could not create xcode project directory")?;
    let mut project = Value::from_file("td-rs-xtask/xcode/chop/project.pbxproj")
        .expect("Could not read xcode project");
    let mut p = project.as_dictionary_mut().unwrap();
    let mut objs = p.get_mut("objects").unwrap().as_dictionary_mut().unwrap();
    let mut lib = objs.get_mut(LIB_KEY).unwrap().as_dictionary_mut().unwrap();
    lib.insert(
        "name".to_string(),
        Value::String(format!("lib{}.a", plugin)),
    );
    lib.insert(
        "path".to_string(),
        Value::String(format!("target/{target}/release/lib{plugin}.a")),
    );
    let mut bundle = objs.get_mut(BUNDLE_KEY).unwrap().as_dictionary_mut().unwrap();
    bundle.insert(
        "name".to_string(),
        Value::String(format!("{plugin}.plugin")),
    );
    let mut bundle_config =  objs.get_mut(BUNDLE_CONFIGURATION_KEY).unwrap().as_dictionary_mut().unwrap();
    bundle_config.insert(
        "name".to_string(),
        Value::String(format!("{plugin}")),
    );
    bundle_config.insert(
        "productName".to_string(),
        Value::String(format!("{plugin}")),
    );
    project.to_file_xml(path)?;
    Ok(())
}

fn write_sop_xcodeproj(target: &str, plugin: &str, path: &PathBuf) -> anyhow::Result<()> {
    const LIB_KEY: &'static str = "9E4ACB8B299AC54200A2B1CE";
    const BUNDLE_KEY: &'static str = "E23329D61DF092AD0002B4FE";
    const BUNDLE_CONFIGURATION_KEY: &'static str = "E23329D51DF092AD0002B4FE";

    std::fs::create_dir_all(&path.parent().unwrap())
        .context("Could not create xcode project directory")?;
    let mut project = Value::from_file("td-rs-xtask/xcode/sop/project.pbxproj")
        .expect("Could not read xcode project");
    let mut p = project.as_dictionary_mut().unwrap();
    let mut objs = p.get_mut("objects").unwrap().as_dictionary_mut().unwrap();
    let mut lib = objs.get_mut(LIB_KEY).unwrap().as_dictionary_mut().unwrap();
    lib.insert(
        "name".to_string(),
        Value::String(format!("lib{}.a", plugin)),
    );
    lib.insert(
        "path".to_string(),
        Value::String(format!("target/{target}/release/lib{plugin}.a")),
    );
    let mut bundle = objs.get_mut(BUNDLE_KEY).unwrap().as_dictionary_mut().unwrap();
    bundle.insert(
        "name".to_string(),
        Value::String(format!("{plugin}.plugin")),
    );
    let mut bundle_config =  objs.get_mut(BUNDLE_CONFIGURATION_KEY).unwrap().as_dictionary_mut().unwrap();
    bundle_config.insert(
        "name".to_string(),
        Value::String(format!("{plugin}")),
    );
    bundle_config.insert(
        "productName".to_string(),
        Value::String(format!("{plugin}")),
    );
    project.to_file_xml(path)?;
    Ok(())
}